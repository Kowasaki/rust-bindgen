#!/usr/bin/env bash

# Usage:
#
#     ./tests/test-one.sh <fuzzy-name>
#
# Generate bindings for the first match of `./tests/headers/*<fuzzy-name>*`, use
# `rustc` to compile the bindings with unit tests enabled, and run the generated
# layout tests.

set -eux

cd $(dirname $0)
cd ..

export RUST_BACKTRACE=1

# Grab the first match
TEST=$(find ./tests/headers -type f -iname "*$1*" | head -n 1)

BINDINGS=$(mktemp -t bindings.rs.XXXXXX)
TEST_BINDINGS_BINARY=$(mktemp -t bindings.XXXXXX)

FLAGS="$(grep "// bindgen-flags: " "$TEST")"
FLAGS="${FLAGS/\/\/ bindgen\-flags:/}"

eval ./target/debug/bindgen \
    "\"$TEST\"" \
    --emit-ir \
    --emit-ir-graphviz ir.dot \
    --emit-clang-ast \
    -o "\"$BINDINGS\"" \
    $FLAGS

dot -Tpng ir.dot -o ir.png

echo "=== Input header ========================================================"
cat "$TEST"

echo "=== Generated bindings =================================================="
cat "$BINDINGS"

echo "=== Building bindings ==================================================="
rustc --test -o "$TEST_BINDINGS_BINARY" "$BINDINGS" --crate-name bindgen_test_one

echo "=== Testing bindings ===================================================="
"$TEST_BINDINGS_BINARY"
