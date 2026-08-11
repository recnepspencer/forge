use super::super::support::{
    digest, matched_file_with_kind, metadata, scan_root, terminology_input, terminology_scope,
    verified_complexity_report,
};
use crate::storage_foundation::s0::{
    ReleaseClaimReport, ReleaseClaimScanPlan, S0AuditInputManifest, S0CounterSnapshot,
    S0InputFileKind, S0RequiredArtifactSet, TerminologyAllowedUse, TerminologyAllowlistEntry,
    TerminologyCleanupRejection, TerminologyRiskReport, TerminologyScanPlan,
};

#[test]
fn phase1_release_claim_gate_rejects_public_overclaim_from_terminology_report() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store"), scan_root(".github")],
        vec![
            matched_file_with_kind(
                ".github/workflows/release.yml",
                S0InputFileKind::Workflow,
                "workflow",
                64,
            ),
            matched_file_with_kind(
                "_docs/worth-store/worth_store_roadmap_2.md",
                S0InputFileKind::RoadmapDoc,
                "roadmap2",
                64,
            ),
        ],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![
        terminology_scope(".github/workflows/release.yml"),
        terminology_scope("_docs/worth-store/worth_store_roadmap_2.md"),
    ])
    .unwrap();
    let allowlist = vec![
        TerminologyAllowlistEntry::new(
            ".github/workflows/release.yml",
            1,
            "platform-grade",
            TerminologyAllowedUse::OverclaimedPhysicalPosture,
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/worth_store_roadmap_2.md",
            1,
            "physical",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/worth_store_roadmap_2.md",
            1,
            "database",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
    ];
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("release"),
        &plan,
        &manifest,
        &[
            terminology_input(
                ".github/workflows/release.yml",
                "platform-grade release lane\n",
            ),
            terminology_input(
                "_docs/worth-store/worth_store_roadmap_2.md",
                "physical database foundation gate\n",
            ),
        ],
        &allowlist,
    )
    .unwrap();
    let release_report = ReleaseClaimReport::from_terminology_report(
        &ReleaseClaimScanPlan::new(vec![".github/workflows/release.yml".to_string()]).unwrap(),
        &report,
    )
    .unwrap();
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(
        &S0RequiredArtifactSet::canonical().validate_present_artifacts([]),
        &verified_complexity_report(),
    )
    .with_terminology_report(&report)
    .with_release_claim_report(&release_report);

    assert_eq!(release_report.scanned_surface_count(), 1);
    assert_eq!(release_report.rejection_count(), 1);
    assert_eq!(counters.release_claim_scan_count(), 1);
    assert_eq!(counters.public_claim_rejection_count(), 1);
    assert_eq!(counters.unqualified_release_claim_count(), 1);
    assert_eq!(counters.overclaimed_physical_phrase_count(), 1);
    assert_eq!(counters.unique_evidence_ref_count(), 3);
    assert!(counters.has_release_blocking_debt());
}

#[test]
fn phase1_release_claim_gate_rejects_unscanned_release_surface() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root(".github")],
        vec![matched_file_with_kind(
            ".github/workflows/release.yml",
            S0InputFileKind::Workflow,
            "workflow",
            64,
        )],
    )
    .unwrap();
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("release-missing"),
        &TerminologyScanPlan::new(vec![terminology_scope(".github")]).unwrap(),
        &manifest,
        &[terminology_input(
            ".github/workflows/release.yml",
            "platform-grade release lane\n",
        )],
        &[TerminologyAllowlistEntry::new(
            ".github/workflows/release.yml",
            1,
            "platform-grade",
            TerminologyAllowedUse::OverclaimedPhysicalPosture,
        )
        .unwrap()],
    )
    .unwrap();

    let error = ReleaseClaimReport::from_terminology_report(
        &ReleaseClaimScanPlan::new(vec![
            ".github/workflows/release.yml".to_string(),
            "README.md".to_string(),
        ])
        .unwrap(),
        &report,
    )
    .expect_err("unscanned configured release surface must reject");

    assert_eq!(error, TerminologyCleanupRejection::UnscannedReleaseSurface);
}
