/** This is generated code */
import FrameType from "./frameType.js";


const FrameHeader = () => {
  let offset = 0;
  return {
    wrap: (newOffset) => {
      offset = newOffset;
    },



    writeIgnore: (value) => {
      // TODO
    },
    readIgnore: () => {
      // TODO
    },


    writeMetadata: (value) => {
      // TODO
    },
    readMetadata: () => {
      // TODO
    },


  };
};

export default FrameHeader();
