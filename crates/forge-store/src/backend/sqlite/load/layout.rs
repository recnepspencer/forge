use crate::backend::records;
use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, sqlite_error};

pub(super) fn load_layout(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    load_embedded_checkpoint_records(connection, state)?;
    load_json_records(
        connection,
        "milestone_6_layout_materialization_records",
        &mut state.milestone_6_layout_materialization_records,
    )?;
    load_json_records(
        connection,
        "milestone_6_commit_coupled_layout_seed_records",
        &mut state.milestone_6_commit_coupled_layout_seed_records,
    )?;
    load_json_records(
        connection,
        "milestone_6_scope_slice_membership_records",
        &mut state.milestone_6_scope_slice_membership_records,
    )?;
    load_json_records(
        connection,
        "milestone_6_chunk_membership_records",
        &mut state.milestone_6_chunk_membership_records,
    )?;
    load_json_records(
        connection,
        "milestone_6_structural_block_records",
        &mut state.milestone_6_structural_block_records,
    )?;
    Ok(())
}

fn load_embedded_checkpoint_records(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "
            SELECT checkpoint_id, source_runtime_id, basis_branch_id, basis_commit_id, classification,
                   contained_commit_ids_payload, metadata_payload
            FROM embedded_checkpoint_records
            ORDER BY checkpoint_id
            ",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            let classification = match row.get::<_, String>(4)?.as_str() {
                "DerivedDurable" => records::EmbeddedCheckpointClassification::DerivedDurable,
                "Ephemeral" => records::EmbeddedCheckpointClassification::Ephemeral,
                other => {
                    return Err(rusqlite::Error::FromSqlConversionFailure(
                        4,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::other(format!(
                            "unknown embedded checkpoint classification `{other}`"
                        ))),
                    ));
                }
            };
            let contained_commit_ids_payload: String = row.get(5)?;
            let contained_commit_ids = serde_json::from_str::<Vec<u64>>(
                &contained_commit_ids_payload,
            )
            .map_err(|error| {
                rusqlite::Error::FromSqlConversionFailure(
                    5,
                    rusqlite::types::Type::Text,
                    Box::new(error),
                )
            })?;
            let metadata_payload: String = row.get(6)?;
            let metadata = serde_json::from_str(&metadata_payload).map_err(|error| {
                rusqlite::Error::FromSqlConversionFailure(
                    6,
                    rusqlite::types::Type::Text,
                    Box::new(error),
                )
            })?;
            Ok(records::EmbeddedCheckpointRecord {
                checkpoint_id: row.get(0)?,
                source_runtime_id: row.get(1)?,
                basis_branch_id: row
                    .get::<_, Option<String>>(2)?
                    .map(forge_relational::facade::history::BranchId),
                basis_commit_id: row
                    .get::<_, Option<i64>>(3)?
                    .map(|value| forge_relational::facade::history::CommitId(value as u64)),
                classification,
                contained_commit_ids: contained_commit_ids
                    .into_iter()
                    .map(forge_relational::facade::history::CommitId)
                    .collect(),
                metadata,
            })
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let record = row.map_err(sqlite_error)?;
        state
            .embedded_checkpoint_records
            .insert(record.checkpoint_id.clone(), record);
    }
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
    records::Milestone6LayoutMaterializationRecord,
    records::Milestone6CommitCoupledLayoutSeedRecord,
    records::Milestone6ScopeSliceMembershipRecord,
    records::Milestone6ChunkMembershipRecord,
    records::Milestone6StructuralBlockRecord,
);
