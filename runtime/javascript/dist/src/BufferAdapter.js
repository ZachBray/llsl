"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var BufferAdapter = /** @class */ (function () {
    function BufferAdapter(view, isLittleEndian) {
        if (isLittleEndian === void 0) { isLittleEndian = true; }
        this.view = view;
        this.isLittleEndian = isLittleEndian;
    }
    BufferAdapter.prototype.readBool = function (schema, codecOffsetInBytes) {
        return this.readU8(schema, codecOffsetInBytes) !== 0;
    };
    BufferAdapter.prototype.writeBool = function (schema, codecOffsetInBytes, value) {
        this.writeU8(schema, codecOffsetInBytes, value ? 1 : 0);
    };
    BufferAdapter.prototype.readU8 = function (schema, codecOffsetInBytes) {
        var absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
        var data = this.view.getUint8(absoluteOffsetInBytes);
        return (data & schema.bitMask) >> schema.shift;
    };
    BufferAdapter.prototype.writeU8 = function (schema, codecOffsetInBytes, value) {
        var absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
        var currentData = this.view.getUint8(absoluteOffsetInBytes);
        var newData = (currentData & ~schema.bitMask) | ((value << schema.shift) & schema.bitMask);
        this.view.setUint8(absoluteOffsetInBytes, newData);
    };
    BufferAdapter.prototype.readU16 = function (schema, codecOffsetInBytes) {
        var absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
        var data = this.view.getUint16(absoluteOffsetInBytes, this.isLittleEndian);
        return (data & schema.bitMask) >> schema.shift;
    };
    BufferAdapter.prototype.writeU16 = function (schema, codecOffsetInBytes, value) {
        var absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
        var currentData = this.view.getUint16(absoluteOffsetInBytes, this.isLittleEndian);
        var newData = (currentData & ~schema.bitMask) | ((value << schema.shift) & schema.bitMask);
        this.view.setUint16(absoluteOffsetInBytes, newData, this.isLittleEndian);
    };
    BufferAdapter.prototype.readU32 = function (schema, codecOffsetInBytes) {
        var absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
        var data = this.view.getUint32(absoluteOffsetInBytes, this.isLittleEndian);
        return (data & schema.bitMask) >> schema.shift;
    };
    BufferAdapter.prototype.writeU32 = function (schema, codecOffsetInBytes, value) {
        var absoluteOffsetInBytes = codecOffsetInBytes + schema.offsetInBytes;
        var currentData = this.view.getUint32(absoluteOffsetInBytes, this.isLittleEndian);
        var newData = (currentData & ~schema.bitMask) | ((value << schema.shift) & schema.bitMask);
        this.view.setUint32(absoluteOffsetInBytes, newData, this.isLittleEndian);
    };
    return BufferAdapter;
}());
exports.BufferAdapter = BufferAdapter;
//# sourceMappingURL=BufferAdapter.js.map