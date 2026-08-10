use super::super::support::{
    digest, matched_file_with_kind, metadata, scan_root, terminology_input, terminology_scope,
};
use crate::storage_foundation::s0::{
    Roadmap2SequenceId, S0AuditInputManifest, S0InputFileKind, TerminologyAllowedUse,
    TerminologyAllowlistEntry, TerminologyCleanupRejection, TerminologyRiskReport,
    TerminologyScanPlan,
};

#[test]
fn phase1_terminology_scan_requires_line_scoped_classification_and_stable_digest() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![
            matched_file_with_kind(
                "_docs/worth-store/worth_store_roadmap.md",
                S0InputFileKind::RoadmapDoc,
                "roadmap",
                64,
            ),
            matched_file_with_kind(
                "_docs/worth-store/test-requirements.md",
                S0InputFileKind::RoadmapDoc,
                "tests",
                64,
            ),
        ],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![
        terminology_scope("_docs/worth-store/worth_store_roadmap.md"),
        terminology_scope("_docs/worth-store/test-requirements.md"),
    ])
    .unwrap();
    let allowlist = vec![
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/worth_store_roadmap.md",
            1,
            "production-grade",
            TerminologyAllowedUse::QualifiedPhysicalDebt {
                deferred_sequence: Roadmap2SequenceId::new("S12").unwrap(),
            },
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/worth_store_roadmap.md",
            1,
            "embedded backend",
            TerminologyAllowedUse::QualifiedPhysicalDebt {
                deferred_sequence: Roadmap2SequenceId::new("S1").unwrap(),
            },
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/test-requirements.md",
            1,
            "durability",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
    ];

    let left = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("terms-left"),
        &plan,
        &manifest,
        &[
            terminology_input(
                "_docs/worth-store/test-requirements.md",
                "Durability semantics remain valid.\n",
            ),
            terminology_input(
                "_docs/worth-store/worth_store_roadmap.md",
                "Production-grade embedded backend until qualified.\n",
            ),
        ],
        &allowlist,
    )
    .unwrap();
    let right = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("terms-right"),
        &plan,
        &manifest,
        &[
            terminology_input(
                "_docs/worth-store/worth_store_roadmap.md",
                "Production-grade embedded backend until qualified.\n",
            ),
            terminology_input(
                "_docs/worth-store/test-requirements.md",
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
        vec![scan_root("_docs/worth-store")],
        vec![matched_file_with_kind(
            "_docs/worth-store/roadmap/physical/posture.md",
            S0InputFileKind::RoadmapDoc,
            "nested-roadmap",
            64,
        )],
    )
    .unwrap();
    let report = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("nested-scope"),
        &TerminologyScanPlan::new(vec![terminology_scope("_docs/worth-store")]).unwrap(),
        &manifest,
        &[terminology_input(
            "_docs/worth-store/roadmap/physical/posture.md",
            "database semantics only\n",
        )],
        &[TerminologyAllowlistEntry::new(
            "_docs/worth-store/roadmap/physical/posture.md",
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
        vec![scan_root("_docs/worth-store")],
        vec![matched_file_with_kind(
            "_docs/worth-store/worth_store_roadmap.md",
            S0InputFileKind::RoadmapDoc,
            "roadmap",
            64,
        )],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![terminology_scope(
        "_docs/worth-store/worth_store_roadmap.md",
    )])
    .unwrap();

    let error = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("terms"),
        &plan,
        &manifest,
        &[terminology_input(
            "_docs/worth-store/worth_store_roadmap.md",
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
        vec![scan_root("_docs/worth-store")],
        vec![matched_file_with_kind(
            "_docs/worth-store/worth_store_roadmap.md",
            S0InputFileKind::RoadmapDoc,
            "roadmap",
            64,
        )],
    )
    .unwrap();
    let plan = TerminologyScanPlan::new(vec![terminology_scope("_docs/worth-store")]).unwrap();
    let allowlist = vec![
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/worth_store_roadmap.md",
            1,
            "database",
            TerminologyAllowedUse::AllowedSemanticUse,
        )
        .unwrap(),
        TerminologyAllowlistEntry::new(
            "_docs/worth-store/worth_store_roadmap.md",
            1,
            "database",
            TerminologyAllowedUse::OverclaimedPhysicalPosture,
        )
        .unwrap(),
    ];

    let error = TerminologyRiskReport::scan(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("duplicate-allowlist"),
        &plan,
        &manifest,
        &[terminology_input(
            "_docs/worth-store/worth_store_roadmap.md",
            "database semantics only\n",
        )],
        &allowlist,
    )
    .expect_err("duplicate line-scoped classifications must reject");

    assert_eq!(error, TerminologyCleanupRejection::DuplicateAllowlistEntry);
}
