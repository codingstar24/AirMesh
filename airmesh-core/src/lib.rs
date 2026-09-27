//! # AirMesh Core
//!
//! `airmesh-core` implements the core protocols, storage engine, cryptography,
//! discovery mechanics, and transfer pipeline for the AirMesh peer-to-peer file mesh.
//!
//! # Architecture & Layering
//!
//! ```text
//! UI -> HTTP API -> Domain -> Transfer Engine -> Protocol -> Transport -> Discovery -> Storage
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]

pub mod crypto;
pub mod discovery;
pub mod index;
pub mod platform;
pub mod protocol;
pub mod transfer;
pub mod transport;
pub mod util;

/// Current wire protocol major version
pub const PROTOCOL_VERSION_MAJOR: u16 = 1;
/// Current wire protocol minor version
pub const PROTOCOL_VERSION_MINOR: u16 = 0;
/// Default port for AirMesh QUIC/TCP service
pub const DEFAULT_SERVICE_PORT: u16 = 47823;
/// Default port for AirMesh UDP discovery broadcast
pub const DEFAULT_DISCOVERY_PORT: u16 = 47824;
/// Default chunk size (1 MiB)
pub const DEFAULT_CHUNK_SIZE: usize = 1024 * 1024;
