#!/usr/bin/env bash
set -euo pipefail

echo "==> Running Chaos Engineering and Fault-Injection Tests..."
cargo test --package tests --test chaos_test -- --nocapture
echo "==> Chaos test suite passed successfully!"
