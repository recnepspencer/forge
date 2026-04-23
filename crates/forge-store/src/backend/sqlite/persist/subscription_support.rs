use crate::failure::StoreError;
use rusqlite::{params, Transaction};

use super::super::super::records::StoreState;
use super::super::helpers::sqlite_error;

pub(super) fn persist_subscription_support(
    transaction: &Transaction<'_>,
    state: &StoreState,
) -> Result<(), StoreError> {
    for (storage_key, record_set) in &state.subscription_support_record_sets {
        transaction
            .execute(
                "INSERT INTO subscription_support_record_sets \
                 (storage_key, family_id, artifact_id, declaration_digest, basis_digest, \
                  cursor_digest, checkpoint_digest, compatibility_digest, initial_classification, \
                  restart_shard, payload_json) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    storage_key,
                    record_set.key().family_id(),
                    record_set.key().artifact_id(),
                    record_set.declaration_digest(),
                    record_set.basis_digest(),
                    record_set.cursor_digest(),
                    record_set.checkpoint_digest(),
                    record_set.compatibility_digest(),
                    record_set.initial_classification_index(),
                    record_set.restart_shard(),
                    serde_json::to_string(record_set)?,
                ],
            )
            .map_err(sqlite_error)?;
    }
    for (record_key, record) in &state.subscription_support_maintenance_descriptor_records {
        transaction
            .execute(
                "INSERT INTO subscription_support_maintenance_descriptor_records \
                 (record_key, family_id, support_role, maintenance_key, declaration_id, descriptor_digest, payload_json) \
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    record_key,
                    record.family_id().as_str(),
                    format!("{:?}", record.support_role()),
                    record.maintenance_key(),
                    record.declaration_id(),
                    record.descriptor_digest(),
                    serde_json::to_string(record)?,
                ],
            )
            .map_err(sqlite_error)?;
    }
    transaction
        .execute(
            "INSERT INTO subscription_support_counter_snapshot (counter_id, payload_json) VALUES (?1, ?2)",
            params![
                "first_ship",
                serde_json::to_string(&state.subscription_support_counter_snapshot)?,
            ],
        )
        .map_err(sqlite_error)?;
    transaction
        .execute(
            "INSERT INTO subscription_support_access_structure_state \
             (state_id, verified, debted_json) VALUES (?1, ?2, ?3)",
            params![
                "first_ship",
                if state.subscription_support_access_structures_verified {
                    1
                } else {
                    0
                },
                serde_json::to_string(&state.subscription_support_access_structure_debts)?,
            ],
        )
        .map_err(sqlite_error)?;
    Ok(())
}
