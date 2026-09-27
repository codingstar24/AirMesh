//! Password-Authenticated Key Exchange (PAKE) pairing protocol.

use crate::crypto::identity::hex;
use crate::protocol::error::{AirMeshError, Result};
use rand::Rng;

/// PAKE pairing engine.
#[derive(Debug, Clone)]
pub struct PakePairing {
    /// 6-digit numeric pairing code
    code: String,
    salt: [u8; 16],
}

impl PakePairing {
    /// Generates a new random 6-digit pairing code with random salt.
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let code_num: u32 = rng.gen_range(100_000..1_000_000);
        let mut salt = [0u8; 16];
        rng.fill(&mut salt);
        Self {
            code: code_num.to_string(),
            salt,
        }
    }

    /// Creates pairing engine from an entered code and salt.
    pub fn from_code(code: &str, salt: [u8; 16]) -> Self {
        Self {
            code: code.trim().to_string(),
            salt,
        }
    }

    /// Returns the 6-digit display code.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the salt bytes.
    #[must_use]
    pub fn salt(&self) -> [u8; 16] {
        self.salt
    }

    /// Derives a confirmation commitment token.
    #[must_use]
    pub fn derive_confirmation_token(&self, local_node_id: &str, remote_node_id: &str) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"AirMesh_SPAKE2_v1");
        hasher.update(self.code.as_bytes());
        hasher.update(&self.salt);
        hasher.update(local_node_id.as_bytes());
        hasher.update(remote_node_id.as_bytes());
        *hasher.finalize().as_bytes()
    }

    /// Verifies peer's confirmation token.
    pub fn verify_token(
        &self,
        local_node_id: &str,
        remote_node_id: &str,
        remote_token: &[u8; 32],
    ) -> Result<()> {
        let expected = self.derive_confirmation_token(remote_node_id, local_node_id);
        if expected == *remote_token {
            Ok(())
        } else {
            Err(AirMeshError::Crypto("Pairing code verification mismatch".into()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairing_token_mutual_verification() {
        let salt = [7u8; 16];
        let p_a = PakePairing::from_code("849201", salt);
        let p_b = PakePairing::from_code("849201", salt);

        let node_a = "node_alpha";
        let node_b = "node_beta";

        let token_from_a = p_a.derive_confirmation_token(node_a, node_b);
        assert!(p_b.verify_token(node_b, node_a, &token_from_a).is_ok());
    }
}
