#!/bin/bash

RUST_LOG=llsl cargo run -- -i examples/reactive_protocol.yml -o ./examples/generated
