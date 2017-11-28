#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(quickcheck_macros))]

extern crate byteorder;
#[cfg(test)]
extern crate quickcheck;
#[macro_use]
extern crate quick_error;

use byteorder::{LE, ByteOrder};

quick_error! {
    #[derive(Debug)]
    pub enum RuntimeError {
        InvalidEnumValue(enum_name: &'static str, value: u32) {
            description("Failed to deserialize enum value.")
            display("Failed to deserialize {} as a {}", value, enum_name)
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldSchema {
    pub name: &'static str,
    pub offset_in_bytes: usize,
    pub bit_mask: usize,
    pub shift: usize,
}

pub struct BufferAdapter<'a> {
    buffer: &'a mut [u8],
}

impl<'a> BufferAdapter<'a> {
    pub fn new(buffer: &'a mut [u8]) -> Self {
        BufferAdapter { buffer }
    }

    pub fn read_bool(&self, schema: &FieldSchema) -> bool {
        self.read_byte(schema) != 0
    }

    pub fn read_byte(&self, schema: &FieldSchema) -> u8 {
        self.read_u16(schema) as u8
    }

    pub fn read_u16(&self, schema: &FieldSchema) -> u16 {
        let stored_value = LE::read_u16(&self.buffer[schema.offset_in_bytes..]);
        let mask = schema.bit_mask as u16;
        let shift = schema.shift as u16;
        (mask & stored_value) >> shift
    }

    pub fn read_u32(&self, schema: &FieldSchema) -> u32 {
        let stored_value = LE::read_u32(&self.buffer[schema.offset_in_bytes..]);
        let mask = schema.bit_mask as u32;
        let shift = schema.shift as u32;
        (mask & stored_value) >> shift
    }

    pub fn read_u64(&self, schema: &FieldSchema) -> u64 {
        let stored_value = LE::read_u64(&self.buffer[schema.offset_in_bytes..]);
        let mask = schema.bit_mask as u64;
        let shift = schema.shift as u64;
        (mask & stored_value) >> shift
    }

    pub fn seek(&mut self, offset_in_bytes: usize) -> BufferAdapter {
        BufferAdapter::new(&mut self.buffer[offset_in_bytes..])
    }

    pub fn seek_field(&mut self, schema: &FieldSchema) -> BufferAdapter {
        self.seek(schema.offset_in_bytes)
    }

    pub fn write_bool(&mut self, schema: &FieldSchema, value: bool) {
        self.write_byte(schema, if value { 1 } else { 0 })
    }

    pub fn write_byte(&mut self, schema: &FieldSchema, value: u8) {
        self.write_u16(schema, value as u16)
    }

    pub fn write_u16(&mut self, schema: &FieldSchema, value: u16) {
        let buffer = &mut self.buffer[schema.offset_in_bytes..];
        let stored_value = LE::read_u16(buffer);
        let mask = schema.bit_mask as u16;
        let shift = schema.shift as u16;
        let stored_value_with_hole = stored_value & !mask;
        let new_stored_value = stored_value_with_hole | ((value << shift) & mask);
        LE::write_u16(buffer, new_stored_value)
    }

    pub fn write_u32(&mut self, schema: &FieldSchema, value: u32) {
        let buffer = &mut self.buffer[schema.offset_in_bytes..];
        let stored_value = LE::read_u32(buffer);
        let mask = schema.bit_mask as u32;
        let shift = schema.shift as u32;
        let stored_value_with_hole = stored_value & !mask;
        let new_stored_value = stored_value_with_hole | ((value << shift) & mask);
        LE::write_u32(buffer, new_stored_value)
    }

    pub fn write_u64(&mut self, schema: &FieldSchema, value: u64) {
        let buffer = &mut self.buffer[schema.offset_in_bytes..];
        let stored_value = LE::read_u64(buffer);
        let mask = schema.bit_mask as u64;
        let shift = schema.shift as u64;
        let stored_value_with_hole = stored_value & !mask;
        let new_stored_value = stored_value_with_hole | ((value << shift) & mask);
        LE::write_u64(buffer, new_stored_value)
    }
}

#[cfg(test)]
mod tests {

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
        schema: FieldSchema,
        old_value: N,
        new_value: N,
    }

    trait GeneratableNumber {
        fn bits() -> usize;

        fn shiftable_bits() -> usize {
            Self::bits()
        }

        fn gen<G: Gen>(g: &mut G, max: usize) -> Self;

        fn read(buffer: &BufferAdapter, schema: &FieldSchema) -> Self;

        fn write(buffer: &mut BufferAdapter, schema: &FieldSchema, value: Self);
    }

    impl<N> Arbitrary for Parameters<N>
    where
        N: GeneratableNumber + Send + Clone + 'static,
    {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let bit_count = g.gen_range(1, N::bits() + 1);
            let mut mask = 0;
            for _ in 0..bit_count {
                mask = (mask << 1) | 1;
            }
            let shift = if bit_count == N::shiftable_bits() {
                0
            } else {
                g.gen_range(0, N::shiftable_bits() - bit_count)
            };
            Parameters {
                schema: FieldSchema {
                    name: "test byte schema",
                    bit_mask: mask << shift,
                    offset_in_bytes: g.gen_range(0, BUFFER_SIZE - (N::bits() / 8 + 1)),
                    shift,
                },
                old_value: N::gen(g, mask),
                new_value: N::gen(g, mask),
            }
        }
    }

    fn round_trip<N>(mut buffer: Buffer, test: Parameters<N>) -> bool
    where
        N: GeneratableNumber + Eq + Copy,
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

    impl GeneratableNumber for bool {
        fn bits() -> usize {
            1
        }

        fn shiftable_bits() -> usize {
            8
        }

        fn gen<G: Gen>(g: &mut G, _: usize) -> Self {
            g.gen()
        }

        // TODO: Structue this differently... we shouldn't need to repeat this here
        fn read(buffer: &BufferAdapter, schema: &FieldSchema) -> Self {
            buffer.read_bool(schema)
        }

        fn write(buffer: &mut BufferAdapter, schema: &FieldSchema, value: Self) {
            buffer.write_bool(schema, value)
        }
    }

    #[quickcheck]
    fn bools_round_trip(buffer: Buffer, test: Parameters<bool>) -> bool {
        round_trip(buffer, test)
    }

    impl GeneratableNumber for u8 {
        fn bits() -> usize {
            8
        }

        fn gen<G: Gen>(g: &mut G, max: usize) -> Self {
            g.gen_range(0, max as u8)
        }

        fn read(buffer: &BufferAdapter, schema: &FieldSchema) -> Self {
            buffer.read_byte(schema)
        }

        fn write(buffer: &mut BufferAdapter, schema: &FieldSchema, value: Self) {
            buffer.write_byte(schema, value)
        }
    }

    #[quickcheck]
    fn bytes_round_trip(buffer: Buffer, test: Parameters<u8>) -> bool {
        round_trip(buffer, test)
    }

    impl GeneratableNumber for u16 {
        fn bits() -> usize {
            16
        }

        fn gen<G: Gen>(g: &mut G, max: usize) -> Self {
            g.gen_range(0, max as u16)
        }

        fn read(buffer: &BufferAdapter, schema: &FieldSchema) -> Self {
            buffer.read_u16(schema)
        }

        fn write(buffer: &mut BufferAdapter, schema: &FieldSchema, value: Self) {
            buffer.write_u16(schema, value)
        }
    }

    #[quickcheck]
    fn u16s_round_trip(buffer: Buffer, test: Parameters<u16>) -> bool {
        round_trip(buffer, test)
    }

    impl GeneratableNumber for u32 {
        fn bits() -> usize {
            32
        }

        fn gen<G: Gen>(g: &mut G, max: usize) -> Self {
            g.gen_range(0, max as u32)
        }

        fn read(buffer: &BufferAdapter, schema: &FieldSchema) -> Self {
            buffer.read_u32(schema)
        }

        fn write(buffer: &mut BufferAdapter, schema: &FieldSchema, value: Self) {
            buffer.write_u32(schema, value)
        }
    }

    #[quickcheck]
    fn u32s_round_trip(buffer: Buffer, test: Parameters<u32>) -> bool {
        round_trip(buffer, test)
    }

    impl GeneratableNumber for u64 {
        fn bits() -> usize {
            64
        }

        fn gen<G: Gen>(g: &mut G, max: usize) -> Self {
            g.gen_range(0, max as u64)
        }

        fn read(buffer: &BufferAdapter, schema: &FieldSchema) -> Self {
            buffer.read_u64(schema)
        }

        fn write(buffer: &mut BufferAdapter, schema: &FieldSchema, value: Self) {
            buffer.write_u64(schema, value)
        }
    }

    #[quickcheck]
    fn u64s_round_trip(buffer: Buffer, test: Parameters<u64>) -> bool {
        round_trip(buffer, test)
    }
}
