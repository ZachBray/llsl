/** This is generated code */

import { BufferAdapter, IBufferWrapper } from "llsl-runtime";
import { FrameType } from "./frameType";


export declare class FrameHeader implements IBufferWrapper {
  constructor();

  wrap(buffer: BufferAdapter, offsetInBytes: number);

  streamId: number;

  frameType: FrameType;

  ignore: boolean;

  metadata: boolean;

  wrapBlobby(wrapper: IBufferWrapper): void;

}
