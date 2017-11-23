export interface IFieldSchema {
    name: string;
    offsetInBytes: number;
    bitMask: number;
    shift: number;
}
export declare class BufferAdapter {
    private view;
    private isLittleEndian;
    constructor(view: DataView, isLittleEndian?: boolean);
    readBool(schema: IFieldSchema, codecOffsetInBytes: number): boolean;
    writeBool(schema: IFieldSchema, codecOffsetInBytes: number, value: boolean): void;
    readByte(schema: IFieldSchema, codecOffsetInBytes: number): number;
    writeByte(schema: IFieldSchema, codecOffsetInBytes: number, value: number): void;
    readU16(schema: IFieldSchema, codecOffsetInBytes: number): number;
    writeU16(schema: IFieldSchema, codecOffsetInBytes: number, value: number): void;
    readU32(schema: IFieldSchema, codecOffsetInBytes: number): number;
    writeU32(schema: IFieldSchema, codecOffsetInBytes: number, value: number): void;
}
