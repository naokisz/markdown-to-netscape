#!/usr/bin/env bash
set -euo pipefail

echo "Building aarch64-apple-darwin release binary..."

# Ensure toolchain
if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup not found; please install Rust toolchain first: https://rustup.rs/"
  exit 1
fi

rustup target add aarch64-apple-darwin

cargo build --release --target aarch64-apple-darwin

OUT=target/aarch64-apple-darwin/release/markdown-to-netscape

if [ -f "$OUT" ]; then
  echo "Built: $OUT"
  echo "You can copy this binary to an aarch64 mac and run it:"
  echo "  scp $OUT user@apple-mac:~/bin/"
else
  echo "Build failed: $OUT not found"
  exit 2
fi
