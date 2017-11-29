/** This is generated code */

import { BufferAdapter, IBufferWrapper } from "llsl-runtime";
import { FrameHeader } from "./frameHeader";


export declare class Frame implements IBufferWrapper {
  constructor();

  wrap(buffer: BufferAdapter, offsetInBytes: number);

  length: number;

  readonly header: FrameHeader;

}
