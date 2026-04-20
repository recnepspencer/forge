use super::*;

pub fn force_branch_delta_replacement_self_reference(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .branch_delta_layer_records
        .values_mut()
        .find(|record| !record.replacement_of_layer_ids.is_empty())
        .expect("replacement branch delta layer should exist");
    record.replacement_of_layer_ids = vec![record.branch_delta_layer_id];
    if let Some(proof_entry) = record.replacement_lineage_proof.first_mut() {
        proof_entry.layer_id = record.branch_delta_layer_id;
    }
    record.replacement_lineage_proof.truncate(1);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta self-reference corruption should write");
}

pub fn force_branch_delta_replacement_gap(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .branch_delta_layer_records
        .values_mut()
        .find(|record| !record.replacement_of_layer_ids.is_empty())
        .expect("replacement branch delta layer should exist");
    record.replacement_of_layer_ids = vec![crate::delta::BranchDeltaLayerId(
        record.branch_delta_layer_id.0 + 10_000,
    )];
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta replacement gap corruption should write");
}

pub fn force_branch_delta_replacement_proof_mismatch(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .branch_delta_layer_records
        .values_mut()
        .find(|record| !record.replacement_lineage_proof.is_empty())
        .expect("replacement branch delta layer should exist");
    let proof_entry = record
        .replacement_lineage_proof
        .first_mut()
        .expect("replacement proof entry should exist");
    proof_entry
        .commit_ids
        .push(CommitId(proof_entry.target_frontier_commit_id.0 + 10_000));
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta replacement proof mismatch corruption should write");
}

pub fn force_branch_delta_replacement_proof_length_drift(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .branch_delta_layer_records
        .values_mut()
        .find(|record| !record.replacement_lineage_proof.is_empty())
        .expect("replacement branch delta layer should exist");
    let duplicate_entry = record.replacement_lineage_proof[0].clone();
    record.replacement_lineage_proof.push(duplicate_entry);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta replacement proof length drift corruption should write");
}

pub fn force_remove_first_branch_delta_layer(path: &std::path::Path, branch_id: &str) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let first_layer_id = *state
        .branch_delta_layer_records
        .iter()
        .find_map(|(layer_id, record)| {
            (record.branch_id.0 == branch_id && record.replacement_of_layer_ids.is_empty())
                .then_some(layer_id)
        })
        .expect("matching branch delta layer should exist");
    state.branch_delta_layer_records.remove(&first_layer_id);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta layer removal corruption should write");
}

pub fn force_clear_branch_delta_layer_artifacts(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .branch_delta_layer_records
        .values_mut()
        .next()
        .expect("branch delta layer should exist");
    record.artifacts = crate::backend::records::BranchDeltaLayerArtifacts::default();
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta artifact clear corruption should write");
}

pub fn force_branch_delta_artifact_commit_mismatch(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .branch_delta_layer_records
        .values_mut()
        .next()
        .expect("branch delta layer should exist");
    let first_commit = record
        .artifacts
        .commit_envelopes
        .first_mut()
        .expect("artifact commit should exist");
    first_commit.envelope.commit.commit_id = forge_relational::facade::history::CommitId(
        first_commit.envelope.commit.commit_id.0 + 10_000,
    );
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch delta artifact mismatch corruption should write");
}
