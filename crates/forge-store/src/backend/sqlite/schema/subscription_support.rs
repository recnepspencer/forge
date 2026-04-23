use crate::{
    failure::StoreError, SubscriptionSupportAccessStructure, SubscriptionSupportStoredRecordSet,
    SupportMaintenanceDescriptorRecord,
};
use rusqlite::{params, Connection};

use super::super::helpers::sqlite_error;

pub(super) fn create_subscription_support_schema(
    connection: &Connection,
) -> Result<(), StoreError> {
    let existing_record_table_missing_lookup_index =
        subscription_support_record_sets_exists(connection)?
            && !all_subscription_support_lookup_indexes_exist(connection)?;
    let missing_access_structures = if subscription_support_record_sets_exists(connection)? {
        missing_subscription_support_access_structures(connection)?
    } else {
        Vec::new()
    };

    connection
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS subscription_support_record_sets (
                storage_key TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                artifact_id TEXT NOT NULL,
                declaration_digest TEXT NOT NULL DEFAULT '',
                basis_digest TEXT NOT NULL DEFAULT '',
                cursor_digest TEXT NOT NULL DEFAULT '',
                checkpoint_digest TEXT NOT NULL DEFAULT '',
                compatibility_digest TEXT NOT NULL DEFAULT '',
                initial_classification TEXT,
                restart_shard TEXT,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_counter_snapshot (
                counter_id TEXT PRIMARY KEY,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_maintenance_descriptor_records (
                record_key TEXT PRIMARY KEY,
                family_id TEXT NOT NULL,
                support_role TEXT NOT NULL,
                maintenance_key TEXT NOT NULL,
                declaration_id TEXT NOT NULL,
                descriptor_digest TEXT NOT NULL,
                payload_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS subscription_support_access_structure_state (
                state_id TEXT PRIMARY KEY,
                verified INTEGER NOT NULL CHECK (verified IN (0, 1)),
                debted_json TEXT NOT NULL DEFAULT '[]'
            );
            INSERT OR IGNORE INTO subscription_support_access_structure_state
                (state_id, verified, debted_json) VALUES ('first_ship', 1, '[]');
            ",
        )
        .map_err(sqlite_error)?;
    ensure_subscription_support_access_state_columns(connection)?;
    ensure_subscription_support_projection_columns(connection)?;
    backfill_missing_subscription_support_projection_columns(connection)?;
    backfill_missing_subscription_support_maintenance_projection_columns(connection)?;
    connection
        .execute_batch(
            "
            CREATE INDEX IF NOT EXISTS idx_subscription_support_family_artifact
                ON subscription_support_record_sets(family_id, artifact_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_family
                ON subscription_support_record_sets(family_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_declaration
                ON subscription_support_record_sets(declaration_digest);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_basis
                ON subscription_support_record_sets(basis_digest);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_cursor
                ON subscription_support_record_sets(cursor_digest);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_checkpoint
                ON subscription_support_record_sets(checkpoint_digest);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_compatibility
                ON subscription_support_record_sets(compatibility_digest);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_classification
                ON subscription_support_record_sets(initial_classification);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_restart_shard
                ON subscription_support_record_sets(restart_shard);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_family
                ON subscription_support_maintenance_descriptor_records(family_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_declaration
                ON subscription_support_maintenance_descriptor_records(declaration_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_key
                ON subscription_support_maintenance_descriptor_records(maintenance_key);
            ",
        )
        .map_err(sqlite_error)?;
    if existing_record_table_missing_lookup_index {
        connection
            .execute(
                "UPDATE subscription_support_access_structure_state \
                 SET verified = 0, debted_json = ?1 WHERE state_id = 'first_ship'",
                [serde_json::to_string(&missing_access_structures)?],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

fn backfill_missing_subscription_support_maintenance_projection_columns(
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

fn ensure_subscription_support_access_state_columns(
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

fn backfill_missing_subscription_support_projection_columns(
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

fn subscription_support_record_sets_exists(connection: &Connection) -> Result<bool, StoreError> {
    sqlite_master_entry_exists(connection, "table", "subscription_support_record_sets")
}

fn all_subscription_support_lookup_indexes_exist(
    connection: &Connection,
) -> Result<bool, StoreError> {
    for (index_name, _) in REQUIRED_SUBSCRIPTION_SUPPORT_INDEXES {
        if !sqlite_master_entry_exists(connection, "index", index_name)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn missing_subscription_support_access_structures(
    connection: &Connection,
) -> Result<Vec<SubscriptionSupportAccessStructure>, StoreError> {
    let mut missing = Vec::new();
    for (index_name, access_structure) in REQUIRED_SUBSCRIPTION_SUPPORT_INDEXES {
        if !sqlite_master_entry_exists(connection, "index", index_name)? {
            missing.push(*access_structure);
        }
    }
    missing.sort();
    missing.dedup();
    Ok(missing)
}

fn sqlite_master_entry_exists(
    connection: &Connection,
    entry_type: &str,
    name: &str,
) -> Result<bool, StoreError> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = ?1 AND name = ?2)",
            [entry_type, name],
            |row| row.get::<_, i64>(0),
        )
        .map(|value| value != 0)
        .map_err(sqlite_error)
}

fn ensure_subscription_support_projection_columns(
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

fn subscription_support_column_exists(
    connection: &Connection,
    column_name: &str,
) -> Result<bool, StoreError> {
    let mut statement = connection
        .prepare("PRAGMA table_info(subscription_support_record_sets)")
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(sqlite_error)?;
    for row in rows {
        if row.map_err(sqlite_error)? == column_name {
            return Ok(true);
        }
    }
    Ok(false)
}

fn subscription_support_access_state_column_exists(
    connection: &Connection,
    column_name: &str,
) -> Result<bool, StoreError> {
    let mut statement = connection
        .prepare("PRAGMA table_info(subscription_support_access_structure_state)")
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(1))
        .map_err(sqlite_error)?;
    for row in rows {
        if row.map_err(sqlite_error)? == column_name {
            return Ok(true);
        }
    }
    Ok(false)
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

const REQUIRED_SUBSCRIPTION_SUPPORT_INDEXES: &[(&str, SubscriptionSupportAccessStructure)] = &[
    (
        "idx_subscription_support_family_artifact",
        SubscriptionSupportAccessStructure::ArtifactLookupByFamilyAndArtifact,
    ),
    (
        "idx_subscription_support_family",
        SubscriptionSupportAccessStructure::FamilyLookup,
    ),
    (
        "idx_subscription_support_declaration",
        SubscriptionSupportAccessStructure::DeclarationLookup,
    ),
    (
        "idx_subscription_support_basis",
        SubscriptionSupportAccessStructure::BasisLookup,
    ),
    (
        "idx_subscription_support_cursor",
        SubscriptionSupportAccessStructure::CursorLookup,
    ),
    (
        "idx_subscription_support_checkpoint",
        SubscriptionSupportAccessStructure::CheckpointLookup,
    ),
    (
        "idx_subscription_support_compatibility",
        SubscriptionSupportAccessStructure::CompatibilityLookup,
    ),
    (
        "idx_subscription_support_classification",
        SubscriptionSupportAccessStructure::ClassificationLookup,
    ),
    (
        "idx_subscription_support_restart_shard",
        SubscriptionSupportAccessStructure::RestartManifestSequence,
    ),
];
