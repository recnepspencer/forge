use crate::consumer_kit::{
    load_support_snapshot_document, project_support_snapshot, ForgeQuerySupportSnapshotErrorKind,
    ForgeQuerySupportSnapshotSchemaVersion,
};

use super::live_support_matrix;

type TerminalSupportSnapshotDocumentJson = serde_json::Value;

#[test]
fn support_snapshot_load_denies_schema_version_boundary_with_typed_error() {
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["schema_version"] = terminal_document_number(2);
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
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["live_row_digest"] = terminal_document_string("fake-live-row-digest");
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
    let json = terminal_support_snapshot_json_from_snapshot_with_mutation(&snapshot, |value| {
        value["rows"][0]["status"] = terminal_document_string("maybe-supported");
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
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["backend_posture"] = terminal_document_string("ambient-runtime");
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
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["facade_family"] = terminal_document_string("side-channel");
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
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["teaching_posture"] = terminal_document_string("ambient-teaching");
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
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["rows"][0]["owner_milestone"] = terminal_document_string(" ");
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
    let json = terminal_support_snapshot_json_with_mutation(|value| {
        value["snapshot_digest"] = terminal_document_string("fake-document-digest");
    });

    let error =
        load_support_snapshot_document(&json, ForgeQuerySupportSnapshotSchemaVersion::current())
            .expect_err("document digest mutation must be denied");

    assert_eq!(
        error.kind(),
        &ForgeQuerySupportSnapshotErrorKind::SnapshotDigestMismatch
    );
}

fn terminal_support_snapshot_json_with_mutation(
    mutate: impl FnOnce(&mut TerminalSupportSnapshotDocumentJson),
) -> String {
    let matrix = live_support_matrix();
    let snapshot = project_support_snapshot(&matrix);
    terminal_support_snapshot_json_from_snapshot_with_mutation(&snapshot, mutate)
}

fn terminal_support_snapshot_json_from_snapshot_with_mutation(
    snapshot: &crate::consumer_kit::ForgeQuerySupportSnapshot,
    mutate: impl FnOnce(&mut TerminalSupportSnapshotDocumentJson),
) -> String {
    let document = snapshot.to_document();
    let mut value = serde_json::to_value(&document).expect("document should serialize");
    mutate(&mut value);
    serde_json::to_string_pretty(&value).expect("mutated JSON should encode")
}

fn terminal_document_string(value: impl Into<String>) -> TerminalSupportSnapshotDocumentJson {
    TerminalSupportSnapshotDocumentJson::String(value.into())
}

fn terminal_document_number(value: u16) -> TerminalSupportSnapshotDocumentJson {
    TerminalSupportSnapshotDocumentJson::Number(value.into())
}
