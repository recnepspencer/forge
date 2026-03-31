use crate::capabilities::{SchemaSource, StorageRead};
use crate::identity::data::VersionId;
use crate::logic::runtime::{RelationalRuntime, RuntimeInstrumentation};
use crate::transactions::data::{CommitConflict, MutationIntent};

use super::entity_validation::validate_entity_intent;
use super::relation_validation::validate_relation_intent;

pub(crate) fn validate_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    branch_basis_version_id: Option<VersionId>,
    intent: &MutationIntent,
) -> Result<(), CommitConflict> {
    validate_entity_intent(
        runtime,
        state,
        schema_source,
        branch_basis_version_id,
        intent,
    )?;
    validate_relation_intent(
        runtime,
        state,
        schema_source,
        default_cross_context_policy,
        instrumentation,
        branch_basis_version_id,
        intent,
    )?;
    Ok(())
}
