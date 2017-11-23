import * as jsc from "jsverify";
import { BufferAdapter, IFieldSchema } from "../src/BufferAdapter";

describe("BufferAdapter", () => {
  const bufferSize = 64;
  let adapter: BufferAdapter;

  beforeEach(() => {
    const buffer = new ArrayBuffer(bufferSize);
    const view = new DataView(buffer);
    adapter = new BufferAdapter(view);
  });

  const codecOffset = jsc.integer(0, bufferSize - 10);
  const fieldOffset = jsc.integer(0, 4);

  const createMask = (shift: number, bitCount: number, maxBits: number) => {
    let mask = 0;
    for (let i = 0; i < bitCount; i++) {
      mask = (mask << 1) | 1;
    }
    const maxMask = (1 << maxBits) - 1;
    return (mask << shift) & maxMask;
  };

  jsc.property("booleans round-trip",
    codecOffset,
    fieldOffset,
    jsc.integer(0, 7),
    jsc.bool,
    (offset, innerOffset, shift, polarity) => {
      // Arrange
      const schema: IFieldSchema = {
        bitMask: createMask(shift, 1, 8),
        name: "Test Schema",
        offsetInBytes: innerOffset,
        shift,
      };
      // Act
      adapter.writeBool(schema, offset, !polarity);
      adapter.writeBool(schema, offset, polarity);
      const observed = adapter.readBool(schema, offset);
      // Assert
      return observed === polarity;
  });

  context("numerics", () => {

    const supportedNumerics = [{
      kind: "byte",
      maxBits: 8,
      value: jsc.uint8,
      read: () => adapter.readByte.bind(adapter),
      write: () => adapter.writeByte.bind(adapter),
    }, {
      kind: "u16",
      maxBits: 16,
      value: jsc.uint16,
      read: () => adapter.readU16.bind(adapter),
      write: () => adapter.writeU16.bind(adapter),
    }, {
      kind: "u32",
      maxBits: 32,
      value: jsc.uint32,
      read: () => adapter.readU32.bind(adapter),
      write: () => adapter.writeU32.bind(adapter),
    }];

    for (const numeric of supportedNumerics) {
      jsc.property(`${numeric.kind}s round-trip`,
        codecOffset,
        fieldOffset,
        jsc.integer(0, numeric.maxBits - 1),
        jsc.integer(1, numeric.maxBits),
        jsc.uint8,
        jsc.uint8,
        (offset, innerOffset, shift, bits, a, b) => {
          // Arrange
          const schema: IFieldSchema = {
            bitMask: createMask(shift, bits, numeric.maxBits),
            name: "Test Schema",
            offsetInBytes: innerOffset,
            shift,
          };
          a = a & (schema.bitMask >> shift);
          b = b & (schema.bitMask >> shift);
          // Act
          numeric.write()(schema, offset, a);
          numeric.write()(schema, offset, b);
          const observed = numeric.read()(schema, offset);
          // Assert
          return observed === b;
      });
    }
  });
});
