use crate::strategy::tests_support::{admit_phase_five_scope, root_manifest_scope};
use crate::strategy::S8LayoutStrategyFamily;
use crate::strategy_registry::{
    layout_admission_registry, S8LayoutAdmissionDeferred, S8LayoutAdmissionDenial,
    S8LayoutAdmissionRequest, S8LayoutRegistrySnapshotOutcome, S8LayoutRequestedCapability,
    S8LayoutStrategyCapability, S8LayoutStrategyRegistrySnapshot,
};
use crate::{
    layout_declarations, ArtifactFamilyAccessLane, S8IndexMaintenanceMode, S8PhysicalMutationShape,
};
use forge_proof::TransitionOutcome;
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

#[test]
fn phase_eight_registry_admits_with_snapshot_and_ready_progression() {
    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let hash_equality_law = layout_declarations().declare_hash_collision_law(page_domain);
    let composite_ordering_law = layout_declarations().declare_composite_key_ordering(page_domain);

    let request = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::ordered_range(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .require_hash_equality_law(hash_equality_law)
    .require_composite_ordering_law(composite_ordering_law);

    let first_snapshot = match layout_admission_registry().admit_with(request) {
        TransitionOutcome::Success(snapshot) => snapshot,
        other => panic!("supported request should admit snapshot: {other:?}"),
    };
    let second_snapshot = match layout_admission_registry().admit_with(request) {
        TransitionOutcome::Success(snapshot) => snapshot,
        other => panic!("replayed request should admit equivalent snapshot: {other:?}"),
    };

    let snapshot: S8LayoutStrategyRegistrySnapshot = first_snapshot;
    let replayed: S8LayoutStrategyRegistrySnapshot = second_snapshot;
    assert_eq!(snapshot, replayed);

    assert_eq!(
        snapshot.granted_capability(),
        S8LayoutStrategyCapability::OrderedRange
    );
    assert!(snapshot.hash_equality_law().is_some());
    assert!(snapshot.composite_ordering_law().is_some());

    let ready = layout_admission_registry().try_admit_ready(snapshot);
    let replayed_ready = layout_admission_registry().try_admit_ready(replayed);
    assert_eq!(ready, replayed_ready);
    match ready {
        TransitionOutcome::Success(admitted) => {
            assert_eq!(
                admitted.family(),
                S8LayoutStrategyFamily::BaselineBTreeRange
            );
            assert!(admitted.supports_range_access());
        }
        other => panic!("supported snapshot should become ready: {other:?}"),
    };
}

#[test]
fn phase_eight_registry_defers_readiness_without_live_exact_maintenance_witness() {
    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let coverage = crate::access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                page_lifecycle.declaration().family(),
            ),
            forge_store_physical_format::PhysicalEpoch::from_raw(11).unwrap(),
        )
        .expect("coverage should admit");
    let request = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .require_exact_materialization(coverage);

    let snapshot = match layout_admission_registry().admit_with(request) {
        TransitionOutcome::Success(snapshot) => snapshot,
        other => panic!("request should admit before readiness: {other:?}"),
    };

    assert_eq!(
        layout_admission_registry().try_admit_ready(snapshot),
        TransitionOutcome::Deferred(
            S8LayoutAdmissionDeferred::LiveExactMaintenanceWitnessRequired {
                family: S8LayoutStrategyFamily::BaselineBTreeRange,
                capability: S8LayoutStrategyCapability::PointLookup,
            }
        )
    );
}

#[test]
fn phase_eight_registry_denies_scope_mode_mutation_and_capability_mismatches_stably() {
    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let (_root_lifecycle, root_domain) = root_manifest_scope();
    let (wal_lifecycle, wal_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let root_hash_law = layout_declarations().declare_hash_collision_law(root_domain);
    let root_composite_law = layout_declarations().declare_composite_key_ordering(root_domain);

    let unsupported_streaming = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::blob_streaming(),
        ArtifactFamilyAccessLane::HotPath,
    );
    let scope_mismatch = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .within_scope_partition(root_domain.scope());
    let verifier_replay_mismatch = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .under_maintenance_mode(S8IndexMaintenanceMode::VerifierOnly);
    let mutation_mismatch = S8LayoutAdmissionRequest::new(
        wal_lifecycle,
        wal_domain,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .for_mutation_shape(S8PhysicalMutationShape::PointRewrite);
    let hash_mismatch = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .require_hash_equality_law(root_hash_law);
    let composite_mismatch = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::ordered_range(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .require_composite_ordering_law(root_composite_law);

    assert_eq!(
        layout_admission_registry().admit(unsupported_streaming),
        TransitionOutcome::Denied(
            S8LayoutAdmissionDenial::StrategyDoesNotSupportRequestedCapability {
                family: S8LayoutStrategyFamily::BaselineBTreeRange,
                capability: S8LayoutRequestedCapability::BlobStreaming,
            }
        )
    );
    assert_eq!(
        layout_admission_registry().admit(scope_mismatch),
        TransitionOutcome::Denied(
            S8LayoutAdmissionDenial::RequestedScopeDoesNotMatchKeyDomain {
                requested_scope: root_domain.scope(),
                key_domain_scope: page_domain.scope(),
            }
        )
    );
    assert_eq!(
        layout_admission_registry().admit(verifier_replay_mismatch),
        TransitionOutcome::Denied(
            S8LayoutAdmissionDenial::MaintenanceModeIncompatibleWithRequestedLane {
                family: S8LayoutStrategyFamily::BaselineBTreeRange,
                maintenance_mode: S8IndexMaintenanceMode::VerifierOnly,
                requested_lane: ArtifactFamilyAccessLane::HotPath,
            }
        )
    );
    assert_eq!(
        layout_admission_registry().admit(hash_mismatch),
        TransitionOutcome::Denied(
            S8LayoutAdmissionDenial::HashEqualityLawDoesNotMatchKeyDomain {
                requested_domain: root_domain,
                strategy_domain: page_domain,
            }
        )
    );
    assert_eq!(
        layout_admission_registry().admit(composite_mismatch),
        TransitionOutcome::Denied(
            S8LayoutAdmissionDenial::CompositeOrderingLawDoesNotMatchKeyDomain {
                requested_domain: root_domain,
                strategy_domain: page_domain,
            }
        )
    );
    let first = layout_admission_registry().admit(mutation_mismatch);
    let second = layout_admission_registry().admit(mutation_mismatch);
    assert_eq!(
        first,
        TransitionOutcome::Denied(
            S8LayoutAdmissionDenial::MutationShapeIncompatibleWithStrategy {
                family: S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
                mutation_shape: S8PhysicalMutationShape::PointRewrite,
            }
        )
    );
    assert_eq!(first, second);
}

#[test]
fn phase_eight_registry_public_facade_exports_complete_snapshot_chain() {
    fn require_public_surface(
        _request: S8LayoutAdmissionRequest,
        _snapshot: S8LayoutStrategyRegistrySnapshot,
        _capability: S8LayoutStrategyCapability,
        _outcome: S8LayoutRegistrySnapshotOutcome,
        _deferred: S8LayoutAdmissionDeferred,
    ) {
    }

    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    );
    let snapshot_outcome = layout_admission_registry().admit_with(request);
    let snapshot = match snapshot_outcome {
        TransitionOutcome::Success(snapshot) => snapshot,
        other => panic!("public snapshot path should succeed in export proof: {other:?}"),
    };

    require_public_surface(
        request,
        snapshot,
        S8LayoutStrategyCapability::PointLookup,
        TransitionOutcome::Success(snapshot),
        S8LayoutAdmissionDeferred::ExactCoverageEvidenceRequired {
            family: S8LayoutStrategyFamily::BaselineBTreeRange,
            capability: S8LayoutStrategyCapability::PointLookup,
        },
    );
}
