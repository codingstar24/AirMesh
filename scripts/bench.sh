#!/usr/bin/env bash
set -euo pipefail

echo "==> Running AirMesh Criterion Benchmarks..."
cargo bench -p benches -- --verbose
echo "==> Benchmarks complete! Report generated in target/criterion."
