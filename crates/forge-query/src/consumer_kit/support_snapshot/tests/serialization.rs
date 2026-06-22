use crate::consumer_kit::project_support_snapshot;

use super::live_support_matrix;

type TerminalSupportSnapshotDocumentJson = serde_json::Value;

#[test]
fn support_snapshot_export_is_deterministic_for_the_same_live_matrix() {
    let matrix = live_support_matrix();

    let left = project_support_snapshot(&matrix);
    let right = project_support_snapshot(&matrix);

    assert_eq!(left.snapshot_digest(), right.snapshot_digest());
    assert_eq!(
        left.to_canonical_json().expect("left JSON should encode"),
        right.to_canonical_json().expect("right JSON should encode")
    );
}

#[test]
fn support_snapshot_load_reexport_is_byte_stable() {
    let matrix = live_support_matrix();
    let snapshot = project_support_snapshot(&matrix);

    let original_json = snapshot
        .to_canonical_json()
        .expect("original canonical JSON should encode");
    let loaded = crate::consumer_kit::load_support_snapshot_document(
        &original_json,
        crate::consumer_kit::ForgeQuerySupportSnapshotSchemaVersion::current(),
    )
    .expect("canonical JSON should load");
    let reexported_json = loaded
        .to_canonical_json()
        .expect("loaded canonical JSON should encode");

    assert_eq!(original_json, reexported_json);
    assert_eq!(snapshot.snapshot_digest(), loaded.snapshot_digest());
}

#[test]
fn support_snapshot_digest_includes_schema_identity() {
    let matrix = live_support_matrix();
    let snapshot = project_support_snapshot(&matrix);
    let document = snapshot.to_document();
    let mut value: TerminalSupportSnapshotDocumentJson =
        serde_json::to_value(&document).expect("document should serialize");

    value["schema_identity"] = TerminalSupportSnapshotDocumentJson::String("fake-schema".into());
    let json = serde_json::to_string_pretty(&value).expect("mutated JSON should encode");

    let error = crate::consumer_kit::load_support_snapshot_document(
        &json,
        crate::consumer_kit::ForgeQuerySupportSnapshotSchemaVersion::current(),
    )
    .expect_err("schema identity mutation should be denied");

    assert_eq!(
        error.kind(),
        &crate::consumer_kit::ForgeQuerySupportSnapshotErrorKind::SchemaIdentityMismatch
    );
}
