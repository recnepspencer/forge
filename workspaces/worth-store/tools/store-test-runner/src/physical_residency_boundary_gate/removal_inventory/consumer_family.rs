use std::collections::BTreeSet;

use super::direct_pool_reference::is_direct_pool_consumer;

pub(super) fn discover_families(path: &str, source: &str) -> BTreeSet<String> {
    let mut families = BTreeSet::new();
    classify_path_families(path, source, &mut families);
    classify_identifier_families(source, &mut families);
    classify_deleted_scheduler_paths(path, &mut families);
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
    if path_tokens(path)
        .chain(path_tokens(source))
        .any(|token| token.starts_with("C6") || token.starts_with("c6_"))
    {
        families.insert("c6-identifier".to_owned());
    }
}

fn classify_identifier_families(source: &str, families: &mut BTreeSet<String>) {
    for (fragment, family) in [
        ("legacy-s2-models", "legacy-s2-feature"),
        (
            "legacy-certification-models",
            "legacy-certification-feature",
        ),
        ("S2PhysicalResidencyEntry", "snapshot-residency-authority"),
        ("S2PhysicalEntryFacts", "snapshot-residency-authority"),
        ("ResidentFrameTable", "legacy-frame-table"),
        ("ZeroCopyRecordView", "legacy-record-view"),
        ("BoundedCopyRecordView", "legacy-record-view"),
        ("RecordViewMaterializationProfile", "legacy-record-view"),
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
            "BackgroundPacingProgressionEvidence",
            "scheduler-isolation-publication",
        ),
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
    ] {
        if source.contains(fragment) {
            families.insert(family.to_owned());
        }
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

fn path_tokens(value: &str) -> impl Iterator<Item = &str> {
    value.split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
}
