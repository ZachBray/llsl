/** This is generated code */
import { BufferAdapter, IFieldSchema, IBufferWrapper } from "llsl-runtime";
import { FrameType } from "./frameType";


declare export class FrameHeader implements IBufferWrapper {
  constructor();

  streamId: number;

  frameType: FrameType;

  ignore: boolean;

  metadata: boolean;

}
