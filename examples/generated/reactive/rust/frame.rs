/** This is generated code */
use std::result::Result;
use llsl_runtime::{BufferAdapter, FieldSchema, RuntimeError};
use super::frame_header::FrameHeader;



static LENGTH_SCHEMA: FieldSchema = FieldSchema {
    name: "length",
    offset_in_bytes: 0,
    bit_mask: 0b111111111111111111111111, // 16777215
    shift: 0,
};

static HEADER_SCHEMA: FieldSchema = FieldSchema {
    name: "header",
    offset_in_bytes: 3,
    bit_mask: 0b0, // 0
    shift: 0,
};


pub struct Frame<'a> {
  buffer: BufferAdapter<'a>
}

impl<'a> Frame<'a> {
    pub fn wrap(buffer: BufferAdapter) -> Self {
        Frame {
            buffer
        }
    }

    pub fn get_length(&self) -> u32 {
      self.buffer.read_u32(&LENGTH_SCHEMA)
    }

    pub fn set_length(&mut self, value: u32) {
      self.buffer.write_u32(&LENGTH_SCHEMA, value)
    }

    pub fn get_header(&mut self) -> FrameHeader {
        let buffer = self.seek(&HEADER_SCHEMA);
        FrameHeader::wrap(buffer)
    }
}
