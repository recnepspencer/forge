//! Runtime-, schema-, plan-, session-, and snapshot-bound capability observation.

use worth_foundational::facade::AspectValue;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityRequestProjection,
    ErasedApplicationCapabilityEntitySelector,
};
use worth_query_installation::facade::ApplicationSchema;

use super::super::request_resolution::{
    resolve_capability_request, resolve_erased_selector, WorthQueryResolvedCapabilityRequest,
};
use super::WorthQueryAdmittedApplicationCapabilityAccess;
use crate::domain_computation::authorization::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::primary_graph::{
    WorthQueryEntityResolutionTruth, WorthQueryPrimaryGraphApplicationRuntime,
    WorthQueryPrincipalResolutionMode, WorthQueryResolvedEntity,
};
use crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity;

mod delegation;
mod elevation;
pub(in crate::domain_computation::authorization) use delegation::WorthQueryDelegationResolvedRequest;

pub(in crate::domain_computation::authorization) struct WorthQueryExactCapabilityObservation<
    'observation,
    Schema,
> {
    runtime: &'observation WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    relational: &'observation worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &'observation worth_relational::facade::snapshots::SnapshotHandle,
    resolution:
        crate::domain_computation::primary_graph::WorthQueryInstalledEntityResolutionContext,
    layout: &'observation crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout,
    session: WorthQueryGraphWorkSessionIdentity,
    principal: worth_relational::facade::identity::EntityId,
}

pub(super) fn with_exact_observation<Schema, Capability, Operation, Input, Output>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    observe: impl FnOnce(&WorthQueryExactCapabilityObservation<'_, Schema>) -> Output,
) -> Option<Output>
where
    Schema: ApplicationSchema,
    Input: worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest<
        Schema,
        Capability,
    >,
{
    let graph = runtime.runtime.primary_graph()?;
    if runtime.runtime.authority_identity() != access.runtime_authority
        || runtime.installed_schema.binding_identity() != access.binding_identity
        || graph.binding_identity() != &access.binding_identity
    {
        return None;
    }
    let installed = runtime
        .authorization
        .capability_plan_by_identity(&access.authorization.installed_capability_identity())?;
    if installed.capability_authority_identity.as_ref()
        != access.authorization.capability_authority_identity()
    {
        return None;
    }
    let snapshot = access.graph_work.mutation_snapshot()?;
    let handle = access.graph_work.mutation_handle()?;
    let resolution = graph.retain_entity_resolution_context();
    Some(handle.with_runtime(|relational| {
        observe(&WorthQueryExactCapabilityObservation {
            runtime,
            relational,
            snapshot,
            resolution,
            layout: graph.layout(),
            session: access.graph_work.identity(),
            principal: access.principal_entity_id,
        })
    }))
}

impl<Schema> WorthQueryExactCapabilityObservation<'_, Schema>
where
    Schema: ApplicationSchema,
{
    fn resolve_request<Scope, Context>(
        &self,
        projection: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    ) -> Result<
        WorthQueryResolvedCapabilityRequest<Schema, Scope>,
        WorthQueryOperationAuthorizationDenial,
    > {
        let truth = self.resolution_truth(projection.resource().field())?;
        resolve_capability_request(&truth, &self.runtime.installed_schema, projection)
    }

    fn resolve_selector(
        &self,
        selector: &ErasedApplicationCapabilityEntitySelector,
    ) -> Result<WorthQueryResolvedEntity, WorthQueryOperationAuthorizationDenial> {
        let truth = self.resolution_truth(selector.field())?;
        resolve_erased_selector(&truth, &self.runtime.installed_schema, selector)
    }

    fn resolve_lifecycle_field(
        &self,
        field: &ApplicationCapabilityFieldBinding,
        value: AspectValue,
    ) -> Result<WorthQueryResolvedEntity, WorthQueryOperationAuthorizationDenial> {
        self.resolution_truth(field.field())?
            .resolve(field.entity(), field.aspect(), field.field(), value)
            .map_err(|_| denial(field.field()))
    }

    fn resolution_truth(
        &self,
        subject: &str,
    ) -> Result<WorthQueryEntityResolutionTruth<'_>, WorthQueryOperationAuthorizationDenial> {
        self.resolution
            .at_snapshot(
                self.relational,
                self.snapshot,
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(|_| denial(subject))
    }
}

fn denial(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(
        WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
        subject,
    )
}
