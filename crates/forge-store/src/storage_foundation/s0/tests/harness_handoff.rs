use super::support::*;
use crate::storage_foundation::s0::*;

#[test]
fn phase1_harness_maturity_report_establishes_s1_required_rows() {
    let backend_matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("harness-matrix"),
    )
    .unwrap();
    let deferred_map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("harness-deferred"),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let (_manifest, terminology_report, release_report) = release_lane_inputs("source:rev:a");

    let harness = HarnessMaturityReport::baseline_for_s1(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("harness"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &release_report,
        1,
        1,
        &[
            S1CompileTimeBoundaryFixture::PhysicalDebtCannotPromoteToPlatform,
            S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests,
        ],
    )
    .unwrap();

    assert_eq!(harness.rows().len(), 6);
    assert_eq!(
        harness.evidence_bundle_readiness(),
        EvidenceBundleReadiness::ReadyForS1Planning
    );
    assert!(harness.rows().iter().any(|row| {
        row.subsystem() == HarnessSubsystemMaturity::CompileTimeBoundaryFixtures
            && row
                .forbidden_shortcuts_covered()
                .contains(&S1ForbiddenShortcut::BackendTierMismatch)
    }));
}

#[test]
fn phase1_s1_handoff_rejects_stale_accepted_inputs() {
    let backend_matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-matrix"),
    )
    .unwrap();
    let deferred_map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-deferred"),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let manifest = S0AuditInputManifest::new(
        "source:rev:b",
        vec![scan_root(".github")],
        vec![matched_file_with_kind(
            ".github/workflows/release.yml",
            S0InputFileKind::Workflow,
            "workflow",
            64,
        )],
    )
    .unwrap();
    let terminology_report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-terms"),
        &TerminologyScanPlan::new(vec![terminology_scope(".github")]).unwrap(),
        &manifest,
        &[terminology_input(
            ".github/workflows/release.yml",
            "semantic durability release lane\n",
        )],
        &[TerminologyAllowlistEntry::new(
            ".github/workflows/release.yml",
            1,
            "durability",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap()],
    )
    .unwrap();
    let release_report = ReleaseClaimReport::from_terminology_report(
        &ReleaseClaimScanPlan::new(vec![".github/workflows/release.yml".to_string()]).unwrap(),
        &terminology_report,
    )
    .unwrap();
    let harness = HarnessMaturityReport::baseline_for_s1(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-harness"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &release_report,
        1,
        1,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    let gate = milestone_sequence_for_13_3()
        .gate_readiness_witness("13.3")
        .unwrap();

    let error = StorageFoundationS1Handoff::from_accepted_inputs(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &manifest,
        &verified_complexity_report(),
        &harness,
        gate,
        &release_report,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .expect_err("stale manifest revision must reject handoff construction");

    assert_eq!(error, S0S1HandoffBuildRejection::StaleAcceptedInput);
}

#[test]
fn phase1_s1_handoff_json_rejects_tampered_candidates() {
    let backend_matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-json-matrix"),
    )
    .unwrap();
    let deferred_map = DeferredPhysicalGuaranteeMap::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-json-deferred"),
        &[semantic_cleanup_row()],
    )
    .unwrap();
    let (manifest, terminology_report, release_report) = release_lane_inputs("source:rev:a");
    let harness = HarnessMaturityReport::baseline_for_s1(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-json-harness"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &release_report,
        1,
        1,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    let gate = milestone_sequence_for_13_3()
        .gate_readiness_witness("13.3")
        .unwrap();
    let handoff = StorageFoundationS1Handoff::from_accepted_inputs(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("handoff-json"),
        &backend_matrix,
        &deferred_map,
        &terminology_report,
        &manifest,
        &verified_complexity_report(),
        &harness,
        gate,
        &release_report,
        &[S1CompileTimeBoundaryFixture::S1HandoffRequiresAcceptedDigests],
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&handoff.to_canonical_json_bytes().unwrap()).unwrap();
    json["allowed_backend_candidates"][0] = serde_json::Value::String("tampered".into());
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = StorageFoundationS1Handoff::validate_canonical_json_bytes(&bytes)
        .expect_err("tampering must stale the handoff digest");

    assert_eq!(
        error,
        S0S1HandoffParseRejection::DeterministicDigestMismatch
    );
}
