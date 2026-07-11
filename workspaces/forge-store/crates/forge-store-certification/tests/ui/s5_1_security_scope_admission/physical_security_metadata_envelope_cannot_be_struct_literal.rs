use forge_store_physical_format::PhysicalSecurityMetadataEnvelope;
use forge_store_security::StoreSecurityMetadata;

fn main() {
    let _forged = PhysicalSecurityMetadataEnvelope::<u64, StoreSecurityMetadata> {
        artifact: 1,
        security_metadata: todo!(),
    };
}
