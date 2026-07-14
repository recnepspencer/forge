use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreKeyVersionPosture, StoreLegacySecurityPosture, StoreSecurityMetadata,
    StoreTenantScope,
};

fn main() {
    let _forged = StoreSecurityMetadata {
        key_scope: StoreKeyScope::PageEnvelope,
        tenant_scope: StoreTenantScope::TenantPhysicalBoundary,
        authenticity_requirement: StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        custody_posture: StoreCustodyPosture::InternalStoreCustody,
        legacy_posture: StoreLegacySecurityPosture::NativeScoped,
        key_version_posture: StoreKeyVersionPosture::Current,
    };
}
