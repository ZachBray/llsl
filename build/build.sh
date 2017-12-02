#!/bin/bash
cd "$(dirname "$0")/.."
(cd runtime/rust && RUST_LOG=llsl=cargo test)
(cd tool && RUST_LOG=llsl=debug cargo run -- -i ../examples/reactive/reactive_protocol.yml)
(cd examples/reactive/src/rust/contract && RUST_LOG=llsl=debug cargo test)
