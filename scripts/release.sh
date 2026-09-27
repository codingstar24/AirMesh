#!/usr/bin/env bash
set -euo pipefail

echo "==> Building optimized release binaries..."
cargo build --release -p airmesh-daemon -p airmesh-cli

echo "==> Stripping symbols for minimal binary size (<20MB)..."
strip target/release/airmesh-daemon target/release/airmesh-cli || true

ls -lh target/release/airmesh-daemon target/release/airmesh-cli
echo "==> Release build ready in target/release/!"
