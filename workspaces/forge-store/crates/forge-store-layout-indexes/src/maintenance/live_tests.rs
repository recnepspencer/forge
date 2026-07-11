use super::{S8IndexMaintenanceTransitionView, S8LayoutMutationAdmissionView};
use crate::facade::layout_maintenance;
use crate::strategy::tests_support::{admit_btree_page_strategy, admit_lsm_wal_strategy};
use crate::{
    access_planning, ArtifactFamilyAccessLane, S8ExactPublicationAuthoritySource,
    S8IndexLagOutcome, S8IndexLagWitness, S8IndexMaintenanceFailureOutcome, S8IndexMaintenanceMode,
    S8IndexMaintenanceTransitionOutcome, S8IndexPublicationProtocol, S8LagReason,
    S8LayoutMutationAdmissionOutcome, S8LiveMaintenanceRequest, S8PhysicalMutationShape,
};
use forge_store_physical_format::{
    PhysicalEpoch, PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference,
};
use forge_store_recovery_physics::LogSequenceNumber;

#[test]
fn live_exact_root_publication_requires_lower_epoch_binding_capability() {
    let strategy = admit_btree_page_strategy();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            PhysicalEpoch::from_raw(17).unwrap(),
        )
        .unwrap();

    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        S8IndexMaintenanceMode::SynchronousExact,
        S8PhysicalMutationShape::ObservationOnly,
        S8IndexPublicationProtocol::StableRootSwap,
    )
    .with_exact_publication_authority(validated_root_publication_authority(17))
    .with_exact_coverage(coverage);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        S8LayoutMutationAdmissionView::Denied(
            S8IndexMaintenanceFailureOutcome::LowerPublicationCapabilityRequired {
                missing: crate::S8PublicationProofRequirement::RootEpochPublicationBinding,
                ..
            }
        )
    ));
}

#[test]
fn exact_manifest_publication_stays_denied_without_lower_owned_manifest_witness() {
    let strategy = admit_lsm_wal_strategy();
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(41),
        )
        .unwrap();

    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        S8IndexMaintenanceMode::SynchronousExact,
        S8PhysicalMutationShape::ObservationOnly,
        S8IndexPublicationProtocol::StableManifestInstall,
    )
    .with_exact_coverage(coverage);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        S8LayoutMutationAdmissionView::Denied(
            S8IndexMaintenanceFailureOutcome::LowerPublicationCapabilityRequired { .. }
        )
    ));
}

#[test]
fn lagged_live_maintenance_requires_explicit_lag_witness() {
    let strategy = admit_lsm_wal_strategy();
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(29),
        )
        .unwrap();
    let lag = S8IndexLagWitness::new(
        strategy.lifecycle().declaration().family(),
        coverage,
        S8IndexMaintenanceMode::AsynchronousLagged,
        S8IndexPublicationProtocol::DeferredCatchUp,
        S8LagReason::BackgroundCatchUp,
    );

    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        S8IndexMaintenanceMode::AsynchronousLagged,
        S8PhysicalMutationShape::ObservationOnly,
        S8IndexPublicationProtocol::DeferredCatchUp,
    )
    .with_exact_coverage(coverage)
    .with_lag_witness(lag);

    let (plan, witness) = layout_maintenance()
        .admit_mutation(request)
        .into_lagged()
        .expect("lagged maintenance should stay caller-visible");
    assert_eq!(witness, lag);

    let lowered = layout_maintenance()
        .lower_protocol(plan)
        .into_lagged()
        .expect("lagged maintenance should lower as lagged");
    assert_eq!(
        layout_maintenance().inspect_lag(&lowered),
        S8IndexLagOutcome::Lagged(lag)
    );
}

#[test]
fn point_rewrite_without_lower_owned_mutation_capability_is_denied() {
    let strategy = admit_lsm_wal_strategy();
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(21),
        )
        .unwrap();
    let lag = S8IndexLagWitness::new(
        strategy.lifecycle().declaration().family(),
        coverage,
        S8IndexMaintenanceMode::AsynchronousLagged,
        S8IndexPublicationProtocol::DeferredCatchUp,
        S8LagReason::BackgroundCatchUp,
    );

    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        S8IndexMaintenanceMode::AsynchronousLagged,
        S8PhysicalMutationShape::LogStructuredAppend,
        S8IndexPublicationProtocol::DeferredCatchUp,
    )
    .with_exact_coverage(coverage)
    .with_lag_witness(lag);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        S8LayoutMutationAdmissionView::Denied(
            S8IndexMaintenanceFailureOutcome::LowerMutationCapabilityRequired { .. }
        )
    ));
}

#[test]
fn exact_maintenance_rejects_deferred_publication_protocols() {
    let strategy = admit_lsm_wal_strategy();
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(31),
        )
        .unwrap();

    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        S8IndexMaintenanceMode::SynchronousExact,
        S8PhysicalMutationShape::ObservationOnly,
        S8IndexPublicationProtocol::DeferredCatchUp,
    )
    .with_exact_coverage(coverage);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        S8LayoutMutationAdmissionView::Denied(
            S8IndexMaintenanceFailureOutcome::PublicationProtocolIncompatibleWithStrategy { .. }
                | S8IndexMaintenanceFailureOutcome::ReplayStablePublicationRequired { .. }
        )
    ));
}

#[test]
fn verifier_only_mode_lowers_without_claiming_exact_publication() {
    let strategy = admit_lsm_wal_strategy();
    let coverage = access_planning()
        .exact_wal_lsn_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            LogSequenceNumber::new(37),
        )
        .unwrap();
    let lag = S8IndexLagWitness::new(
        strategy.lifecycle().declaration().family(),
        coverage,
        S8IndexMaintenanceMode::VerifierOnly,
        S8IndexPublicationProtocol::VerifierObservationOnly,
        S8LagReason::AdvisoryResidue,
    );
    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::VerifierPath,
        S8IndexMaintenanceMode::VerifierOnly,
        S8PhysicalMutationShape::ObservationOnly,
        S8IndexPublicationProtocol::VerifierObservationOnly,
    )
    .with_exact_coverage(coverage)
    .with_lag_witness(lag);

    let (plan, witness) = layout_maintenance()
        .admit_mutation(request)
        .into_deferred()
        .expect("verifier mode should stay non-exact");
    assert_eq!(witness, lag);

    let lowered = layout_maintenance()
        .lower_protocol(plan)
        .into_verifier_only()
        .expect("verifier mode should lower to verifier-only");
    assert_eq!(
        layout_maintenance().inspect_lag(&lowered),
        S8IndexLagOutcome::Lagged(lag)
    );
    assert!(layout_maintenance().certify_live_exact(&lowered).is_none());
}

#[test]
fn root_publication_validation_cannot_be_reused_for_mismatched_exact_coverage() {
    let strategy = admit_btree_page_strategy();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(
                strategy.lifecycle().declaration().family(),
            ),
            PhysicalEpoch::from_raw(19).unwrap(),
        )
        .unwrap();

    let request = S8LiveMaintenanceRequest::new(
        strategy.lifecycle(),
        strategy.key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        S8IndexMaintenanceMode::SynchronousExact,
        S8PhysicalMutationShape::ObservationOnly,
        S8IndexPublicationProtocol::StableRootSwap,
    )
    .with_exact_publication_authority(validated_root_publication_authority(17))
    .with_exact_coverage(coverage);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        S8LayoutMutationAdmissionView::Denied(
            S8IndexMaintenanceFailureOutcome::LowerPublicationCapabilityRequired {
                missing: crate::S8PublicationProofRequirement::RootEpochPublicationBinding,
                ..
            }
        )
    ));
}

fn validated_root_publication_authority(generation: u64) -> S8ExactPublicationAuthoritySource {
    let root = PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(PhysicalRootReference::from_raw(1).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(generation).unwrap());
    let admission = PhysicalReferenceAuthority::for_canonical_physical_format().admit_root_publication(root);
    let validation = PhysicalReferenceAuthority::for_canonical_physical_format()
        .validate_root_publication(admission, root)
        .expect("test root publication should validate");
    S8ExactPublicationAuthoritySource::current_root_publication(validation)
}
