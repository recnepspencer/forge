use super::support::*;
use crate::storage_foundation::s0::*;

fn build_evidence_bundle(label: &str) -> S0EvidenceBundle {
    let backend_matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-matrix")),
    )
    .unwrap();
    let milestone_matrix = MilestonePhysicalStatusMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-milestones")),
        milestone_sequence_for_13_3(),
        vec!["13.3".to_string()],
        vec![semantic_cleanup_row()],
    )
    .unwrap();
    let claim_report = SemanticPhysicalClaimReport::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-claims")),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let deferred_map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-deferred")),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let (manifest, terminology_report, release_report) = release_lane_inputs("source:rev:a");
    let migration_notes = TestMigrationNotes::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-migration")),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let harness = HarnessMaturityReport::baseline_for_s1(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-harness")),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &release_report,
        1,
        1,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    let handoff = StorageFoundationS1Handoff::from_accepted_inputs(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(&format!("{label}-handoff")),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &manifest,
        &verified_complexity_report(),
        &harness,
        milestone_sequence_for_13_3()
            .gate_readiness_witness("13.3")
            .unwrap(),
        &release_report,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    S0EvidenceBundle::from_certified_inputs(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata(label),
        &BackendCapabilityMatrix::validate_canonical_json_bytes(
            &backend_matrix.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &MilestonePhysicalStatusMatrix::validate_canonical_json_bytes(
            &milestone_matrix.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &SemanticPhysicalClaimReport::validate_canonical_json_bytes(
            &claim_report.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &DeferredPhysicalGuaranteeMap::validate_canonical_json_bytes(
            &deferred_map.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &TerminologyRiskReport::validate_canonical_json_bytes(
            &terminology_report.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &TestMigrationNotes::validate_canonical_json_bytes(
            &migration_notes.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &HarnessMaturityReport::validate_canonical_json_bytes(
            &harness.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &StorageFoundationS1Handoff::validate_canonical_json_bytes(
            &handoff.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &manifest,
        &verified_complexity_report(),
        &release_report,
        S0RegenerationRequirement::new("cargo test -p worth-store storage_foundation::s0").unwrap(),
    )
    .unwrap()
}

#[test]
fn phase1_evidence_bundle_rejects_stale_complexity_digest() {
    let backend_matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-matrix"),
    )
    .unwrap();
    let milestone_matrix = MilestonePhysicalStatusMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-milestones"),
        milestone_sequence_for_13_3(),
        vec!["13.3".to_string()],
        vec![semantic_cleanup_row()],
    )
    .unwrap();
    let claim_report = SemanticPhysicalClaimReport::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-claims"),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let deferred_map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-deferred"),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let (manifest, terminology_report, release_report) = release_lane_inputs("source:rev:a");
    let migration_notes = TestMigrationNotes::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-migration"),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let harness = HarnessMaturityReport::baseline_for_s1(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-harness"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &release_report,
        1,
        1,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    let handoff = StorageFoundationS1Handoff::from_accepted_inputs(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle-handoff"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &manifest,
        &verified_complexity_report(),
        &harness,
        milestone_sequence_for_13_3()
            .gate_readiness_witness("13.3")
            .unwrap(),
        &release_report,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    let stale_complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        [S0ComplexityContract::verified(
            "s0_input_manifest_construction",
            1,
            1,
        )],
    );

    let error = S0EvidenceBundle::from_certified_inputs(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("bundle"),
        &BackendCapabilityMatrix::validate_canonical_json_bytes(
            &backend_matrix.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &MilestonePhysicalStatusMatrix::validate_canonical_json_bytes(
            &milestone_matrix.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &SemanticPhysicalClaimReport::validate_canonical_json_bytes(
            &claim_report.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &DeferredPhysicalGuaranteeMap::validate_canonical_json_bytes(
            &deferred_map.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &TerminologyRiskReport::validate_canonical_json_bytes(
            &terminology_report.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &TestMigrationNotes::validate_canonical_json_bytes(
            &migration_notes.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &HarnessMaturityReport::validate_canonical_json_bytes(
            &harness.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &StorageFoundationS1Handoff::validate_canonical_json_bytes(
            &handoff.to_canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        &manifest,
        &stale_complexity,
        &release_report,
        S0RegenerationRequirement::new("cargo test -p worth-store storage_foundation::s0").unwrap(),
    )
    .expect_err("mismatched accepted complexity digest must reject");

    assert_eq!(
        error,
        S0EvidenceBundleBuildRejection::StaleEvidence(
            S0StaleEvidenceRejection::ComplexitySummaryDigestMismatch
        )
    );
}

#[test]
fn phase1_evidence_bundle_json_round_trips_through_schema_gate() {
    let bundle = build_evidence_bundle("bundle-ok");
    let bytes = bundle.to_canonical_json_bytes().unwrap();
    let parsed = S0EvidenceBundle::validate_canonical_json_bytes(&bytes).unwrap();

    assert_eq!(
        parsed.bundle().envelope().deterministic_digest(),
        bundle.envelope().deterministic_digest()
    );
    assert!(!parsed.bundle().certification_rows().is_empty());
    assert!(parsed.bundle().counter_snapshot().digest_row_byte_count() > 0);
    assert_eq!(
        parsed.bundle().witness().evidence_bundle_digest(),
        bundle.witness().evidence_bundle_digest()
    );
}

#[test]
fn phase1_evidence_bundle_json_rejects_failure_digest_tampering() {
    let bundle = build_evidence_bundle("bundle-tamper");
    let mut json: serde_json::Value =
        serde_json::from_slice(&bundle.to_canonical_json_bytes().unwrap()).unwrap();
    json["failure_digest"] = serde_json::Value::String("tampered".into());
    let tampered = serde_json::to_vec(&json).unwrap();

    let error = S0EvidenceBundle::validate_canonical_json_bytes(&tampered)
        .expect_err("failure digest tampering must reject");

    assert_eq!(error, S0EvidenceBundleParseRejection::FailureDigestMismatch);
}
