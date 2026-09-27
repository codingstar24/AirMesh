# AirMesh REST, WebSocket & SSE API Reference

Base URL: `http://127.0.0.1:47823`

All API endpoints are protected by localhost CORS rules to defend against browser-based drive-by attacks from untrusted tabs.

---

## 1. REST Endpoints

### `GET /api/health`
Returns node liveness, version, and current online peer count.

**Response `200 OK`**:
```json
{
  "status": "ok",
  "node_id": "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08",
  "device_name": "Node-MacBook",
  "version": "1.0.0",
  "peer_count": 3
}
```

---

### `GET /api/peers`
Lists all active discovered peers on the local subnet.

**Response `200 OK`**:
```json
[
  {
    "node_id": "7f8c92a1884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a09",
    "device_name": "Pixel 8 Pro",
    "address": "192.168.1.145:47823",
    "method": "Mdns",
    "last_seen": 1700000042
  }
]
```

---

### `POST /api/peers/pair`
Performs out-of-band PAKE pairing with a remote peer.

**Request Body**:
```json
{
  "peer_node_id": "7f8c92a1884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a09",
  "pin_code": "849201",
  "salt_hex": "00000000000000000000000000000000"
}
```

**Response `200 OK`**:
```json
{
  "status": "paired",
  "peer_node_id": "7f8c92a1884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a09",
  "token": "31d683707736ee7bf9e02ffcb63a48e7e174b6dbb5a9dd5287f3b33372c3d0b2"
}
```

---

### `GET /api/files`
Lists local indexed files available to mesh peers.

**Response `200 OK`**:
```json
{
  "files": [
    {
      "id": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "local_path": "/home/user/Shared/report.pdf",
      "virtual_path": "report.pdf",
      "size": 1548291,
      "mime_type": "application/pdf",
      "modified_at": 1700000000,
      "chunk_count": 2,
      "root_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    }
  ]
}
```

---

### `GET /api/search?q={query}&mime={mime}&limit={limit}`
Queries SQLite FTS5 index. Returns results ranked by relevance score in <50ms.

**Parameters**:
- `q`: Search query string
- `mime`: (Optional) MIME filter prefix (e.g. `video/`)
- `limit`: (Optional) Maximum items returned (default 50)

---

### `GET /api/stream/:file_id`
Supports HTTP Range byte-level media streaming (`206 Partial Content`) for instant audio/video playback without pre-downloading.

**Headers**:
- `Range: bytes=0-1048575`

**Response `206 Partial Content`**:
- `Content-Range: bytes 0-1048575/104857600`
- `Content-Length: 1048576`
- `Content-Type: video/mp4`

---

### `GET /api/metrics`
Exposes system counters in standard Prometheus text exposition format.

---

## 2. WebSocket Real-Time Events (`GET /api/ws`)
Subscribes to live mesh notifications. Emits heartbeat status, discovered peers, and transfer progress updates.

---

## 3. Server-Sent Events (`GET /api/sse`)
Lightweight unidirectional event stream for resource-constrained browsers and smart TVs.
