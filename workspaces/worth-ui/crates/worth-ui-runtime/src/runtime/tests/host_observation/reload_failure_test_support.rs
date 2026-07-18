use super::identity_match_graph_test_support::{artifact_from_nodes, component_node};
use super::query_binding_comparison_test_support::{
    lifecycle_drift_query_app, query_artifact, standard_query_app,
};
use crate::facade::WorthUi;
use crate::runtime::{
    WorthUiActiveRuntimeObservation, WorthUiCandidateArtifactBundle,
    WorthUiCandidateDependencyMetadata, WorthUiLastValidObservation, WorthUiReloadFailure,
    WorthUiReloadFailureCounters, WorthUiReloadFailureStage, WorthUiReloadPreservationReceipt,
    WorthUiReplacementCandidateDenial,
};
use crate::source::{WorthUiArtifact, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis};

pub(super) fn assert_failure_preserves_active_runtime(
    failure: WorthUiReloadFailure,
    previous_active: WorthUiActiveRuntimeObservation,
    previous_last_valid: WorthUiLastValidObservation,
    expected_stage: WorthUiReloadFailureStage,
) {
    assert_eq!(failure.denial().stage(), expected_stage);
    assert_eq!(
        failure.denial().stage(),
        failure.failed_activation_report().stage()
    );
    assert_preservation_receipt(
        failure.preservation_receipt(),
        &previous_active,
        &previous_last_valid,
    );
    assert_preserved_counters(failure.counters());
    assert_preserved_counters(failure.failed_activation_report().counters());
    assert!(!failure
        .failed_activation_report()
        .fallback_runtime_created());
    assert_eq!(
        failure
            .failed_activation_report()
            .preserved_active_artifact_digest(),
        previous_active.artifact_digest()
    );
    assert_eq!(
        failure
            .failed_activation_report()
            .preserved_active_plan_digest(),
        previous_active.active_plan_digest()
    );
}

fn assert_preservation_receipt(
    receipt: WorthUiReloadPreservationReceipt,
    previous_active: &WorthUiActiveRuntimeObservation,
    previous_last_valid: &WorthUiLastValidObservation,
) {
    assert_eq!(
        receipt.active_artifact_digest(),
        previous_active.artifact_digest()
    );
    assert_eq!(
        receipt.active_plan_digest(),
        previous_active.active_plan_digest()
    );
    assert_eq!(
        receipt.active_snapshot_digest(),
        previous_active.snapshot_digest()
    );
    assert_eq!(receipt.active_lifecycle(), previous_active.lifecycle());
    assert_eq!(receipt.active_status(), previous_active.status());
    assert_eq!(receipt.active_frame_epoch(), previous_active.frame_epoch());
    assert_eq!(
        receipt.last_valid_artifact_digest(),
        previous_last_valid.artifact_digest()
    );
    assert_eq!(
        receipt.last_valid_plan_digest(),
        previous_last_valid.active_plan_digest()
    );
    assert_eq!(
        receipt.last_valid_frame_epoch(),
        previous_last_valid.recorded_frame_epoch()
    );
}

pub(super) fn assert_preserved_counters(counters: WorthUiReloadFailureCounters) {
    assert_eq!(counters.preservation_receipt_count(), 1);
    assert_eq!(counters.active_state_mutation_count(), 0);
    assert_eq!(counters.durable_state_mutation_count(), 0);
    assert_eq!(counters.query_binding_mutation_count(), 0);
    assert_eq!(counters.fallback_runtime_creation_count(), 0);
    assert_eq!(counters.source_reparse_count(), 0);
    assert_eq!(counters.registry_rebuild_count(), 0);
    assert_eq!(counters.semantic_replanning_count(), 0);
    assert_eq!(counters.query_replanning_count(), 0);
}

pub(super) fn missing_artifact_candidate_denial() -> WorthUiReplacementCandidateDenial {
    let artifact = invalid_candidate_artifact("component:missing-artifact");
    WorthUiCandidateArtifactBundle::from_optional_parts_for_test(artifact, None, None, None)
        .expect_err("missing artifact proof denies")
}

pub(super) fn missing_dependency_candidate_denial() -> WorthUiReplacementCandidateDenial {
    let artifact = invalid_candidate_artifact("component:missing-dependency-metadata");
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
        artifact,
        Some(artifact_digest),
        None,
        None,
    )
    .expect_err("missing dependency metadata denies")
}

pub(super) fn missing_lowering_basis_candidate_denial() -> WorthUiReplacementCandidateDenial {
    let artifact = invalid_candidate_artifact("component:missing-lowering-basis");
    let artifact_digest =
        WorthUiArtifactDigestor::digest(&artifact, WorthUiArtifactEquivalenceBasis::semantic());
    let dependency_metadata = WorthUiCandidateDependencyMetadata::derive_for_artifact(&artifact);
    WorthUiCandidateArtifactBundle::from_optional_parts_for_test(
        artifact,
        Some(artifact_digest),
        Some(dependency_metadata),
        None,
    )
    .expect_err("missing lowering basis denies")
}

pub(super) fn stale_dependency_candidate_denial() -> WorthUiReplacementCandidateDenial {
    let stale_app = standard_query_app();
    let candidate_app = lifecycle_drift_query_app();
    let stale_artifact = query_artifact(&stale_app, "workspace.view_binding.selection");
    let candidate_artifact = query_artifact(&candidate_app, "workspace.view_binding.selection");
    let candidate_digest = WorthUiArtifactDigestor::digest(
        &candidate_artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    );
    let stale_metadata = WorthUiCandidateDependencyMetadata::derive_for_artifact(&stale_artifact)
        .with_artifact_digest_for_test(candidate_digest);
    WorthUiCandidateArtifactBundle::seal(
        candidate_artifact,
        stale_metadata,
        WorthUi::app()
            .freeze()
            .expect("application preparation should succeed")
            .capabilities()
            .digest(),
    )
    .expect_err("stale dependency metadata denies")
}

fn invalid_candidate_artifact(component_identity: &str) -> WorthUiArtifact {
    artifact_from_nodes([("app/main.wui", vec![component_node(component_identity, 0)])])
}
