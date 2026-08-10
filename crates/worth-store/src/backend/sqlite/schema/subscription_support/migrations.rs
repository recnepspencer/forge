use crate::{
    failure::StoreError, SubscriptionSupportStoredRecordSet, SupportActionDurableRecord,
    SupportMaintenanceDebtRecord, SupportMaintenanceDescriptorRecord,
};

use rusqlite::{params, Connection};

use super::super::super::helpers::sqlite_error;

use super::access::{
    subscription_support_access_state_column_exists, subscription_support_column_exists,
};

pub(super) fn migrate_subscription_support_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    ensure_subscription_support_access_state_columns(connection)?;
    ensure_subscription_support_projection_columns(connection)?;
    backfill_missing_subscription_support_projection_columns(connection)?;
    backfill_missing_subscription_support_action_projection_columns(connection)?;
    backfill_missing_subscription_support_maintenance_projection_columns(connection)?;
    backfill_missing_subscription_support_maintenance_debt_projection_columns(connection)?;
    Ok(())
}

pub(super) fn backfill_missing_subscription_support_maintenance_projection_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT record_key, family_id, support_role, maintenance_key, declaration_id, \
             descriptor_digest, payload_json \
             FROM subscription_support_maintenance_descriptor_records ORDER BY record_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
            ))
        })
        .map_err(sqlite_error)?;
    let mut candidates = Vec::new();
    for row in rows {
        candidates.push(row.map_err(sqlite_error)?);
    }
    drop(statement);
    for (
        record_key,
        family_id,
        support_role,
        maintenance_key,
        declaration_id,
        descriptor_digest,
        payload_json,
    ) in candidates
    {
        if !family_id.is_empty()
            && !support_role.is_empty()
            && !maintenance_key.is_empty()
            && !declaration_id.is_empty()
            && !descriptor_digest.is_empty()
        {
            continue;
        }
        let record: SupportMaintenanceDescriptorRecord = serde_json::from_str(&payload_json)?;
        connection
            .execute(
                "UPDATE subscription_support_maintenance_descriptor_records SET \
                 family_id = ?1, support_role = ?2, maintenance_key = ?3, declaration_id = ?4, descriptor_digest = ?5 \
                 WHERE record_key = ?6",
                params![
                    record.family_id().as_str(),
                    format!("{:?}", record.support_role()),
                    record.maintenance_key(),
                    record.declaration_id(),
                    record.descriptor_digest(),
                    record_key,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

pub(super) fn backfill_missing_subscription_support_maintenance_debt_projection_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT record_key, action_id, family_id, support_role, verdict, payload_json \
             FROM subscription_support_maintenance_debt_records ORDER BY record_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
            ))
        })
        .map_err(sqlite_error)?;
    let mut candidates = Vec::new();
    for row in rows {
        candidates.push(row.map_err(sqlite_error)?);
    }
    drop(statement);
    for (record_key, action_id, family_id, support_role, verdict, payload_json) in candidates {
        if !action_id.is_empty()
            && !family_id.is_empty()
            && !support_role.is_empty()
            && !verdict.is_empty()
        {
            continue;
        }
        let record: SupportMaintenanceDebtRecord = serde_json::from_str(&payload_json)?;
        connection
            .execute(
                "UPDATE subscription_support_maintenance_debt_records SET \
                 action_id = ?1, family_id = ?2, support_role = ?3, verdict = ?4 WHERE record_key = ?5",
                params![
                    record.action_id().as_str(),
                    record.family_id().as_str(),
                    format!("{:?}", record.support_role()),
                    format!("{:?}", record.verdict()),
                    record_key,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

pub(super) fn backfill_missing_subscription_support_action_projection_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT action_id, artifact_id, action_origin, publication_state, payload_json \
             FROM subscription_support_action_records ORDER BY action_id",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
            ))
        })
        .map_err(sqlite_error)?;
    let mut candidates = Vec::new();
    for row in rows {
        candidates.push(row.map_err(sqlite_error)?);
    }
    drop(statement);
    for (action_id, artifact_id, action_origin, publication_state, payload_json) in candidates {
        if !artifact_id.is_empty() && !action_origin.is_empty() && !publication_state.is_empty() {
            continue;
        }
        let record: SupportActionDurableRecord = serde_json::from_str(&payload_json)?;
        connection
            .execute(
                "UPDATE subscription_support_action_records SET artifact_id = ?1, action_origin = ?2, publication_state = ?3 \
                 WHERE action_id = ?4",
                params![
                    record.artifact_id().as_str(),
                    format!("{:?}", record.action_origin()),
                    format!("{:?}", record.publication_state()),
                    action_id,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

pub(super) fn ensure_subscription_support_access_state_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    if !subscription_support_access_state_column_exists(connection, "debted_json")? {
        connection
            .execute(
                "ALTER TABLE subscription_support_access_structure_state \
                 ADD COLUMN debted_json TEXT NOT NULL DEFAULT '[]'",
                [],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

pub(super) fn backfill_missing_subscription_support_projection_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT storage_key, declaration_digest, basis_digest, cursor_digest, \
             checkpoint_digest, compatibility_digest, initial_classification, restart_shard, \
             payload_json FROM subscription_support_record_sets ORDER BY storage_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok(SubscriptionSupportProjectionBackfillCandidate {
                storage_key: row.get(0)?,
                declaration_digest: row.get(1)?,
                basis_digest: row.get(2)?,
                cursor_digest: row.get(3)?,
                checkpoint_digest: row.get(4)?,
                compatibility_digest: row.get(5)?,
                initial_classification: row.get(6)?,
                restart_shard: row.get(7)?,
                payload_json: row.get(8)?,
            })
        })
        .map_err(sqlite_error)?;
    let mut candidates = Vec::new();
    for row in rows {
        candidates.push(row.map_err(sqlite_error)?);
    }
    drop(statement);

    for candidate in candidates {
        if !candidate.has_legacy_empty_projection() {
            continue;
        }
        let record_set: SubscriptionSupportStoredRecordSet =
            serde_json::from_str(&candidate.payload_json)?;
        connection
            .execute(
                "UPDATE subscription_support_record_sets SET \
                 declaration_digest = ?1, basis_digest = ?2, cursor_digest = ?3, \
                 checkpoint_digest = ?4, compatibility_digest = ?5, \
                 initial_classification = ?6, restart_shard = ?7 \
                 WHERE storage_key = ?8",
                params![
                    record_set.declaration_digest(),
                    record_set.basis_digest(),
                    record_set.cursor_digest(),
                    record_set.checkpoint_digest(),
                    record_set.compatibility_digest(),
                    record_set.initial_classification_index(),
                    record_set.restart_shard(),
                    candidate.storage_key,
                ],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}
pub(super) fn ensure_subscription_support_projection_columns(
    connection: &Connection,
) -> Result<(), StoreError> {
    for column in REQUIRED_SUBSCRIPTION_SUPPORT_COLUMNS {
        if !subscription_support_column_exists(connection, column.name)? {
            connection
                .execute(column.add_column_sql, [])
                .map_err(sqlite_error)?;
        }
    }
    Ok(())
}

struct SubscriptionSupportColumnMigration {
    name: &'static str,
    add_column_sql: &'static str,
}

struct SubscriptionSupportProjectionBackfillCandidate {
    storage_key: String,
    declaration_digest: String,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    initial_classification: Option<String>,
    restart_shard: Option<String>,
    payload_json: String,
}

impl SubscriptionSupportProjectionBackfillCandidate {
    fn has_legacy_empty_projection(&self) -> bool {
        self.declaration_digest.is_empty()
            && self.basis_digest.is_empty()
            && self.cursor_digest.is_empty()
            && self.checkpoint_digest.is_empty()
            && self.compatibility_digest.is_empty()
            && self.initial_classification.is_none()
            && self.restart_shard.is_none()
    }
}

const REQUIRED_SUBSCRIPTION_SUPPORT_COLUMNS: &[SubscriptionSupportColumnMigration] = &[
    SubscriptionSupportColumnMigration {
        name: "declaration_digest",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN declaration_digest TEXT NOT NULL DEFAULT ''",
    },
    SubscriptionSupportColumnMigration {
        name: "basis_digest",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN basis_digest TEXT NOT NULL DEFAULT ''",
    },
    SubscriptionSupportColumnMigration {
        name: "cursor_digest",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN cursor_digest TEXT NOT NULL DEFAULT ''",
    },
    SubscriptionSupportColumnMigration {
        name: "checkpoint_digest",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN checkpoint_digest TEXT NOT NULL DEFAULT ''",
    },
    SubscriptionSupportColumnMigration {
        name: "compatibility_digest",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN compatibility_digest TEXT NOT NULL DEFAULT ''",
    },
    SubscriptionSupportColumnMigration {
        name: "initial_classification",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN initial_classification TEXT",
    },
    SubscriptionSupportColumnMigration {
        name: "restart_shard",
        add_column_sql:
            "ALTER TABLE subscription_support_record_sets ADD COLUMN restart_shard TEXT",
    },
];
