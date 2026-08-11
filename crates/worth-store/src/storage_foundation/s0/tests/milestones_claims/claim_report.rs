use super::super::support::{digest, metadata, semantic_cleanup_row, verified_complexity_report};
use crate::storage_foundation::s0::{
    BackendForbiddenClaim, BackendForbiddenClaimKind, MilestonePhysicalStatusRow,
    Roadmap2SequenceId, S0ClaimReportParseRejection, S0CounterSnapshot, S0PhysicalStatus,
    S0RequiredArtifactSet, SemanticPhysicalClaimFamily, SemanticPhysicalClaimReport,
    SemanticPhysicalClaimStatus,
};

#[test]
fn phase1_claim_report_classifies_semantic_and_physical_claims_from_milestone_rows() {
    let row = semantic_cleanup_row();
    let report = SemanticPhysicalClaimReport::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
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
fn phase1_claim_report_json_rejects_tampered_rows() {
    let row = MilestonePhysicalStatusRow::new(
        "13.3",
        "semantic trust closeout",
        "_docs/worth-store/milestone-13.3-closeout.md",
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
        "worth-store-s0",
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
