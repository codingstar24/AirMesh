# Contributing to AirMesh

Thank you for contributing to AirMesh! We enforce strict engineering and cryptographic safety standards.

---

## 1. Code Quality Mandates

Every pull request must pass the following checks:
1. **Linter**: `#![deny(warnings, clippy::all, clippy::pedantic)]` must compile cleanly with 0 warnings.
2. **Formatting**: Formatted with standard `cargo fmt --all -- --check`.
3. **No Unsafe**: Zero unsafe code in `airmesh-core`, `airmesh-daemon`, or `airmesh-cli` unless mathematically and mechanically justified with a `# Safety` block.
4. **No Unwraps in Library**: Library code must use `Result<T, AirMeshError>` and never call `.unwrap()` or `.expect()`.
5. **No Blocking I/O**: Never perform synchronous blocking filesystem or network operations on Tokio async worker threads. Always use `tokio::fs` or `tokio::task::spawn_blocking`.

---

## 2. Testing Requirements

- Unit test coverage `≥ 80%`.
- Every new frame or codec parser must include a fuzz target in `fuzz/`.
- Integration tests must verify multi-node handshakes and resume capabilities.
- Zero-telemetry audit test must pass.

---

## 3. Running Checks Locally

```bash
# Run unit & integration tests
make test

# Run benchmarks
make bench

# Run security audit
cargo audit
cargo deny check
```
