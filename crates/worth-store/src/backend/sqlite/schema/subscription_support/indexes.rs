use crate::failure::StoreError;
use rusqlite::Connection;

use super::super::super::helpers::sqlite_error;

pub(super) fn create_subscription_support_lookup_indexes(
    connection: &Connection,
) -> Result<(), StoreError> {
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
            CREATE INDEX IF NOT EXISTS idx_subscription_support_action_artifact
                ON subscription_support_action_records(artifact_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_action_origin
                ON subscription_support_action_records(action_origin);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_action_state
                ON subscription_support_action_records(publication_state);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_family
                ON subscription_support_maintenance_descriptor_records(family_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_declaration
                ON subscription_support_maintenance_descriptor_records(declaration_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_key
                ON subscription_support_maintenance_descriptor_records(maintenance_key);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_debt_action
                ON subscription_support_maintenance_debt_records(action_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_debt_family
                ON subscription_support_maintenance_debt_records(family_id);
            CREATE INDEX IF NOT EXISTS idx_subscription_support_maintenance_debt_verdict
                ON subscription_support_maintenance_debt_records(verdict);
            ",
        )
        .map_err(sqlite_error)?;
    Ok(())
}
