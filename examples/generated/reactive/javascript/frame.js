/** This is generated code */


const Frame = () => {

  let lengthSchema = {
    name: "length",
    offsetInBytes: 0,
    bitMask: 0b111111111111111111111111, // 16777215
    shift: 0,
  };


  let buffer;
  let codecOffsetInBytes = 0;

  return {
    wrap: (newBuffer, newOffsetInBytes) => {
      buffer = newBuffer;
      codecOffsetInBytes = newOffsetInBytes;
    },

    writeLength: (value) => {
      buffer.writeU32(lengthSchema, codecOffsetInBytes, value);
    },

    readLength: () => {
      return buffer.readU32(lengthSchema, codecOffsetInBytes);
    },

  };
};

export default Frame();
