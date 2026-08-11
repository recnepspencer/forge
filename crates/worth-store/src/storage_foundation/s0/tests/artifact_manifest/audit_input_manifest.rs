use super::super::support::{matched_file, scan_root};
use crate::storage_foundation::s0::{
    S0AuditInputManifest, S0ComplexityContract, S0ComplexityContractReport, S0CounterSnapshot,
    S0DeclaredScanRoot, S0InputManifestDelta, S0RequiredArtifactSet, S0ScanScopeRejection,
};

#[test]
fn phase1_audit_manifest_rejects_broad_or_generated_scan_roots() {
    assert_eq!(
        S0DeclaredScanRoot::new(".", "global").expect_err("workspace global must reject"),
        S0ScanScopeRejection::WorkspaceGlobalScope
    );
    assert_eq!(
        S0DeclaredScanRoot::new("target", "generated").expect_err("target must reject"),
        S0ScanScopeRejection::ForbiddenGeneratedScope
    );
    assert_eq!(
        S0DeclaredScanRoot::new("C:/repo", "absolute").expect_err("absolute must reject"),
        S0ScanScopeRejection::AbsolutePath
    );
}

#[test]
fn phase1_audit_manifest_rejects_duplicate_and_out_of_scope_files() {
    let duplicate = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![
            matched_file("_docs/worth-store/storage-foundation-s0.md", "a", 10),
            matched_file("_docs/worth-store/storage-foundation-s0.md", "b", 11),
        ],
    )
    .expect_err("duplicate file paths must reject");
    assert_eq!(duplicate, S0ScanScopeRejection::DuplicateMatchedFile);

    let outside = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![matched_file("crates/worth-store/src/lib.rs", "a", 10)],
    )
    .expect_err("files outside declared roots must reject");
    assert_eq!(
        outside,
        S0ScanScopeRejection::MatchedFileOutsideDeclaredRoots
    );
}

#[test]
fn phase1_audit_manifest_digest_is_stable_after_canonical_sorting() {
    let left = S0AuditInputManifest::new(
        "source:rev:a",
        vec![
            scan_root("crates/worth-store"),
            scan_root("_docs/worth-store"),
        ],
        vec![
            matched_file("crates/worth-store/src/lib.rs", "lib", 20),
            matched_file("_docs/worth-store/storage-foundation-s0.md", "spec", 10),
        ],
    )
    .unwrap();
    let right = S0AuditInputManifest::new(
        "source:rev:a",
        vec![
            scan_root("_docs/worth-store"),
            scan_root("crates/worth-store"),
        ],
        vec![
            matched_file("_docs/worth-store/storage-foundation-s0.md", "spec", 10),
            matched_file("crates/worth-store/src/lib.rs", "lib", 20),
        ],
    )
    .unwrap();

    assert_eq!(left.manifest_digest(), right.manifest_digest());
    assert_eq!(left.breadth_summary().declared_scan_root_count(), 2);
    assert_eq!(left.breadth_summary().matched_file_count(), 2);
    assert_eq!(left.breadth_summary().matched_byte_count(), 30);
    assert_eq!(left.scan_cost().requested_scan_scope_count(), 2);
    assert_eq!(left.scan_cost().rejected_scan_scope_count(), 0);
}

#[test]
fn phase1_audit_manifest_witness_rejects_stale_source_or_digest() {
    let manifest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![matched_file(
            "_docs/worth-store/storage-foundation-s0.md",
            "spec",
            10,
        )],
    )
    .unwrap();
    let stale_source = S0AuditInputManifest::new(
        "source:rev:b",
        vec![scan_root("_docs/worth-store")],
        vec![matched_file(
            "_docs/worth-store/storage-foundation-s0.md",
            "spec",
            10,
        )],
    )
    .unwrap();
    let stale_digest = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![matched_file(
            "_docs/worth-store/storage-foundation-s0.md",
            "changed",
            10,
        )],
    )
    .unwrap();
    let witness = manifest.witness();

    assert_eq!(
        stale_source.validate_witness(&witness),
        Err(S0ScanScopeRejection::StaleSourceRevision)
    );
    assert_eq!(
        stale_digest.validate_witness(&witness),
        Err(S0ScanScopeRejection::StaleManifestDigest)
    );
}

#[test]
fn phase1_audit_manifest_delta_classifies_reuse_rescan_add_remove() {
    let previous = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![
            matched_file("_docs/worth-store/a.md", "a", 1),
            matched_file("_docs/worth-store/b.md", "b", 1),
            matched_file("_docs/worth-store/removed.md", "removed", 1),
        ],
    )
    .unwrap();
    let current = S0AuditInputManifest::new(
        "source:rev:b",
        vec![scan_root("_docs/worth-store")],
        vec![
            matched_file("_docs/worth-store/a.md", "a", 1),
            matched_file("_docs/worth-store/b.md", "b2", 1),
            matched_file("_docs/worth-store/added.md", "added", 1),
        ],
    )
    .unwrap();
    let delta = S0InputManifestDelta::between(&previous, &current);

    assert_eq!(delta.reused_file_count(), 1);
    assert_eq!(delta.rescanned_file_count(), 1);
    assert_eq!(delta.added_file_count(), 1);
    assert_eq!(delta.removed_file_count(), 1);
}

#[test]
fn phase1_audit_manifest_projects_exact_counters() {
    let previous = S0AuditInputManifest::new(
        "source:rev:a",
        vec![scan_root("_docs/worth-store")],
        vec![matched_file("_docs/worth-store/a.md", "a", 5)],
    )
    .unwrap();
    let current = S0AuditInputManifest::new(
        "source:rev:b",
        vec![scan_root("_docs/worth-store")],
        vec![
            matched_file("_docs/worth-store/a.md", "a", 5),
            matched_file("_docs/worth-store/b.md", "b", 7),
        ],
    )
    .unwrap();
    let delta = S0InputManifestDelta::between(&previous, &current);
    let report = S0RequiredArtifactSet::canonical().validate_present_artifacts([]);
    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        S0RequiredArtifactSet::canonical_complexity_contracts()
            .into_iter()
            .map(|name| S0ComplexityContract::verified(name.as_str(), 0, 0)),
    );
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity)
        .with_input_manifest(&current, Some(&delta));

    assert_eq!(counters.input_manifest_file_count(), 2);
    assert_eq!(counters.input_manifest_byte_count(), 12);
    assert_eq!(counters.input_manifest_reused_file_count(), 1);
    assert_eq!(counters.input_manifest_rescanned_file_count(), 0);
    assert_eq!(counters.requested_scan_scope_count(), 1);
    assert_eq!(counters.admitted_scan_scope_count(), 1);
    assert_eq!(counters.rejected_scan_scope_count(), 0);
}
