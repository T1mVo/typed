#!/bin/sh

set -eu

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

cargo build --manifest-path "$DIR/../test/Cargo.toml" --target "wasm32-unknown-unknown"
cp "$DIR/../test/target/wasm32-unknown-unknown/debug/typwire_test.wasm" "$DIR/../test/src-typst/typwire_test.wasm"
typst compile --root "$DIR/.." "$DIR/../test/src-typst/test.typ"
