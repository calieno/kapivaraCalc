#!/bin/bash
set -e
echo "=============================================="
echo " Building Rusty HP 49G for Linux (x86_64)    "
echo "=============================================="

cargo build --release --target x86_64-unknown-linux-musl
mkdir -p dist
cp target/x86_64-unknown-linux-musl/release/rusty_hp49 dist/rusty_hp49_linux_x86_64
chmod +x dist/rusty_hp49_linux_x86_64

echo "Build complete: dist/rusty_hp49_linux_x86_64"
