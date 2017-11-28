/** This is generated code */
use std::result::Result;
use llsl_runtime::{BufferAdapter, FieldSchema, RuntimeError};
use super::frame_type::FrameType;



static STREAM_ID_SCHEMA: FieldSchema = FieldSchema {
    name: "Stream Id",
    offset_in_bytes: 0,
    bit_mask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
};

static FRAME_TYPE_SCHEMA: FieldSchema = FieldSchema {
    name: "Frame Type",
    offset_in_bytes: 4,
    bit_mask: 0b111111, // 63
    shift: 0,
};

static IGNORE_SCHEMA: FieldSchema = FieldSchema {
    name: "Ignore",
    offset_in_bytes: 4,
    bit_mask: 0b1000000, // 64
    shift: 6,
};

static METADATA_SCHEMA: FieldSchema = FieldSchema {
    name: "Metadata",
    offset_in_bytes: 4,
    bit_mask: 0b10000000, // 128
    shift: 7,
};

static BLOBBY_SCHEMA: FieldSchema = FieldSchema {
    name: "Blobby",
    offset_in_bytes: 5,
    bit_mask: 0b0, // 0
    shift: 0,
};


pub struct FrameHeader<'a> {
  buffer: BufferAdapter<'a>
}

impl<'a> FrameHeader<'a> {
    pub fn wrap(buffer: BufferAdapter) -> Self {
        FrameHeader {
            buffer
        }
    }

    pub fn get_stream_id(&self) -> u32 {
      self.buffer.read_u32(&STREAM_ID_SCHEMA)
    }

    pub fn set_stream_id(&mut self, value: u32) {
      self.buffer.write_u32(&STREAM_ID_SCHEMA, value)
    }

    pub fn get_frame_type(&self) -> Result<FrameType, RuntimeError> {
        let value = self.buffer.read_u32(&FRAME_TYPE_SCHEMA);
        FrameType::try_from(value)
    }

    pub fn set_frame_type(&mut self, value: FrameType) {
        let u32_value: u32 = value.into();
        self.buffer.write_u32(&FRAME_TYPE_SCHEMA, u32_value)
    }

    pub fn get_ignore(&self) -> bool {
        self.buffer.read_bool(&IGNORE_SCHEMA)
    }

    pub fn set_ignore(&mut self, value: bool) {
        self.buffer.write_bool(&IGNORE_SCHEMA, value)
    }

    pub fn get_metadata(&self) -> bool {
        self.buffer.read_bool(&METADATA_SCHEMA)
    }

    pub fn set_metadata(&mut self, value: bool) {
        self.buffer.write_bool(&METADATA_SCHEMA, value)
    }

    pub fn get_blobby(&mut self) -> BufferAdapter {
        self.seek(&BLOBBY_SCHEMA)
    }
}
