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
  let offset = 0;

  return {
    wrap: (buf, newOffset) => {
      buffer = buf;
      offset = newOffset;
    },

    writeLease: (value) => {
      buffer.write(leaseSchema, offset, value);
    },
    readLease: () => {
      return buffer.read(leaseSchema, offset);
    },
    writeMajorVersion: (value) => {
      buffer.write(majorVersionSchema, offset, value);
    },
    readMajorVersion: () => {
      return buffer.read(majorVersionSchema, offset);
    },
    writeMinorVersion: (value) => {
      buffer.write(minorVersionSchema, offset, value);
    },
    readMinorVersion: () => {
      return buffer.read(minorVersionSchema, offset);
    },
    writeTimeBetweenKEEPALIVEFrames: (value) => {
      buffer.write(timeBetweenKEEPALIVEFramesSchema, offset, value);
    },
    readTimeBetweenKEEPALIVEFrames: () => {
      return buffer.read(timeBetweenKEEPALIVEFramesSchema, offset);
    },
    writeMaxLifetime: (value) => {
      buffer.write(maxLifetimeSchema, offset, value);
    },
    readMaxLifetime: () => {
      return buffer.read(maxLifetimeSchema, offset);
    },
  };
};

export default Setup();
