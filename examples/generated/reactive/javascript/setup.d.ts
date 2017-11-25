/** This is generated code */

import { BufferAdapter, IBufferWrapper } from "llsl-runtime";


export declare class Setup implements IBufferWrapper {
  constructor();

  wrap(buffer: BufferAdapter, offsetInBytes: number);

  lease: boolean;

  majorVersion: number;

  minorVersion: number;

  timeBetweenKEEPALIVEFrames: number;

  maxLifetime: number;

}
