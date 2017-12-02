#!/bin/bash

runtimes=(    \
  rust        \
  javascript
)

for runtime in ${runtimes[*]}
do

# BUILD TOOLING
  echo "Building tooling image for $runtime"
  docker build -t llsl-build-$runtime-image build/$runtime \
  && echo "Finished building tooling for $runtime"

# RUN BUILD
  echo "Running build for $runtime"
  docker run -it --rm \
    -v $(pwd):/usr/workspace \
    llsl-build-$runtime-image \
  && echo "Finished running build for $runtime"
done
