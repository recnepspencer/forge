use super::*;

pub fn force_bulk_witness_missing_commit(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    chunk_ordinal: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-chunk-witness:{program_id}:{plan_id}:{chunk_ordinal}");
    let commit_id = state
        .bulk_chunk_witness_records
        .get(&artifact_id)
        .expect("bulk witness record should exist")
        .witness
        .canonical_commit_id();
    state.commit_envelopes.remove(&commit_id.0);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("bulk witness missing commit state should write");
}

pub fn force_bulk_checkpoint_gap(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    checkpoint_sequence: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-checkpoint:{program_id}:{plan_id}:{checkpoint_sequence}");
    state.bulk_progress_checkpoint_records.remove(&artifact_id);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("bulk checkpoint gap state should write");
}

pub fn force_bulk_checkpoint_completed_chunk_regression(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    checkpoint_sequence: u64,
    completed_chunk_ordinal: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-checkpoint:{program_id}:{plan_id}:{checkpoint_sequence}");
    let completed_chunk_ordinal = ChunkOrdinal::new(completed_chunk_ordinal);
    let next_chunk_ordinal = ChunkOrdinal::new(completed_chunk_ordinal.value() + 1);
    let record = state
        .bulk_progress_checkpoint_records
        .get_mut(&artifact_id)
        .expect("bulk checkpoint record should exist");
    let checkpoint_digest = compute_checkpoint_digest(
        program_id,
        plan_id,
        checkpoint_sequence,
        completed_chunk_ordinal,
        next_chunk_ordinal,
        record.checkpoint.last_committed_chunk_witness_artifact_id(),
    );
    record.checkpoint = PublishedBulkProgressCheckpoint::new(
        program_id.to_string(),
        plan_id.to_string(),
        checkpoint_sequence,
        completed_chunk_ordinal,
        next_chunk_ordinal,
        record
            .checkpoint
            .last_committed_chunk_witness_artifact_id()
            .to_string(),
        checkpoint_digest,
    );
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("bulk checkpoint regression state should write");
}

pub fn force_bulk_witness_index_highest_ordinal_regression(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    regressed_chunk_ordinal: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-witness-index:{program_id}:{plan_id}");
    let record = state
        .program_chunk_witness_index_records
        .get_mut(&artifact_id)
        .expect("bulk witness index record should exist");
    record.index = crate::ProgramChunkWitnessIndex::new(
        program_id.to_string(),
        plan_id.to_string(),
        ChunkOrdinal::new(regressed_chunk_ordinal),
        record.index.highest_committed_commit_id(),
        record.index.latest_checkpoint_sequence(),
        record.index.witness_count(),
    );
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("bulk witness index regression state should write");
}

pub fn force_bulk_witness_index_witness_count_drift(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
    drifted_witness_count: u64,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-witness-index:{program_id}:{plan_id}");
    let record = state
        .program_chunk_witness_index_records
        .get_mut(&artifact_id)
        .expect("bulk witness index record should exist");
    record.index = crate::ProgramChunkWitnessIndex::new(
        program_id.to_string(),
        plan_id.to_string(),
        record.index.highest_committed_chunk_ordinal(),
        record.index.highest_committed_commit_id(),
        record.index.latest_checkpoint_sequence(),
        drifted_witness_count,
    );
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("bulk witness index witness-count drift state should write");
}

pub fn force_frozen_transform_basis_payload_scope_drift(
    path: &std::path::Path,
    program_id: &str,
    basis_digest: &str,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: serde_json::Value =
        serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-transform-basis:{program_id}:{basis_digest}");
    state["frozen_transform_basis_records"][&artifact_id]["basis"]["target_branch_scope"] =
        serde_json::json!("corrupted-branch");
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("frozen transform basis drift state should write");
}

pub fn force_bulk_plan_payload_chunk_width_drift(
    path: &std::path::Path,
    program_id: &str,
    plan_id: &str,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: serde_json::Value =
        serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = format!("bulk-plan:{program_id}:{plan_id}");
    let original_width = state["bulk_deterministic_plan_records"][&artifact_id]["plan"]["chunks"]
        [0]["width_units"]
        .as_u64()
        .expect("bulk plan chunk width should exist");
    state["bulk_deterministic_plan_records"][&artifact_id]["plan"]["chunks"][0]["width_units"] =
        serde_json::json!(original_width + 1);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("bulk plan drift state should write");
}
