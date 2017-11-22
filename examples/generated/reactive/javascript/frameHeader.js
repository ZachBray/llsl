/** This is generated code */
import FrameType from "./frameType.js";


const FrameHeader = () => {

  let streamIdSchema = {
    name: "Stream Id",
    offsetInBytes: 0,
    bitMask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
  };

  let frameTypeSchema = {
    name: "Frame Type",
    offsetInBytes: 4,
    bitMask: 0b111111, // 63
    shift: 0,
  };

  let ignoreSchema = {
    name: "Ignore",
    offsetInBytes: 4,
    bitMask: 0b1000000, // 64
    shift: 6,
  };

  let metadataSchema = {
    name: "Metadata",
    offsetInBytes: 4,
    bitMask: 0b10000000, // 128
    shift: 7,
  };


  let buffer;
  let codecOffsetInBytes = 0;

  return {
    wrap: (newBuffer, newOffsetInBytes) => {
      buffer = newBuffer;
      codecOffsetInBytes = newOffsetInBytes;
    },

    writeStreamId: (value) => {
      buffer.writeU32(streamIdSchema, codecOffsetInBytes, value);
    },

    readStreamId: () => {
      return buffer.readU32(streamIdSchema, codecOffsetInBytes);
    },

    writeFrameType: (value) => {
      buffer.writeEnum(frameTypeSchema, codecOffsetInBytes, value);
    },

    readFrameType: () => {
      return buffer.readEnum(frameTypeSchema, codecOffsetInBytes);
    },

    writeIgnore: (value) => {
      buffer.writeBool(ignoreSchema, codecOffsetInBytes, value);
    },

    readIgnore: () => {
      return buffer.readBool(ignoreSchema, codecOffsetInBytes);
    },

    writeMetadata: (value) => {
      buffer.writeBool(metadataSchema, codecOffsetInBytes, value);
    },

    readMetadata: () => {
      return buffer.readBool(metadataSchema, codecOffsetInBytes);
    },

  };
};

export default FrameHeader();
