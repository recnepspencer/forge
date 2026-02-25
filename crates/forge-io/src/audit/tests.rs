use serde::{Deserialize, Serialize};

use super::eval::{
    append_audit_record_jsonl, load_audit_record, read_jsonl_records, save_audit_record,
    write_audit_bundle,
};
use super::schema::{
    AuditBundleManifest, VersionedAuditRecord,
    AuditFieldLabel, AuditIdentityScope, AuditConventionError,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DummyAuditPayload {
    step_count: u32,
    outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct DummyTrace {
    entries: Vec<String>,
}

#[test]
fn audit_record_round_trips_json_file() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("operation.json");

    let record = VersionedAuditRecord::new(
        "region_merge",
        3,
        DummyAuditPayload { step_count: 2, outcome: "ok".into() },
    );

    save_audit_record(&record, &path).expect("save audit record");
    let loaded: VersionedAuditRecord<DummyAuditPayload> =
        load_audit_record(&path).expect("load audit record");

    assert_eq!(loaded, record);
}

#[test]
fn append_audit_record_jsonl_is_append_only() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("audit-log.jsonl");

    let a = VersionedAuditRecord::new(
        "region_merge",
        1,
        DummyAuditPayload { step_count: 1, outcome: "ok".into() },
    );
    let b = VersionedAuditRecord::new(
        "region_merge",
        1,
        DummyAuditPayload { step_count: 2, outcome: "rejected".into() },
    );

    append_audit_record_jsonl(&a, &path).expect("append A");
    append_audit_record_jsonl(&b, &path).expect("append B");

    let records: Vec<VersionedAuditRecord<DummyAuditPayload>> =
        read_jsonl_records(&path).expect("read jsonl");
    assert_eq!(records.len(), 2);
    assert_eq!(records[0], a);
    assert_eq!(records[1], b);
}

#[test]
fn write_audit_bundle_emits_manifest_operation_and_trace() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bundles_root = dir.path().join("bundles");

    let record = VersionedAuditRecord::new(
        "region_merge",
        7,
        DummyAuditPayload { step_count: 4, outcome: "ok".into() },
    );
    let trace = DummyTrace { entries: vec!["decision-1".into()] };

    let manifest = write_audit_bundle(&bundles_root, "op-0001", &record, Some(&trace))
        .expect("write bundle");

    let bundle_dir = bundles_root.join("op-0001");
    assert!(bundle_dir.join("manifest.json").exists());
    assert!(bundle_dir.join("operation.json").exists());
    assert!(bundle_dir.join("trace.json").exists());

    let manifest_text = std::fs::read_to_string(bundle_dir.join("manifest.json")).expect("read manifest");
    let manifest_roundtrip: AuditBundleManifest =
        serde_json::from_str(&manifest_text).expect("parse manifest");
    assert_eq!(manifest_roundtrip, manifest);
    assert_eq!(manifest.operation_type, "region_merge");
    assert_eq!(manifest.operation_version, 7);

    let loaded: VersionedAuditRecord<DummyAuditPayload> =
        load_audit_record(bundle_dir.join("operation.json")).expect("load operation record");
    assert_eq!(loaded, record);
}

#[test]
fn write_audit_bundle_fails_if_operation_id_directory_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bundles_root = dir.path().join("bundles");

    let record = VersionedAuditRecord::new(
        "region_merge",
        1,
        DummyAuditPayload { step_count: 0, outcome: "noop".into() },
    );

    write_audit_bundle(&bundles_root, "op-dup", &record, Option::<&DummyTrace>::None)
        .expect("first bundle write");
    let err = write_audit_bundle(&bundles_root, "op-dup", &record, Option::<&DummyTrace>::None)
        .expect_err("second write must fail (append-only bundle semantics)");

    match err {
        crate::IoError::Io(io) => {
            assert_eq!(io.kind(), std::io::ErrorKind::AlreadyExists);
        }
        other => panic!("expected IoError::Io(AlreadyExists), got {:?}", other),
    }
}

#[test]
fn versioned_audit_record_requires_schema_and_operation_versions() {
    let good = VersionedAuditRecord::new(
        "region_merge",
        1,
        DummyAuditPayload { step_count: 1, outcome: "ok".into() },
    );
    assert_eq!(good.validate_conventions(), Ok(()));

    let mut bad_schema = good.clone();
    bad_schema.schema_version = 0;
    assert!(matches!(
        bad_schema.validate_conventions(),
        Err(AuditConventionError::InvalidSchemaVersion { found: 0 })
    ));

    let mut bad_op_ver = good.clone();
    bad_op_ver.operation_version = 0;
    assert!(matches!(
        bad_op_ver.validate_conventions(),
        Err(AuditConventionError::InvalidOperationVersion { found: 0 })
    ));
}

#[test]
fn audit_schema_fields_label_snapshot_vs_persistent_identity() {
    assert_eq!(
        AuditFieldLabel::new("intent_snapshot", AuditIdentityScope::Snapshot).validate(),
        Ok(())
    );
    assert_eq!(
        AuditFieldLabel::new("surviving_faces_persistent", AuditIdentityScope::Persistent).validate(),
        Ok(())
    );
    assert_eq!(
        AuditFieldLabel::new("trace_hash", AuditIdentityScope::Hash).validate(),
        Ok(())
    );

    assert!(matches!(
        AuditFieldLabel::new("intent", AuditIdentityScope::Snapshot).validate(),
        Err(AuditConventionError::FieldNameScopeMismatch { .. })
    ));
    assert!(matches!(
        AuditFieldLabel::new("surviving_faces", AuditIdentityScope::Persistent).validate(),
        Err(AuditConventionError::FieldNameScopeMismatch { .. })
    ));
    assert!(matches!(
        AuditFieldLabel::new("snapshotted_faces", AuditIdentityScope::Snapshot).validate(),
        Err(AuditConventionError::FieldNameScopeMismatch { .. })
    ));
    assert!(matches!(
        AuditFieldLabel::new("hashmap_stats", AuditIdentityScope::Hash).validate(),
        Err(AuditConventionError::FieldNameScopeMismatch { .. })
    ));
    assert!(matches!(
        AuditFieldLabel::new("persistenting_count", AuditIdentityScope::Persistent).validate(),
        Err(AuditConventionError::FieldNameScopeMismatch { .. })
    ));
}

#[test]
fn audit_record_serialization_is_deterministic_for_same_input() {
    let record = VersionedAuditRecord::new(
        "region_merge",
        3,
        DummyAuditPayload { step_count: 5, outcome: "ok".into() },
    );

    let a = serde_json::to_string(&record).expect("serialize A");
    let b = serde_json::to_string(&record).expect("serialize B");
    assert_eq!(a, b);
}
