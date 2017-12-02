#!/bin/bash

# BUILD TOOLING
docker build -t llsl-build-image build

# RUN BUILD
docker run -it --rm \
  -v $(pwd):/usr/workspace \
  llsl-build-image
