/** This is generated code */


const Setup = () => {

  let leaseSchema = {
    name: "Lease",
    offsetInBytes: 0,
    bitMask: 0b1, // 1
    shift: 0,
  };

  let majorVersionSchema = {
    name: "Major Version",
    offsetInBytes: 2,
    bitMask: 0b1111111111111111, // 65535
    shift: 0,
  };

  let minorVersionSchema = {
    name: "Minor Version",
    offsetInBytes: 4,
    bitMask: 0b1111111111111111, // 65535
    shift: 0,
  };

  let timeBetweenKEEPALIVEFramesSchema = {
    name: "Time Between KEEPALIVE Frames",
    offsetInBytes: 6,
    bitMask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
  };

  let maxLifetimeSchema = {
    name: "Max Lifetime",
    offsetInBytes: 10,
    bitMask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
  };


  let buffer;
  let codecOffsetInBytes = 0;

  return {
    wrap: (newBuffer, newOffsetInBytes) => {
      buffer = newBuffer;
      codecOffsetInBytes = newOffsetInBytes;
    },

    writeLease: (value) => {
      buffer.writeBool(leaseSchema, codecOffsetInBytes, value);
    },

    readLease: () => {
      return buffer.readBool(leaseSchema, codecOffsetInBytes);
    },

    writeMajorVersion: (value) => {
      buffer.writeU16(majorVersionSchema, codecOffsetInBytes, value);
    },

    readMajorVersion: () => {
      return buffer.readU16(majorVersionSchema, codecOffsetInBytes);
    },

    writeMinorVersion: (value) => {
      buffer.writeU16(minorVersionSchema, codecOffsetInBytes, value);
    },

    readMinorVersion: () => {
      return buffer.readU16(minorVersionSchema, codecOffsetInBytes);
    },

    writeTimeBetweenKEEPALIVEFrames: (value) => {
      buffer.writeU32(timeBetweenKEEPALIVEFramesSchema, codecOffsetInBytes, value);
    },

    readTimeBetweenKEEPALIVEFrames: () => {
      return buffer.readU32(timeBetweenKEEPALIVEFramesSchema, codecOffsetInBytes);
    },

    writeMaxLifetime: (value) => {
      buffer.writeU32(maxLifetimeSchema, codecOffsetInBytes, value);
    },

    readMaxLifetime: () => {
      return buffer.readU32(maxLifetimeSchema, codecOffsetInBytes);
    },

  };
};

export default Setup();
