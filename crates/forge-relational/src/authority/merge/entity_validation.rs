use crate::diagnostics::data::DiagnosticCode;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::logic::state::PartitionAccess;
use crate::transactions::data::{CommitConflict, TransactionIntent};
use crate::validation::logic::schema_error_to_commit_conflict;

use super::record_lookup::entity_exists_in_state;

pub(super) fn validate_entity_intent(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    match intent {
        TransactionIntent::CreateEntity(spec) => schema_registry
            .resolve_entity(spec.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::BulkCreateEntities { kind_id, .. } => schema_registry
            .resolve_entity(*kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        TransactionIntent::UpdateEntity { entity_id, .. }
        | TransactionIntent::DeleteEntity { entity_id }
        | TransactionIntent::ReplaceEntity { entity_id, .. } => {
            validate_existing_entity_intent(state, schema_registry, *entity_id, intent)
        }
        TransactionIntent::CreateRelation(_)
        | TransactionIntent::BulkCreateRelations { .. }
        | TransactionIntent::DeleteRelation { .. } => Ok(()),
    }
}

fn validate_existing_entity_intent(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    entity_id: crate::identity::data::EntityId,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    if !entity_exists_in_state(state, entity_id) {
        return Err(CommitConflict {
            code: DiagnosticCode::StaleHandle,
            detail: format!("entity {:?} is stale or absent", entity_id),
        });
    }

    match intent {
        TransactionIntent::ReplaceEntity { replacement, .. } => schema_registry
            .resolve_entity(replacement.kind_id)
            .map(|_| ())
            .map_err(schema_error_to_commit_conflict),
        _ => Ok(()),
    }
}
