use super::super::support::{digest, metadata};
use crate::storage_foundation::s0::*;

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
