use crate::consumer_kit::{
    load_support_snapshot_document, project_support_snapshot, ForgeQuerySupportSnapshotErrorKind,
    ForgeQuerySupportSnapshotSchemaVersion,
};

use super::live_support_matrix;

#[test]
fn support_snapshot_load_denies_schema_version_boundary_with_typed_error() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["schema_version"] = serde_json::Value::Number(2.into());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("schema version boundary must be denied");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::SchemaVersionMismatch
    );
}

#[test]
fn support_snapshot_load_denies_row_digest_drift_with_typed_error() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["live_row_digest"] =
            serde_json::Value::String("fake-live-row-digest".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("row digest mutation must change the snapshot digest");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::RowDigestMismatch
    );
}

#[test]
fn support_snapshot_load_denies_unknown_status_before_digest_acceptance() {
    let matrix = live_support_matrix();
    let snapshot = project_support_snapshot(&matrix);
    let json = support_snapshot_json_from_snapshot_with_mutation(&snapshot, |value| {
        value["rows"][0]["status"] = serde_json::Value::String("maybe-supported".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("unknown support status must be denied semantically");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::InvalidSupportStatus
    );
    assert_eq!(error.found(), Some("maybe-supported"));
    assert_eq!(error.surface(), Some(snapshot.rows()[0].surface()));
}

#[test]
fn support_snapshot_load_denies_unknown_backend_posture_with_structured_context() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["backend_posture"] = serde_json::Value::String("ambient-runtime".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("unknown backend posture must be denied semantically");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::InvalidBackendPosture
    );
    assert_eq!(error.expected(), Some("primary|scaffold"));
    assert_eq!(error.found(), Some("ambient-runtime"));
}

#[test]
fn support_snapshot_load_denies_unknown_facade_family() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["facade_family"] = serde_json::Value::String("side-channel".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("unknown facade family must be denied semantically");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::InvalidFacadeFamily
    );
    assert_eq!(error.found(), Some("side-channel"));
}

#[test]
fn support_snapshot_load_denies_unknown_teaching_posture() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["teaching_posture"] =
            serde_json::Value::String("ambient-teaching".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("unknown teaching posture must be denied semantically");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::InvalidTeachingPosture
    );
    assert_eq!(error.found(), Some("ambient-teaching"));
}

#[test]
fn support_snapshot_load_denies_blank_required_field() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["owner_milestone"] = serde_json::Value::String(" ".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("blank required field must be denied semantically");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::InvalidRequiredField
    );
    assert_eq!(error.found(), Some("owner_milestone"));
}

#[test]
fn support_snapshot_load_denies_malformed_json_with_typed_error() {
    let error = load_support_snapshot_document(
        "{not-json",
        ForgeQuerySupportSnapshotSchemaVersion::current(),
    )
    .expect_err("malformed JSON must be denied");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::JsonDecodeFailed
    );
}

#[test]
fn support_snapshot_load_denies_document_digest_drift() {
    let json = support_snapshot_json_with_mutation(|value| {
        value["snapshot_digest"] = serde_json::Value::String("fake-document-digest".to_string());
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("document digest mutation must be denied");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::SnapshotDigestMismatch
    );
}

fn support_snapshot_json_with_mutation(mutate: impl FnOnce(&mut serde_json::Value)) -> String {
    let matrix = live_support_matrix();
    let snapshot = project_support_snapshot(&matrix);
    support_snapshot_json_from_snapshot_with_mutation(&snapshot, mutate)
}

fn support_snapshot_json_from_snapshot_with_mutation(
    snapshot: &crate::consumer_kit::ForgeQuerySupportSnapshot,
    mutate: impl FnOnce(&mut serde_json::Value),
) -> String {
    let document = snapshot.to_document();
    let mut value = serde_json::to_value(&document).expect("document should serialize");
    mutate(&mut value);
    serde_json::to_string_pretty(&value).expect("mutated JSON should encode")
}
