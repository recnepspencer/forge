use crate::capabilities::{SchemaSource, StorageRead};
use crate::identity::data::VersionId;
use crate::runtime::{RelationalRuntime, RuntimeInstrumentation};
use crate::transactions::data::{CommitConflict, CreateIntent, CreatedEntityRef, MutationIntent};
use std::collections::BTreeSet;

use super::entity_validation::validate_entity_intent;
use super::relation_validation::validate_relation_intent;

pub(crate) fn validate_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    branch_basis_version_id: Option<VersionId>,
    created_entities: &BTreeSet<CreatedEntityRef>,
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
        created_entities,
        intent,
    )?;
    Ok(())
}

pub(crate) fn collect_created_entity_refs(
    intents: &[MutationIntent],
) -> BTreeSet<CreatedEntityRef> {
    let mut created = BTreeSet::new();
    for intent in intents {
        match intent {
            MutationIntent::Create(CreateIntent::Entity(spec)) => {
                created.insert(CreatedEntityRef {
                    partition_id: spec.partition_id,
                    kind_id: spec.kind_id,
                    client_key: spec.client_key.clone(),
                });
            }
            MutationIntent::Create(CreateIntent::EntityAspects(spec)) => {
                created.insert(CreatedEntityRef {
                    partition_id: spec.partition_id,
                    kind_id: spec.kind_id,
                    client_key: spec.client_key.clone(),
                });
            }
            MutationIntent::Create(CreateIntent::BulkEntities(spec)) => {
                for client_key in &spec.client_keys {
                    created.insert(CreatedEntityRef {
                        partition_id: spec.partition_id,
                        kind_id: spec.kind_id,
                        client_key: client_key.clone(),
                    });
                }
            }
            MutationIntent::Create(CreateIntent::Relation(_))
            | MutationIntent::Create(CreateIntent::RelationAspects(_))
            | MutationIntent::Create(CreateIntent::BulkRelations(_))
            | MutationIntent::Entity(_)
            | MutationIntent::Relation(_) => {}
        }
    }
    created
}
