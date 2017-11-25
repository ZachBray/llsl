/** This is generated code */

import { IBufferWrapper } from "llsl-runtime";
import { FrameType } from "./frameType";


export declare class FrameHeader implements IBufferWrapper {
  constructor();

  streamId: number;

  frameType: FrameType;

  ignore: boolean;

  metadata: boolean;

  wrapBlobby(wrapper: IBufferWrapper): void;

}
