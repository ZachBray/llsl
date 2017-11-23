/** This is generated code */
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });

var FrameType = require("./frameType.js");


var FrameHeader = (function () {

  var streamIdSchema = {
    name: "Stream Id",
    offsetInBytes: 0,
    bitMask: 0b11111111111111111111111111111110, // 4294967294
    shift: 0,
  };

  var frameTypeSchema = {
    name: "Frame Type",
    offsetInBytes: 4,
    bitMask: 0b111111, // 63
    shift: 0,
  };

  var ignoreSchema = {
    name: "Ignore",
    offsetInBytes: 4,
    bitMask: 0b1000000, // 64
    shift: 6,
  };

  var metadataSchema = {
    name: "Metadata",
    offsetInBytes: 4,
    bitMask: 0b10000000, // 128
    shift: 7,
  };


  function FrameHeader() {
    this.buffer = undefined;
    this.codecOffsetInBytes = -1;
  };

  FrameHeader.prototype.wrap = function(newBuffer, newOffsetInBytes) {
    this.buffer = newBuffer;
    this.codecOffsetInBytes = newOffsetInBytes;
  };


  Object.defineProperty(FrameHeader.prototype, "streamId", {
    enumerable: true,
    get: function() {
      return this.buffer.readU32(streamIdSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeU32(streamIdSchema, this.codecOffsetInBytes, value);
    }
  });

  Object.defineProperty(FrameHeader.prototype, "frameType", {
    enumerable: true,
    get: function() {
      return this.buffer.readEnum(frameTypeSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeEnum(frameTypeSchema, this.codecOffsetInBytes, value);
    }
  });

  Object.defineProperty(FrameHeader.prototype, "ignore", {
    enumerable: true,
    get: function() {
      return this.buffer.readBool(ignoreSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeBool(ignoreSchema, this.codecOffsetInBytes, value);
    }
  });

  Object.defineProperty(FrameHeader.prototype, "metadata", {
    enumerable: true,
    get: function() {
      return this.buffer.readBool(metadataSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeBool(metadataSchema, this.codecOffsetInBytes, value);
    }
  });

})();

exports.FrameHeader = FrameHeader;
