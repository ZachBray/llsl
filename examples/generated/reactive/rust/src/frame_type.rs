use std::result::Result;
use std::convert::{Into, TryFrom};
use llsl_runtime::RuntimeError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FrameType {
    Reserved = 0,
    Setup = 1,
    Lease = 2,
}

impl Into<u32> for FrameType {
    fn into(self) -> u32 {
        match self {
            FrameType::Reserved => 0,
            FrameType::Setup => 1,
            FrameType::Lease => 2,
        }
    }
}

impl TryFrom<u32> for FrameType {
    type Error = RuntimeError;

    fn try_from(value: u32) -> Result<Self, RuntimeError> {
        match value {
             0 => Ok(FrameType::Reserved),
             1 => Ok(FrameType::Setup),
             2 => Ok(FrameType::Lease),
             _ => Err(RuntimeError::InvalidEnumValue("FrameType", value)),
        }
    }
}
