use std::fs;

use crate::{
    ForgeStoreBuilder, Milestone12CertificationRunner, StoreErrorKind,
    FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT,
};
use rusqlite::{params, Connection};
use serde_json::Value;

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::{unique_test_sqlite_path, unique_test_store_path},
};

#[test]
fn local_file_manifest_recovery_reopens_with_first_ship_frontier() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-compatibility-local");

    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let reopened = ForgeStoreBuilder::new().local_file(path).build().unwrap();
    let summary = reopened.compatibility_manifest_recovery_summary();
    let recovered = reopened.recover_compatibility_manifest_index();
    let manifest_summaries = reopened.compatibility_manifest_summaries();

    assert_eq!(
        summary.manifest_count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
    );
    assert_eq!(
        summary.recovered_summary_count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
    );
    assert_eq!(summary.publication_gap_count(), 0);
    assert_eq!(
        recovered.frontier().publication_count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
    );
    assert_eq!(
        recovered.records().count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT
    );
    assert_eq!(
        manifest_summaries.len(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT
    );
}

#[test]
fn local_file_missing_compatibility_manifest_record_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-compatibility-local-gap");

    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let raw = fs::read_to_string(&path).unwrap();
    let needle = "\"compatibility_manifest:commit_envelope\": {";
    let start = raw
        .find(needle)
        .expect("commit-envelope manifest record should exist");
    let after_start = &raw[start..];
    let mut depth = 0_i32;
    let mut end_offset = None;
    for (offset, ch) in after_start.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end_offset = Some(offset);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = start + end_offset.expect("record block should close") + 1;
    let mut rewritten = String::new();
    rewritten.push_str(&raw[..start]);
    if raw[..start].trim_end().ends_with(',') {
        let trimmed = rewritten.trim_end_matches(|c: char| c.is_whitespace());
        rewritten = trimmed.trim_end_matches(',').to_string();
        rewritten.push('\n');
    }
    rewritten.push_str(&raw[end..]);
    fs::write(&path, rewritten).unwrap();

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should reject missing compatibility manifest records");
    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}

#[test]
fn local_file_manifest_digest_drift_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_store_path("forge-store-compatibility-local-digest-drift");

    {
        let mut store = ForgeStoreBuilder::new()
            .local_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let raw = fs::read_to_string(&path).unwrap();
    let record_start = raw
        .find("\"compatibility_manifest:commit_envelope\": {")
        .expect("commit-envelope manifest record should exist");
    let manifest_digest_key = raw[record_start..]
        .find("\"manifest_digest\": \"")
        .map(|offset| record_start + offset)
        .expect("commit-envelope manifest digest should exist");
    let digest_value_start = manifest_digest_key + "\"manifest_digest\": \"".len();
    let digest_value_end = raw[digest_value_start..]
        .find('"')
        .map(|offset| digest_value_start + offset)
        .expect("manifest digest should terminate");
    let mut rewritten = String::new();
    rewritten.push_str(&raw[..digest_value_start]);
    rewritten.push_str("drifted-digest");
    rewritten.push_str(&raw[digest_value_end..]);
    fs::write(&path, rewritten).unwrap();

    let error = ForgeStoreBuilder::new()
        .local_file(path)
        .build()
        .expect_err("reopen should reject manifest digest drift");
    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityArtifactManifestMalformed
    );
}

#[test]
fn sqlite_manifest_recovery_reopens_with_first_ship_frontier() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_sqlite_path("forge-store-compatibility-sqlite");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let summary = reopened.compatibility_manifest_recovery_summary();
    let recovered = reopened.recover_compatibility_manifest_index();

    assert_eq!(
        summary.manifest_count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
    );
    assert_eq!(
        summary.recovered_summary_count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
    );
    assert_eq!(summary.publication_gap_count(), 0);
    assert_eq!(
        recovered.frontier().publication_count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT as u64
    );
    assert_eq!(
        recovered.records().count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT
    );
}

#[test]
fn sqlite_missing_compatibility_manifest_row_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_sqlite_path("forge-store-compatibility-gap");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let connection = Connection::open(&path).unwrap();
    connection
        .execute(
            "DELETE FROM compatibility_manifest_records WHERE artifact_id = ?1",
            params!["compatibility_manifest:commit_envelope"],
        )
        .unwrap();

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("reopen should reject missing compatibility manifest rows");
    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityManifestPublicationGap
    );
}

#[test]
fn sqlite_manifest_digest_drift_is_rejected_on_open() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_sqlite_path("forge-store-compatibility-digest-drift");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let connection = Connection::open(&path).unwrap();
    let payload_json: String = connection
        .query_row(
            "SELECT payload_json FROM compatibility_manifest_records WHERE artifact_id = ?1",
            params!["compatibility_manifest:commit_envelope"],
            |row| row.get(0),
        )
        .unwrap();
    let mut payload: Value = serde_json::from_str(&payload_json).unwrap();
    payload["record"]["manifest_digest"] = Value::String("drifted-digest".to_string());
    connection
        .execute(
            "UPDATE compatibility_manifest_records SET payload_json = ?1 WHERE artifact_id = ?2",
            params![
                serde_json::to_string(&payload).unwrap(),
                "compatibility_manifest:commit_envelope"
            ],
        )
        .unwrap();

    let error = ForgeStoreBuilder::new()
        .sqlite_file(path)
        .build()
        .expect_err("reopen should reject manifest digest drift");
    assert_eq!(
        error.kind(),
        &StoreErrorKind::CompatibilityArtifactManifestMalformed
    );
}

#[test]
fn persisted_manifest_reopen_matches_certification_runtime_gap_status() {
    let mut runtime = runtime_with_demo_schema();
    create_entity(&mut runtime, "alpha");
    let envelope = latest_envelope(&runtime);
    let path = unique_test_sqlite_path("forge-store-compatibility-certification-persistence");

    {
        let mut store = ForgeStoreBuilder::new()
            .sqlite_file(path.clone())
            .build()
            .unwrap();
        store.append_canonical_commit(envelope).unwrap();
    }

    let reopened = ForgeStoreBuilder::new().sqlite_file(path).build().unwrap();
    let recovered = reopened.recover_compatibility_manifest_index();
    let certification = Milestone12CertificationRunner::first_ship().run().unwrap();

    assert_eq!(
        recovered.records().count(),
        FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT
    );
    assert!(!certification
        .diagnostics()
        .runtime_gap_labels()
        .contains(&"durable_manifest_persistence_deferred"));
}
