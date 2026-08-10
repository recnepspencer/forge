use crate::failure::{StoreError, StoreErrorKind};
use crate::SubscriptionSupportStoredRecordSet;

use super::super::super::super::records::StoreState;
use super::super::super::helpers::{deserialize_json, sqlite_error};
use rusqlite::Connection;

pub(super) fn load_stored_record_sets(
    connection: &Connection,
    state: &mut StoreState,
) -> Result<(), StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT storage_key, family_id, artifact_id, declaration_digest, basis_digest, \
             cursor_digest, checkpoint_digest, compatibility_digest, initial_classification, \
             restart_shard, payload_json FROM subscription_support_record_sets ORDER BY storage_key",
        )
        .map_err(sqlite_error)?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                StoredRecordSetIndexedProjection {
                    family_id: row.get(1)?,
                    artifact_id: row.get(2)?,
                    declaration_digest: row.get(3)?,
                    basis_digest: row.get(4)?,
                    cursor_digest: row.get(5)?,
                    checkpoint_digest: row.get(6)?,
                    compatibility_digest: row.get(7)?,
                    initial_classification: row.get(8)?,
                    restart_shard: row.get(9)?,
                },
                deserialize_json::<SubscriptionSupportStoredRecordSet>(row.get(10)?)?,
            ))
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
    Ok(())
}

struct StoredRecordSetIndexedProjection {
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

impl StoredRecordSetIndexedProjection {
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
