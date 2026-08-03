//! Exact typed capability-request resolution against the primary graph.

use std::collections::BTreeMap;

use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityRequestProjection, ErasedApplicationCapabilityEntitySelector,
    },
    application_schema::ApplicationSchemaMember,
};
use worth_query_installation::facade::{ApplicationSchema, WorthQueryInstalledApplicationSchema};
use worth_relational::facade::identity::EntityId;

use super::{WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind};
use crate::domain_computation::primary_graph::{
    resolve_at_snapshot, WorthQueryApplicationEntityIdentity, WorthQueryPrimaryGraphLayout,
    WorthQueryPrincipalResolutionMode, WorthQueryResolvedEntityEvidence,
};

pub(super) struct WorthQueryResolvedCapabilityRequest<Schema, Scope> {
    pub(super) resource: WorthQueryApplicationEntityIdentity<Schema, Scope>,
    pub(super) related: Option<EntityId>,
    pub(super) context: BTreeMap<WorthQueryCapabilityContextKey, EntityId>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct WorthQueryCapabilityContextKey {
    pub(super) context: String,
    pub(super) context_type: String,
    pub(super) slot: String,
    pub(super) slot_type: String,
    pub(super) entity: String,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn resolve_capability_request<Schema, Scope, Context>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryPrimaryGraphLayout,
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
) -> Result<
    WorthQueryResolvedCapabilityRequest<Schema, Scope>,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
{
    let resource = resolve_selector(
        relational,
        snapshot,
        layout,
        schema,
        projection.resource().entity(),
        projection.resource().aspect(),
        projection.resource().field(),
        projection.resource().scalar_family(),
        projection.resource().value_type(),
        projection.resource().value(),
        runtime_authority,
    )?;
    let related = projection
        .related()
        .map(|related| {
            let selector = related.selector();
            resolve_erased_selector(
                relational,
                snapshot,
                layout,
                schema,
                selector,
                runtime_authority,
            )
            .map(|evidence| evidence.entity_id)
        })
        .transpose()?;
    let mut context = BTreeMap::new();
    for selected in projection.context_value().entities() {
        let slot = selected.slot();
        let selector = selected.selector();
        if selector.entity() != slot.entity() {
            return Err(denial(slot.slot()));
        }
        let evidence = resolve_erased_selector(
            relational,
            snapshot,
            layout,
            schema,
            selector,
            runtime_authority,
        )?;
        let key = WorthQueryCapabilityContextKey {
            context: slot.context().to_string(),
            context_type: slot.context_type().to_string(),
            slot: slot.slot().to_string(),
            slot_type: slot.slot_type().to_string(),
            entity: slot.entity().to_string(),
        };
        if context.insert(key, evidence.entity_id).is_some() {
            return Err(denial(slot.slot()));
        }
    }
    Ok(WorthQueryResolvedCapabilityRequest {
        resource: WorthQueryApplicationEntityIdentity::mint(resource),
        related,
        context,
    })
}

fn resolve_erased_selector<Schema>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryPrimaryGraphLayout,
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    selector: &ErasedApplicationCapabilityEntitySelector,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
) -> Result<WorthQueryResolvedEntityEvidence, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    resolve_selector(
        relational,
        snapshot,
        layout,
        schema,
        selector.entity(),
        selector.aspect(),
        selector.field(),
        selector.scalar_family(),
        selector.value_type(),
        selector.value(),
        runtime_authority,
    )
}

#[allow(clippy::too_many_arguments)]
fn resolve_selector<Schema>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryPrimaryGraphLayout,
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    entity: &str,
    aspect: &str,
    field: &str,
    scalar_family: worth_foundational::facade::ScalarAspectType,
    value_type: &str,
    value: &worth_foundational::facade::AspectValue,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
) -> Result<WorthQueryResolvedEntityEvidence, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let installed = schema
        .installed_declaration()
        .members()
        .iter()
        .any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Field {
                    entity: installed_entity,
                    aspect: installed_aspect,
                    field: installed_field,
                    scalar_family: installed_scalar,
                    value_type: installed_value_type,
                    equality_queryable: true,
                    ..
                } if installed_entity == entity
                    && installed_aspect == aspect
                    && installed_field == field
                    && *installed_scalar == scalar_family
                    && installed_value_type == value_type
            )
        });
    if !installed {
        return Err(denial(field));
    }
    let field_layout = layout
        .equality_field(entity, aspect, field)
        .ok_or_else(|| denial(field))?;
    resolve_at_snapshot(
        relational,
        snapshot,
        field_layout,
        value.clone(),
        WorthQueryPrincipalResolutionMode::Ordinary,
        runtime_authority,
        schema.binding_identity(),
        entity,
        field,
    )
    .map_err(|_| denial(field))
}

fn denial(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
        subject,
    )
}
