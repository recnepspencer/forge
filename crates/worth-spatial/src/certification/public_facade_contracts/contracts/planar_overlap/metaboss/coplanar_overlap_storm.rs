use super::diagnostics::{assert_tiny_rotation_diagnostic, certify_tiny_rotation_diagnostic};
use super::outcome_matrix::assert_mb_m6_outcome_matrix;
use super::platform_storm_subject::{
    certify_platform_storm, certify_platform_storm_with_transform, manual_stage_substitution_error,
    mismatched_operator_stage_link_error,
};
use super::scenario::near_graze_region;
use super::storm_extraction_subject::deny_storm_tiny_rotation;
use worth_kernel::workload_composition::{TransformRecipe, WorkloadTopologyBreadth};
use worth_spatial::facade::coplanar_overlap_storm::CoplanarOverlapStormWorkloadError;
use worth_spatial::facade::planar_overlap::CoplanarOverlapUserOutcomeKind;
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

#[test]
fn mb_m6_1_coplanar_overlap_storm_end_to_end_receipts() {
    let subject = certify_platform_storm("mb-real-coplanar-storm-identity");
    let counters = subject.storm_receipt.counters();

    assert_eq!(counters.topology_face_count(), 64);
    assert!(counters.topology_entity_count() > 64);
    assert!(counters.topology_relation_count() > counters.topology_face_count());
    assert!(counters.projected_entity_count() > 64);
    assert!(counters.transform_step_count() > 0);
    assert!(counters.transform_cancellation_step_count() > 0);
    assert!(counters.retained_artifact_count() > 0);
    assert!(counters.replay_checkpoint_count() > 0);
    assert_eq!(counters.operator_input_count(), 44);
    assert!(counters.operator_receipt_count() > 0);
    assert_eq!(counters.overlap_extraction_receipt_count(), 36);
    assert!(counters.overlap_candidate_pair_breadth() >= 576);
    assert!(counters.overlap_segment_contacts_certified() >= 192);
    assert!(counters.overlap_shared_intervals() >= 12);
    assert!(counters.overlap_islands() >= 12);
    assert!(counters.overlap_ambiguous_contacts() >= 12);
    assert!(!subject.storm_receipt.storm_digest().is_empty());
    assert!(!subject.storm_receipt.workload_identity().is_empty());
    assert_eq!(
        subject.storm_receipt.operator_identity(),
        subject.operator_receipt.operator_digest()
    );
    assert_eq!(
        subject.user_outcome.kind(),
        CoplanarOverlapUserOutcomeKind::ContractsCertified
    );
    assert!(!subject.user_outcome.message().contains('_'));

    let tiny_rotation_denial =
        deny_storm_tiny_rotation("mb-real-coplanar-storm-tiny-rotation", &near_graze_region());
    assert_eq!(
        tiny_rotation_denial.reason(),
        "movement and rotation posture must match before coplanar overlap extraction"
    );
    let diagnostic = certify_tiny_rotation_diagnostic(tiny_rotation_denial.reason());
    assert_tiny_rotation_diagnostic(&diagnostic, tiny_rotation_denial.reason());
}

#[test]
fn mb_m6_1_equivalent_motion_subset_converges_without_full_storm_replay() {
    let movement_stack = certify_platform_storm_with_transform(
        "mb-real-coplanar-subset-movement-stack",
        WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 },
        TransformRecipe::MovementRotationStack,
    );
    let hostile_cancellation = certify_platform_storm_with_transform(
        "mb-real-coplanar-subset-hostile-cancellation",
        WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 },
        TransformRecipe::HostileCancellation,
    );

    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .topology_face_count(),
        8
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .topology_face_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .topology_face_count()
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .topology_relation_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .topology_relation_count()
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .projected_entity_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .projected_entity_count()
    );
    assert_ne!(
        movement_stack
            .storm_receipt
            .counters()
            .transform_cancellation_step_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .transform_cancellation_step_count(),
        "movement and hostile cancellation branches must prove different replay breadth"
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .operator_input_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .operator_input_count(),
        "operator consumption must converge after replay despite transform breadth differences"
    );
    assert_ne!(
        movement_stack.storm_receipt.workload_identity(),
        hostile_cancellation.storm_receipt.workload_identity(),
        "distinct Query declarations must remain visible even when storm breadth converges"
    );
}

#[test]
fn mb_m6_1_user_outcome_matrix_branches_every_stop() {
    let tiny_rotation_denial = deny_storm_tiny_rotation(
        "mb-real-coplanar-matrix-tiny-rotation",
        &near_graze_region(),
    );
    assert_mb_m6_outcome_matrix(&tiny_rotation_denial);
}

#[test]
fn mb_m6_1_fixture_arithmetic_cannot_satisfy_storm_truth() {
    for stage in [
        WorkloadEvidenceStage::Topology,
        WorkloadEvidenceStage::GeometryBinding,
        WorkloadEvidenceStage::SurfaceSupport,
        WorkloadEvidenceStage::Projection,
        WorkloadEvidenceStage::Transform,
        WorkloadEvidenceStage::RetainedReplay,
        WorkloadEvidenceStage::Diagnostics,
        WorkloadEvidenceStage::Response,
    ] {
        let error = manual_stage_substitution_error(stage)
            .expect_err("hand-filled authority row must not complete storm evidence");

        assert_eq!(
            error,
            WorkloadEvidenceLedgerError::ManualAuthorityStage(stage)
        );
        assert_eq!(
            error.human_reason(),
            format!(
                "workload evidence ledger has hand-filled {} instead of a source receipt",
                stage.human_name()
            )
        );
    }
}

#[test]
fn mb_m6_1_operator_receipt_must_match_workload_ledger() {
    let error = mismatched_operator_stage_link_error()
        .expect_err("operator receipt from a different workload must not certify this storm");

    assert_eq!(
        error,
        CoplanarOverlapStormWorkloadError::MismatchedOperatorStageLink(
            WorkloadEvidenceStage::Projection
        )
    );
    assert_eq!(
        error.human_reason(),
        "coplanar overlap storm operator receipt must consume the same projection evidence as the workload ledger"
    );
}
