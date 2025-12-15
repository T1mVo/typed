#!/bin/sh

set -eu

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

# Clean packaging dir
rm -rf "$DIR/typwire"

# Clean example dir
rm -rf "$DIR/../example/target" "$DIR/../example/src-typst/example.pdf" "$DIR/../example/src-typst/custom_plugin.wasm"

# Clean test dir
rm -rf "$DIR/../test/target" "$DIR/../test/src-typst/test.pdf" "$DIR/../test/src-typst/typwire_test.wasm"
