use super::{IndexPublicationProtocol, LayoutMutationAdmissionView};
use crate::maintenance::layout_maintenance;
use crate::strategy::tests_support::{admit_btree_page_strategy, admit_lsm_wal_strategy};
use crate::{
    access_planning, ArtifactFamilyAccessLane, ExactPublicationAuthoritySource, IndexLagOutcome,
    IndexLagWitness, IndexMaintenanceFailureOutcome, IndexMaintenanceMode, LagReason,
    LiveMaintenanceRequest, PhysicalMutationShape,
};
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference,
};
use forge_store_recovery_physics::LogSequenceNumber;

#[test]
fn live_exact_root_publication_accepts_matching_lower_epoch_binding() {
    let strategy = admit_btree_page_strategy();
    let authority = validated_root_publication_authority(17);
    let ExactPublicationAuthoritySource::CurrentRootPublication(validation) = authority else {
        panic!("root authority helper must issue root publication authority")
    };
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let coverage = access_planning()
        .admit_btree_publication_materialization(strategy.admitted_family(), &catalog, validation)
        .unwrap();

    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableRootSwap,
    )
    .with_exact_publication_authority(authority)
    .with_exact_coverage(coverage.coverage().clone());

    let plan = layout_maintenance()
        .admit_mutation(request)
        .into_exact()
        .expect("matching lower root publication must admit exact maintenance");
    let lowered = layout_maintenance().lower_exact(plan);
    assert!(layout_maintenance().certify_live_exact(&lowered).is_some());
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

    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableManifestInstall,
    )
    .with_exact_coverage(coverage);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::ExactPublicationAuthorityRequired { .. }
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
    let lag = IndexLagWitness::new(
        strategy.lifecycle().declaration().family(),
        coverage.clone(),
        IndexMaintenanceMode::AsynchronousLagged,
        IndexPublicationProtocol::DeferredCatchUp,
        LagReason::BackgroundCatchUp,
    );

    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::AsynchronousLagged,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::DeferredCatchUp,
    )
    .with_exact_coverage(coverage.clone())
    .with_lag_witness(lag.clone());

    let (plan, witness) = layout_maintenance()
        .admit_mutation(request)
        .into_lagged()
        .expect("lagged maintenance should stay caller-visible");
    assert_eq!(witness, lag);

    let lowered = layout_maintenance().lower_lagged(plan);
    assert_eq!(
        layout_maintenance().inspect_lagged(&lowered),
        IndexLagOutcome::Lagged(lag)
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
    let lag = IndexLagWitness::new(
        strategy.lifecycle().declaration().family(),
        coverage.clone(),
        IndexMaintenanceMode::AsynchronousLagged,
        IndexPublicationProtocol::DeferredCatchUp,
        LagReason::BackgroundCatchUp,
    );

    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::AsynchronousLagged,
        PhysicalMutationShape::LogStructuredAppend,
        IndexPublicationProtocol::DeferredCatchUp,
    )
    .with_exact_coverage(coverage.clone())
    .with_lag_witness(lag);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::LowerMutationCapabilityRequired { .. }
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

    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::DeferredCatchUp,
    )
    .with_exact_coverage(coverage);

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::PublicationProtocolIncompatibleWithStrategy { .. }
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
    let lag = IndexLagWitness::new(
        strategy.lifecycle().declaration().family(),
        coverage.clone(),
        IndexMaintenanceMode::VerifierOnly,
        IndexPublicationProtocol::VerifierObservationOnly,
        LagReason::AdvisoryResidue,
    );
    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::VerifierPath,
        IndexMaintenanceMode::VerifierOnly,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::VerifierObservationOnly,
    )
    .with_exact_coverage(coverage.clone())
    .with_lag_witness(lag.clone());

    let (plan, witness) = layout_maintenance()
        .admit_mutation(request)
        .into_verifier()
        .expect("verifier mode should stay non-exact");
    assert_eq!(witness, lag);

    let lowered = layout_maintenance().lower_verifier(plan);
    assert_eq!(
        layout_maintenance().inspect_verifier(&lowered),
        IndexLagOutcome::Lagged(lag)
    );
}

#[test]
fn root_publication_validation_cannot_be_reused_for_mismatched_exact_coverage() {
    let strategy = admit_btree_page_strategy();
    let coverage_authority = validated_root_publication_authority(19);
    let ExactPublicationAuthoritySource::CurrentRootPublication(coverage_validation) =
        coverage_authority
    else {
        panic!("root authority helper must issue root publication authority")
    };
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let coverage = access_planning()
        .admit_btree_publication_materialization(
            strategy.admitted_family(),
            &catalog,
            coverage_validation,
        )
        .unwrap();

    let request = LiveMaintenanceRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableRootSwap,
    )
    .with_exact_publication_authority(validated_root_publication_authority(17))
    .with_exact_coverage(coverage.coverage().clone());

    assert!(matches!(
        layout_maintenance().admit_mutation(request).view(),
        LayoutMutationAdmissionView::Denied(
            IndexMaintenanceFailureOutcome::PublicationAuthorityDoesNotMatchExactCoverage { .. }
        )
    ));
}

pub(super) fn validated_root_publication_authority(
    generation: u64,
) -> ExactPublicationAuthoritySource {
    let root = PhysicalGenerationAuthority::for_canonical_physical_format()
        .root_publication_cell(PhysicalRootReference::from_raw(1).unwrap())
        .with_root_publication_generation(PhysicalGeneration::from_raw(generation).unwrap());
    let admission =
        PhysicalReferenceAuthority::for_canonical_physical_format().admit_root_publication(root);
    let validation = PhysicalReferenceAuthority::for_canonical_physical_format()
        .validate_root_publication(admission, root)
        .expect("test root publication should validate");
    ExactPublicationAuthoritySource::current_root_publication(validation)
}
