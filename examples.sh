#!/bin/bash

cd tool
RUST_LOG=llsl=debug cargo run -- -i ../examples/reactive_protocol.yml -o ../examples/generated/reactive/
cd ..
