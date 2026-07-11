use crate::strategy::registry::{
    layout_admission_registry, S8LayoutAdmissionDenial, S8LayoutAdmissionRequest,
    S8LayoutAdmissionView, S8LayoutRequestedCapability, S8LayoutStrategyCapability,
};
use crate::strategy::tests_support::{admit_phase_five_scope, root_manifest_scope};
use crate::strategy::S8LayoutStrategyFamily;
use crate::{ArtifactFamilyAccessLane, S8IndexMaintenanceMode, S8PhysicalMutationShape};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

#[test]
fn registry_admits_supported_requests_to_stable_owner_snapshots() {
    let (lifecycle, domain) = page_scope();
    let request = S8LayoutAdmissionRequest::new(
        lifecycle,
        domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    );

    let first = layout_admission_registry().admit(request).unwrap();
    let second = layout_admission_registry().admit(request).unwrap();

    assert_eq!(first, second);
    assert_eq!(
        first.admitted_strategy().family(),
        S8LayoutStrategyFamily::BaselineBTreeRange
    );
    assert_eq!(
        first.granted_capability(),
        S8LayoutStrategyCapability::PointLookup
    );
}

#[test]
fn registry_denies_unsupported_capability_and_scope_mismatch() {
    let (lifecycle, domain) = page_scope();
    let (_, root_domain) = root_manifest_scope();
    let unsupported = S8LayoutAdmissionRequest::new(
        lifecycle,
        domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::blob_streaming(),
        ArtifactFamilyAccessLane::HotPath,
    );
    let scope_mismatch = S8LayoutAdmissionRequest::new(
        lifecycle,
        domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .within_scope_partition(root_domain.scope());

    assert!(matches!(
        layout_admission_registry().admit(unsupported).view(),
        S8LayoutAdmissionView::Denied(
            S8LayoutAdmissionDenial::StrategyDoesNotSupportRequestedCapability { .. }
        )
    ));
    assert!(matches!(
        layout_admission_registry().admit(scope_mismatch).view(),
        S8LayoutAdmissionView::Denied(
            S8LayoutAdmissionDenial::RequestedScopeDoesNotMatchKeyDomain { .. }
        )
    ));
}

#[test]
fn registry_denies_mode_and_mutation_mismatches() {
    let (page_lifecycle, page_domain) = page_scope();
    let verifier = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .under_maintenance_mode(S8IndexMaintenanceMode::VerifierOnly);
    let (wal_lifecycle, wal_domain) = wal_scope();
    let mutation = S8LayoutAdmissionRequest::new(
        wal_lifecycle,
        wal_domain,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .for_mutation_shape(S8PhysicalMutationShape::PointRewrite);

    assert!(matches!(
        layout_admission_registry().admit(verifier).view(),
        S8LayoutAdmissionView::Denied(
            S8LayoutAdmissionDenial::MaintenanceModeIncompatibleWithRequestedLane { .. }
        )
    ));
    assert!(matches!(
        layout_admission_registry().admit(mutation).view(),
        S8LayoutAdmissionView::Denied(
            S8LayoutAdmissionDenial::MutationShapeIncompatibleWithStrategy { .. }
        )
    ));
}

fn page_scope() -> (
    crate::ArtifactFamilyLifecycleAdmission,
    crate::PhysicalKeyDomainWitness,
) {
    admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}

fn wal_scope() -> (
    crate::ArtifactFamilyLifecycleAdmission,
    crate::PhysicalKeyDomainWitness,
) {
    admit_phase_five_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    )
}
