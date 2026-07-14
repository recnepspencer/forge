use crate::failure::StoreError;
use rusqlite::Transaction;

use super::super::super::records::StoreState;
use super::super::helpers::persist_bulk_json_record;

pub(super) fn persist_bulk(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    persist_json_records(
        transaction,
        "bulk_program_identity_records",
        state.bulk_program_identity_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("kind".to_string(), format!("{:?}", record.kind)),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "frozen_bulk_manifest_records",
        state.frozen_bulk_manifest_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                (
                    "manifest_digest".to_string(),
                    record.manifest.manifest_digest().to_string(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "frozen_transform_basis_records",
        state.frozen_transform_basis_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                (
                    "basis_digest".to_string(),
                    record.basis.basis_digest().to_string(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "frozen_transform_partition_records",
        state.frozen_transform_partition_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                (
                    "partition_digest".to_string(),
                    record.partition.partition_digest().to_string(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "bulk_deterministic_plan_records",
        state.bulk_deterministic_plan_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan.plan_id().to_string()),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "bulk_progress_checkpoint_records",
        state.bulk_progress_checkpoint_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan_id.clone()),
                (
                    "checkpoint_sequence".to_string(),
                    record.checkpoint.checkpoint_sequence().to_string(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "bulk_chunk_witness_records",
        state.bulk_chunk_witness_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan_id.clone()),
                (
                    "chunk_ordinal".to_string(),
                    record.witness.chunk_ordinal().value().to_string(),
                ),
            ]
        },
    )?;
    persist_json_records(
        transaction,
        "program_chunk_witness_index_records",
        state.program_chunk_witness_index_records.values(),
        |record| {
            vec![
                ("program_id".to_string(), record.program_id.clone()),
                ("plan_id".to_string(), record.plan_id.clone()),
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
    crate::backend::records::BulkProgramIdentityRecord,
    crate::backend::records::FrozenBulkManifestRecord,
    crate::backend::records::FrozenTransformBasisRecord,
    crate::backend::records::FrozenTransformPartitionRecord,
    crate::backend::records::BulkDeterministicPlanRecord,
    crate::backend::records::BulkProgressCheckpointRecord,
    crate::backend::records::BulkChunkWitnessRecord,
    crate::backend::records::ProgramChunkWitnessIndexRecord,
);
