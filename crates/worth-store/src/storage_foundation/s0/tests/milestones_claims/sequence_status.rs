use super::super::support::{evidence_ref, verified_complexity_report};
use crate::storage_foundation::s0::{
    MilestoneCloseoutStatus, MilestonePrerequisiteEdge, MilestoneSequenceInconsistency,
    MilestoneSpecStatus, MilestoneStatusDeclaration, PrerequisiteWaiverRationale,
    RoadmapSequenceStatusMatrix, S0CounterSnapshot, S0MilestoneAuditRejection,
    S0RequiredArtifactSet,
};

#[test]
fn phase1_sequence_matrix_reports_spec_closeout_status_mismatch() {
    let matrix = RoadmapSequenceStatusMatrix::new(
        vec![MilestoneStatusDeclaration::new(
            "13.2",
            MilestoneSpecStatus::Planned,
            MilestoneCloseoutStatus::Closed,
            vec![evidence_ref("13.2")],
        )
        .unwrap()],
        vec![],
    )
    .unwrap();

    assert_eq!(matrix.unwaived_inconsistency_count(), 1);
    assert_eq!(
        matrix.inconsistencies(),
        &[(
            "13.2".to_string(),
            MilestoneSequenceInconsistency::SpecCloseoutStatusMismatch
        )]
    );
}

#[test]
fn phase1_sequence_matrix_reports_missing_gate_predecessor_evidence() {
    let matrix = RoadmapSequenceStatusMatrix::new(
        vec![MilestoneStatusDeclaration::new(
            "13.3",
            MilestoneSpecStatus::Closed,
            MilestoneCloseoutStatus::Closed,
            vec![],
        )
        .unwrap()],
        vec![],
    )
    .unwrap();

    assert_eq!(
        matrix.inconsistencies(),
        &[(
            "13.3".to_string(),
            MilestoneSequenceInconsistency::MissingGatePredecessorEvidence
        )]
    );
    assert_eq!(
        matrix.gate_readiness_witness("13.3"),
        Err(S0MilestoneAuditRejection::GateReadinessBlockedBySequenceInconsistency)
    );
}

#[test]
fn phase1_sequence_matrix_reports_closed_with_unclosed_prerequisite() {
    let matrix = RoadmapSequenceStatusMatrix::new(
        vec![
            MilestoneStatusDeclaration::new(
                "13.2",
                MilestoneSpecStatus::InProgress,
                MilestoneCloseoutStatus::Planned,
                vec![],
            )
            .unwrap(),
            MilestoneStatusDeclaration::new(
                "13.3",
                MilestoneSpecStatus::Closed,
                MilestoneCloseoutStatus::Closed,
                vec![evidence_ref("13.3")],
            )
            .unwrap(),
        ],
        vec![MilestonePrerequisiteEdge::new("13.3", "13.2").unwrap()],
    )
    .unwrap();

    assert!(matrix
        .inconsistencies()
        .iter()
        .any(|(milestone, inconsistency)| {
            milestone == "13.3"
                && *inconsistency == MilestoneSequenceInconsistency::ClosedWithUnclosedPrerequisite
        }));
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(
        &S0RequiredArtifactSet::canonical().validate_present_artifacts([]),
        &verified_complexity_report(),
    )
    .with_sequence_matrix(&matrix);

    assert_eq!(counters.roadmap_sequence_edge_count(), 1);
    assert_eq!(counters.sequence_inconsistency_count(), 1);
    assert_eq!(counters.closed_with_unclosed_prerequisite_count(), 1);
}

#[test]
fn phase1_sequence_matrix_allows_typed_prerequisite_waiver() {
    let matrix = RoadmapSequenceStatusMatrix::new(
        vec![
            MilestoneStatusDeclaration::new(
                "13.2",
                MilestoneSpecStatus::InProgress,
                MilestoneCloseoutStatus::Planned,
                vec![],
            )
            .unwrap(),
            MilestoneStatusDeclaration::new(
                "13.3",
                MilestoneSpecStatus::Closed,
                MilestoneCloseoutStatus::Closed,
                vec![evidence_ref("13.3")],
            )
            .unwrap(),
        ],
        vec![MilestonePrerequisiteEdge::new("13.3", "13.2")
            .unwrap()
            .waived(PrerequisiteWaiverRationale::SemanticDocumentationDrift)],
    )
    .unwrap();

    assert_eq!(matrix.unwaived_inconsistency_count(), 0);
    let witness = matrix.gate_readiness_witness("13.3").unwrap();
    assert_eq!(witness.milestone_id(), "13.3");
    assert_eq!(witness.predecessor_evidence_count(), 1);
}

#[test]
fn phase1_sequence_matrix_blocks_gate_witness_when_inconsistencies_exist() {
    let matrix = RoadmapSequenceStatusMatrix::new(
        vec![MilestoneStatusDeclaration::new(
            "13.3",
            MilestoneSpecStatus::Planned,
            MilestoneCloseoutStatus::Closed,
            vec![evidence_ref("13.3")],
        )
        .unwrap()],
        vec![],
    )
    .unwrap();

    assert_eq!(
        matrix.gate_readiness_witness("13.3"),
        Err(S0MilestoneAuditRejection::GateReadinessBlockedBySequenceInconsistency)
    );
}
