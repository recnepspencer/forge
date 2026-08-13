mod endpoint_admission;
mod endpoint_update_admission;
mod relation_creation_admission;
mod relation_identity_scan;
mod relation_target_admission;
mod schema_relation_admission;

use std::collections::BTreeSet;

use crate::capabilities::{SchemaSource, StorageRead};
use crate::identity::data::VersionId;
use crate::runtime::{RelationalRuntime, RuntimeInstrumentation};
use crate::transactions::data::{
    CreateIntent, CreatedEntityRef, MutationIntent, RelationMutationIntent,
};

pub(super) fn validate_relation_intent(
    runtime: &RelationalRuntime,
    state: &impl StorageRead,
    schema_source: &impl SchemaSource,
    default_cross_context_policy: crate::config::data::CrossContextPolicy,
    instrumentation: &RuntimeInstrumentation,
    branch_basis_version_id: Option<VersionId>,
    created_entities: &BTreeSet<CreatedEntityRef>,
    intent: &MutationIntent,
) -> Result<(), crate::transactions::data::CommitConflict> {
    match intent {
        MutationIntent::Create(CreateIntent::Relation(spec)) => {
            relation_creation_admission::validate_relation_creation_intent(
                state,
                schema_source,
                default_cross_context_policy,
                instrumentation,
                created_entities,
                spec,
            )
        }
        MutationIntent::Create(CreateIntent::RelationAspects(spec)) => {
            relation_creation_admission::validate_relation_creation(
                state,
                schema_source,
                default_cross_context_policy,
                instrumentation,
                created_entities,
                spec.partition_id,
                spec.kind_id,
                &spec.source,
                &spec.target,
            )
        }
        MutationIntent::Create(CreateIntent::BulkRelations(spec)) => {
            relation_creation_admission::validate_bulk_relation_creation_intent(
                state,
                schema_source,
                default_cross_context_policy,
                instrumentation,
                created_entities,
                spec.partition_id,
                spec.kind_id,
                &spec.endpoints,
            )
        }
        MutationIntent::Relation(RelationMutationIntent::UpdateEndpoints(spec)) => {
            endpoint_update_admission::validate_relation_endpoint_update_intent(
                runtime,
                state,
                schema_source,
                default_cross_context_policy,
                instrumentation,
                branch_basis_version_id,
                created_entities,
                spec,
            )
        }
        MutationIntent::Relation(RelationMutationIntent::ApplyAspectPatch(spec)) => {
            relation_target_admission::validate_existing_relation_target(
                runtime,
                state,
                branch_basis_version_id,
                spec.relation_id,
            )
        }
        MutationIntent::Relation(RelationMutationIntent::Delete(spec)) => {
            relation_target_admission::validate_existing_relation_target(
                runtime,
                state,
                branch_basis_version_id,
                spec.relation_id,
            )
        }
        MutationIntent::Create(CreateIntent::Entity(_))
        | MutationIntent::Create(CreateIntent::EntityAspects(_))
        | MutationIntent::Create(CreateIntent::BulkEntities(_))
        | MutationIntent::Entity(_) => Ok(()),
    }
}
