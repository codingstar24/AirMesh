//! Mobile entry points and JNI / ObjC bridge bindings for Android & iOS.

use airmesh_core::crypto::identity::NodeIdentity;

/// Initializes background node for mobile application lifecycle.
pub fn init_mobile_node() {
    let identity = NodeIdentity::generate();
    tracing::info!("Mobile AirMesh node initialized: {}", identity.node_id());
}
