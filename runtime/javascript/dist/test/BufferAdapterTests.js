"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var jsc = require("jsverify");
var BufferAdapter_1 = require("../src/BufferAdapter");
describe("BufferAdapter", function () {
    var bufferSize = 64;
    var adapter;
    beforeEach(function () {
        var buffer = new ArrayBuffer(bufferSize);
        var view = new DataView(buffer);
        adapter = new BufferAdapter_1.BufferAdapter(view);
    });
    var codecOffset = jsc.integer(0, bufferSize - 10);
    var fieldOffset = jsc.integer(0, 4);
    var createMask = function (shift, bitCount, maxBits) {
        var mask = 0;
        for (var i = 0; i < bitCount; i++) {
            mask = (mask << 1) | 1;
        }
        var maxMask = (1 << maxBits) - 1;
        return (mask << shift) & maxMask;
    };
    jsc.property("booleans round-trip", codecOffset, fieldOffset, jsc.integer(0, 7), jsc.bool, function (offset, innerOffset, shift, polarity) {
        // Arrange
        var schema = {
            bitMask: createMask(shift, 1, 8),
            name: "Test Schema",
            offsetInBytes: innerOffset,
            shift: shift,
        };
        // Act
        adapter.writeBool(schema, offset, !polarity);
        adapter.writeBool(schema, offset, polarity);
        var observed = adapter.readBool(schema, offset);
        // Assert
        return observed === polarity;
    });
    context("numerics", function () {
        var supportedNumerics = [{
                kind: "u8",
                maxBits: 8,
                value: jsc.uint8,
                read: function () { return adapter.readU8.bind(adapter); },
                write: function () { return adapter.writeU8.bind(adapter); },
            }, {
                kind: "u16",
                maxBits: 16,
                value: jsc.uint16,
                read: function () { return adapter.readU16.bind(adapter); },
                write: function () { return adapter.writeU16.bind(adapter); },
            }, {
                kind: "u32",
                maxBits: 32,
                value: jsc.uint32,
                read: function () { return adapter.readU32.bind(adapter); },
                write: function () { return adapter.writeU32.bind(adapter); },
            }];
        var _loop_1 = function (numeric) {
            jsc.property(numeric.kind + "s round-trip", codecOffset, fieldOffset, jsc.integer(0, numeric.maxBits - 1), jsc.integer(1, numeric.maxBits), jsc.uint8, jsc.uint8, function (offset, innerOffset, shift, bits, a, b) {
                // Arrange
                var schema = {
                    bitMask: createMask(shift, bits, numeric.maxBits),
                    name: "Test Schema",
                    offsetInBytes: innerOffset,
                    shift: shift,
                };
                a = a & (schema.bitMask >> shift);
                b = b & (schema.bitMask >> shift);
                // Act
                numeric.write()(schema, offset, a);
                numeric.write()(schema, offset, b);
                var observed = numeric.read()(schema, offset);
                // Assert
                return observed === b;
            });
        };
        for (var _i = 0, supportedNumerics_1 = supportedNumerics; _i < supportedNumerics_1.length; _i++) {
            var numeric = supportedNumerics_1[_i];
            _loop_1(numeric);
        }
    });
});
//# sourceMappingURL=BufferAdapterTests.js.map