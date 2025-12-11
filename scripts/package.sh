#!/bin/sh

set -eu

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

VERSION=$(awk -F'"' '/^version *=/ {print $2}' $DIR/../src-typst/typst.toml)

mkdir -p "$DIR/typed/$VERSION"

cp -Lr $DIR/../src-typst/** $DIR/typed/$VERSION

if sed --version >/dev/null 2>&1; then
    # GNU sed (Linux)
    sed -i 's/\[ \]/❌/g; s/\[[xX]\]/✅/g' "$DIR/typed/$VERSION/README.md"
else
    # BSD sed (macOS)
    sed -i '' 's/\[ \]/❌/g; s/\[[xX]\]/✅/g' "$DIR/typed/$VERSION/README.md"
fi
