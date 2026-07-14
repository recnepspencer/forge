use forge_store_physical_format::{PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use forge_store_test_support::SecurityScopeFixtureAuthority;

use super::super::fixture_admission::security_scope;

pub(super) fn page_security(
    authority: SecurityScopeFixtureAuthority,
) -> forge_store_security::StoreAdmittedSecurityScope {
    security_scope(
        authority,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn wal_security(
    authority: SecurityScopeFixtureAuthority,
) -> forge_store_security::StoreAdmittedSecurityScope {
    security_scope(
        authority,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn root_security() -> forge_store_security::StoreAdmittedSecurityScope {
    security_scope(
        SecurityScopeFixtureAuthority::Current,
        StoreKeyScope::StoreManagedRoot,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedManifest,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn blob_security(
    authority: SecurityScopeFixtureAuthority,
) -> forge_store_security::StoreAdmittedSecurityScope {
    security_scope(
        authority,
        StoreKeyScope::BlobChunkEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedBlobChunk,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

pub(super) fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).expect("scenario segment is nonzero")
}

pub(super) fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).expect("scenario page is nonzero")
}

pub(super) fn record_slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).expect("scenario record slot is nonzero")
}
