use std::result::Result;
use std::convert::TryFrom;
use llsl_runtime::{BufferAdapter, FieldSchema, RuntimeError, Serializable};



static LEASE_SCHEMA: FieldSchema<u8> = FieldSchema {
    name: "Lease",
    offset_in_bytes: 0,
    bit_mask: 0b1, // 1
    shift: 0,
};

static MAJOR_VERSION_SCHEMA: FieldSchema<u16> = FieldSchema {
    name: "Major Version",
    offset_in_bytes: 2,
    bit_mask: 0b1111111111111111, // 65535
    shift: 0,
};

static MINOR_VERSION_SCHEMA: FieldSchema<u16> = FieldSchema {
    name: "Minor Version",
    offset_in_bytes: 4,
    bit_mask: 0b1111111111111111, // 65535
    shift: 0,
};

static TIME_BETWEEN_KEEPALIVE_FRAMES_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "Time Between KEEPALIVE Frames",
    offset_in_bytes: 6,
    bit_mask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
};

static MAX_LIFETIME_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "Max Lifetime",
    offset_in_bytes: 10,
    bit_mask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
};


pub struct Setup<'a> {
  buffer: BufferAdapter<'a>
}

impl<'a> Setup<'a> {
    pub fn wrap(buffer: BufferAdapter<'a>) -> Result<Self, RuntimeError> {
        if buffer.len() < 14 {
            Err(RuntimeError::Overflow(
                "Setup",
                14,
                buffer.len()
            ))
        } else {
            Ok(Setup {
                buffer
            })
        }
    }

    pub fn wrap_unchecked(buffer: BufferAdapter<'a>) -> Self {
        Setup {
            buffer
        }
    }

    pub fn get_lease(&self) -> bool {
        bool::read(&self.buffer, &LEASE_SCHEMA)
    }

    pub fn set_lease(&mut self, value: bool) {
        bool::write(&mut self.buffer, &LEASE_SCHEMA, value)
    }

    pub fn get_major_version(&self) -> u16 {
        u16::read(&self.buffer, &MAJOR_VERSION_SCHEMA)
    }

    pub fn set_major_version(&mut self, value: u16) {
        u16::write(&mut self.buffer, &MAJOR_VERSION_SCHEMA, value)
    }

    pub fn get_minor_version(&self) -> u16 {
        u16::read(&self.buffer, &MINOR_VERSION_SCHEMA)
    }

    pub fn set_minor_version(&mut self, value: u16) {
        u16::write(&mut self.buffer, &MINOR_VERSION_SCHEMA, value)
    }

    pub fn get_time_between_keepalive_frames(&self) -> u32 {
      u32::read(&self.buffer, &TIME_BETWEEN_KEEPALIVE_FRAMES_SCHEMA)
    }

    pub fn set_time_between_keepalive_frames(&mut self, value: u32) {
      u32::write(&mut self.buffer, &TIME_BETWEEN_KEEPALIVE_FRAMES_SCHEMA, value)
    }

    pub fn get_max_lifetime(&self) -> u32 {
      u32::read(&self.buffer, &MAX_LIFETIME_SCHEMA)
    }

    pub fn set_max_lifetime(&mut self, value: u32) {
      u32::write(&mut self.buffer, &MAX_LIFETIME_SCHEMA, value)
    }
}
