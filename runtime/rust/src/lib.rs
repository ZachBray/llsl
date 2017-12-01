#![cfg_attr(test, feature(test))]
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

#[cfg(test)]
extern crate byteorder;
#[cfg(test)]
extern crate num;
#[cfg(test)]
extern crate quickcheck;
#[macro_use]
extern crate quick_error;
#[cfg(test)]
extern crate rand;
#[cfg(test)]
extern crate test;

use std::ops::{BitAnd, BitOr, Shr, Not, Shl};

quick_error! {
    #[derive(Debug)]
    pub enum RuntimeError {
        InvalidEnumValue(enum_name: &'static str, value: u32) {
            description("Failed to deserialize enum value.")
            display("Failed to deserialize {} as a {}", value, enum_name)
        }
        Overflow(codec_name: &'static str, codec_size: usize, buffer_size: usize) {
            description("Not enough space in buffer for codec.")
            display("Not enough space in the buffer (remaining = {} bytes) for {} (min. size = {})",
                    buffer_size, codec_name, codec_size)
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldSchema<M> {
    pub name: &'static str,
    pub offset_in_bytes: usize,
    pub bit_mask: M,
    pub shift: u8,
}

pub struct BufferAdapter<'a> {
    buffer: &'a mut [u8],
}

pub trait Serializable<M> {
    fn read(adapter: &BufferAdapter, schema: &FieldSchema<M>) -> Self;
    fn write(adapter: &mut BufferAdapter, schema: &FieldSchema<M>, value: Self);
}

impl<T> Serializable<T> for T
where
    T: From<u8>,
    T: BitAnd<Output = T>,
    T: BitOr<Output = T>,
    T: Not<Output = T>,
    T: Shl<Self, Output = T>,
    T: Shr<Self, Output = T>,
    T: Copy,
{
    fn read(adapter: &BufferAdapter, schema: &FieldSchema<T>) -> Self {
        let stored_value = unsafe {
            let base_ptr = adapter.buffer.as_ptr();
            let field_ptr = (base_ptr as usize + schema.offset_in_bytes) as *const Self;
            *field_ptr
        };
        let mask = schema.bit_mask;
        let shift = Self::from(schema.shift);
        (mask & stored_value) >> shift
    }

    fn write(adapter: &mut BufferAdapter, schema: &FieldSchema<T>, value: Self) {
        let stored_value = Self::read(&adapter, &schema);
        let mask = schema.bit_mask;
        let shift = Self::from(schema.shift);
        let stored_value_with_hole = stored_value & !mask;
        let new_stored_value = stored_value_with_hole | ((value << shift) & mask);
        unsafe {
            let base_ptr = adapter.buffer.as_ptr();
            let field_ptr = (base_ptr as usize + schema.offset_in_bytes) as *mut Self;
            *field_ptr = new_stored_value
        }
    }
}

impl Serializable<u8> for bool {
    fn read(adapter: &BufferAdapter, schema: &FieldSchema<u8>) -> Self {
        u8::read(adapter, schema) != 0
    }

    fn write(adapter: &mut BufferAdapter, schema: &FieldSchema<u8>, value: Self) {
        let u8_value = if value { 1 } else { 0 };
        u8::write(adapter, schema, u8_value)
    }
}

impl<'a> BufferAdapter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        BufferAdapter { buffer }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn seek(&mut self, offset_in_bytes: usize) -> BufferAdapter {
        BufferAdapter::new(&mut self.buffer[offset_in_bytes..])
    }

    pub fn seek_field<M>(&mut self, schema: &FieldSchema<M>) -> BufferAdapter {
        self.seek(schema.offset_in_bytes)
    }
}

#[cfg(test)]
mod tests {

    use std::u64;
    use std::mem::size_of;
    use std::ops::*;
    use test::Bencher;
    use byteorder::{LE, ByteOrder};
    use num::traits::*;
    use quickcheck::{Arbitrary, Gen};
    use super::*;

    const BUFFER_SIZE: usize = 16;

    #[derive(Clone, Debug)]
    struct Buffer([u8; BUFFER_SIZE]);

    impl Arbitrary for Buffer {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut buffer = [0; BUFFER_SIZE];
            g.fill_bytes(&mut buffer);
            Buffer(buffer)
        }
    }

    #[derive(Clone, Debug)]
    struct Parameters<N> {
        schema: FieldSchema<N>,
        old_value: N,
        new_value: N,
    }

    impl<N> Arbitrary for Parameters<N>
    where
        N: Send,
        N: Copy,
        N: 'static,
        N: Bounded,
        N: Zero,
        N: One,
        N: Into<u64>,
        N: Shl<N, Output = N>,
        N: BitOr<Output = N>,
        N: From<u8>,
        N: PartialOrd,
        N: rand::distributions::range::SampleRange,
    {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let bits = size_of::<N>() as u8 * 8u8;
            let bit_count = g.gen_range(1, bits + 1);
            let mut mask = N::zero();
            for _ in 0..bit_count {
                mask = (mask << N::one()) | N::one();
            }
            let shift = if bit_count == bits {
                0u8
            } else {
                g.gen_range(0u8, bits - bit_count)
            };
            Parameters {
                schema: FieldSchema {
                    name: "test schema",
                    bit_mask: mask << N::from(shift),
                    offset_in_bytes: g.gen_range(0, BUFFER_SIZE - (bits as usize / 8 + 1)),
                    shift: shift as u8,
                },
                old_value: g.gen_range(N::zero(), mask),
                new_value: g.gen_range(N::zero(), mask),
            }
        }
    }

    fn round_trip<N>(mut buffer: Buffer, test: Parameters<N>) -> bool
    where
        N: Eq + Copy + Serializable<N>,
    {
        // Act
        {
            let mut adapter = BufferAdapter::new(&mut buffer.0);
            N::write(&mut adapter, &test.schema, test.old_value);
            N::write(&mut adapter, &test.schema, test.new_value);
        }
        let adapter = BufferAdapter::new(&mut buffer.0);
        let observed = N::read(&adapter, &test.schema);
        // Assert
        observed == test.new_value
    }

    #[quickcheck]
    fn bools_round_trip(mut buffer: Buffer, test: Parameters<u8>) -> bool {
        // Arrange
        let schema = FieldSchema {
            name: test.schema.name,
            bit_mask: 1 << test.schema.shift,
            offset_in_bytes: test.schema.offset_in_bytes,
            shift: test.schema.shift as u8,
        };
        // Act
        {
            let mut adapter = BufferAdapter::new(&mut buffer.0);
            bool::write(&mut adapter, &schema, test.old_value != 0);
            bool::write(&mut adapter, &schema, test.new_value != 0);
        }
        let adapter = BufferAdapter::new(&mut buffer.0);
        let observed = bool::read(&adapter, &test.schema);
        // Assert
        observed == (test.new_value != 0)
    }

    #[quickcheck]
    fn bytes_round_trip(buffer: Buffer, test: Parameters<u8>) -> bool {
        round_trip(buffer, test)
    }

    #[quickcheck]
    fn u16s_round_trip(buffer: Buffer, test: Parameters<u16>) -> bool {
        round_trip(buffer, test)
    }

    #[quickcheck]
    fn u32s_round_trip(buffer: Buffer, test: Parameters<u32>) -> bool {
        round_trip(buffer, test)
    }

    #[quickcheck]
    fn u64s_round_trip(buffer: Buffer, test: Parameters<u64>) -> bool {
        round_trip(buffer, test)
    }

    fn benchmark_1000_u64_read_writes<F, G>(b: &mut Bencher, offset: usize, read: F, write: G)
    where
        F: Fn(&mut BufferAdapter, &FieldSchema<u64>) -> u64,
        G: Fn(&mut BufferAdapter, &FieldSchema<u64>, u64) -> (),
    {
        let mut buffer = [0; 8008];
        let offset_in_bytes = {
            let ptr = &buffer as *const u8;
            (4 - (ptr as usize % 4) + offset) % 4
        };
        let mut adapter = BufferAdapter::new(&mut buffer);
        let mut i = 0;
        let bit_mask = u64::max_value();
        b.iter(|| for j in 0..1000 {
            let schema = FieldSchema {
                name: "test schema",
                bit_mask,
                offset_in_bytes: offset_in_bytes + 8 * j,
                shift: 0,
            };
            write(&mut adapter, &schema, i + 1);
            i = read(&mut adapter, &schema);
        });
    }

    fn read_u64_checked(adapter: &BufferAdapter, schema: &FieldSchema<u64>) -> u64 {
        let stored_value = LE::read_u64(&adapter.buffer[schema.offset_in_bytes..]);
        let mask = schema.bit_mask as u64;
        let shift = schema.shift as u64;
        (mask & stored_value) >> shift
    }

    fn write_u64_checked(adapter: &mut BufferAdapter, schema: &FieldSchema<u64>, value: u64) {
        let buffer = &mut adapter.buffer[schema.offset_in_bytes..];
        let stored_value = LE::read_u64(buffer);
        let mask = schema.bit_mask as u64;
        let shift = schema.shift as u64;
        let stored_value_with_hole = stored_value & !mask;
        let new_stored_value = stored_value_with_hole | ((value << shift) & mask);
        LE::write_u64(buffer, new_stored_value)
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_with_alignment_0(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 0, |buf, f| read_u64_checked(buf, f), |buf, f, i| {
            write_u64_checked(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_with_alignment_1(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 1, |buf, f| read_u64_checked(buf, f), |buf, f, i| {
            write_u64_checked(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_with_alignment_2(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 2, |buf, f| read_u64_checked(buf, f), |buf, f, i| {
            write_u64_checked(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_with_alignment_3(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 3, |buf, f| read_u64_checked(buf, f), |buf, f, i| {
            write_u64_checked(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_unchecked_with_alignment_0(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 0, |buf, f| u64::read(buf, f), |buf, f, i| {
            u64::write(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_unchecked_with_alignment_1(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 1, |buf, f| u64::read(buf, f), |buf, f, i| {
            u64::write(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_unchecked_with_alignment_2(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 2, |buf, f| u64::read(buf, f), |buf, f, i| {
            u64::write(buf, f, i)
        });
    }

    #[bench]
    fn benchmark_1000_u64_read_writes_unchecked_with_alignment_3(b: &mut Bencher) {
        benchmark_1000_u64_read_writes(b, 3, |buf, f| u64::read(buf, f), |buf, f, i| {
            u64::write(buf, f, i)
        });
    }
}
