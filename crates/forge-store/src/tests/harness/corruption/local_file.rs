use crate::{
    backend::records::{SchemaSupportRecord, StoreState},
    bulk::{compute_checkpoint_digest, ChunkOrdinal, PublishedBulkProgressCheckpoint},
    layout::{Milestone6LayoutMaterialization, Milestone9PhysicalChunkReference},
    wal::{WalRecord, WalRecordPayload},
};
use forge_relational::facade::history::CommitId;

pub fn force_publication_commit_id_conflict(
    path: &std::path::Path,
    replacement_commit_id: CommitId,
) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    let wal_sequence = state
        .wal_records
        .iter()
        .rev()
        .find_map(|(wal_sequence, record)| match &record.payload {
            WalRecordPayload::DurablePublicationProgress(progress)
                if progress.commit_id.is_some() =>
            {
                Some(*wal_sequence)
            }
            _ => None,
        })
        .expect("store should contain a publication progress wal record");
    let original = state
        .wal_records
        .get(&wal_sequence)
        .cloned()
        .expect("target wal record should exist");
    let replacement = match original.payload {
        WalRecordPayload::DurablePublicationProgress(progress) => {
            WalRecord::durable_publication_progress(
                original.wal_sequence,
                original.durable_mutation_id,
                original.runtime_session_id,
                progress.phase,
                Some(replacement_commit_id),
            )
            .expect("replacement wal record should encode")
        }
        _ => unreachable!("selected wal record should be publication progress"),
    };
    state.wal_records.insert(wal_sequence, replacement);
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("conflicted store state should write");
}

pub fn force_branch_head_gap(path: &std::path::Path) {
    let raw = std::fs::read(path).expect("store file should exist");
    let mut state: StoreState = serde_json::from_slice(&raw).expect("store state should decode");
    state.branch_head_records.clear();
    std::fs::write(
        path,
        serde_json::to_vec_pretty(&state).expect("store state should encode"),
    )
    .expect("branch head gap state should write");
}

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
