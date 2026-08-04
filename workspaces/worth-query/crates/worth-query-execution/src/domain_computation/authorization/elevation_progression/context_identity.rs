use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityRequest,
};
use worth_query_installation::facade::ApplicationSchema;
use worth_relational::facade::identity::EntityId;

use super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::super::capability_request_resolution::WorthQueryCapabilityContextKey;
use super::super::WorthQueryAdmittedApplicationCapabilityAccess;
use crate::domain_computation::primary_graph::{
    resolve_at_snapshot, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode,
};

pub(super) fn selected_elevation_entity<Schema, Capability, Operation, Input>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Option<EntityId>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let lifecycle = installed.contract.elevation().definition()?.lifecycle();
    let slot = lifecycle.elevation_slot();
    access
        .resolved
        .context
        .get(&WorthQueryCapabilityContextKey {
            context: slot.context().to_string(),
            context_type: slot.context_type().to_string(),
            slot: slot.slot().to_string(),
            slot_type: slot.slot_type().to_string(),
            entity: slot.entity().to_string(),
        })
        .copied()
}

pub(super) fn selected_review_entity<Schema, Capability, Operation, Input>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    installed: &WorthQueryInstalledCapabilityPlan,
) -> Option<EntityId>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let lifecycle = installed.contract.elevation().definition()?.lifecycle();
    let slot = lifecycle.review_slot();
    access
        .resolved
        .context
        .get(&WorthQueryCapabilityContextKey {
            context: slot.context().to_string(),
            context_type: slot.context_type().to_string(),
            slot: slot.slot().to_string(),
            slot_type: slot.slot_type().to_string(),
            entity: slot.entity().to_string(),
        })
        .copied()
}

pub(super) fn resolve_lifecycle_identity<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    field: &ApplicationCapabilityFieldBinding,
    value: &AspectValue,
) -> Option<EntityId>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let graph = runtime.runtime.primary_graph()?;
    let layout = graph
        .layout()
        .equality_field(field.entity(), field.aspect(), field.field())?;
    let snapshot = access.graph_work.mutation_snapshot()?;
    access
        .graph_work
        .mutation_handle()?
        .with_runtime(|relational| {
            resolve_at_snapshot(
                relational,
                snapshot,
                layout,
                value.clone(),
                WorthQueryPrincipalResolutionMode::Ordinary,
                runtime.runtime.authority_identity(),
                runtime.installed_schema.binding_identity(),
                field.entity(),
                field.field(),
            )
        })
        .ok()
        .map(|evidence| evidence.entity_id)
}
