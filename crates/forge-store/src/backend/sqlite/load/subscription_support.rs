use crate::failure::{StoreError, StoreErrorKind};
use crate::{
    SubscriptionSupportAccessStructure, SubscriptionSupportCatalog,
    SubscriptionSupportCounterSnapshot, SubscriptionSupportStoredRecordSet,
    SupportMaintenanceDescriptorRecord,
};
use rusqlite::Connection;

use super::super::super::records::StoreState;
use super::super::helpers::{deserialize_json, sqlite_error};

pub(super) fn load_subscription_support(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    state.subscription_support_record_sets.clear();
    state
        .subscription_support_maintenance_descriptor_records
        .clear();
    let mut statement = connection
        .prepare(
            "SELECT storage_key, family_id, artifact_id, declaration_digest, basis_digest, \
             cursor_digest, checkpoint_digest, compatibility_digest, initial_classification, \
             restart_shard, payload_json FROM subscription_support_record_sets ORDER BY storage_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            let storage_key: String = row.get(0)?;
            let indexed = SubscriptionSupportIndexedProjection {
                family_id: row.get(1)?,
                artifact_id: row.get(2)?,
                declaration_digest: row.get(3)?,
                basis_digest: row.get(4)?,
                cursor_digest: row.get(5)?,
                checkpoint_digest: row.get(6)?,
                compatibility_digest: row.get(7)?,
                initial_classification: row.get(8)?,
                restart_shard: row.get(9)?,
            };
            let record_set: SubscriptionSupportStoredRecordSet = deserialize_json(row.get(10)?)?;
            Ok((storage_key, indexed, record_set))
        })
        .map_err(sqlite_error)?;
    for row in rows {
        let (storage_key, indexed, record_set) = row.map_err(sqlite_error)?;
        if storage_key != record_set.key().storage_key() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                format!(
                    "sqlite subscription-support record key `{storage_key}` does not match payload family/artifact key"
                ),
            ));
        }
        indexed.verify_matches(&record_set)?;
        state
            .subscription_support_record_sets
            .insert(storage_key, record_set);
    }
    drop(statement);

    let mut descriptor_statement = connection
        .prepare(
            "SELECT record_key, family_id, support_role, maintenance_key, declaration_id, \
             descriptor_digest, payload_json \
             FROM subscription_support_maintenance_descriptor_records ORDER BY record_key",
        )
        .map_err(sqlite_error)?;
    let descriptor_rows = descriptor_statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                SubscriptionSupportMaintenanceIndexedProjection {
                    family_id: row.get(1)?,
                    support_role: row.get(2)?,
                    maintenance_key: row.get(3)?,
                    declaration_id: row.get(4)?,
                    descriptor_digest: row.get(5)?,
                },
                deserialize_json::<SupportMaintenanceDescriptorRecord>(row.get(6)?)?,
            ))
        })
        .map_err(sqlite_error)?;
    for row in descriptor_rows {
        let (record_key, indexed, record) = row.map_err(sqlite_error)?;
        if record.record_key() != record_key {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support maintenance descriptor record key does not match payload",
            ));
        }
        indexed.verify_matches(&record)?;
        state
            .subscription_support_maintenance_descriptor_records
            .insert(record_key, record);
    }
    state.subscription_support_counter_snapshot = connection
        .query_row(
            "SELECT payload_json FROM subscription_support_counter_snapshot WHERE counter_id = 'first_ship'",
            [],
            |row| deserialize_json::<SubscriptionSupportCounterSnapshot>(row.get(0)?),
        )
        .or_else(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => Ok(SubscriptionSupportCounterSnapshot::default()),
            other => Err(other),
        })
        .map_err(sqlite_error)?;
    let (access_structures_verified, access_structure_debts) = connection
        .query_row(
            "SELECT verified, debted_json FROM subscription_support_access_structure_state WHERE state_id = 'first_ship'",
            [],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    deserialize_json::<Vec<SubscriptionSupportAccessStructure>>(row.get(1)?)?,
                ))
            },
        )
        .or_else(|error| match error {
            rusqlite::Error::QueryReturnedNoRows => Ok((1, Vec::new())),
            other => Err(other),
        })
        .and_then(|(value, debts)| match value {
            0 => Ok((false, normalize_access_structure_debts(debts))),
            1 => Ok((true, Vec::new())),
            other => Err(rusqlite::Error::IntegralValueOutOfRange(0, other)),
        })
        .map_err(sqlite_error)?;
    state.subscription_support_access_structures_verified = access_structures_verified;
    state.subscription_support_access_structure_debts =
        if !access_structures_verified && access_structure_debts.is_empty() {
            SubscriptionSupportCatalog::first_ship()
                .access_structures()
                .required()
                .to_vec()
        } else {
            access_structure_debts
        };
    Ok(())
}

fn normalize_access_structure_debts(
    mut debts: Vec<SubscriptionSupportAccessStructure>,
) -> Vec<SubscriptionSupportAccessStructure> {
    debts.sort();
    debts.dedup();
    debts
}

struct SubscriptionSupportIndexedProjection {
    family_id: String,
    artifact_id: String,
    declaration_digest: String,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    initial_classification: Option<String>,
    restart_shard: Option<String>,
}

struct SubscriptionSupportMaintenanceIndexedProjection {
    family_id: String,
    support_role: String,
    maintenance_key: String,
    declaration_id: String,
    descriptor_digest: String,
}

impl SubscriptionSupportIndexedProjection {
    fn verify_matches(
        &self,
        record_set: &SubscriptionSupportStoredRecordSet,
    ) -> Result<(), StoreError> {
        let expected = [
            (
                "family id",
                self.family_id.as_str(),
                record_set.key().family_id(),
            ),
            (
                "artifact id",
                self.artifact_id.as_str(),
                record_set.key().artifact_id(),
            ),
            (
                "declaration digest",
                self.declaration_digest.as_str(),
                record_set.declaration_digest(),
            ),
            (
                "basis digest",
                self.basis_digest.as_str(),
                record_set.basis_digest(),
            ),
            (
                "cursor digest",
                self.cursor_digest.as_str(),
                record_set.cursor_digest(),
            ),
            (
                "checkpoint digest",
                self.checkpoint_digest.as_str(),
                record_set.checkpoint_digest(),
            ),
            (
                "compatibility digest",
                self.compatibility_digest.as_str(),
                record_set.compatibility_digest(),
            ),
        ];
        for (label, indexed, payload) in expected {
            if indexed != payload {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    format!(
                        "sqlite subscription-support {label} index projection does not match payload"
                    ),
                ));
            }
        }
        if self.initial_classification != record_set.initial_classification_index() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support classification index projection does not match payload",
            ));
        }
        if self.restart_shard.as_deref() != record_set.restart_shard() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "sqlite subscription-support restart-shard index projection does not match payload",
            ));
        }
        Ok(())
    }
}

impl SubscriptionSupportMaintenanceIndexedProjection {
    fn verify_matches(
        &self,
        record: &SupportMaintenanceDescriptorRecord,
    ) -> Result<(), StoreError> {
        let support_role = format!("{:?}", record.support_role());
        let expected = [
            (
                "family id",
                self.family_id.as_str(),
                record.family_id().as_str(),
            ),
            (
                "support role",
                self.support_role.as_str(),
                support_role.as_str(),
            ),
            (
                "maintenance key",
                self.maintenance_key.as_str(),
                record.maintenance_key(),
            ),
            (
                "declaration id",
                self.declaration_id.as_str(),
                record.declaration_id(),
            ),
            (
                "descriptor digest",
                self.descriptor_digest.as_str(),
                record.descriptor_digest(),
            ),
        ];
        for (label, indexed, payload) in expected {
            if indexed != payload {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    format!(
                        "sqlite subscription-support maintenance descriptor {label} index projection does not match payload"
                    ),
                ));
            }
        }
        Ok(())
    }
}
