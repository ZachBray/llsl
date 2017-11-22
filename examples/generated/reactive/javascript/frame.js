/** This is generated code */


const Frame = () => {

  let lengthSchema = {
    name: "length",
    offsetInBytes: 0,
    bitMask: 0b111111111111111111111111, // 16777215
    shift: 0,
  };

  let buffer;
  let offset = 0;

  return {
    wrap: (buf, newOffset) => {
      buffer = buf;
      offset = newOffset;
    },

    writeLength: (value) => {
      buffer.write(lengthSchema, offset, value);
    },
    readLength: () => {
      return buffer.read(lengthSchema, offset);
    },
  };
};

export default Frame();
