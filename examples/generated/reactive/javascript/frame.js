/** This is generated code */
"use strict";
Object.defineProperty(exports, "__esModule", { value: true });

var FrameHeader = require("./frameHeader");


var Frame = exports.Frame = (function () {

  var lengthSchema = {
    name: "length",
    offsetInBytes: 0,
    bitMask: 0b111111111111111111111111, // 16777215
    shift: 0,
  };


  var headerSchema = {
    name: "header",
    offsetInBytes: 3,
    bitMask: 0b0, // 0
    shift: 0,
  };

  var headerCodec = new FrameHeader();



  function Frame() {
    this.buffer = undefined;
    this.codecOffsetInBytes = -1;
  };

  Frame.prototype.wrap = function(newBuffer, newOffsetInBytes) {
    this.buffer = newBuffer;
    this.codecOffsetInBytes = newOffsetInBytes;
  };



  Object.defineProperty(Frame.prototype, "length", {
    enumerable: true,
    get: function() {
      return this.buffer.readU32(lengthSchema, this.codecOffsetInBytes);
    },
    set: function(value) {
      this.buffer.writeU32(lengthSchema, this.codecOffsetInBytes, value);
    },
  });

  Object.defineProperty(Frame.prototype, "header", {
    enumerable: true,
    get: function() {
      var codec = headerCodec;
      var schema = headerSchema;
      codec.wrap(this.buffer, this.codecOffsetInBytes + schema.offsetInBytes);
      return codec;
    },
  });
return Frame;
})();
