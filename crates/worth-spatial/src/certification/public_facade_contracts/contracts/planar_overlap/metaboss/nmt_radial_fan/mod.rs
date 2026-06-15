pub(crate) mod subject;

use subject::{
    cube_checkpoint_denial, label_only_motion_denial, manual_stage_substitution_errors,
    mismatched_projection_denial, mismatched_replay_denial, mismatched_topology_ledger_denial,
    mismatched_transform_denial, missing_open_boundary_evidence_outcome,
    missing_radial_evidence_outcome, radial_fan_closeout_evidence, radial_fan_outcome_matrix,
    radial_fan_subject, storm_checkpoint_denial, unsupported_non_plane_surface_denial,
};
use worth_spatial::facade::nmt_certification_context::{NmtBossCloseoutReceipt, NmtBossId};
use worth_spatial::facade::nmt_radial_fan::{NmtRadialFanDenial, NmtRadialFanOutcomeKind};
use worth_spatial::facade::surface_support::UnsupportedSurfaceSupportReasonCode;
use worth_spatial::facade::transform_workload::UnsupportedTransformReasonCode;
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

#[test]
fn mb_m6_nmt_1_open_radial_fan_cannot_be_manifold_laundered() {
    for incident_faces in [3, 4] {
        let subject = radial_fan_subject("mb-m6-nmt-1-admitted", incident_faces);
        let counters = subject.receipt.counters();

        assert_eq!(counters.incident_face_count(), incident_faces);
        assert!(counters.open_boundary_half_edge_count() > 0);
        assert_eq!(counters.non_manifold_edge_count(), 1);
        assert_eq!(counters.topology_face_count(), incident_faces);
        assert!(counters.projected_entity_count() >= incident_faces);
        assert!(counters.transform_step_count() > 0);
        assert!(counters.changed_coordinate_count() > 0);
        assert!(counters.retained_artifact_count() > 0);
        assert!(counters.replay_checkpoint_count() > 0);
        assert_eq!(counters.diagnostic_count(), 1);
        assert_eq!(counters.user_outcome_count(), 1);
        assert!(!subject.receipt.fan_digest().is_empty());
        assert_eq!(subject.receipt.topology_posture(), "OpenNonManifold");
        assert!(!subject.receipt.projected_workload_identity().is_empty());
        assert!(!subject.receipt.open_boundary_digest().is_empty());
        assert!(!subject.receipt.radial_adjacency_digest().is_empty());
        assert_branch(&subject.user_outcome, WorthUserOutcomeKind::Admitted, None);
        assert!(subject
            .user_outcome
            .human_response()
            .summary()
            .contains("open non-manifold posture"));
        assert_human_readable(subject.user_outcome.human_response().summary());
    }
}

#[test]
fn mb_m6_nmt_1_rejects_closed_and_foreign_retained_checkpoint_substitution() {
    for denial in [storm_checkpoint_denial(), cube_checkpoint_denial()] {
        assert!(matches!(
            denial,
            NmtRadialFanDenial::ClosedManifoldLaunderingAttempt { .. }
        ));
        assert_human_readable(&denial.human_reason());
        assert!(denial.human_reason().contains("cannot be laundered"));
    }

    assert_eq!(
        mismatched_topology_ledger_denial(),
        NmtRadialFanDenial::MismatchedTopologyConstructionReceipt
    );
    assert_eq!(
        mismatched_projection_denial(),
        NmtRadialFanDenial::MismatchedProjectionReceipt
    );
    assert_eq!(
        mismatched_transform_denial(),
        NmtRadialFanDenial::MismatchedTransformReceipt
    );
    assert_eq!(
        mismatched_replay_denial(),
        NmtRadialFanDenial::MismatchedRetainedReplayReceipt
    );
}

#[test]
fn mb_m6_nmt_1_rejects_manual_authority_stage_substitution() {
    let errors = manual_stage_substitution_errors();
    assert_eq!(errors.len(), WorkloadEvidenceStage::AUTHORITY_STAGES.len());
    for (error, stage) in errors
        .into_iter()
        .zip(WorkloadEvidenceStage::AUTHORITY_STAGES)
    {
        assert_eq!(
            error,
            WorkloadEvidenceLedgerError::ManualAuthorityStage(stage)
        );
        assert_human_readable(&error.human_reason());
    }
}

#[test]
fn mb_m6_nmt_1_denies_label_only_motion_and_missing_radial_evidence() {
    let (reason, label_only_outcome) = label_only_motion_denial();
    assert_eq!(
        reason,
        UnsupportedTransformReasonCode::LabelOnlyMotionEvidence
    );
    assert_branch(
        &label_only_outcome,
        WorthUserOutcomeKind::Denied,
        Some(WorthUserOutcomeCauseKind::DeniedMovementOrRotation),
    );
    assert!(label_only_outcome
        .human_response()
        .summary()
        .contains("no coordinates changed"));
    assert_human_readable(label_only_outcome.human_response().summary());

    let missing_radial = missing_radial_evidence_outcome();
    assert_branch(
        &missing_radial,
        WorthUserOutcomeKind::NoOptions,
        Some(WorthUserOutcomeCauseKind::MissingEvidence),
    );
    assert!(missing_radial
        .human_response()
        .summary()
        .contains("radial adjacency evidence"));
    assert!(missing_radial.choices().is_empty());
    assert_human_readable(missing_radial.human_response().summary());

    let missing_boundary = missing_open_boundary_evidence_outcome();
    assert_branch(
        &missing_boundary,
        WorthUserOutcomeKind::NoOptions,
        Some(WorthUserOutcomeCauseKind::MissingEvidence),
    );
    assert!(missing_boundary
        .human_response()
        .summary()
        .contains("open-boundary evidence"));
    assert!(missing_boundary.choices().is_empty());
    assert_human_readable(missing_boundary.human_response().summary());
}

#[test]
fn mb_m6_nmt_1_outcome_matrix_names_each_blocker() {
    let (_, unsupported_reason) =
        unsupported_non_plane_surface_denial("mb-m6-nmt-1-unsupported-surface");
    assert_eq!(
        unsupported_reason,
        UnsupportedSurfaceSupportReasonCode::FamilyNotAdmitted
    );

    let matrix = radial_fan_outcome_matrix("mb-m6-nmt-1-matrix");

    assert_eq!(matrix.rows().len(), 7);
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::Admitted)
        .is_some());
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::IntegrityMismatch)
        .is_some());
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::Denied)
        .is_some());
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::UnsupportedInput)
        .is_some());
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::DirtyInput)
        .is_some());
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::PredicateUncertain)
        .is_some());
    assert!(matrix
        .row_for_kind(NmtRadialFanOutcomeKind::MissingEvidence)
        .is_some());
    let dirty = matrix
        .row_for_kind(NmtRadialFanOutcomeKind::DirtyInput)
        .expect("dirty topology boundary row");
    assert!(dirty.human_reason().contains("non-manifold wire"));
    assert!(
        !dirty.evidence_identity().contains("NmtRadialFanDenial"),
        "dirty row must carry response evidence, not a local NMT denial debug string"
    );
    let predicate = matrix
        .row_for_kind(NmtRadialFanOutcomeKind::PredicateUncertain)
        .expect("predicate authority row");
    assert!(predicate.human_reason().contains("Exact predicate"));
    assert!(
        !predicate.evidence_identity().contains("NmtRadialFanDenial"),
        "predicate row must carry predicate response evidence, not a local NMT denial debug string"
    );

    for row in matrix.rows() {
        assert!(!row.evidence_identity().is_empty());
        assert_human_readable(row.human_reason());
    }

    let closeout = radial_fan_closeout_evidence("mb-m6-nmt-1-closeout");
    let receipt = NmtBossCloseoutReceipt::from_certified_scope_set(
        NmtBossId::OpenRadialFan,
        &closeout.certified_scopes,
        &closeout.matrix,
    )
    .expect("NMT radial fan boss must close out from certified scope evidence");
    assert_eq!(receipt.boss(), NmtBossId::OpenRadialFan);
    assert_eq!(receipt.outcome_count(), 5);
}

fn assert_branch(
    outcome: &WorthUserOutcome,
    kind: WorthUserOutcomeKind,
    cause_kind: Option<WorthUserOutcomeCauseKind>,
) {
    assert_eq!(outcome.kind(), kind);
    assert_eq!(outcome.cause().map(|cause| cause.kind()), cause_kind);
}

fn assert_human_readable(message: &str) {
    assert!(!message.trim().is_empty());
    assert!(
        !message.contains('_'),
        "NMT radial fan response must not leak machine tokens: {message}"
    );
    assert!(
        !message
            .split_whitespace()
            .any(|word| word.matches('-').count() >= 3),
        "NMT radial fan response must explain causes in prose: {message}"
    );
}
