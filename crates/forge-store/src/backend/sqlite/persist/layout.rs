use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::{as_i64, persist_bulk_json_record, sqlite_error};

pub(super) fn persist_layout(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_embedded_checkpoint_records(transaction, state)?;
    persist_milestone_6_layout_records(transaction, state)?;
    Ok(())
}

fn persist_embedded_checkpoint_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for record in state.embedded_checkpoint_records.values() {
        let contained_commit_ids_payload = serde_json::to_string(
            &record
                .contained_commit_ids
                .iter()
                .map(|commit_id| commit_id.0)
                .collect::<Vec<_>>(),
        )?;
        let metadata_payload = serde_json::to_string(&record.metadata)?;
        transaction
            .execute(
                "
                INSERT INTO embedded_checkpoint_records(
                    checkpoint_id,
                    source_runtime_id,
                    basis_branch_id,
                    basis_commit_id,
                    classification,
                    contained_commit_ids_payload,
                    metadata_payload
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                ",
                params![
                    record.checkpoint_id,
                    record.source_runtime_id,
                    record.basis_branch_id.as_ref().map(|value| value.0.clone()),
                    record.basis_commit_id.map(as_i64),
                    format!("{:?}", record.classification),
                    contained_commit_ids_payload,
                    metadata_payload,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn persist_milestone_6_layout_records(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_json_records(
        transaction,
        "milestone_6_layout_materialization_records",
        state.milestone_6_layout_materialization_records.values(),
        |_| Vec::new(),
    )?;
    persist_json_records(
        transaction,
        "milestone_6_commit_coupled_layout_seed_records",
        state
            .milestone_6_commit_coupled_layout_seed_records
            .values(),
        |record| {
            vec![
                (
                    "branch_id".to_string(),
                    record.request.target().branch_id().0.clone(),
                ),
                (
                    "frontier_commit_id".to_string(),
                    record.request.target().frontier_commit_id().0.to_string(),
                ),
                (
                    "scope_class".to_string(),
                    record.request.scope_class().label().to_string(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "milestone_6_scope_slice_membership_records",
        state.milestone_6_scope_slice_membership_records.values(),
        |record| {
            vec![
                ("branch_id".to_string(), record.branch_id.0.clone()),
                (
                    "frontier_commit_id".to_string(),
                    record.frontier_commit_id.0.to_string(),
                ),
                ("scope_class".to_string(), record.scope_class.clone()),
                (
                    "projection_digest".to_string(),
                    record.projection_digest.clone(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "milestone_6_chunk_membership_records",
        state.milestone_6_chunk_membership_records.values(),
        |record| {
            vec![
                (
                    "physical_chunk_id".to_string(),
                    record.physical_chunk_id.as_str().to_string(),
                ),
                (
                    "chunk_shape_version".to_string(),
                    record.chunk_shape_version.value().to_string(),
                ),
                (
                    "determinism_digest".to_string(),
                    record.determinism_digest.clone(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "milestone_6_structural_block_records",
        state.milestone_6_structural_block_records.values(),
        |record| {
            vec![
                (
                    "structural_block_id".to_string(),
                    record.structural_block_id.as_str().to_string(),
                ),
                ("scope_class".to_string(), record.scope_class.clone()),
                (
                    "equivalence_contract_version".to_string(),
                    record.equivalence_contract_version.value().to_string(),
                ),
                (
                    "supporting_layout_materialization_count".to_string(),
                    record
                        .supporting_layout_materialization_artifact_ids
                        .len()
                        .to_string(),
                ),
            ]
        },
    )?;
    Ok(())
}

fn persist_json_records<'a, T, I, F>(
    transaction: &Transaction<'_>,
    table: &str,
    records: I,
    indexed_columns: F,
) -> Result<(), StoreError>
where
    T: serde::Serialize + HasArtifactId + 'a,
    I: IntoIterator<Item = &'a T>,
    F: Fn(&T) -> Vec<(String, String)>,
{
    for record in records {
        persist_bulk_json_record(
            transaction,
            table,
            record.artifact_id(),
            indexed_columns(record),
            record,
        )?;
    }
    Ok(())
}

trait HasArtifactId {
    fn artifact_id(&self) -> &str;
}

macro_rules! impl_artifact_id {
    ($($ty:path),+ $(,)?) => {
        $(impl HasArtifactId for $ty {
            fn artifact_id(&self) -> &str {
                &self.artifact_id
            }
        })+
    };
}

impl_artifact_id!(
    crate::backend::records::Milestone6LayoutMaterializationRecord,
    crate::backend::records::Milestone6CommitCoupledLayoutSeedRecord,
    crate::backend::records::Milestone6ScopeSliceMembershipRecord,
    crate::backend::records::Milestone6ChunkMembershipRecord,
    crate::backend::records::Milestone6StructuralBlockRecord,
);
