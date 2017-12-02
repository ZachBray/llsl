use std::result::Result;
use std::convert::TryFrom;
use llsl_runtime::{BufferAdapter, FieldSchema, RuntimeError, Serializable};
use super::frame_type::FrameType;



static STREAM_ID_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "Stream Id",
    offset_in_bytes: 0,
    bit_mask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
};

static FRAME_TYPE_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "Frame Type",
    offset_in_bytes: 4,
    bit_mask: 0b111111, // 63
    shift: 0,
};

static IGNORE_SCHEMA: FieldSchema<u8> = FieldSchema {
    name: "Ignore",
    offset_in_bytes: 4,
    bit_mask: 0b1000000, // 64
    shift: 6,
};

static METADATA_SCHEMA: FieldSchema<u8> = FieldSchema {
    name: "Metadata",
    offset_in_bytes: 4,
    bit_mask: 0b10000000, // 128
    shift: 7,
};

static BLOBBY_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "Blobby",
    offset_in_bytes: 5,
    bit_mask: 0b0, // 0
    shift: 0,
};


pub struct FrameHeader<'a> {
  buffer: BufferAdapter<'a>
}

impl<'a> FrameHeader<'a> {
    pub fn wrap(buffer: BufferAdapter<'a>) -> Result<Self, RuntimeError> {
        if buffer.len() < 5 {
            Err(RuntimeError::Overflow(
                "FrameHeader",
                5,
                buffer.len()
            ))
        } else {
            Ok(FrameHeader {
                buffer
            })
        }
    }

    pub fn wrap_unchecked(buffer: BufferAdapter<'a>) -> Self {
        FrameHeader {
            buffer
        }
    }

    pub fn get_stream_id(&self) -> u32 {
      u32::read(&self.buffer, &STREAM_ID_SCHEMA)
    }

    pub fn set_stream_id(&mut self, value: u32) {
      u32::write(&mut self.buffer, &STREAM_ID_SCHEMA, value)
    }

    pub fn get_frame_type(&self) -> Result<FrameType, RuntimeError> {
        let value = u32::read(&self.buffer, &FRAME_TYPE_SCHEMA);
        FrameType::try_from(value)
    }

    pub fn set_frame_type(&mut self, value: FrameType) {
        let u32_value: u32 = value.into();
        u32::write(&mut self.buffer, &FRAME_TYPE_SCHEMA, u32_value)
    }

    pub fn get_ignore(&self) -> bool {
        bool::read(&self.buffer, &IGNORE_SCHEMA)
    }

    pub fn set_ignore(&mut self, value: bool) {
        bool::write(&mut self.buffer, &IGNORE_SCHEMA, value)
    }

    pub fn get_metadata(&self) -> bool {
        bool::read(&self.buffer, &METADATA_SCHEMA)
    }

    pub fn set_metadata(&mut self, value: bool) {
        bool::write(&mut self.buffer, &METADATA_SCHEMA, value)
    }

    pub fn get_blobby(&mut self) -> BufferAdapter {
        self.buffer.seek_field(&BLOBBY_SCHEMA)
    }
}
