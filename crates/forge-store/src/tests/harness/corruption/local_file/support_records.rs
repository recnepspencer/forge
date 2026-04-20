use super::*;

pub fn force_cursor_checkpoint_gap(
    path: &std::path::Path,
    cursor_id: &str,
    checkpoint_sequence: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    state.subscriber_checkpoint_records.remove(&format!(
        "subscriber-checkpoint:{cursor_id}:{checkpoint_sequence}"
    ));
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("cursor checkpoint gap state should write");
}

pub fn force_cursor_identity_key_mismatch(path: &std::path::Path, cursor_id: &str) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let expected_key = format!("durable-cursor:{cursor_id}");
    let record = state
        .durable_cursor_identity_records
        .remove(&expected_key)
        .expect("durable cursor identity record should exist");
    state
        .durable_cursor_identity_records
        .insert(format!("{expected_key}:corrupted"), record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("cursor identity key mismatch state should write");
}

pub fn force_subscriber_checkpoint_key_mismatch(
    path: &std::path::Path,
    cursor_id: &str,
    checkpoint_sequence: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let expected_key = format!("subscriber-checkpoint:{cursor_id}:{checkpoint_sequence}");
    let record = state
        .subscriber_checkpoint_records
        .remove(&expected_key)
        .expect("subscriber checkpoint record should exist");
    state
        .subscriber_checkpoint_records
        .insert(format!("{expected_key}:corrupted"), record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("subscriber checkpoint key mismatch state should write");
}

pub fn force_schema_support_key_mismatch(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    if state.schema_support_records.is_empty() {
        let commit_record = state
            .commit_envelopes
            .values()
            .next()
            .cloned()
            .expect("commit envelope should exist to synthesize schema support");
        let artifact_id = format!(
            "schema-support:{}",
            commit_record.envelope.commit.commit_id.0
        );
        state.schema_support_records.insert(
            artifact_id.clone(),
            SchemaSupportRecord {
                artifact_id: artifact_id.clone(),
                commit_id: commit_record.envelope.commit.commit_id,
                branch_id: commit_record.envelope.branch_context.clone(),
                schema_version_id: commit_record.envelope.schema_version,
                descriptor_semantics_version: commit_record.envelope.descriptor_semantics_version,
                schema_transition: commit_record.envelope.schema_transition.clone(),
                schema_continuation_descriptor: commit_record
                    .envelope
                    .schema_continuation_descriptor
                    .clone(),
                schema_reconciliation_descriptor: commit_record
                    .envelope
                    .schema_reconciliation_descriptor
                    .clone(),
            },
        );
    }
    let expected_key = state
        .schema_support_records
        .keys()
        .next()
        .cloned()
        .expect("schema support record should exist");
    let record = state
        .schema_support_records
        .remove(&expected_key)
        .expect("schema support record should still exist");
    state
        .schema_support_records
        .insert(format!("{expected_key}:corrupted"), record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("schema support key mismatch state should write");
}

pub fn force_lineage_support_key_mismatch(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let expected_key = state
        .lineage_support_records
        .keys()
        .next()
        .cloned()
        .expect("lineage support record should exist");
    let record = state
        .lineage_support_records
        .remove(&expected_key)
        .expect("lineage support record should still exist");
    state
        .lineage_support_records
        .insert(format!("{expected_key}:corrupted"), record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("lineage support key mismatch state should write");
}

pub fn force_commit_support_summary_key_mismatch(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let expected_key = *state
        .commit_support_summaries
        .keys()
        .next()
        .expect("commit support summary should exist");
    let record = state
        .commit_support_summaries
        .remove(&expected_key)
        .expect("commit support summary should still exist");
    state
        .commit_support_summaries
        .insert(expected_key + 10_000, record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("commit support summary key mismatch state should write");
}

pub fn force_embedded_checkpoint_key_mismatch(path: &std::path::Path, checkpoint_id: &str) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .embedded_checkpoint_records
        .remove(checkpoint_id)
        .expect("embedded checkpoint should exist");
    state
        .embedded_checkpoint_records
        .insert(format!("{checkpoint_id}:corrupted"), record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("embedded checkpoint key mismatch state should write");
}

pub fn force_embedded_checkpoint_shape_violation(path: &std::path::Path, checkpoint_id: &str) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .embedded_checkpoint_records
        .get_mut(checkpoint_id)
        .expect("embedded checkpoint should exist");
    record.basis_commit_id = None;
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("embedded checkpoint shape violation state should write");
}


pub fn force_first_lineage_support_gap(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let lineage_artifact_id = state
        .lineage_support_records
        .keys()
        .next()
        .cloned()
        .expect("lineage support record should exist");
    state.lineage_support_records.remove(&lineage_artifact_id);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("lineage support gap state should write");
}

