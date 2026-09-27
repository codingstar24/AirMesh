# AirMesh Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-09-26

### Added
- **Core Transport & Discovery**:
  - Quinn-powered QUIC transport with ALPN `airmesh-v1` and fallback TCP with TLS 1.3 framing.
  - Multi-modal peer discovery: Multicast DNS (mDNS) `_airmesh._udp.local` + UDP Broadcast on port 47824.
  - Optical bootstrap via QR code (`airmesh://`) and Bluetooth Low Energy (BLE) fallback beacons.
  - Browser-to-peer interoperability via WebRTC DataChannels.
- **Cryptographic Security**:
  - Ed25519 identity key generation and persistent storage in hardware/OS keychains.
  - Forward-secret Noise channel encryption (`Noise_XX_25519_ChaChaPoly_BLAKE3`).
  - SPAKE2 password-authenticated key exchange (PAKE) for 6-digit out-of-band PIN pairing.
  - Per-chunk BLAKE3 256-bit cryptographic integrity verification.
  - Strict path traversal defenses and symlink jail verification.
- **High-Performance Transfer Engine**:
  - 1 MiB chunk partitioning with parallel streams auto-tuning (8–32 streams).
  - FastCDC content-defined variable chunking for delta synchronization.
  - Persistent bit-vector resume bitmaps surviving abrupt restarts and network dropouts.
  - Real-time LZ4 compression with dynamic entropy detection.
  - HTTP Range byte-level media streaming for `<120ms` first-byte video/audio playback without pre-downloading.
- **SQLite FTS5 Full-Text Index**:
  - Sub-50ms full-text search across 1,000,000+ files with WAL mode and memory-mapped pages.
  - Background filesystem watcher with event debouncing.
- **Universal User Interfaces**:
  - Self-hosted embedded PWA with zero external CDNs, dark mode, and WCAG AA accessibility.
  - Smart TV remote control D-Pad spatial navigation.
  - Full CLI companion (`airmesh-cli`) with `--json` output and command parity.
  - Mobile Tauri wrapper for Android and iOS.
