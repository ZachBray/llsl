/** This is generated code */
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });



var Setup = exports.Setup = (function () {

  var leaseSchema = {
    name: "Lease",
    offsetInBytes: 0,
    bitMask: 0b1, // 1
    shift: 0,
  };


  var majorVersionSchema = {
    name: "Major Version",
    offsetInBytes: 2,
    bitMask: 0b1111111111111111, // 65535
    shift: 0,
  };


  var minorVersionSchema = {
    name: "Minor Version",
    offsetInBytes: 4,
    bitMask: 0b1111111111111111, // 65535
    shift: 0,
  };


  var timeBetweenKEEPALIVEFramesSchema = {
    name: "Time Between KEEPALIVE Frames",
    offsetInBytes: 6,
    bitMask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
  };


  var maxLifetimeSchema = {
    name: "Max Lifetime",
    offsetInBytes: 10,
    bitMask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
  };



  function Setup() {
    this.buffer = undefined;
    this.codecOffsetInBytes = -1;
  };

  Setup.prototype.wrap = function(newBuffer, newOffsetInBytes) {
    this.buffer = newBuffer;
    this.codecOffsetInBytes = newOffsetInBytes;
  };



  Object.defineProperty(Setup.prototype, "lease", {
    enumerable: true,
    get: function() {
      return this.buffer.readBool(leaseSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeBool(leaseSchema, this.codecOffsetInBytes, value);
    },
  });


  Object.defineProperty(Setup.prototype, "majorVersion", {
    enumerable: true,
    get: function() {
      return this.buffer.readU16(majorVersionSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeU16(majorVersionSchema, this.codecOffsetInBytes, value);
    },
  });


  Object.defineProperty(Setup.prototype, "minorVersion", {
    enumerable: true,
    get: function() {
      return this.buffer.readU16(minorVersionSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeU16(minorVersionSchema, this.codecOffsetInBytes, value);
    },
  });


  Object.defineProperty(Setup.prototype, "timeBetweenKEEPALIVEFrames", {
    enumerable: true,
    get: function() {
      return this.buffer.readU32(timeBetweenKEEPALIVEFramesSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeU32(timeBetweenKEEPALIVEFramesSchema, this.codecOffsetInBytes, value);
    },
  });


  Object.defineProperty(Setup.prototype, "maxLifetime", {
    enumerable: true,
    get: function() {
      return this.buffer.readU32(maxLifetimeSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeU32(maxLifetimeSchema, this.codecOffsetInBytes, value);
    },
  });
return Setup;
})();
