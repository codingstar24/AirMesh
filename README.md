# AirMesh 🌐⚡
### 100% Local, Offline-First, Zero-Cloud Peer-to-Peer File & Media Mesh

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Zero Telemetry](https://img.shields.io/badge/Telemetry-0%25%20Verified-brightgreen.svg)](#security--privacy)
[![Offline First](https://img.shields.io/badge/Network-100%25%20Offline--First-success.svg)](#offline-architecture)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS%20%7C%20Linux%20%7C%20iOS%20%7C%20Android-informational.svg)](#universal-device-compatibility)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/CONTRIBUTING.md)

> **Created & Architected by [Zohair Sathaliya](https://github.com/airmesh)**  
> Turn every device on your local Wi-Fi into an ultra-fast, secure, searchable file mesh — operating at full line-rate speeds with **zero internet connection required**.

---

## 💡 Why AirMesh?

| Traditional Transfer Method | Limitation | The AirMesh Solution |
| :--- | :--- | :--- |
| **Apple AirDrop** | Restricted strictly to Apple ecosystem | **Cross-platform**: Works on Windows, Mac, Linux, iPhone, Android, and Smart TVs |
| **Google Quick Share** | Requires Google Play services & accounts | **Zero accounts & zero apps**: Operates directly in any modern mobile browser |
| **Bluetooth Transfer** | Extremely slow (1–2 MB/s) | **Line-rate local Wi-Fi**: 50–200 MB/s on standard Wi-Fi 5 / Wi-Fi 6 |
| **Cloud Storage (Drive, Dropbox)** | Wastes mobile data, requires internet, slow uploads | **100% Offline & Free**: Uses zero bytes of cellular data or internet bandwidth |
| **Standard HTTP Uploads** | If Wi-Fi flickers or phone locks, transfer dies at 99% | **Resumable Chunked Engine**: Auto-resumes from exact byte with zero data loss |

---

## ✨ Key Features

### 🔄 1. Resumable Chunked Transfers with Zero Data Loss
- **2 MB Chunk Slicing**: Large files and videos are automatically sliced in the browser and transmitted sequentially.
- **Safe Partial Buffering (`.part`)**: Disconnections, screen locks, or Wi-Fi blips never delete partial files. Every byte received is preserved on disk.
- **Auto-Reconnect & Resume**: When network connectivity fluctuates, AirMesh enters an auto-reconnect state and resumes from the exact byte where it paused.
- **Manual Pause / Resume**: Integrated controls in the upload HUD allow you to pause transfers and resume them at any time.

### ⚡ 2. Progressive "Stream-While-Receiving"
- Why wait for a 4 GB 4K video or album to finish uploading before watching it?
- The moment another device starts sending a video, other peers can immediately click **Play** and begin streaming the live in-flight buffer without waiting for the upload to complete.

### 📁 3. Hierarchical Folder & Batch Archive Support
- **Folder Upload**: Drag and drop nested directory trees with subfolders directly into the browser.
- **Native PKZIP Folder Downloads**: Download entire folders or multiple selected items as a single `.zip` file with RFC 6266 headers and valid `PK\x03\x04` magic bytes — natively extractable by Windows Explorer, macOS Archive Utility, iOS Files, and Android.

### 📱 4. Mobile-First Responsive PWA (Zero Mobile Data Guarantee)
- **Zero Internet Required**: Operates flawlessly on private subnets (`192.168.x.x`, `10.x.x.x`) even if your router's WAN cable is unplugged.
- **Screen Wake Lock API**: Automatically prevents mobile devices from sleeping during transfers to avoid OS-level Wi-Fi throttling.
- **1-Tap QR Connect**: Simply scan the QR code displayed on your PC with your phone camera to instantly connect to the mesh.

### 🎬 5. Universal Video Player & On-the-Fly Audio Transcoding
- Supports direct browser playback for MP4, WebM, and MKV.
- Automatic **ffmpeg** audio transcoding for incompatible surround codecs (`E-AC3`, `AC3`, `DTS`, `TrueHD`) to crisp stereo AAC in real-time.
- Built-in subtitle support (`.vtt`, `.srt`), keyboard shortcuts (Space to toggle, F for fullscreen, M to mute), and dual audio mode toggle.

### 📋 6. Real-Time Shared Mesh Clipboard
- Instant cross-device copy-paste for text, code snippets, notes, and links across all connected computers and phones on your network.

---

## 🏗️ Project Architecture & File Organization

AirMesh is organized systematically into clean, modular layers:

```
AirMesh/
├── index.html                   # High-performance, zero-framework reactive Web UI & PWA
├── dev_server.cjs               # Lightweight, zero-config Node.js host server & streaming engine
├── sync_assets.cjs              # Automated asset synchronizer between index.html and Rust daemon
├── start.bat                    # Windows 1-Click launcher (starts server & opens browser)
├── run_rust.bat                 # Windows native Rust daemon launcher
├── package.json                 # Node.js project manifest & script commands
├── Cargo.toml                   # Rust workspace configuration (Core, Daemon, CLI)
├── LICENSE                      # MIT Open Source License (c) Zohair Sathaliya
│
├── airmesh-core/                # Rust core library (Cryptography, FastCDC, Transfer Protocols)
├── airmesh-daemon/              # Native high-concurrency Rust daemon with embedded web assets
├── airmesh-cli/                 # Command-line interface for headless servers and terminal users
│
├── shared/                      # Local shared storage directory (isolated with strict path jail)
│   └── .gitkeep                 # Keeps shared folder tracked while ignoring personal files
│
├── scripts/                     # Developer, benchmark, and maintenance utilities
│   ├── create_samples.cjs       # Generates sample audio tones and vector artwork
│   ├── verify_robustness.cjs    # Automated test suite for endpoints and range requests
│   └── dev.sh                   # Unix development launch script
│
└── docs/                        # Complete technical documentation suite
    ├── ARCHITECTURE.md          # Multi-tiered system architecture and concurrency model
    ├── API.md                   # Complete REST, SSE, and WebSocket endpoint reference
    ├── PROTOCOL.md              # Wire protocol and packet frame specifications
    ├── SECURITY.md              # Zero-trust security, path sanitization, and threat analysis
    ├── COMPATIBILITY.md         # Hardware and browser compatibility matrix
    └── CONTRIBUTING.md          # Guidelines for community contributions
```

---

## 🚀 Quick Start Guide

### Option 1: 1-Click Instant Start (Windows)
Double-click **`start.bat`**. This starts the local node server and automatically launches the web interface in your browser:
```
http://localhost:47823
```

### Option 2: Node.js (All Platforms)
```bash
# Clone the repository
git clone https://github.com/airmesh/airmesh.git
cd airmesh

# Install dependencies (only lightweight qrcode generator needed)
npm install

# Start the node
npm start
```

### Option 3: High-Performance Native Rust Daemon
```bash
# Build and run the native Rust core daemon
cargo run -p airmesh-daemon
```

---

## 📱 Connecting from Mobile Devices (Phone / Tablet)

1. Ensure your phone is connected to the **same Wi-Fi network** as your PC.
2. In the AirMesh web interface on your PC, click **"Connect Phone"** or the QR code button in the top navigation bar.
3. Open your phone's camera app and scan the QR code (or type `http://<YOUR_PC_IP>:47823` in Safari/Chrome).
4. Tap **"Add to Home Screen"** to install AirMesh as a standalone full-screen Progressive Web App.
5. **Upload photos, stream 4K movies, or share folders** with full line-rate local speed and 0% cellular data usage!

---

## 🔌 API Reference Summary

AirMesh exposes a clean, systematic HTTP & SSE interface:

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `GET` | `/api/files?path=<rel>` | List files and folders with metadata, audio status, and duration |
| `GET` | `/api/upload-status` | Query byte offset of partial upload (`.part`) for seamless resuming |
| `POST` | `/api/upload` | Upload file/folder chunks with byte-offset headers and wake lock |
| `POST` | `/api/upload-cancel` | Cancel an in-flight upload and safely remove temporary buffers |
| `GET` | `/api/download-folder` | Stream entire directory as a native PKZIP archive |
| `GET` | `/api/archive?files=...`| Stream multi-item selection as a native PKZIP archive |
| `GET` | `/api/stream/:id` | Progressive stream with HTTP 206 Partial Content (Range requests) |
| `GET` | `/api/events` | Real-time Server-Sent Events (SSE) for instant peer & transfer updates |
| `GET` | `/api/network-info` | Host storage capacity, IP addresses, and offline verification |
| `GET/POST`| `/api/clipboard` | Real-time shared cross-device clipboard |

---

## 🔒 Security & Privacy

AirMesh operates on strict zero-trust local networking principles:
- **Verified Zero Telemetry**: Absolutely no external network calls, tracking pixels, or third-party CDNs. All fonts, icons, and scripts are 100% self-hosted.
- **Path Traversal Sandboxing**: The `resolveSafePath` security engine prevents directory traversal attacks (`../`, UNC paths, symlink escapes). Files are confined strictly within the designated storage directory.
- **Local Network Boundary**: Bound exclusively to RFC 1918 private IP subnets (`192.168.0.0/16`, `10.0.0.0/8`, `172.16.0.0/12`).

---

## 👤 Credits & Author

**AirMesh** was designed, architected, and built by:

* **Zohair Sathaliya** — *Creator, Lead Architect & Developer*  
  GitHub: [@airmesh](https://github.com/airmesh)

---

## 📄 License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for complete details.
