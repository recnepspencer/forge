use crate::{failure::StoreError, SubscriptionSupportAccessStructure};

use super::super::super::helpers::sqlite_error;
use rusqlite::Connection;

pub(super) struct SubscriptionSupportAccessGap {
    pub(super) record_table_missing_lookup_index: bool,
    pub(super) missing_access_structures: Vec<SubscriptionSupportAccessStructure>,
}

pub(super) fn capture_subscription_support_access_gap(
    connection: &Connection,
) -> Result<SubscriptionSupportAccessGap, StoreError> {
    let record_table_exists = subscription_support_record_sets_exists(connection)?;
    Ok(SubscriptionSupportAccessGap {
        record_table_missing_lookup_index: record_table_exists
            && !all_subscription_support_lookup_indexes_exist(connection)?,
        missing_access_structures: if record_table_exists {
            missing_subscription_support_access_structures(connection)?
        } else {
            Vec::new()
        },
    })
}

pub(super) fn mark_subscription_support_access_debt(
    connection: &Connection,
    gap: SubscriptionSupportAccessGap,
) -> Result<(), StoreError> {
    if gap.record_table_missing_lookup_index {
        connection
            .execute(
                "UPDATE subscription_support_access_structure_state \
                 SET verified = 0, debted_json = ?1 WHERE state_id = 'first_ship'",
                [serde_json::to_string(&gap.missing_access_structures)?],
            )
            .map_err(sqlite_error)?;
    }
    Ok(())
}

pub(super) fn subscription_support_record_sets_exists(
    connection: &Connection,
) -> Result<bool, StoreError> {
    sqlite_master_entry_exists(connection, "table", "subscription_support_record_sets")
}

pub(super) fn all_subscription_support_lookup_indexes_exist(
    connection: &Connection,
) -> Result<bool, StoreError> {
    for (index_name, _) in REQUIRED_SUBSCRIPTION_SUPPORT_INDEXES {
        if !sqlite_master_entry_exists(connection, "index", index_name)? {
            return Ok(false);
        }
    }
    Ok(true)
}

pub(super) fn missing_subscription_support_access_structures(
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

pub(super) fn sqlite_master_entry_exists(
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

pub(super) fn subscription_support_column_exists(
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

pub(super) fn subscription_support_access_state_column_exists(
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

pub(super) const REQUIRED_SUBSCRIPTION_SUPPORT_INDEXES: &[(
    &str,
    SubscriptionSupportAccessStructure,
)] = &[
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
