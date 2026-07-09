use crate::{
    backend::records::BranchDeltaLayerArtifacts,
    failure::{StoreError, StoreErrorKind},
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};

pub(super) fn load_meta_u64(connection: &Connection, key: &str) -> Result<Option<u64>, StoreError> {
    connection
        .query_row(
            "SELECT value FROM store_meta WHERE key = ?1",
            params![key],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .map_err(sqlite_error)?
        .map(|value| {
            value.parse::<u64>().map_err(|error| {
                StoreError::backend_integrity(format!("invalid u64 store_meta `{key}`: {error}"))
            })
        })
        .transpose()
}

pub(super) fn load_meta_u32(connection: &Connection, key: &str) -> Result<Option<u32>, StoreError> {
    load_meta_u64(connection, key).map(|value| value.map(|value| value as u32))
}

pub(super) fn deserialize_json<T: serde::de::DeserializeOwned>(
    payload: String,
) -> rusqlite::Result<T> {
    serde_json::from_str(&payload).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(error))
    })
}

pub(super) fn deserialize_optional_json<T: serde::de::DeserializeOwned>(
    payload: Option<String>,
) -> rusqlite::Result<Option<T>> {
    payload.map(deserialize_json).transpose()
}

pub(super) fn serialize_optional_json<T: serde::Serialize>(
    value: &Option<T>,
) -> Result<Option<String>, StoreError> {
    value
        .as_ref()
        .map(serde_json::to_string)
        .transpose()
        .map_err(Into::into)
}

pub(super) fn as_i64(commit_id: worth_relational::facade::history::CommitId) -> i64 {
    commit_id.0 as i64
}

pub(super) fn as_i64_u64(value: u64) -> i64 {
    value as i64
}

pub(super) fn sqlite_error(error: rusqlite::Error) -> StoreError {
    match error {
        rusqlite::Error::FromSqlConversionFailure(_, _, error) => {
            let message = error.to_string();
            if message.contains("unknown placement execution origin") {
                StoreError::new(StoreErrorKind::PlacementExecutionOriginIllegal, message)
            } else if message.contains("unknown tier residence")
                || message.contains("unknown source tier residence")
                || message.contains("unknown target tier residence")
            {
                StoreError::new(StoreErrorKind::TierResidencyManifestViolation, message)
            } else if message.contains("unknown placement artifact family")
                || message.contains("unknown placement observation scope")
            {
                StoreError::new(
                    StoreErrorKind::PlacementWitnessConstructionViolation,
                    message,
                )
            } else if message.contains("unknown recall cost class")
                || message.contains("unknown recall amplification budget")
                || message.contains("unknown tier recall completion state")
            {
                StoreError::new(StoreErrorKind::TierRecallExecutionViolation, message)
            } else {
                StoreError::new(
                    StoreErrorKind::BackendIntegrityViolation,
                    format!("sqlite backend conversion failure: {message}"),
                )
            }
        }
        rusqlite::Error::SqliteFailure(code, message) => {
            if code.code == rusqlite::ErrorCode::ConstraintViolation {
                StoreError::new(
                    StoreErrorKind::DuplicateArtifactIdentity,
                    format!(
                        "sqlite constraint rejected authoritative write: {}",
                        message.unwrap_or_else(|| code.to_string())
                    ),
                )
            } else {
                StoreError::new(
                    StoreErrorKind::Io,
                    format!(
                        "sqlite backend failure {}: {}",
                        code,
                        message.unwrap_or_else(|| code.to_string())
                    ),
                )
            }
        }
        other => StoreError::new(
            StoreErrorKind::Io,
            format!("sqlite backend failure: {other}"),
        ),
    }
}

pub(super) fn table_exists(connection: &Connection, table_name: &str) -> Result<bool, StoreError> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1)",
            [table_name],
            |row| row.get::<_, i64>(0),
        )
        .map(|value| value != 0)
        .map_err(sqlite_error)
}

pub(super) fn table_row_count(
    connection: &Connection,
    table_name: &str,
) -> Result<i64, StoreError> {
    let sql = format!("SELECT COUNT(*) FROM {table_name}");
    connection
        .query_row(&sql, [], |row| row.get::<_, i64>(0))
        .map_err(sqlite_error)
}

pub(super) fn persist_bulk_json_record<T: serde::Serialize>(
    transaction: &Transaction<'_>,
    table: &str,
    artifact_id: &str,
    indexed_columns: Vec<(String, String)>,
    record: &T,
) -> Result<(), StoreError> {
    let payload = serde_json::to_string(record)?;
    let mut columns = vec!["artifact_id".to_string()];
    let mut placeholders = vec!["?1".to_string()];
    let mut values = vec![rusqlite::types::Value::Text(artifact_id.to_string())];
    let mut payload_index = 2usize;
    for (idx, (name, value)) in indexed_columns.iter().enumerate() {
        columns.push(name.clone());
        placeholders.push(format!("?{}", idx + 2));
        values.push(rusqlite::types::Value::Text(value.clone()));
        payload_index = idx + 3;
    }
    columns.push("payload_json".to_string());
    placeholders.push(format!("?{}", payload_index));
    values.push(rusqlite::types::Value::Text(payload));
    let sql = format!(
        "INSERT INTO {table}({}) VALUES ({})",
        columns.join(", "),
        placeholders.join(", ")
    );
    transaction
        .execute(&sql, rusqlite::params_from_iter(values))
        .map_err(sqlite_error)?;
    Ok(())
}

pub(super) fn ensure_branch_delta_layer_artifacts_column(
    connection: &Connection,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare("PRAGMA table_info(branch_delta_layer_records)")
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(sqlite_error)?;
    let mut has_artifacts_column = false;
    for row in rows {
        if row.map_err(sqlite_error)? == "artifacts_payload" {
            has_artifacts_column = true;
            break;
        }
    }
    if !has_artifacts_column {
        let default_payload = serde_json::to_string(&BranchDeltaLayerArtifacts::default())?;
        connection
            .execute(
                &format!(
                    "ALTER TABLE branch_delta_layer_records ADD COLUMN artifacts_payload TEXT NOT NULL DEFAULT '{}'",
                    default_payload.replace('\'', "''")
                ),
                [],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

pub(super) fn migrate_milestone_6_commit_coupled_layout_seed_storage(
    connection: &Connection,
) -> Result<(), StoreError> {
    if !table_exists(connection, "milestone_6_published_layout_request_records")? {
        return Ok(());
    }
    if table_row_count(connection, "milestone_6_commit_coupled_layout_seed_records")? > 0 {
        return Ok(());
    }
    connection
        .execute(
            "
        INSERT INTO milestone_6_commit_coupled_layout_seed_records(
            artifact_id,
            branch_id,
            frontier_commit_id,
            scope_class,
            payload_json
        )
        SELECT artifact_id, branch_id, frontier_commit_id, scope_class, payload_json
        FROM milestone_6_published_layout_request_records
        ",
            [],
        )
        .map_err(sqlite_error)?;
    Ok(())
}
