# Universal Compatibility Matrix

AirMesh is designed to execute on every computing device without runtime dependencies or privileged access.

---

## 1. Operating System & Platform Support Matrix

| Platform | Binary / Package | Discovery | Transports | Status |
|---|---|---|---|---|
| **Windows 10/11** (x86_64, aarch64) | MSI, EXE, Winget, Scoop | mDNS + UDP + BLE | QUIC + TCP | **Tier 1 (Tested)** |
| **macOS 12+** (Apple Silicon, Intel) | Universal DMG, Homebrew | mDNS + UDP + BLE | QUIC + TCP | **Tier 1 (Tested)** |
| **Linux** (Debian, Ubuntu, Fedora, Arch) | Musl Static, DEB, RPM, Flatpak, Snap, AUR | mDNS + UDP | QUIC + TCP | **Tier 1 (Tested)** |
| **Android 8.0+** (ARM64, x86_64) | APK, F-Droid | mDNS + UDP + BLE | QUIC + TCP + WebRTC | **Tier 1 (Tested)** |
| **iOS / iPadOS 15+** | IPA, TestFlight | mDNS + UDP + BLE | QUIC + TCP + WebRTC | **Tier 1 (Tested)** |
| **Raspberry Pi OS** (ARMv7, ARM64) | Single static binary | mDNS + UDP | QUIC + TCP | **Tier 1 (Tested)** |
| **FreeBSD / OpenBSD** | Ports, Static Binary | mDNS + UDP | TCP + QUIC | **Tier 2 (Supported)** |
| **ChromeOS / Browsers** | WebAssembly + PWA | WebRTC + QR Bootstrap | WebRTC DataChannel | **Tier 1 (Zero Install)** |
| **Smart TVs** (Android TV, Tizen, webOS) | Embedded Web + PWA | mDNS / HTTP | HTTP Range + WebSockets | **Tier 1 (Tested)** |
| **NAS** (Synology DSM, QNAP QTS) | Docker, SPK, QPKG | mDNS + UDP | QUIC + TCP | **Tier 2 (Supported)** |
| **OpenWrt / DD-WRT Routers** | MIPS / ARM Musl binary | UDP Broadcast | TCP + QUIC | **Tier 2 (Supported)** |
| **Steam Deck / Handhelds** | AppImage / Flatpak | mDNS + UDP | QUIC + TCP | **Tier 1 (Tested)** |

---

## 2. Universal Device Test Specification

To validate universal multi-device interoperability:
1. **Setup**: Six diverse hardware targets on an isolated WiFi network with internet disconnected:
   - Windows 11 PC (x86_64)
   - MacBook Pro M-Series (macOS ARM64)
   - Android Smartphone (Android 14)
   - Raspberry Pi 4 (Linux ARM64)
   - Chromebook running browser-only PWA
   - Smart TV with web browser
2. **Acceptance Criteria**:
   - All 6 devices appear in peer lists within `<3s`.
   - Simultaneous 1GB file transfer achieves `≥100 MB/s` on WiFi 5 and `≥300 MB/s` on WiFi 6.
   - Smart TV and Chromebook stream 4K video simultaneously with zero buffering.
   - Unplugging router mid-transfer and reconnecting automatically resumes without corrupted bytes.
   - Zero crashes or memory leaks recorded across 24-hour soak test.
