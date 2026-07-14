use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_layout_indexes::{declarations::layout_declarations, ObserveOwnerCase};
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};
use forge_store_test_support::SecurityScopeFixtureAuthority;

use super::super::fixture_admission::{admit_family, security_scope};
use super::super::LayoutOwnerObservationLedger;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    use SecurityScopeFixtureAuthority::{Current, Foreign};

    let page = security_scope(
        Current,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        authenticated_frame(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let page_family = admit_family(DurableArtifactFamilyId::PhysicalPage, &page);
    record(ledger, page_family, &page);

    let foreign_page = security_scope(
        Foreign,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        authenticated_frame(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    record(ledger, page_family, &foreign_page);

    for scope in [
        security_scope(
            Current,
            StoreKeyScope::StoreManagedRoot,
            StoreTenantScope::StoreInternal,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        security_scope(
            Current,
            StoreKeyScope::ArtifactEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            authenticated_frame(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        security_scope(
            Current,
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            StoreAuthenticityRequirement::not_required(),
            StoreCustodyPosture::InternalStoreCustody,
        ),
        security_scope(
            Current,
            StoreKeyScope::PageEnvelope,
            StoreTenantScope::TenantPhysicalBoundary,
            authenticated_frame(),
            StoreCustodyPosture::ExportPrepared,
        ),
    ] {
        let mismatched_family = admit_family(DurableArtifactFamilyId::PhysicalPage, &scope);
        record(ledger, mismatched_family, &scope);
    }

    let unsupported_scope = security_scope(
        Current,
        StoreKeyScope::ArtifactEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::not_required(),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let unsupported_family = admit_family(
        DurableArtifactFamilyId::PlacementAuthoritativeBranchHead,
        &unsupported_scope,
    );
    record(ledger, unsupported_family, &unsupported_scope);
}

fn record(
    ledger: &mut LayoutOwnerObservationLedger,
    family: forge_store_layout_indexes::AdmittedPhysicalArtifactFamily,
    security: &forge_store_security::StoreAdmittedSecurityScope,
) {
    let outcome = layout_declarations().admit_physical_key_domain(family, security.witnesses());
    ledger.record_physical_key_domain_admission(outcome.owner_case_observation());
}

const fn authenticated_frame() -> StoreAuthenticityRequirement {
    StoreAuthenticityRequirement::required(StoreAuthenticityRequirementClass::AuthenticatedFrame)
}
