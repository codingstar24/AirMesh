#!/usr/bin/env bash
set -euo pipefail

echo "==> Building AirMesh Daemon & Core..."
cargo build -p airmesh-daemon -p airmesh-cli

echo "==> Starting AirMesh Daemon on port 47823..."
cargo run -p airmesh-daemon -- --port 47823 &
DAEMON_PID=$!

trap "kill $DAEMON_PID 2>/dev/null || true" EXIT

echo "==> Opening AirMesh Web Interface..."
if which xdg-open > /dev/null; then
  xdg-open http://localhost:47823
elif which open > /dev/null; then
  open http://localhost:47823
elif which start > /dev/null; then
  start http://localhost:47823
fi

wait $DAEMON_PID
