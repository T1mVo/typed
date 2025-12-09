#!/bin/sh

DIR=$(cd "$(dirname "$0")" || exit 1; pwd)

VERSION=$(awk -F'"' '/^version *=/ {print $2}' $DIR/../src-typst/typst.toml)

mkdir -p $DIR/typed/$VERSION

cp -Lr $DIR/../src-typst/** $DIR/typed/$VERSION
