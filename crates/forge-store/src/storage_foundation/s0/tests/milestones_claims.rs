use super::support::*;
use crate::storage_foundation::s0::*;

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

#[test]
fn phase1_milestone_row_rejects_platform_grade_without_gate_witness() {
    let error = MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/forge-store/milestone-13.3-closeout.md",
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
        "_docs/forge-store/milestone-13.3-closeout.md",
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
        "_docs/forge-store/milestone-13.3-closeout.md",
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

#[test]
fn phase1_claim_report_classifies_semantic_and_physical_claims_from_milestone_rows() {
    let row = semantic_cleanup_row();
    let report = SemanticPhysicalClaimReport::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("claims"),
        &[row],
    )
    .unwrap();
    let parsed = SemanticPhysicalClaimReport::validate_canonical_json_bytes(
        &report.to_canonical_json_bytes().unwrap(),
    )
    .unwrap();
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(
        &S0RequiredArtifactSet::canonical().validate_present_artifacts([]),
        &verified_complexity_report(),
    )
    .with_claim_report(parsed.report());

    assert_eq!(parsed.report().rows().len(), 3);
    assert!(parsed.report().rows().iter().any(|row| {
        row.claim_family() == SemanticPhysicalClaimFamily::SubscriptionSupport
            && row.claim_status() == SemanticPhysicalClaimStatus::SemanticProven
    }));
    assert!(parsed.report().rows().iter().any(|row| {
        row.claim_family() == SemanticPhysicalClaimFamily::PhysicalSubstrate
            && row.claim_status() == SemanticPhysicalClaimStatus::PhysicalDebt
    }));
    assert!(parsed.report().rows().iter().any(|row| {
        row.claim_family() == SemanticPhysicalClaimFamily::PhysicalIntegrity
            && row.claim_status() == SemanticPhysicalClaimStatus::BootstrapPhysical
    }));
    assert_eq!(parsed.validation_cost().row_count(), 3);
    assert_eq!(counters.semantic_claim_count(), 1);
    assert_eq!(counters.physical_claim_count(), 2);
}

#[test]
fn phase1_deferred_guarantee_map_extracts_required_s_sequence_rows() {
    let row = semantic_cleanup_row();
    let map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("deferred"),
        &[row],
    )
    .unwrap();
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(
        &S0RequiredArtifactSet::canonical().validate_present_artifacts([]),
        &verified_complexity_report(),
    )
    .with_deferred_guarantee_map(&map);

    assert_eq!(map.rows().len(), 3);
    assert!(map
        .rows()
        .iter()
        .any(|row| row.row_id().as_str().contains("PageSegmentExtentSubstrate")));
    assert!(map
        .rows()
        .iter()
        .any(|row| row.row_id().as_str().contains("PageFrameChunkIntegrity")));
    assert!(map.rows().iter().any(|row| row
        .row_id()
        .as_str()
        .contains("PhysicalDatabaseCertification")));
    assert_eq!(counters.unmapped_deferred_guarantee_count(), 0);
}

#[test]
fn phase1_deferred_guarantee_map_rejects_category_without_required_anchor_sequence() {
    let error = DeferredPhysicalGuaranteeRow::new(
        S0ArtifactRowId::new("Milestone13_3PageSegmentExtentSubstrate").unwrap(),
        S0ArtifactSubjectKind::Milestone,
        "13.3",
        "deferred-physical-guarantee",
        vec![evidence_ref("13.3")],
        vec![
            BackendForbiddenClaim::new(BackendForbiddenClaimKind::PhysicalPersistence, "S2")
                .unwrap(),
        ],
        vec![Roadmap2SequenceId::new("S2").unwrap()],
        S0ArtifactRowStatus::Deferred,
        "deferred guarantee row",
        DeferredPhysicalGuaranteeCategory::PageSegmentExtentSubstrate,
        S0PhysicalStatus::PhysicalDebt,
        "page substrate proof remains unearned",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
    )
    .expect_err("S1 anchor must be required for page substrate debt");

    assert_eq!(
        error,
        S0DeferredGuaranteeBuildRejection::GuaranteeCategorySequenceMismatch
    );
}

#[test]
fn phase1_claim_report_json_rejects_tampered_rows() {
    let row = MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/forge-store/milestone-13.3-closeout.md",
        "Shipped store capability reclassification test",
        vec!["subscription-support trust".to_string()],
        vec![
            SemanticPhysicalClaimFamily::SubscriptionSupport,
            SemanticPhysicalClaimFamily::PhysicalSubstrate,
        ],
        S0PhysicalStatus::PhysicalDebt,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        S0PhysicalStatus::SemanticOnly,
        None,
        None,
        vec![
            BackendForbiddenClaim::new(BackendForbiddenClaimKind::PhysicalPersistence, "S1")
                .unwrap(),
        ],
        vec![Roadmap2SequenceId::new("S1").unwrap()],
        vec![],
        None,
    )
    .unwrap();
    let report = SemanticPhysicalClaimReport::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("tamper"),
        &[row],
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&report.to_canonical_json_bytes().unwrap()).unwrap();
    json["rows"][0]["classification"] = serde_json::Value::String("tampered".into());
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = SemanticPhysicalClaimReport::validate_canonical_json_bytes(&bytes)
        .expect_err("tampering must stale the deterministic digest");

    assert_eq!(
        error,
        S0ClaimReportParseRejection::DeterministicDigestMismatch
    );
}
