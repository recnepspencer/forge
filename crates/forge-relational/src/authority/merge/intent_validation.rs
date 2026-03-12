use crate::capabilities::{SchemaSource, StorageRead};
use crate::logic::runtime::RuntimeInstrumentation;
use crate::transactions::data::{CommitConflict, MutationIntent};

use super::entity_validation::validate_entity_intent;
use super::relation_validation::validate_relation_intent;

pub(crate) fn validate_intent(
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    validate_entity_intent(state, schema_source, intent)?;
    validate_relation_intent(
        state,
        schema_source,
        default_cross_context_policy,
        instrumentation,
        intent,
    )?;
    Ok(())
}
