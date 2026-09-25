#!/usr/bin/env bash
set -euo pipefail

cargo build --release

if [ -f "target/release/liblo_rust_spellcheck.so" ]; then
  echo "Built shared library: target/release/liblo_rust_spellcheck.so"
fi

if [ -f "target/release/liblo_rust_spellcheck.dylib" ]; then
  echo "Built shared library: target/release/liblo_rust_spellcheck.dylib"
fi

if [ -f "target/release/lo_rust_spellcheck.dll" ]; then
  echo "Built shared library: target/release/lo_rust_spellcheck.dll"
fi
