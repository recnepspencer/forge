use crate::logic::runtime::RuntimeInstrumentation;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::logic::state::PartitionAccess;
use crate::transactions::data::{CommitConflict, TransactionIntent};

use super::entity_validation::validate_entity_intent;
use super::relation_validation::validate_relation_intent;

pub(crate) fn validate_intent(
    state: &impl PartitionAccess,
    schema_registry: &RelationalSchemaRegistry,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    intent: &TransactionIntent,
) -> Result<(), CommitConflict> {
    validate_entity_intent(state, schema_registry, intent)?;
    validate_relation_intent(
        state,
        schema_registry,
        default_cross_context_policy,
        instrumentation,
        intent,
    )?;
    Ok(())
}
