#!/bin/bash -Cex

# 'error' | 'warn' | 'info' | 'debug' | 'trace'
export LOG_LEVEL="${1}"

export LIBRARY_NAME='zinc-compiler'
export LIBRARY_NAME_LOG='zinc_compiler'
export BINARY_NAME='znc'
export BUILD_MODE='debug'
export RUST_LOG="${LIBRARY_NAME_LOG}=${LOG_LEVEL},${BINARY_NAME}=${LOG_LEVEL}"
export RUST_BACKTRACE=1

cargo fmt --package "${LIBRARY_NAME}"
cargo build --package "${LIBRARY_NAME}"
cargo test --package "${LIBRARY_NAME}"

"./target/${BUILD_MODE}/${BINARY_NAME}" --output "${2}" "${@:3}"

cargo run --bin zinc -- -v exec --circuit "${2}"
