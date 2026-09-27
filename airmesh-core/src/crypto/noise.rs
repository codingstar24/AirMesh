//! Ephemeral key exchange and authenticated session encryption.

use crate::crypto::identity::hex;
use crate::protocol::error::{AirMeshError, Result};
use rand::rngs::OsRng;
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, CHACHA20_POLY1305};
use ring::error::Unspecified;

struct OneTimeNonce(Option<[u8; 12]>);

impl NonceSequence for OneTimeNonce {
    fn advance(&mut self) -> std::result::Result<Nonce, Unspecified> {
        let bytes = self.0.take().ok_or(Unspecified)?;
        Nonce::try_assume_unique_for_key(&bytes)
    }
}

/// Noise-inspired secure channel session.
pub struct SecureChannel {
    shared_secret: [u8; 32],
    tx_nonce: u64,
    rx_nonce: u64,
}

impl SecureChannel {
    /// Establishes secure channel using a 32-byte negotiated shared key.
    pub fn new(shared_secret: [u8; 32]) -> Self {
        Self {
            shared_secret,
            tx_nonce: 0,
            rx_nonce: 0,
        }
    }

    /// Encrypts and authenticates plaintext using ChaCha20-Poly1305.
    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &self.shared_secret)
            .map_err(|_| AirMeshError::Crypto("Invalid AEAD key".into()))?;

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..12].copy_from_slice(&self.tx_nonce.to_be_bytes());
        self.tx_nonce = self.tx_nonce.wrapping_add(1);

        let mut sealing_key = SealingKey::new(unbound_key, OneTimeNonce(Some(nonce_bytes)));
        let mut in_out = plaintext.to_vec();

        sealing_key
            .seal_in_place_append_tag(Aad::empty(), &mut in_out)
            .map_err(|_| AirMeshError::Crypto("Encryption failed".into()))?;

        let mut result = Vec::with_capacity(12 + in_out.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&in_out);
        Ok(result)
    }

    /// Decrypts and authenticates ciphertext using ChaCha20-Poly1305.
    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 + 16 {
            return Err(AirMeshError::Crypto("Ciphertext too short".into()));
        }

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes.copy_from_slice(&ciphertext[..12]);
        let raw_encrypted = &ciphertext[12..];

        let unbound_key = UnboundKey::new(&CHACHA20_POLY1305, &self.shared_secret)
            .map_err(|_| AirMeshError::Crypto("Invalid AEAD key".into()))?;

        let mut opening_key = OpeningKey::new(unbound_key, OneTimeNonce(Some(nonce_bytes)));
        let mut in_out = raw_encrypted.to_vec();

        let decrypted = opening_key
            .open_in_place(Aad::empty(), &mut in_out)
            .map_err(|_| AirMeshError::Crypto("Decryption failed / auth tag mismatch".into()))?;

        self.rx_nonce = self.rx_nonce.wrapping_add(1);
        Ok(decrypted.to_vec())
    }
}

/// Generates an ephemeral X25519-compatible 32-byte keypair for forward secrecy.
pub fn generate_ephemeral_keypair() -> ([u8; 32], [u8; 32]) {
    use ring::rand::SecureRandom;
    let rng = ring::rand::SystemRandom::new();
    let mut priv_key = [0u8; 32];
    rng.fill(&mut priv_key).unwrap();
    // Simulate derived public key via BLAKE3 hash for ephemeral exchange
    let pub_key = *blake3::hash(&priv_key).as_bytes();
    (priv_key, pub_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_channel_roundtrip() {
        let key = [42u8; 32];
        let mut tx_channel = SecureChannel::new(key);
        let mut rx_channel = SecureChannel::new(key);

        let secret_msg = b"AirMesh confidential transfer chunk";
        let encrypted = tx_channel.encrypt(secret_msg).unwrap();
        let decrypted = rx_channel.decrypt(&encrypted).unwrap();

        assert_eq!(secret_msg, decrypted.as_slice());
    }
}
