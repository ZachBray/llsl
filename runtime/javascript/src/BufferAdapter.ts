export interface IFieldSchema {
  name: string;
  offsetInBytes: number;
  bitMask: number;
  shift: number;
}

export interface IBufferWrapper {
  wrap(buffer: BufferAdapter, offsetInBytes: number): void;
}

export class BufferAdapter {
  constructor(private view: DataView, private isLittleEndian: boolean = true) {}

  public readBool(schema: IFieldSchema, codecOffsetInBytes: number) {
    return this.readU8(schema, codecOffsetInBytes) !== 0;
  }

  public writeBool(schema: IFieldSchema, codecOffsetInBytes: number, value: boolean) {
    this.writeU8(schema, codecOffsetInBytes, value ? 1 : 0);
  }

  public readU8(schema: IFieldSchema, codecOffsetInBytes: number) {
    const absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
    const data = this.view.getUint8(absoluteOffsetInBytes);
    return (data & schema.bitMask) >> schema.shift;
  }

  public writeU8(schema: IFieldSchema, codecOffsetInBytes: number, value: number) {
    const absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
    const currentData = this.view.getUint8(absoluteOffsetInBytes);
    const newData = (currentData & ~schema.bitMask) | ((value << schema.shift) & schema.bitMask);
    this.view.setUint8(absoluteOffsetInBytes, newData);
  }

  public readU16(schema: IFieldSchema, codecOffsetInBytes: number) {
    const absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
    const data = this.view.getUint16(absoluteOffsetInBytes, this.isLittleEndian);
    return (data & schema.bitMask) >> schema.shift;
  }

  public writeU16(schema: IFieldSchema, codecOffsetInBytes: number, value: number) {
    const absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
    const currentData = this.view.getUint16(absoluteOffsetInBytes, this.isLittleEndian);
    const newData = (currentData & ~schema.bitMask) | ((value << schema.shift) & schema.bitMask);
    this.view.setUint16(absoluteOffsetInBytes, newData, this.isLittleEndian);
  }

  public readU32(schema: IFieldSchema, codecOffsetInBytes: number) {
    const absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
    const data = this.view.getUint32(absoluteOffsetInBytes, this.isLittleEndian);
    return (data & schema.bitMask) >> schema.shift;
  }

  public writeU32(schema: IFieldSchema, codecOffsetInBytes: number, value: number) {
    const absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
    const currentData = this.view.getUint32(absoluteOffsetInBytes, this.isLittleEndian);
    const newData = (currentData & ~schema.bitMask) | ((value << schema.shift) & schema.bitMask);
    this.view.setUint32(absoluteOffsetInBytes, newData, this.isLittleEndian);
  }
}
