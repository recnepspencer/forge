use super::support::*;
use crate::storage_foundation::s0::*;

#[test]
fn phase1_terminology_scan_requires_line_scoped_classification_and_stable_digest() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/forge-store")],
        vec![
            matched_file_with_kind(
                "_docs/forge-store/forge_store_roadmap.md",
                S0InputFileKind::RoadmapDoc,
                "roadmap",
                64,
            ),
            matched_file_with_kind(
                "_docs/forge-store/test-requirements.md",
                S0InputFileKind::RoadmapDoc,
                "tests",
                64,
            ),
        ],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![
        terminology_scope("_docs/forge-store/forge_store_roadmap.md"),
        terminology_scope("_docs/forge-store/test-requirements.md"),
    ])
    .unwrap();
    let allowlist = vec![
        TerminologyAllowlistEntry::new(
            "_docs/forge-store/forge_store_roadmap.md",
            1,
            "production-grade",
            TerminologyAllowedUse::QualifiedPhysicalDebt {
                deferred_sequence: Roadmap2SequenceId::new("S12").unwrap(),
            },
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/forge-store/forge_store_roadmap.md",
            1,
            "embedded backend",
            TerminologyAllowedUse::QualifiedPhysicalDebt {
                deferred_sequence: Roadmap2SequenceId::new("S1").unwrap(),
            },
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/forge-store/test-requirements.md",
            1,
            "durability",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
    ];

    let left = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("terms-left"),
        &plan,
        &manifest,
        &[
            terminology_input(
                "_docs/forge-store/test-requirements.md",
                "Durability semantics remain valid.\n",
            ),
            terminology_input(
                "_docs/forge-store/forge_store_roadmap.md",
                "Production-grade embedded backend until qualified.\n",
            ),
        ],
        &allowlist,
    )
    .unwrap();
    let right = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("terms-right"),
        &plan,
        &manifest,
        &[
            terminology_input(
                "_docs/forge-store/forge_store_roadmap.md",
                "Production-grade embedded backend until qualified.\n",
            ),
            terminology_input(
                "_docs/forge-store/test-requirements.md",
                "Durability semantics remain valid.\n",
            ),
        ],
        &allowlist,
    )
    .unwrap();

    assert_eq!(left.scan_digest(), right.scan_digest());
    assert_eq!(
        left.envelope().deterministic_digest(),
        right.envelope().deterministic_digest()
    );
    assert_eq!(left.rows().len(), 3);
    assert!(left.rows().iter().any(|row| matches!(
        row.allowed_use(),
        TerminologyAllowedUse::QualifiedPhysicalDebt { .. }
    )));
}

#[test]
fn phase1_terminology_scan_accepts_declared_root_scope_for_nested_files() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/forge-store")],
        vec![matched_file_with_kind(
            "_docs/forge-store/roadmap/physical/posture.md",
            S0InputFileKind::RoadmapDoc,
            "nested-roadmap",
            64,
        )],
    )
    .unwrap();
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("nested-scope"),
        &TerminologyScanPlan::new(vec![terminology_scope("_docs/forge-store")]).unwrap(),
        &manifest,
        &[terminology_input(
            "_docs/forge-store/roadmap/physical/posture.md",
            "database semantics only\n",
        )],
        &[TerminologyAllowlistEntry::new(
            "_docs/forge-store/roadmap/physical/posture.md",
            1,
            "database",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap()],
    )
    .unwrap();

    assert_eq!(report.rows().len(), 1);
}

#[test]
fn phase1_terminology_scan_rejects_unclassified_risky_phrase() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/forge-store")],
        vec![matched_file_with_kind(
            "_docs/forge-store/forge_store_roadmap.md",
            S0InputFileKind::RoadmapDoc,
            "roadmap",
            64,
        )],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![terminology_scope(
        "_docs/forge-store/forge_store_roadmap.md",
    )])
    .unwrap();

    let error = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("terms"),
        &plan,
        &manifest,
        &[terminology_input(
            "_docs/forge-store/forge_store_roadmap.md",
            "This database is ready.\n",
        )],
        &[],
    )
    .expect_err("risky phrase without classification must reject");

    assert_eq!(
        error,
        TerminologyCleanupRejection::UnclassifiedPhraseFinding
    );
}

#[test]
fn phase1_terminology_scan_rejects_duplicate_allowlist_entry() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/forge-store")],
        vec![matched_file_with_kind(
            "_docs/forge-store/forge_store_roadmap.md",
            S0InputFileKind::RoadmapDoc,
            "roadmap",
            64,
        )],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![terminology_scope("_docs/forge-store")]).unwrap();
    let allowlist = vec![
        TerminologyAllowlistEntry::new(
            "_docs/forge-store/forge_store_roadmap.md",
            1,
            "database",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/forge-store/forge_store_roadmap.md",
            1,
            "database",
            TerminologyAllowedUse::OverclaimedPhysicalPosture,
        )
        .unwrap(),
    ];

    let error = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("duplicate-allowlist"),
        &plan,
        &manifest,
        &[terminology_input(
            "_docs/forge-store/forge_store_roadmap.md",
            "database semantics only\n",
        )],
        &allowlist,
    )
    .expect_err("duplicate line-scoped classifications must reject");

    assert_eq!(error, TerminologyCleanupRejection::DuplicateAllowlistEntry);
}

#[test]
fn phase1_release_claim_gate_rejects_public_overclaim_from_terminology_report() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/forge-store"), scan_root(".github")],
        vec![
            matched_file_with_kind(
                ".github/workflows/release.yml",
                S0InputFileKind::Workflow,
                "workflow",
                64,
            ),
            matched_file_with_kind(
                "_docs/forge-store/forge_store_roadmap_2.md",
                S0InputFileKind::RoadmapDoc,
                "roadmap2",
                64,
            ),
        ],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![
        terminology_scope(".github/workflows/release.yml"),
        terminology_scope("_docs/forge-store/forge_store_roadmap_2.md"),
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
            "_docs/forge-store/forge_store_roadmap_2.md",
            1,
            "physical",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/forge-store/forge_store_roadmap_2.md",
            1,
            "database",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
    ];
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("release"),
        &plan,
        &manifest,
        &[
            terminology_input(
                ".github/workflows/release.yml",
                "platform-grade release lane\n",
            ),
            terminology_input(
                "_docs/forge-store/forge_store_roadmap_2.md",
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
        "forge-store-s0",
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

#[test]
fn phase1_terminology_report_json_rejects_tampered_rows() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/forge-store")],
        vec![matched_file_with_kind(
            "_docs/forge-store/forge_store_roadmap.md",
            S0InputFileKind::RoadmapDoc,
            "roadmap",
            64,
        )],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![terminology_scope(
        "_docs/forge-store/forge_store_roadmap.md",
    )])
    .unwrap();
    let allowlist = vec![TerminologyAllowlistEntry::new(
        "_docs/forge-store/forge_store_roadmap.md",
        1,
        "database",
        TerminologyAllowedUse::AllowedSemanticUse,
    )
    .unwrap()];
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("terms-json"),
        &plan,
        &manifest,
        &[terminology_input(
            "_docs/forge-store/forge_store_roadmap.md",
            "database semantics only\n",
        )],
        &allowlist,
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&report.to_canonical_json_bytes().unwrap()).unwrap();
    json["rows"][0]["notes"] = serde_json::Value::String("tampered".into());
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = TerminologyRiskReport::validate_canonical_json_bytes(&bytes)
        .expect_err("tampering must stale the digest");

    assert_eq!(
        error,
        TerminologyCleanupRejection::DeterministicDigestMismatch
    );
}

#[test]
fn phase1_test_migration_notes_classify_named_suite_scope() {
    let report = TestMigrationNotes::from_milestone_rows(
        "source:rev:a",
        digest("roadmap:digest"),
        "forge-store-s0",
        metadata("migration"),
        &[semantic_cleanup_row()],
    )
    .unwrap();

    assert_eq!(report.rows().len(), 1);
    assert_eq!(
        report.rows()[0].evidence_scope(),
        SemanticPhysicalClaimStatus::PhysicalDebt
    );
}
