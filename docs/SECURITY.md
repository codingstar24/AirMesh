# AirMesh Security Architecture & Threat Model

## 1. Threat Model & Trust Boundaries

AirMesh operates on untrusted or semi-trusted local networks (public coffee shop WiFi, hotel networks, student dorms, conference halls, and ad-hoc device hotspots). 

The primary security goals are:
1. **Confidentiality**: Unauthorized eavesdroppers on the same broadcast domain cannot inspect file content or transfer metadata.
2. **Integrity**: Any bitflips, packet manipulation, or malicious chunk injection are rejected with 100% cryptographic certainty.
3. **Sandbox Confinement**: A malicious peer cannot read, write, modify, or list files outside the user's explicitly configured shared folder.
4. **Availability & Resource Exhaustion Defense**: Nodes cannot be crashed or paralyzed by high-frequency beacon floods or oversized frame injections.
5. **Absolute Privacy / Zero Telemetry**: Under zero circumstances may any packets be transmitted to internet hosts or public endpoints.

---

## 2. Path Traversal & Filesystem Confinement

### Protection Mechanisms
- **Lexical and Canonical Traversal Rejection**: All inbound virtual paths are normalized and inspected. Any presence of `../`, `..\`, absolute root prefixes (`/`, `C:\`, `\\?\`), or null byte injection immediately aborts the transaction with `ErrorCode::InvalidPath (1006)`.
- **Symlink Jail**: Symlinks pointing outside the shared directory boundary are rejected during indexing and transfer resolution.
- **Hidden / Metadata File Exclusion**: System metadata (`.git`, `.DS_Store`, `desktop.ini`, `Thumbs.db`, `.env`, `id_rsa`) are barred from automatic indexing by default.

---

## 3. Cryptographic Hygiene & Secret Handling

1. **Memory Zeroization**:
   - Private key buffers implement the `zeroize::Zeroize` and `zeroize::ZeroizeOnDrop` traits, ensuring sensitive memory is overwritten with zeroes when dropped.
2. **No Secrets in Logs**:
   - Structured tracing subscriber filters explicitly censor keys, tokens, and PAKE PINs from all log outputs.
3. **Forward Secrecy**:
   - All session keys are generated ephemerally using Curve25519; compromise of static node identity keys does not compromise past recorded sessions.

---

## 4. Denial of Service (DoS) Protections

1. **Max Frame Bounds**:
   - Hard upper limit of 16 MiB per wire frame. Buffers exceeding 16 MiB cause immediate connection termination.
2. **Token Bucket Rate Limiting**:
   - Ingress requests are capped per peer IP (default 120 requests/second burst, 60 requests/second sustained).
3. **Asynchronous Non-Blocking I/O**:
   - All disk operations are executed through worker task pools, ensuring the async network runtime never blocks.

---

## 5. Zero-Telemetry Continuous Verification

AirMesh enforces zero telemetry through an automated CI network isolation test (`tests/chaos/zero_telemetry_test.rs`).
During test execution:
- An isolated network namespace monitors all outbound network traffic via packet sockets.
- If ANY packet is addressed to an IP outside private RFC 1918 / RFC 4193 subnets (`10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `fc00::/7`, `fe80::/10`), the test immediately fails CI.
