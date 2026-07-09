use super::{
    layout_customization_boundary, S8FutureLayoutCapabilityRequest,
    S8FutureLayoutCustomizationDenial, S8FutureLayoutCustomizationRequest,
    S8FutureLayoutWorkloadEnvelope,
};
use crate::strategy::tests_support::{admit_phase_five_scope, root_manifest_scope};
use crate::strategy::S8LayoutStrategyFamily;
use crate::strategy_registry::{S8LayoutAdmissionDenial, S8LayoutRequestedCapability};
use worth_proof::TransitionOutcome;
use worth_store_contracts::DurableArtifactFamilyId;
use worth_store_security::{
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
    StoreKeyScope, StoreTenantScope,
};

#[test]
fn phase_nine_customization_admits_supported_foreground_requests_as_registry_snapshots() {
    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let request = S8FutureLayoutCustomizationRequest::new(
        page_lifecycle,
        S8FutureLayoutCapabilityRequest::ordered_range(page_domain),
        S8FutureLayoutWorkloadEnvelope::foreground_bounded_traversal(),
    );

    match layout_customization_boundary().admit(request) {
        TransitionOutcome::Success(admitted) => {
            let snapshot = admitted.registry_snapshot();
            assert_eq!(
                snapshot.admitted_strategy().family(),
                S8LayoutStrategyFamily::BaselineBTreeRange
            );
            assert_eq!(
                snapshot.request().family(),
                S8LayoutStrategyFamily::BaselineBTreeRange
            );
        }
        other => panic!("supported customization request should admit: {other:?}"),
    }
}

#[test]
fn phase_nine_customization_denies_envelope_masquerade_and_unready_projection_requests() {
    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let masquerade = S8FutureLayoutCustomizationRequest::new(
        page_lifecycle,
        S8FutureLayoutCapabilityRequest::ordered_range(page_domain),
        S8FutureLayoutWorkloadEnvelope::verifier_corpus_inspection(),
    );
    let rebuildable_projection = S8FutureLayoutCustomizationRequest::new(
        page_lifecycle,
        S8FutureLayoutCapabilityRequest::rebuildable_projection(page_domain),
        S8FutureLayoutWorkloadEnvelope::background_rebuild_projection(),
    );

    assert_eq!(
        layout_customization_boundary().admit(masquerade),
        TransitionOutcome::Denied(
            S8FutureLayoutCustomizationDenial::WorkloadEnvelopeDoesNotSupportCapability {
                capability: S8FutureLayoutCapabilityRequest::ordered_range(page_domain),
                envelope: S8FutureLayoutWorkloadEnvelope::VerifierCorpusInspection,
            }
        )
    );
    assert_eq!(
        layout_customization_boundary().admit(rebuildable_projection),
        TransitionOutcome::Denied(
            S8FutureLayoutCustomizationDenial::RebuildableProjectionNotYetSupported {
                key_domain: page_domain,
            }
        )
    );
}

#[test]
fn phase_nine_customization_propagates_store_missing_layout_facts_honestly() {
    let (wal_lifecycle, wal_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PublicationWalIntent,
        StoreKeyScope::WalCheckpointEnvelope,
        StoreTenantScope::StoreInternal,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedWalRecord,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let unsupported_range = S8FutureLayoutCustomizationRequest::new(
        wal_lifecycle,
        S8FutureLayoutCapabilityRequest::ordered_range(wal_domain),
        S8FutureLayoutWorkloadEnvelope::foreground_bounded_traversal(),
    );
    let verifier_scan = S8FutureLayoutCustomizationRequest::new(
        wal_lifecycle,
        S8FutureLayoutCapabilityRequest::verifier_declared_scan(wal_domain),
        S8FutureLayoutWorkloadEnvelope::verifier_corpus_inspection(),
    );

    assert_eq!(
        layout_customization_boundary().admit(unsupported_range),
        TransitionOutcome::Denied(S8FutureLayoutCustomizationDenial::StoreAdmissionDenied(
            S8LayoutAdmissionDenial::StrategyDoesNotSupportRequestedCapability {
                family: S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
                capability: S8LayoutRequestedCapability::OrderedRange,
            }
        ))
    );
    assert_eq!(
        layout_customization_boundary().admit(verifier_scan),
        TransitionOutcome::Denied(S8FutureLayoutCustomizationDenial::StoreAdmissionDenied(
            S8LayoutAdmissionDenial::RequestedLaneDoesNotMatchFamilyLane {
                family: S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
                requested_lane: crate::ArtifactFamilyAccessLane::VerifierPath,
                declared_lane: crate::ArtifactFamilyAccessLane::HotPath,
            }
        ))
    );
}

#[test]
fn phase_nine_customization_denies_key_domains_without_supported_store_strategy() {
    let (root_manifest_lifecycle, root_manifest_domain) = root_manifest_scope();
    let unsupported = S8FutureLayoutCustomizationRequest::new(
        root_manifest_lifecycle,
        S8FutureLayoutCapabilityRequest::point_lookup(root_manifest_domain),
        S8FutureLayoutWorkloadEnvelope::foreground_low_fanout(),
    );

    assert_eq!(
        layout_customization_boundary().admit(unsupported),
        TransitionOutcome::Denied(
            S8FutureLayoutCustomizationDenial::NoStrategySupportsRequestedCapability {
                capability: S8FutureLayoutCapabilityRequest::point_lookup(root_manifest_domain),
                key_domain: root_manifest_domain,
            }
        )
    );
}
