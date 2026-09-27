//! QR code optical bootstrap generation and parsing.

use crate::crypto::identity::hex;
use crate::discovery::types::{DiscoveredPeer, DiscoveryMethod};
use crate::protocol::error::{AirMeshError, Result};
use std::net::SocketAddr;

/// QR bootstrap data structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QrBootstrapData {
    /// Public node identity
    pub node_id: String,
    /// Friendly device name
    pub device_name: String,
    /// Direct network address
    pub address: SocketAddr,
    /// Optional PAKE pin code for immediate pairing
    pub pake_code: Option<String>,
    /// Optional salt for PAKE
    pub salt: Option<[u8; 16]>,
}

impl QrBootstrapData {
    /// Encodes bootstrap data into an `airmesh://` URI string.
    pub fn to_uri(&self) -> String {
        let mut uri = format!(
            "airmesh://{}@{}:{}?name={}",
            self.node_id,
            self.address.ip(),
            self.address.port(),
            urlencoding::encode(&self.device_name)
        );

        if let Some(code) = &self.pake_code {
            uri.push_str(&format!("&pake={code}"));
        }
        if let Some(salt) = &self.salt {
            uri.push_str(&format!("&salt={}", hex::encode(salt)));
        }
        uri
    }

    /// Parses an `airmesh://` URI string into `QrBootstrapData`.
    pub fn from_uri(uri: &str) -> Result<Self> {
        let uri = uri.trim();
        if !uri.starts_with("airmesh://") {
            return Err(AirMeshError::Serialization("Invalid URI scheme".into()));
        }

        let without_scheme = &uri["airmesh://".len()..];
        let parts: Vec<&str> = without_scheme.splitn(2, '?').collect();
        let main_part = parts[0];
        let query_part = parts.get(1).copied().unwrap_or("");

        let at_parts: Vec<&str> = main_part.splitn(2, '@').collect();
        if at_parts.len() != 2 {
            return Err(AirMeshError::Serialization("Missing @ in URI".into()));
        }
        let node_id = at_parts[0].to_string();
        let addr_str = at_parts[1];

        let address: SocketAddr = addr_str
            .parse()
            .map_err(|e| AirMeshError::Serialization(format!("Invalid socket address: {e}")))?;

        let mut device_name = "Peer".to_string();
        let mut pake_code = None;
        let mut salt = None;

        for param in query_part.split('&') {
            let kv: Vec<&str> = param.splitn(2, '=').collect();
            if kv.len() == 2 {
                match kv[0] {
                    "name" => {
                        device_name = urlencoding::decode(kv[1]).unwrap_or_default().to_string();
                    }
                    "pake" => {
                        pake_code = Some(kv[1].to_string());
                    }
                    "salt" => {
                        if let Ok(bytes) = hex::decode(kv[1]) {
                            if bytes.len() == 16 {
                                let mut arr = [0u8; 16];
                                arr.copy_from_slice(&bytes);
                                salt = Some(arr);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(Self {
            node_id,
            device_name,
            address,
            pake_code,
            salt,
        })
    }

    /// Converts to DiscoveredPeer object.
    pub fn to_discovered_peer(&self) -> DiscoveredPeer {
        DiscoveredPeer {
            node_id: self.node_id.clone(),
            device_name: self.device_name.clone(),
            address: self.address,
            method: DiscoveryMethod::QrCode,
            last_seen: chrono::Utc::now().timestamp(),
        }
    }
}

// Minimal urlencoding utility
mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for b in s.bytes() {
            if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
                result.push(b as char);
            } else {
                result.push_str(&format!("%{:02X}", b));
            }
        }
        result
    }

    pub fn decode(s: &str) -> Result<String, ()> {
        let mut bytes = Vec::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '%' {
                let h1 = chars.next().ok_or(())?;
                let h2 = chars.next().ok_or(())?;
                let hex_str = format!("{}{}", h1, h2);
                let byte = u8::from_str_radix(&hex_str, 16).map_err(|_| ())?;
                bytes.push(byte);
            } else if c == '+' {
                bytes.push(b' ');
            } else {
                bytes.push(c as u8);
            }
        }
        String::from_utf8(bytes).map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_uri_roundtrip() {
        let data = QrBootstrapData {
            node_id: "abcdef123456".into(),
            device_name: "MacBook Pro M3".into(),
            address: "192.168.1.150:47823".parse().unwrap(),
            pake_code: Some("582914".into()),
            salt: Some([1u8; 16]),
        };

        let uri = data.to_uri();
        let parsed = QrBootstrapData::from_uri(&uri).unwrap();
        assert_eq!(data, parsed);
    }
}
