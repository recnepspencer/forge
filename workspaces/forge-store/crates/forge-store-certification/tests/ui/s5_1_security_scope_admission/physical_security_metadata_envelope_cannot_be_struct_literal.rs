use forge_store_physical_format::PhysicalSecurityMetadataEnvelope;
use forge_store_security::StorePhysicalSecurityMetadataCarrier;

fn main() {
    let _forged = PhysicalSecurityMetadataEnvelope::<u64, StorePhysicalSecurityMetadataCarrier> {
        artifact: 1,
        security_metadata: todo!(),
    };
}
