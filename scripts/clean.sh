#!/bin/sh

set -eu

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

# Clean packaging dir
rm -rf "$DIR/typed"

# Clean example dir
rm -rf "$DIR/../example/target" "$DIR/../example/src-typst/example.pdf" "$DIR/../example/src-typst/custom_plugin.wasm"
