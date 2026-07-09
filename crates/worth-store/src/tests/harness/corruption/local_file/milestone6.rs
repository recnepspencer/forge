use super::*;

pub fn force_milestone_6_layout_materialization_key_mismatch(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let expected_key = state
        .milestone_6_layout_materialization_records
        .keys()
        .next()
        .cloned()
        .expect("milestone 6 layout materialization record should exist");
    let record = state
        .milestone_6_layout_materialization_records
        .remove(&expected_key)
        .expect("milestone 6 layout materialization record should still exist");
    state
        .milestone_6_layout_materialization_records
        .insert(format!("{expected_key}:corrupted"), record);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 layout materialization key mismatch state should write");
}

pub fn force_milestone_6_layout_materialization_chunk_member_count_drift(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .milestone_6_layout_materialization_records
        .values_mut()
        .next()
        .expect("milestone 6 layout materialization record should exist");
    let drifted_reference = Milestone9PhysicalChunkReference::new(
        record
            .materialization
            .milestone_9_reference()
            .physical_chunk_id()
            .clone(),
        record
            .materialization
            .milestone_9_reference()
            .chunk_shape_version(),
        record
            .materialization
            .milestone_9_reference()
            .determinism_digest()
            .to_string(),
        record
            .materialization
            .milestone_9_reference()
            .chunk_member_count()
            + 1,
    );
    record.materialization = Milestone6LayoutMaterialization::new(
        record.materialization.artifact_id().to_string(),
        record.materialization.admitted_plan().clone(),
        record.materialization.block_reuse().clone(),
        record.materialization.frozen_layout().clone(),
        record.materialization.milestone_7_reference().clone(),
        drifted_reference,
        record.materialization.semantic_truth_digest().to_string(),
        record.materialization.authoritative_commit_count(),
    );
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 layout materialization chunk member drift state should write");
}

pub fn force_milestone_6_chunk_membership_boundary_drift(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .milestone_6_chunk_membership_records
        .values_mut()
        .next()
        .expect("milestone 6 chunk membership record should exist");
    record.layout_materialization_artifact_id =
        format!("{}:drifted", record.layout_materialization_artifact_id);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 chunk membership boundary drift state should write");
}

pub fn force_milestone_6_commit_coupled_layout_seed_authority_digest_drift(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .milestone_6_commit_coupled_layout_seed_records
        .values_mut()
        .next()
        .expect("milestone 6 commit-coupled layout seed record should exist");
    record.authority_basis_commit_digest =
        format!("{}:drifted", record.authority_basis_commit_digest);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 commit-coupled layout seed authority digest drift state should write");
}

pub fn force_milestone_6_commit_support_summary_seed_gap(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let summary = state
        .commit_support_summaries
        .values_mut()
        .next()
        .expect("commit support summary should exist");
    summary
        .milestone_6_published_layout_request_artifact_ids
        .clear();
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 commit support summary seed gap state should write");
}

pub fn force_milestone_6_commit_coupled_layout_seed_payload_gap(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let artifact_id = state
        .milestone_6_commit_coupled_layout_seed_records
        .keys()
        .next()
        .cloned()
        .expect("milestone 6 commit-coupled layout seed record should exist");
    state
        .milestone_6_commit_coupled_layout_seed_records
        .remove(&artifact_id)
        .expect("milestone 6 commit-coupled layout seed record should still exist");
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 commit-coupled layout seed payload gap state should write");
}

pub fn force_milestone_6_commit_coupled_layout_seed_payload_drift(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let record = state
        .milestone_6_commit_coupled_layout_seed_records
        .values_mut()
        .next()
        .expect("milestone 6 commit-coupled layout seed record should exist");
    record.request = crate::AspectLayoutReadRequest::new(
        crate::AspectLayoutTarget::new(
            record.request.target().branch_id().clone(),
            record.request.target().frontier_commit_id(),
        ),
        record.request.scope_class().clone(),
        crate::AspectProjectionSet::new(vec![
            "profile".to_string(),
            "status".to_string(),
            "drifted".to_string(),
        ]),
    );
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 commit-coupled layout seed payload drift state should write");
}

pub fn force_clear_milestone_6_derived_access_structures(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    state.milestone_6_scope_slice_membership_records.clear();
    state.milestone_6_chunk_membership_records.clear();
    state.milestone_6_structural_block_records.clear();
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 derived access structure clear should write");
}

pub fn force_clear_milestone_6_materializations_and_derived_access_structures(
    path: &std::path::Path,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    state.milestone_6_layout_materialization_records.clear();
    state.milestone_6_scope_slice_membership_records.clear();
    state.milestone_6_chunk_membership_records.clear();
    state.milestone_6_structural_block_records.clear();
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("milestone 6 materialization and derived clear should write");
}
