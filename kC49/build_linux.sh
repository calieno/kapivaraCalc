#!/bin/bash
set -e
echo "=============================================="
echo " Building kapivaraCalc (kC49) for Linux       "
echo "=============================================="

cargo build --release --target x86_64-unknown-linux-musl
mkdir -p dist
cp target/x86_64-unknown-linux-musl/release/kC49 dist/kC49_linux_x86_64
chmod +x dist/kC49_linux_x86_64

echo "Build complete: dist/kC49_linux_x86_64"
