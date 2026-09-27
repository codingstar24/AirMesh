//! Self-signed TLS 1.3 certificate generation for QUIC transport.

use crate::protocol::error::{AirMeshError, Result};
use rcgen::{generate_simple_self_signed, CertifiedKey};

/// TLS certificate and private key bundle for QUIC/TLS server.
pub struct TlsCertificateBundle {
    /// DER-encoded certificate chain
    pub cert_chain_der: Vec<Vec<u8>>,
    /// DER-encoded PKCS#8 private key
    pub private_key_der: Vec<u8>,
}

impl TlsCertificateBundle {
    /// Generates an in-memory self-signed certificate valid for local peer communication.
    pub fn generate_self_signed(node_id: &str) -> Result<Self> {
        let subject_alt_names = vec![
            format!("{node_id}.airmesh.local"),
            "localhost".to_string(),
            "127.0.0.1".to_string(),
        ];

        let CertifiedKey { cert, key_pair } =
            generate_simple_self_signed(subject_alt_names)
                .map_err(|e| AirMeshError::Crypto(format!("Failed to generate TLS certificate: {e}")))?;

        let cert_der = cert.der().to_vec();
        let key_der = key_pair.serialized_der().to_vec();

        Ok(Self {
            cert_chain_der: vec![cert_der],
            private_key_der: key_der,
        })
    }
}
