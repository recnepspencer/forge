use super::support::*;
use crate::storage_foundation::s0::*;

#[test]
fn phase1_required_artifacts_report_missing_and_counter_shape() {
    let set = S0RequiredArtifactSet::canonical();
    assert_eq!(
        set.canonical_artifact_dir(),
        "_docs/worth-store/artifacts/storage-foundation-s0"
    );
    let report = set.validate_present_artifacts([
        S0CanonicalArtifactSpec::new(
            S0ArtifactKind::BackendCapabilityMatrix,
            digest("schema:backend-capability-matrix"),
            S0ArtifactSchemaCompatibility::Compatible,
        ),
        S0CanonicalArtifactSpec::new(
            S0ArtifactKind::S0EvidenceBundle,
            digest("schema:evidence-bundle"),
            S0ArtifactSchemaCompatibility::Compatible,
        ),
    ]);

    assert_eq!(report.required_artifact_count(), 9);
    assert_eq!(report.present_artifact_count(), 2);
    assert_eq!(report.missing_required_artifact_count(), 7);
    assert_eq!(report.schema_incompatible_artifact_count(), 0);
    assert!(!report.is_complete());

    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        S0RequiredArtifactSet::canonical_complexity_contracts()
            .into_iter()
            .map(|name| S0ComplexityContract::verified(name.as_str(), 0, 0)),
    );
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity);
    assert_eq!(counters.required_artifact_count(), 9);
    assert_eq!(counters.missing_required_artifact_count(), 7);
    assert_eq!(counters.complexity_contract_count(), 9);
    assert_eq!(counters.missing_complexity_contract_count(), 0);
    assert_eq!(counters.duplicate_complexity_contract_count(), 0);
    assert_eq!(counters.complexity_debt_count(), 0);
    assert_eq!(counters.evidence_ref_reresolution_count(), 0);
}

#[test]
fn phase1_missing_complexity_contracts_are_not_reported_as_zero() {
    let report = S0RequiredArtifactSet::canonical().validate_present_artifacts([]);
    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        [S0ComplexityContract::verified(
            "s0_input_manifest_construction",
            0,
            0,
        )],
    );
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity);

    assert_eq!(complexity.missing_complexity_contract_count(), 8);
    assert_eq!(counters.missing_complexity_contract_count(), 8);
    assert!(counters.has_release_blocking_debt());
}

#[test]
fn phase1_duplicate_complexity_contracts_are_blocking_debt() {
    let complexity = S0ComplexityContractReport::from_contracts(
        S0RequiredArtifactSet::canonical_complexity_contracts(),
        [
            S0ComplexityContract::verified("s0_input_manifest_construction", 0, 0),
            S0ComplexityContract::verified("s0_input_manifest_construction", 1, 1),
        ],
    );
    let report = S0RequiredArtifactSet::canonical().validate_present_artifacts([]);
    let counters = S0CounterSnapshot::from_artifact_and_complexity_reports(&report, &complexity);

    assert_eq!(complexity.duplicate_complexity_contract_count(), 1);
    assert_eq!(counters.duplicate_complexity_contract_count(), 1);
    assert!(counters.has_release_blocking_debt());
}

#[test]
fn phase1_backend_capability_matrix_requires_first_audit_baseline() {
    let matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap();

    assert_eq!(matrix.rows().len(), 10);
    assert_eq!(
        matrix.envelope().artifact_kind(),
        S0ArtifactKind::BackendCapabilityMatrix
    );
    assert!(matrix
        .rows()
        .iter()
        .any(|row| row.row_id().as_str() == "FuturePlatformGradeBackend"
            && row.status() == S0ArtifactRowStatus::Deferred
            && row.capability_tier() == StoreBackendCapabilityTier::PlatformGrade));
}

#[test]
fn phase1_backend_capability_matrix_digest_excludes_nondeterministic_metadata() {
    let left = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("left"),
    )
    .unwrap();
    let right = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("right"),
    )
    .unwrap();

    assert_eq!(
        left.envelope().deterministic_digest(),
        right.envelope().deterministic_digest()
    );
}

#[test]
fn phase1_backend_capability_matrix_rejects_missing_baseline_row() {
    let mut rows = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap()
    .rows()
    .to_vec();
    rows.retain(|row| row.row_id().as_str() != "SqliteBackend");

    let error = BackendCapabilityMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("b"),
        rows,
    )
    .expect_err("first-audit baseline row removal must reject");

    assert_eq!(
        error,
        S0ArtifactBuildRejection::MissingFirstAuditBaselineRow
    );
}

#[test]
fn phase1_backend_capability_matrix_rejects_duplicate_rows() {
    let mut rows = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap()
    .rows()
    .to_vec();
    rows.push(rows[0].clone());

    let error = BackendCapabilityMatrix::new(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("b"),
        rows,
    )
    .expect_err("duplicate row ids must reject");

    assert_eq!(error, S0ArtifactBuildRejection::DuplicateRowId);
}

#[test]
fn phase1_backend_capability_matrix_json_round_trips_through_schema_gate() {
    let matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap();
    let bytes = matrix.to_canonical_json_bytes().unwrap();
    let parsed = BackendCapabilityMatrix::validate_canonical_json_bytes(&bytes).unwrap();

    assert_eq!(
        parsed.matrix().envelope().deterministic_digest(),
        matrix.envelope().deterministic_digest()
    );
    assert_eq!(parsed.validation_cost().row_count(), 10);
    assert_eq!(parsed.validation_cost().sort_row_count(), 10);
    assert_eq!(
        parsed.validation_cost().artifact_byte_count(),
        bytes.len() as u64
    );
    assert!(parsed.validation_cost().canonicalized_row_byte_count() > 0);
}

#[test]
fn phase1_backend_capability_matrix_json_rejects_non_parseable_input() {
    let error = BackendCapabilityMatrix::validate_canonical_json_bytes(b"{not-json")
        .expect_err("malformed canonical artifact must reject");

    assert_eq!(error, S0ArtifactParseRejection::NonParseable);
}

#[test]
fn phase1_backend_capability_matrix_json_rejects_schema_version_drift() {
    let matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&matrix.to_canonical_json_bytes().unwrap()).unwrap();
    json["schema_version"] = serde_json::Value::String("storage-foundation-s0/v0".into());
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = BackendCapabilityMatrix::validate_canonical_json_bytes(&bytes)
        .expect_err("schema version drift must reject");

    assert_eq!(error, S0ArtifactParseRejection::SchemaVersionMismatch);
}

#[test]
fn phase1_backend_capability_matrix_json_rejects_digest_tampering() {
    let matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&matrix.to_canonical_json_bytes().unwrap()).unwrap();
    json["rows"][0]["classification"] = serde_json::Value::String("tampered".into());
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = BackendCapabilityMatrix::validate_canonical_json_bytes(&bytes)
        .expect_err("row tampering must reject when digest is stale");

    assert_eq!(error, S0ArtifactParseRejection::DeterministicDigestMismatch);
}

#[test]
fn phase1_backend_capability_matrix_json_rejects_missing_baseline_rows() {
    let matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap();
    let mut json: serde_json::Value =
        serde_json::from_slice(&matrix.to_canonical_json_bytes().unwrap()).unwrap();
    json["rows"].as_array_mut().unwrap().pop();
    let bytes = serde_json::to_vec(&json).unwrap();

    let error = BackendCapabilityMatrix::validate_canonical_json_bytes(&bytes)
        .expect_err("missing required baseline row must reject");

    assert_eq!(
        error,
        S0ArtifactParseRejection::MatrixBuildRejected(
            S0ArtifactBuildRejection::MissingFirstAuditBaselineRow
        )
    );
}

#[test]
fn phase1_backend_capability_matrix_json_write_is_stable_for_same_matrix() {
    let matrix = BackendCapabilityMatrix::first_audit_baseline(
        "source:rev:a",
        digest("roadmap:digest"),
        "worth-store-s0",
        metadata("a"),
    )
    .unwrap();

    assert_eq!(
        matrix.to_canonical_json_bytes().unwrap(),
        matrix.to_canonical_json_bytes().unwrap()
    );
}

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
