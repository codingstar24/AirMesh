# AirMesh Wire Protocol Specification v1.0.0

## 1. Protocol Architecture & Overview

This document specifies the exact wire format, cryptographic handshake sequence, message frame layout, and test vectors for the AirMesh protocol. An independent implementer can construct a fully compatible client or server from this document alone.

---

## 2. Discovery Layer

AirMesh nodes discover each other using either Multicast DNS (mDNS) or Direct UDP Broadcast.

### 2.1 Multicast DNS (mDNS)
- **Service Type**: `_airmesh._udp.local`
- **Port**: 5353 (Multicast address: `224.0.0.251` for IPv4, `ff02::fb` for IPv6)
- **TXT Records**:
  - `id`: 64-character lowercase hex string of the node's Ed25519 public key.
  - `v`: Protocol version string (`1.0.0`).
  - `name`: UTF-8 device name (max 64 bytes).
  - `port`: TCP/QUIC listening port (default `47823`).

### 2.2 UDP Broadcast Fallback
When mDNS is blocked by WiFi Access Point Isolation:
- **Port**: `47824`
- **Destination**: `255.255.255.255:47824` (or subnet broadcast)
- **Payload**: JSON serialization of `DiscoveryBeacon`:
```json
{
  "magic": "AIRMESH_BEACON_V1",
  "node_id": "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
  "device_name": "ThinkPad-P16",
  "port": 47823,
  "timestamp": 1700000000
}
```

---

## 3. Frame Encoding Format

All data frames over QUIC streams or TCP connections MUST be framed with a 10-byte header prefix:

```text
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       'A'     |       'M'     |       'E'     |       'S'     |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|       'H'     |      0x01     |      Length (Big-Endian 32-bit)|
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                      Payload (Length bytes)                   |
|                              ...                              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

- **Magic Prefix**: 6 bytes: `0x41 0x4D 0x45 0x53 0x48 0x01` (`"AMESH\x01"`).
- **Payload Length**: 4 bytes, unsigned 32-bit integer in big-endian network byte order.
- **Maximum Frame Size**: `16,777,216` bytes (16 MiB). Frames exceeding this limit MUST be discarded and terminate the connection.

---

## 4. Handshake Sequence

```text
Initiator (Client)                              Responder (Server)
      |                                                 |
      | -------- Frame::HandshakeInit ----------------> |
      |          (version, ephemeral_pub, identity)     |
      |                                                 |
      | <------- Frame::HandshakeResponse ------------- |
      |          (version, ephemeral_pub, identity)     |
      |                                                 |
      | -------- Frame::HandshakeAck -----------------> |
      |                                                 |
      [ Secure Noise_XX Channel Established (ChaCha20)  ]
```

### HandshakeInit Frame
```json
{
  "HandshakeInit": {
    "version": { "major": 1, "minor": 0, "patch": 0 },
    "capabilities": {
      "quic": true,
      "tcp_tls": true,
      "webrtc": true,
      "lz4": true,
      "fastcdc": true,
      "streaming": true
    },
    "node_id": "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
    "ephemeral_pubkey": [ ... 32 bytes ... ],
    "device_name": "Initiator Device"
  }
}
```

---

## 5. Chunk Transfer Frames

### ChunkRequest
```json
{
  "ChunkRequest": {
    "file_id": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "chunk_index": 42
  }
}
```

### ChunkResponse
```json
{
  "ChunkResponse": {
    "file_id": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    "chunk_index": 42,
    "blake3_hash": [ ... 32 bytes ... ],
    "is_compressed": true,
    "data": [ ... raw or LZ4-compressed payload bytes ... ]
  }
}
```

---

## 6. Error Codes

| Error Code | Name | Description |
|---|---|---|
| `1000` | `InternalError` | Unexpected internal node failure |
| `1001` | `IncompatibleVersion` | Major version mismatch |
| `1002` | `AuthenticationFailed`| Noise handshake or signature failed |
| `1003` | `AccessDenied` | Peer unverified or blocked in ACL |
| `1004` | `NotFound` | Requested chunk or file not found |
| `1005` | `ChecksumMismatch` | Transferred chunk failed BLAKE3 verification |
| `1006` | `InvalidPath` | Traversal path outside shared root rejected |
| `1007` | `RateLimited` | Peer exceeded ingress request quota |
| `1008` | `Timeout` | Stream timed out without frame delivery |
| `1009` | `StorageFull` | Disk quota or capacity exceeded on destination |
| `1010` | `MalformedFrame` | Frame magic invalid or exceeded 16 MiB |
| `1011` | `TransferCancelled` | Transfer aborted by local or remote user |

---

## 7. Cryptographic Test Vectors

### BLAKE3 Empty String Test Vector
- Input: `""` (0 bytes)
- BLAKE3 Hash:
  `af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262`

### BLAKE3 Standard Test Vector
- Input: `"AirMesh 2026 local peer mesh network"`
- BLAKE3 Hash:
  `1c6ec9be9ff1589d816a75905183305a415fc176008779956fe4524458d6014e`

### PAKE Pairing Confirmation Token Vector
- Code: `"849201"`
- Salt: `00000000000000000000000000000000` (16 zero bytes)
- Local Node: `"nodeA"`
- Remote Node: `"nodeB"`
- BLAKE3 Confirmation Token:
  `31d683707736ee7bf9e02ffcb63a48e7e174b6dbb5a9dd5287f3b33372c3d0b2`
