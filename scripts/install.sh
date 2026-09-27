#!/usr/bin/env bash
set -euo pipefail

DEST_DIR="${DEST_DIR:-/usr/local/bin}"

echo "==> Installing AirMesh binaries to ${DEST_DIR}..."
cp target/release/airmesh-daemon "${DEST_DIR}/airmesh-daemon"
cp target/release/airmesh-cli "${DEST_DIR}/airmesh"
chmod +x "${DEST_DIR}/airmesh-daemon" "${DEST_DIR}/airmesh"

echo "==> Installation complete! Run 'airmesh' or 'airmesh-daemon'."
