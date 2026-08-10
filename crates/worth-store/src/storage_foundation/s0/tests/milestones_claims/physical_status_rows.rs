use super::super::support::{evidence_ref, verified_complexity_report};
use crate::storage_foundation::s0::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, MilestoneCloseoutStatus,
    MilestonePhysicalStatusRow, MilestoneSpecStatus, MilestoneStatusDeclaration,
    Roadmap2SequenceId, RoadmapSequenceStatusMatrix, S0CounterSnapshot, S0MilestoneAuditRejection,
    S0PhysicalStatus, S0RequiredArtifactSet, SemanticPhysicalClaimFamily,
};

#[test]
fn phase1_milestone_row_rejects_platform_grade_without_gate_witness() {
    let error = MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/worth-store/milestone-13.3-closeout.md",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
        vec![SemanticPhysicalClaimFamily::SubscriptionSupport],
        S0PhysicalStatus::PlatformGrade,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        None,
        None,
        vec![],
        vec![],
        vec![],
        None,
    )
    .expect_err("platform-grade rows must require gate witness");

    assert_eq!(
        error,
        S0MilestoneAuditRejection::PlatformGradeStatusRequiresGateReadiness
    );
}

#[test]
fn phase1_milestone_row_rejects_physical_debt_without_sequence_mapping() {
    let error = MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/worth-store/milestone-13.3-closeout.md",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
        vec![SemanticPhysicalClaimFamily::SubscriptionSupport],
        S0PhysicalStatus::PhysicalDebt,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        None,
        None,
        vec![],
        vec![],
        vec![],
        None,
    )
    .expect_err("physical debt rows must map to deferred sequences");

    assert_eq!(
        error,
        S0MilestoneAuditRejection::PhysicalDebtRequiresDeferredSequence
    );
}

#[test]
fn phase1_milestone_row_accepts_foundation_backed_semantic_row() {
    let matrix = RoadmapSequenceStatusMatrix::new(
        vec![MilestoneStatusDeclaration::new(
            "13.3",
            MilestoneSpecStatus::Closed,
            MilestoneCloseoutStatus::Closed,
            vec![evidence_ref("13.3")],
        )
        .unwrap()],
        vec![],
    )
    .unwrap();
    let witness = matrix.gate_readiness_witness("13.3").unwrap();
    let row = MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/worth-store/milestone-13.3-closeout.md",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
        vec![
            SemanticPhysicalClaimFamily::SubscriptionSupport,
            SemanticPhysicalClaimFamily::SemanticAuthority,
        ],
        S0PhysicalStatus::FoundationBacked,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        None,
        None,
        vec![
            BackendForbiddenClaim::new(BackendForbiddenClaimKind::PlatformGradeDurability, "S12")
                .unwrap(),
        ],
        vec![Roadmap2SequenceId::new("S12").unwrap()],
        vec!["clarify physical database posture".to_string()],
        Some(&witness),
    )
    .unwrap();
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(
        &S0RequiredArtifactSet::canonical().validate_present_artifacts([]),
        &verified_complexity_report(),
    )
    .with_sequence_matrix(&matrix)
    .with_milestone_status_rows(&[row], 14);

    assert_eq!(counters.roadmap_sequence_edge_count(), 0);
    assert_eq!(counters.sequence_inconsistency_count(), 0);
    assert_eq!(counters.milestone_status_row_count(), 1);
    assert_eq!(counters.missing_milestone_status_row_count(), 13);
    assert_eq!(counters.semantic_claim_count(), 2);
    assert_eq!(counters.physical_claim_count(), 0);
    assert!(counters.has_release_blocking_debt());
}
