#!/bin/bash

cd tool
RUST_LOG=llsl=debug cargo run -- -i ../examples/reactive/reactive_protocol.yml
cd ..
