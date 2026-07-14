use crate::backend::records;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, sqlite_error};

pub(super) fn load_bulk(connection: &Connection, state: &mut StoreState) -> Result<(), StoreError> {
    load_json_records(
        connection,
        "bulk_program_identity_records",
        &mut state.bulk_program_identity_records,
    )?;
    load_json_records(
        connection,
        "frozen_bulk_manifest_records",
        &mut state.frozen_bulk_manifest_records,
    )?;
    load_json_records(
        connection,
        "frozen_transform_basis_records",
        &mut state.frozen_transform_basis_records,
    )?;
    load_json_records(
        connection,
        "frozen_transform_partition_records",
        &mut state.frozen_transform_partition_records,
    )?;
    load_json_records(
        connection,
        "bulk_deterministic_plan_records",
        &mut state.bulk_deterministic_plan_records,
    )?;
    load_json_records(
        connection,
        "bulk_progress_checkpoint_records",
        &mut state.bulk_progress_checkpoint_records,
    )?;
    load_json_records(
        connection,
        "bulk_chunk_witness_records",
        &mut state.bulk_chunk_witness_records,
    )?;
    load_json_records(
        connection,
        "program_chunk_witness_index_records",
        &mut state.program_chunk_witness_index_records,
    )?;
    Ok(())
}

fn load_json_records<T>(
    connection: &Connection,
    table: &str,
    target: &mut std::collections::BTreeMap<String, T>,
) -> Result<(), StoreError>
where
    T: serde::de::DeserializeOwned + HasArtifactId,
{
    let sql = format!("SELECT payload_json FROM {table} ORDER BY artifact_id");
    let mut statement = connection.prepare(&sql).map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| deserialize_json(row.get(0)?))
        .map_err(sqlite_error)?;
    for row in rows {
        let record: T = row.map_err(sqlite_error)?;
        target.insert(record.artifact_id().to_string(), record);
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
    records::BulkProgramIdentityRecord,
    records::FrozenBulkManifestRecord,
    records::FrozenTransformBasisRecord,
    records::FrozenTransformPartitionRecord,
    records::BulkDeterministicPlanRecord,
    records::BulkProgressCheckpointRecord,
    records::BulkChunkWitnessRecord,
    records::ProgramChunkWitnessIndexRecord,
);
