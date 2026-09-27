# AirMesh User Guide

Welcome to **AirMesh** — the 100% local, offline-first peer-to-peer file mesh.

---

## 1. Quick Start Guide

### Step 1: Start the Daemon
On your laptop or computer:
```bash
airmesh-daemon
```
The daemon starts up in `<1.2s`, opens your shared folder, and begins advertising over local WiFi.

### Step 2: Open the Web Interface
Open your web browser to:
```text
http://localhost:47823
```
Or open the URL on your phone or tablet on the same WiFi (e.g. `http://192.168.1.150:47823`). No installation is required on client devices!

---

## 2. Instant Zero-Wait Media Streaming

You do not need to wait for multi-gigabyte 4K movies or audio files to finish downloading before watching them.
1. In the Web UI or Smart TV browser, click **Stream (0-Wait)** next to any audio or video file.
2. The built-in HTML5 player will immediately stream byte-ranges on demand with `<120ms` first-byte latency.

---

## 3. Pairing Devices (PAKE)

To authorize full transfer privileges with an untrusted device:
1. Click **Pair Device (PAKE)** in the header.
2. Enter the 6-digit PIN displayed on the other device.
3. Once verified, both nodes establish mutual trust without ever passing a shared secret over the network.

---

## 4. Smart TV Navigation

When opening the AirMesh web application on an Android TV, Apple TV (browser), LG webOS, or Samsung Tizen browser:
- Use your **Remote Control D-Pad** (Up, Down, Left, Right) to cycle between search, peer cards, and file actions.
- Press **Select / OK** to trigger streaming or download.
- Press **Back** to dismiss media players and dialogs.

---

## 5. Troubleshooting & FAQs

### Q: Why can't my devices discover each other?
- **Guest WiFi / AP Isolation**: Some hotel, university, or corporate networks block multicast (mDNS) and UDP broadcasts between devices.
  - **Solution**: Click **Pair Device** and scan the QR code, or manually enter the peer's local IP address. AirMesh immediately establishes direct QUIC/TCP communication.
- **Firewall Prompt**: When starting AirMesh for the first time, allow UDP port `47824` and TCP/UDP port `47823` in Windows Defender or macOS Firewall.
