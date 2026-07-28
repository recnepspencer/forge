use std::collections::BTreeSet;

use super::direct_pool_reference::is_direct_pool_consumer;

pub(super) fn discover_families(path: &str, source: &str) -> BTreeSet<String> {
    let mut families = BTreeSet::new();
    classify_path_families(path, source, &mut families);
    classify_identifier_families(path, source, &mut families);
    classify_deleted_closeout_paths(path, &mut families);
    classify_legacy_record_view_paths(path, &mut families);
    classify_deleted_scheduler_paths(path, &mut families);
    classify_deleted_capacity_paths(path, &mut families);
    classify_unidentified_buffer_pool_predecessor(path, &mut families);
    if families.is_empty() && is_direct_pool_consumer(path, source) {
        families.insert("direct-pool-consumer".to_owned());
    }
    families
}

fn classify_path_families(path: &str, source: &str, families: &mut BTreeSet<String>) {
    if path.contains("c6_handoff") {
        families.insert("temporary-handoff".to_owned());
    }
    if path.contains("crates/worth-store-buffer-pool/src/background_work/queue_execution") {
        families.insert("isolated-speculative-queue".to_owned());
    }
    if path.contains("/page_lsn_publication/") || source.contains("page_lsn_publication") {
        families.insert("legacy-page-publication-authority".to_owned());
    }
    if path_tokens(path)
        .chain(path_tokens(source))
        .any(|token| token.starts_with("C6") || token.starts_with("c6_"))
    {
        families.insert("c6-identifier".to_owned());
    }
}

fn classify_identifier_families(path: &str, source: &str, families: &mut BTreeSet<String>) {
    classify_fragments(source, families, LEGACY_IDENTIFIER_FAMILIES);
    classify_fragments(source, families, SCHEDULER_IDENTIFIER_FAMILIES);
    if !path.ends_with(".md") {
        classify_fragments(source, families, RUST_ONLY_IDENTIFIER_FAMILIES);
    }
    classify_fragments(source, families, CLOSEOUT_IDENTIFIER_FAMILIES);
}

fn classify_fragments(
    source: &str,
    families: &mut BTreeSet<String>,
    classifications: &[(&str, &str)],
) {
    for (fragment, family) in classifications {
        if source.contains(fragment) {
            families.insert((*family).to_owned());
        }
    }
}

const LEGACY_IDENTIFIER_FAMILIES: &[(&str, &str)] = &[
    ("legacy-s2-models", "legacy-s2-feature"),
    (
        "legacy-certification-models",
        "legacy-certification-feature",
    ),
    ("S2PhysicalResidencyEntry", "snapshot-residency-authority"),
    ("S2PhysicalEntryFacts", "snapshot-residency-authority"),
    (
        "PhysicalSubstrateReadinessSnapshot",
        "snapshot-residency-authority",
    ),
    ("ResidentFrameTable", "legacy-frame-table"),
    ("ResidentFrameToken", "legacy-frame-table"),
    ("PinnedPageLease", "legacy-frame-table"),
    ("LegacyResidentFrame", "legacy-frame-table"),
    ("for_legacy_resident_frame", "legacy-frame-table"),
    ("legacy_resident_frame_token", "legacy-frame-table"),
    ("ZeroCopyRecordView", "legacy-record-view"),
    ("BoundedCopyRecordView", "legacy-record-view"),
    ("PinnedFrameView", "legacy-record-view"),
    ("RecordViewMaterializationProfile", "legacy-record-view"),
    ("RecordViewEvidence", "legacy-record-view"),
    ("OwnedReadBuffer", "legacy-record-view"),
    ("for_owned_read_buffer", "legacy-record-view"),
    ("from_bounded_copy", "legacy-record-view"),
    ("from_pinned_frame", "legacy-record-view"),
    (
        "DirtyPublicationEvidence",
        "legacy-page-publication-authority",
    ),
    (
        "PageFlushRecoveryReceipt",
        "legacy-page-publication-authority",
    ),
    (
        "PageFlushRecoveryPolicyReceipt",
        "legacy-page-publication-authority",
    ),
    (
        "WalBeforeDataOrderingProof",
        "legacy-page-publication-authority",
    ),
    (
        "NoUndoPublicationProof",
        "legacy-page-publication-authority",
    ),
    (
        "ReopenedPageRecoveryEvidence",
        "legacy-page-publication-authority",
    ),
    (
        "StalePageRecoveryClassification",
        "legacy-page-publication-authority",
    ),
    (
        "RollbackImagePublicationDeclaration",
        "legacy-page-publication-authority",
    ),
    (
        "PageWritePolicyObservation",
        "legacy-page-publication-authority",
    ),
    (
        "WalCoveragePolicyAssessment",
        "legacy-page-publication-authority",
    ),
    (
        "NoUndoRecoveryPolicyAssessment",
        "legacy-page-publication-authority",
    ),
    (
        "UnadmittedDirtyPagePublicationDenial",
        "legacy-page-publication-authority",
    ),
];

const SCHEDULER_IDENTIFIER_FAMILIES: &[(&str, &str)] = &[
    (
        "SchedulerIsolationCapability",
        "scheduler-isolation-publication",
    ),
    (
        "IoSchedulerIsolationAdmission",
        "scheduler-isolation-publication",
    ),
    (
        "TierPlacementIoAdmission",
        "scheduler-isolation-publication",
    ),
    (
        "S7PlacementIoReadinessSeed",
        "scheduler-isolation-publication",
    ),
    (
        "BackgroundPacingProgressionEvidence",
        "scheduler-isolation-publication",
    ),
    (
        "BackgroundPacingCapability",
        "scheduler-capacity-publication",
    ),
    (
        "BackgroundPacingAuthority",
        "scheduler-capacity-publication",
    ),
    ("BackgroundPacingReady", "scheduler-capacity-publication"),
    (
        "BackgroundPacingProgressionOutcome",
        "scheduler-capacity-publication",
    ),
    (
        "prove_background_pacing_current",
        "scheduler-capacity-publication",
    ),
    (
        "from_scheduler_capability",
        "scheduler-capacity-publication",
    ),
    ("with_pacing_admission", "scheduler-capacity-publication"),
    ("io_readmission_satisfied", "scheduler-capacity-publication"),
];

// A call to a removed self-admission constructor is live bypass use in Rust, but
// an intentional compile-fail specimen in executable Markdown.
const RUST_ONLY_IDENTIFIER_FAMILIES: &[(&str, &str)] =
    &[("admitted_compaction(", "scheduler-capacity-publication")];

const CLOSEOUT_IDENTIFIER_FAMILIES: &[(&str, &str)] = &[
    ("S2AcceptanceSuiteKind", "legacy-certification-closeout"),
    (
        "HarnessCloseoutEvidenceReport",
        "legacy-certification-closeout",
    ),
    (
        "HarnessCloseoutTranscriptEvidence",
        "legacy-certification-closeout",
    ),
    (
        "BoundedMemoryResidencySuite",
        "legacy-certification-closeout",
    ),
    (
        "bounded_memory_harness_closeout",
        "legacy-certification-closeout",
    ),
    (
        "bounded_memory_residency_suite",
        "legacy-certification-closeout",
    ),
    (
        "acceptance_suite_transcript",
        "legacy-certification-closeout",
    ),
];

fn classify_deleted_closeout_paths(path: &str, families: &mut BTreeSet<String>) {
    if [
        "courtroom/memory/bounded_memory_residency_suite.rs",
        "courtroom/physical_substrate/acceptance_suite_transcript.rs",
        "scenario/memory/bounded_memory_harness_closeout.rs",
    ]
    .iter()
    .any(|deleted| path.ends_with(deleted))
    {
        families.insert("legacy-certification-closeout".to_owned());
    }
}

fn classify_legacy_record_view_paths(path: &str, families: &mut BTreeSet<String>) {
    if [
        "evidence/cross_cutting/record_view_evidence.rs",
        "evidence/cross_cutting/record_view_evidence_admission_tests.rs",
        "evidence/cross_cutting/record_view_evidence_conflict_tests.rs",
        "courtroom/harness/test_support/record_view_evidence_test_support.rs",
    ]
    .iter()
    .any(|legacy| path.ends_with(legacy))
    {
        families.insert("legacy-record-view".to_owned());
    }
}

fn classify_deleted_scheduler_paths(path: &str, families: &mut BTreeSet<String>) {
    if [
        "readiness/scheduler_capability.rs",
        "readiness/isolation_denial.rs",
        "readiness/isolation_evidence/basis.rs",
        "readiness/interference/assumptions.rs",
        "executed_isolation_evidence/performance_receipt.rs",
        "io_readiness/mod.rs",
        "io_readiness/placement.rs",
        "io_qos_readiness_handoff.rs",
        "placement/admission/verification/readiness_basis_match.rs",
        "certification_test_authority/placement_readiness.rs",
    ]
    .iter()
    .any(|deleted| path.ends_with(deleted))
    {
        families.insert("scheduler-isolation-publication".to_owned());
    }
}

fn classify_deleted_capacity_paths(path: &str, families: &mut BTreeSet<String>) {
    if [
        "background_pacing/capability.rs",
        "background_pacing/proof.rs",
        "background_pacing/tests/progression.rs",
        "compaction/verification/pacing_admission.rs",
    ]
    .iter()
    .any(|deleted| path.ends_with(deleted))
    {
        families.insert("scheduler-capacity-publication".to_owned());
    }
}

fn classify_unidentified_buffer_pool_predecessor(path: &str, families: &mut BTreeSet<String>) {
    if families.is_empty()
        && path.starts_with("crates/worth-store-buffer-pool/src/")
        && path != "crates/worth-store-buffer-pool/src/lib.rs"
        && !path.starts_with("crates/worth-store-buffer-pool/src/physical_residency/")
    {
        families.insert("legacy-buffer-pool-predecessor".to_owned());
    }
}

fn path_tokens(value: &str) -> impl Iterator<Item = &str> {
    value.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
}
