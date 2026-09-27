# AirMesh Architecture Specification

## 1. Executive Summary & Design Principles

AirMesh is an offline-first, local-area peer-to-peer file mesh. The design is strictly governed by zero-trust, zero-cloud, and zero-telemetry constraints. Devices communicate directly across local network boundaries (WiFi 5/6/7, Ethernet, Ad-Hoc networks) using QUIC, TLS 1.3, and BLAKE3 content addressing.

```
┌────────────────────────────────────────────────────────┐
│     User Interfaces (Embedded Web, PWA, CLI, Mobile)   │
└───────────────────────────┬────────────────────────────┘
                            │ REST / WebSocket / SSE / IPC
┌───────────────────────────▼────────────────────────────┐
│      HTTP & Control API (Axum, RFC 7807 Errors, CORS)   │
├────────────────────────────────────────────────────────┤
│   Domain Logic & State (Peer Registry, ACLs, Quotas)   │
├────────────────────────────────────────────────────────┤
│ Transfer Engine (FastCDC, 1MB Chunks, Bitmaps, Stream) │
├────────────────────────────────────────────────────────┤
│ Cryptographic Security (Ed25519, Noise, BLAKE3, SPAKE2)│
├────────────────────────────────────────────────────────┤
│ Transport Layer (QUIC / Quinn, TLS 1.3 TCP, WebRTC)    │
├────────────────────────────────────────────────────────┤
│ Discovery Layer (mDNS, UDP Broadcast, QR, BLE Fallback)│
├────────────────────────────────────────────────────────┤
│ Storage & Index (SQLite FTS5, Fast Watcher, ContentDB) │
├────────────────────────────────────────────────────────┤
│ Abstracted Platform Layer (FS, Network, Keychain, UI)  │
└────────────────────────────────────────────────────────┘
```

---

## 2. Layering & Invariants

1. **Platform Layer (`airmesh-core::platform`)**:
   - Isolates operating system dependencies behind abstract traits: `FileSystem`, `Network`, `Keychain`, `UserInterfaceBridge`.
   - Every trait must have at least two implementations: production native and in-memory mock.
   - Guaranteed deterministic unit and chaos testing without disk or socket access.

2. **Storage & Index Layer (`airmesh-core::index`)**:
   - Backed by SQLite in WAL (Write-Ahead Logging) mode with memory-mapped I/O (`PRAGMA mmap_size = 30GB`).
   - SQLite FTS5 virtual table for full-text tokenized search (<50ms across 1,000,000+ records).
   - Content-addressed indexing using BLAKE3 256-bit hashes.

3. **Discovery Layer (`airmesh-core::discovery`)**:
   - Multi-modal broadcast engine combining mDNS (`_airmesh._udp.local`) and UDP broadcast (port 47824).
   - Optical bootstrap via QR code (`airmesh://<id>@<ip>:<port>?pake=...`) for AP-isolated networks.
   - Bluetooth Low Energy (BLE) advertisement fallback.

4. **Transport Layer (`airmesh-core::transport`)**:
   - Primary: QUIC over UDP powered by Quinn with ALPN `airmesh-v1`.
   - Secondary: TCP with TLS 1.3 framing for UDP-filtered captive networks.
   - Browser Fallback: WebRTC DataChannel for zero-install browser PWA nodes.

5. **Cryptographic Security (`airmesh-core::crypto`)**:
   - Ed25519 node identities.
   - Noise Protocol Framework (`Noise_XX_25519_ChaChaPoly_BLAKE3`) for session encryption.
   - SPAKE2 password-authenticated key exchange for 6-digit out-of-band PIN pairing.
   - Per-chunk BLAKE3 integrity verification.

6. **Transfer Engine (`airmesh-core::transfer`)**:
   - Fixed 1 MiB chunk partitioning for transfers.
   - FastCDC variable content-defined chunking for delta synchronization.
   - Persistent `TransferBitmap` for zero-overhead resumption after network interruption.
   - Real-time LZ4 compression with entropy check.
   - HTTP Range slice reader for sub-120ms media streaming without pre-downloading.

7. **HTTP & WebSocket API (`airmesh-daemon::api`)**:
   - Axum-based asynchronous HTTP REST server.
   - RFC 7807 Problem Details error formatting.
   - Localhost-only CORS protection preventing browser drive-by attacks.
   - Real-time WebSocket and SSE notification streams.

---

## 3. Performance Trade-Off Analysis

| Architectural Decision | Chosen Strategy | Alternative Rejected | Rationale |
|---|---|---|---|
| **Chunk Sizing** | Fixed 1 MiB chunks | Dynamic 64 KB – 4 MB | Minimizes SQLite transaction overhead and QUIC frame header inflation while maintaining low memory footprint (<88MB at 16 parallel streams). |
| **Integrity Hashing** | BLAKE3 (256-bit) | SHA-256 / SHA-3 | BLAKE3 achieves >3 GB/s throughput on modern SIMD cores, running 6-10x faster than SHA-256 while offering immune Merkle tree properties. |
| **Search Engine** | SQLite FTS5 (embedded) | Tantivy / MeiliSearch | Embedded in-process with zero external processes or JVM/Node dependencies; maintains <18ms latency across 100k items. |
| **Database Mode** | WAL + Normal Sync | Synchronous Full | Guarantees crash durability while achieving >40,000 upserts/sec during cold directory ingestion. |
