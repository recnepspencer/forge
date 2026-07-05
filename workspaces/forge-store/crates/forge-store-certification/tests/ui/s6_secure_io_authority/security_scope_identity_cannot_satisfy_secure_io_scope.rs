use forge_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use forge_store_io_scheduler::{
    IoSchedulerBackendCapabilityAdmission, SecureIoOperation, SecureIoPreservationRequest,
};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope,
};

fn main() {
    let backend: IoSchedulerBackendCapabilityAdmission = todo!();
    let identity = StoreSecurityScopeIdentity::from_physical_security_scope(
        forge_store_aspect_native::StorePhysicalBoundaryWitness::from_physical_authority(
            StorePhysicalAuthorityWitness::for_aspect_native_boundary(
                ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
            )
            .unwrap(),
        )
        .unwrap(),
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(StoreAuthenticityRequirementClass::AuthenticatedFrame),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let _ = SecureIoPreservationRequest::new(SecureIoOperation::ReadAhead, &identity, &backend);
}
