#!/bin/sh

set -eu

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

cargo build --manifest-path "$DIR/../example/Cargo.toml" --target "wasm32-unknown-unknown"
cp "$DIR/../example/target/wasm32-unknown-unknown/debug/typed_example.wasm" "$DIR/../example/src-typst/custom_plugin.wasm"
typst compile --root "$DIR/.." "$DIR/../example/src-typst/example.typ"
