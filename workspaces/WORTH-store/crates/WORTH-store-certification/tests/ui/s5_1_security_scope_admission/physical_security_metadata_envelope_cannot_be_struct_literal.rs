use worth_store_physical_format::PhysicalSecurityMetadataEnvelope;
use worth_store_security::StorePhysicalSecurityMetadataCarrier;

fn main() {
    let _WORTHd = PhysicalSecurityMetadataEnvelope::<u64, StorePhysicalSecurityMetadataCarrier> {
        artifact: 1,
        security_metadata: todo!(),
    };
}
