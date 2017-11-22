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
  let offset = 0;

  return {
    wrap: (buf, newOffset) => {
      buffer = buf;
      offset = newOffset;
    },

    writeStreamId: (value) => {
      buffer.write(streamIdSchema, offset, value);
    },
    readStreamId: () => {
      return buffer.read(streamIdSchema, offset);
    },
    writeFrameType: (value) => {
      buffer.write(frameTypeSchema, offset, value);
    },
    readFrameType: () => {
      return buffer.read(frameTypeSchema, offset);
    },
    writeIgnore: (value) => {
      buffer.write(ignoreSchema, offset, value);
    },
    readIgnore: () => {
      return buffer.read(ignoreSchema, offset);
    },
    writeMetadata: (value) => {
      buffer.write(metadataSchema, offset, value);
    },
    readMetadata: () => {
      return buffer.read(metadataSchema, offset);
    },
  };
};

export default FrameHeader();
