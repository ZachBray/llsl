use std::result::Result;
use std::convert::TryFrom;
use llsl_runtime::{BufferAdapter, FieldSchema, RuntimeError, Serializable};
use super::frame_header::FrameHeader;



static LENGTH_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "length",
    offset_in_bytes: 0,
    bit_mask: 0b111111111111111111111111, // 16777215
    shift: 0,
};

static HEADER_SCHEMA: FieldSchema<u32> = FieldSchema {
    name: "header",
    offset_in_bytes: 3,
    bit_mask: 0b0, // 0
    shift: 0,
};


pub struct Frame<'a> {
  buffer: BufferAdapter<'a>
}

impl<'a> Frame<'a> {
    pub fn wrap(buffer: BufferAdapter<'a>) -> Result<Self, RuntimeError> {
        if buffer.len() < 4 {
            Err(RuntimeError::Overflow(
                "Frame",
                4,
                buffer.len()
            ))
        } else {
            Ok(Frame {
                buffer
            })
        }
    }

    pub fn wrap_unchecked(buffer: BufferAdapter<'a>) -> Self {
        Frame {
            buffer
        }
    }

    pub fn get_length(&self) -> u32 {
      u32::read(&self.buffer, &LENGTH_SCHEMA)
    }

    pub fn set_length(&mut self, value: u32) {
      u32::write(&mut self.buffer, &LENGTH_SCHEMA, value)
    }

    pub fn get_header(&mut self) -> FrameHeader {
        let buffer = self.buffer.seek_field(&HEADER_SCHEMA);
        FrameHeader::wrap_unchecked(buffer)
    }
}
