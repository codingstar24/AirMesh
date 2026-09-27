//! Node identity management via Ed25519 asymmetric cryptography.

use crate::protocol::error::{AirMeshError, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// Cryptographic identity of an AirMesh node.
#[derive(Clone)]
pub struct NodeIdentity {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    node_id: String,
}

impl NodeIdentity {
    /// Generates a fresh cryptographically secure Ed25519 node identity.
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let node_id = hex::encode(verifying_key.as_bytes());
        Self {
            signing_key,
            verifying_key,
            node_id,
        }
    }

    /// Reconstructs identity from a 32-byte private seed.
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(bytes);
        let verifying_key = signing_key.verifying_key();
        let node_id = hex::encode(verifying_key.as_bytes());
        Self {
            signing_key,
            verifying_key,
            node_id,
        }
    }

    /// Returns the public node identifier (hex-encoded 32-byte Ed25519 public key).
    #[must_use]
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Returns the raw public key bytes.
    #[must_use]
    pub fn public_key_bytes(&self) -> [u8; 32] {
        *self.verifying_key.as_bytes()
    }

    /// Signs an arbitrary message using the node's private key.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        self.signing_key.sign(message).to_bytes()
    }

    /// Verifies a signature from a remote peer given their public key.
    pub fn verify_peer(peer_public_key: &[u8; 32], message: &[u8], signature_bytes: &[u8; 64]) -> Result<()> {
        let verifying_key = VerifyingKey::from_bytes(peer_public_key)
            .map_err(|e| AirMeshError::Crypto(format!("Invalid peer public key: {e}")))?;
        let signature = Signature::from_bytes(signature_bytes);
        verifying_key
            .verify(message, &signature)
            .map_err(|e| AirMeshError::Crypto(format!("Signature verification failed: {e}")))?;
        Ok(())
    }

    /// Returns the raw secret bytes for persistent keychain storage.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
}

// Simple hex encoder/decoder avoiding external hex crate requirement if desired
pub mod hex {
    use std::fmt::Write;

    /// Encodes slice to lowercase hex string.
    pub fn encode(data: &[u8]) -> String {
        let mut s = String::with_capacity(data.len() * 2);
        for &b in data {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }

    /// Decodes hex string to byte vector.
    pub fn decode(hex_str: &str) -> Result<Vec<u8>, String> {
        if hex_str.len() % 2 != 0 {
            return Err("Hex string has odd length".to_string());
        }
        (0..hex_str.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16).map_err(|e| e.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_generation_and_signing() {
        let id = NodeIdentity::generate();
        let message = b"AirMesh local transfer assertion";
        let sig = id.sign(message);
        assert!(NodeIdentity::verify_peer(&id.public_key_bytes(), message, &sig).is_ok());
    }
}
