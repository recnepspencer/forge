use super::model::LivePerturbationClass;
use super::MilestoneFiveLiveCertificationAdapter;
use crate::harness::certification::{milestone_five_requirements, unmet_required_rows};

#[test]
fn live_query_patch_policy_adapter_emits_named_matrix() {
    let matrix =
        MilestoneFiveLiveCertificationAdapter::live_promotion_convergence_and_suppression_test();

    assert_eq!(
        matrix.suite_name,
        "Live Promotion Convergence And Suppression Test"
    );
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "detail-live-patch-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "ordered-collection-live-patch-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "bounded-materialization-live-patch-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "detail-live-convergence"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "ordered-collection-live-convergence"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "bounded-materialization-live-convergence"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "detail-live-replay-end-state-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "detail-live-replay-stepwise-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "ordered-collection-live-replay-end-state-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "ordered-collection-live-replay-stepwise-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "bounded-materialization-live-replay-end-state-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "bounded-materialization-live-replay-stepwise-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "irrelevant-update-suppression"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "live-progress-basis-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "refresh-fallback-equivalence"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "coalesced-sequence-replay-parity"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "patch-width-budget-overflow-policy"));
    assert!(matrix
        .rows
        .iter()
        .any(|row| row.row_name == "work-avoided-counter-parity"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "forbidden-width-budget-overflow-behavior"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "forbidden-coalescing-class"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "forbidden-refresh-escape-hatch"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "non-monotonic-change-sequence"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "gapful-change-sequence"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "invalid-live-basis-promotion"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-patch-family"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "unsupported-live-family"));
    assert!(matrix
        .rejection_rows
        .iter()
        .any(|row| row.row_name == "raw-cdc-leakage-forbidden"));
}

#[test]
fn live_query_patch_policy_matrix_meets_milestone_five_required_rows() {
    let matrix =
        MilestoneFiveLiveCertificationAdapter::live_promotion_convergence_and_suppression_test();
    let requirements = milestone_five_requirements();
    let missing = unmet_required_rows(
        &matrix,
        requirements.required_canonical_rows,
        requirements.required_rejection_rows,
    );

    assert!(
        missing.is_empty(),
        "missing milestone five rows: {missing:?}"
    );
}

#[test]
fn live_query_patch_policy_artifact_is_offline_ready() {
    let artifact =
        MilestoneFiveLiveCertificationAdapter::
            live_promotion_convergence_and_suppression_certification_artifact();

    assert_eq!(
        artifact.suite_name,
        "Live Promotion Convergence And Suppression Test"
    );
    assert!(!artifact.certification_bundle_digest.is_empty());
    assert!(!artifact.coverage_matrix_digest.is_empty());
    assert!(artifact.counter_snapshot.live_patch_delivery_count() > 0);
    assert!(artifact.counter_snapshot.live_invalidation_event_count() > 0);
    assert!(artifact.counter_snapshot.live_relevance_match_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .live_irrelevant_suppression_count()
            > 0
    );
    assert!(artifact.counter_snapshot.live_patch_count() > 0);
    assert!(artifact.counter_snapshot.live_patch_field_delta_count() > 0);
    assert!(artifact.counter_snapshot.live_collection_reorder_count() > 0);
    assert!(artifact.counter_snapshot.live_materialization_patch_count() > 0);
    assert!(artifact.counter_snapshot.live_refresh_fallback_count() > 0);
    assert!(artifact.counter_snapshot.live_replay_change_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .live_coalesced_change_bundle_count()
            > 0
    );
    assert!(artifact.counter_snapshot.live_patch_width_overflow_count() > 0);
    assert!(artifact.counter_snapshot.live_refresh_cost_class_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .live_work_avoided_by_irrelevance_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .live_work_avoided_by_stable_ordering_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .live_work_avoided_by_scope_proof_count()
            > 0
    );
    assert_eq!(
        artifact.counter_snapshot.live_executor_rediscovery_count(),
        0
    );
    assert!(artifact.counter_snapshot.live_progress_advance_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .live_non_monotonic_sequence_rejection_count()
            > 0
    );
    assert!(artifact.counter_snapshot.live_change_sequence_gap_count() > 0);
    assert!(
        artifact
            .counter_snapshot
            .live_invalid_promotion_rejection_count()
            > 0
    );
    assert!(
        artifact
            .counter_snapshot
            .live_unsupported_patch_family_rejection_count()
            > 0
    );
    assert!(artifact
        .matrix
        .rows
        .iter()
        .all(|row| row.has_required_outputs()));
    let uncovered_rows: Vec<_> = artifact
        .matrix
        .rows
        .iter()
        .filter(|row| !row.has_hostile_coverage())
        .map(|row| row.row_name)
        .collect();
    assert!(
        uncovered_rows.is_empty(),
        "rows without hostile coverage: {uncovered_rows:?}"
    );
    assert!(artifact
        .matrix
        .rows
        .iter()
        .all(|row| !row.control_lane.query_digest.is_empty()));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .all(|row| !row.control_lane.result_digest.is_empty()));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .all(|row| !row.control_lane.delivery_digest.is_empty()));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .all(|row| !row.control_lane.replay_digest.is_empty()));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| !row.control_lane.replay_step_delivery_digests.is_empty()));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .all(|row| row.has_required_outputs()));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .all(|row| row.has_hostile_coverage()));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::DetailReplayEndStateParity));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::DetailReplayStepwiseParity));
    assert!(artifact.matrix.rows.iter().any(|row| row.perturbation_class
        == LivePerturbationClass::OrderedCollectionReplayEndStateParity));
    assert!(artifact.matrix.rows.iter().any(|row| row.perturbation_class
        == LivePerturbationClass::OrderedCollectionReplayStepwiseParity));
    assert!(artifact.matrix.rows.iter().any(|row| row.perturbation_class
        == LivePerturbationClass::BoundedMaterializationReplayEndStateParity));
    assert!(artifact.matrix.rows.iter().any(|row| row.perturbation_class
        == LivePerturbationClass::BoundedMaterializationReplayStepwiseParity));
    assert!(artifact
        .matrix
        .rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::RefreshFallbackParity));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::RefreshRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::CoalescingRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::NonMonotonicSequenceRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::SequenceGapRejection));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::InvalidLivePromotionRejection));
    assert!(
        artifact
            .matrix
            .rejection_rows
            .iter()
            .any(|row| row.perturbation_class
                == LivePerturbationClass::UnsupportedLiveFamilyRejection)
    );
    assert!(artifact.matrix.rejection_rows.iter().any(
        |row| row.perturbation_class == LivePerturbationClass::UnsupportedPatchFamilyRejection
    ));
    assert!(artifact
        .matrix
        .rejection_rows
        .iter()
        .any(|row| row.perturbation_class == LivePerturbationClass::RawCdcLeakageRejection));
}

#[test]
fn live_query_patch_policy_artifact_is_deterministic() {
    let left =
        MilestoneFiveLiveCertificationAdapter::
            live_promotion_convergence_and_suppression_certification_artifact();
    let right =
        MilestoneFiveLiveCertificationAdapter::
            live_promotion_convergence_and_suppression_certification_artifact();

    assert_eq!(
        left.certification_bundle_digest,
        right.certification_bundle_digest
    );
    assert_eq!(left.coverage_matrix_digest, right.coverage_matrix_digest);
    assert_eq!(left.counter_snapshot, right.counter_snapshot);
    assert_eq!(left.matrix, right.matrix);
}
