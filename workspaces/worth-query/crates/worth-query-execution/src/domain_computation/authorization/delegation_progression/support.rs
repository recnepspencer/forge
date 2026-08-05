use std::sync::Arc;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityDelegationRequest, ApplicationCapabilityRequest,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
};

use super::super::capability_binding_lowering::relation;
use super::super::capability_request_resolution::{
    resolve_capability_request, resolve_erased_selector, WorthQueryResolvedCapabilityRequest,
};
use super::super::decision_facts::WorthQueryDelegationActivationDecisionFact;
use super::super::delegation_admission::observe_capability;
use super::super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAuthorizationDecisionFact,
    WorthQueryAuthorizationTimeSample, WorthQueryOperationAuthorizationDenial,
    WorthQueryRetainedCapabilitySupport,
};
use super::{delegation_denial, inconsistent, observe_narrowing, stale};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::primary_graph::WorthQueryResolvedEntityEvidence;

pub(super) struct WorthQueryDelegationResolvedRequest {
    pub(super) parent: worth_relational::facade::identity::EntityId,
    pub(super) grantor: worth_relational::facade::identity::EntityId,
    pub(super) grantee: worth_relational::facade::identity::EntityId,
    pub(super) resource: worth_relational::facade::identity::EntityId,
    pub(super) related: Option<worth_relational::facade::identity::EntityId>,
    pub(super) activation_context: Vec<WorthQueryResolvedActivationContext>,
}

pub(super) struct WorthQueryResolvedActivationContext {
    pub(super) traversal: worth_relational::facade::authorization::RelationalAuthorizationTraversal,
    pub(super) entity: worth_relational::facade::identity::EntityId,
}

struct WorthQueryDelegationRequestResolution<Schema, Scope> {
    target: WorthQueryResolvedCapabilityRequest<Schema, Scope>,
    parent: WorthQueryResolvedEntityEvidence,
    grantee: WorthQueryResolvedEntityEvidence,
    activation_context: Vec<WorthQueryResolvedActivationContext>,
}

#[allow(clippy::too_many_arguments)]
pub(super) fn authorize_target_support<
    Schema,
    CommandCapability,
    TargetCapability,
    TargetOperation,
    TargetInput,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    target_capability: &WorthQueryInstalledApplicationCapability<
        Schema,
        TargetCapability,
        TargetOperation,
        TargetInput,
    >,
    installed: &super::super::capability_registry::WorthQueryInstalledCapabilityPlan,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<
        Schema,
        CommandCapability,
        Operation,
        Input,
    >,
    proposed: &worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationRequestProjection<
        Schema,
        <Input as ApplicationCapabilityDelegationRequest<Schema, TargetCapability>>::Scope,
        <Input as ApplicationCapabilityDelegationRequest<Schema, TargetCapability>>::Context,
    >,
) -> Result<
    (
        WorthQueryDelegationResolvedRequest,
        WorthQueryRetainedCapabilitySupport,
    ),
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, CommandCapability>
        + ApplicationCapabilityDelegationRequest<Schema, TargetCapability>,
{
    let graph = runtime
        .runtime
        .primary_graph()
        .ok_or_else(|| stale(installed.contract.name()))?;
    let snapshot = access
        .graph_work
        .mutation_snapshot()
        .ok_or_else(|| inconsistent(access.operation()))?
        .clone();
    let handle = access
        .graph_work
        .mutation_handle()
        .ok_or_else(|| inconsistent(access.operation()))?
        .clone();
    let sample = runtime.sample_capability_time(installed)?;
    let resolution = handle.with_runtime(|relational| {
        resolve_delegation_request(
            relational,
            &snapshot,
            graph.layout(),
            runtime,
            installed,
            proposed,
        )
    })?;
    if resolution.parent.entity_kind != installed.grant_kind
        || resolution.grantee.entity_kind != installed.principal_kind
    {
        return Err(delegation_denial(installed));
    }
    let retained = WorthQueryRetainedCapabilityRequest::capture(
        *target_capability.identity().bytes(),
        access.principal_entity_id,
        proposed.target(),
        &resolution.target,
    );
    let resolved = WorthQueryDelegationResolvedRequest {
        parent: resolution.parent.entity_id,
        grantor: access.principal_entity_id,
        grantee: resolution.grantee.entity_id,
        resource: resolution.target.resource.entity_id(),
        related: resolution.target.related,
        activation_context: resolution.activation_context,
    };
    let (mut parent_decision, narrowing) = handle.with_runtime(|relational| {
        observe_exact_parent_support(
            relational,
            snapshot,
            runtime,
            installed,
            access.graph_work.identity(),
            &retained,
            &sample,
            proposed,
            &resolved,
        )
    })?;
    parent_decision
        .attach_delegation_activation(WorthQueryDelegationActivationDecisionFact::new(
            access.graph_work.identity(),
            narrowing,
        ))
        .map_err(|()| inconsistent(installed.contract.name()))?;
    let supporting = WorthQueryRetainedCapabilitySupport::active(
        parent_decision,
        Arc::clone(&installed.capability_authority_identity),
        resolved.parent,
        retained,
        sample,
    );
    Ok((resolved, supporting))
}

#[allow(clippy::too_many_arguments)]
fn resolve_delegation_request<Schema, Scope, Context>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout,
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    installed: &super::super::capability_registry::WorthQueryInstalledCapabilityPlan,
    proposed: &worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
) -> Result<
    WorthQueryDelegationRequestResolution<Schema, Scope>,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
{
    Ok(WorthQueryDelegationRequestResolution {
        target: resolve_capability_request(
            relational,
            snapshot,
            layout,
            &runtime.installed_schema,
            proposed.target(),
            runtime.runtime.authority_identity(),
        )?,
        parent: resolve_erased_selector(
            relational,
            snapshot,
            layout,
            &runtime.installed_schema,
            proposed.parent(),
            runtime.runtime.authority_identity(),
        )?,
        grantee: resolve_erased_selector(
            relational,
            snapshot,
            layout,
            &runtime.installed_schema,
            proposed.grantee(),
            runtime.runtime.authority_identity(),
        )?,
        activation_context: resolve_activation_context(
            relational,
            snapshot,
            layout,
            &runtime.installed_schema,
            runtime.runtime.authority_identity(),
            installed,
            proposed.activation_context(),
        )?,
    })
}

#[allow(clippy::too_many_arguments)]
fn observe_exact_parent_support<Schema, Scope, Context>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: worth_relational::facade::snapshots::SnapshotHandle,
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    installed: &super::super::capability_registry::WorthQueryInstalledCapabilityPlan,
    session: crate::domain_computation::provider_session::WorthQueryGraphWorkSessionIdentity,
    retained: &WorthQueryRetainedCapabilityRequest,
    sample: &WorthQueryAuthorizationTimeSample,
    proposed: &worth_query_declaration::facade::application_capability::ApplicationCapabilityDelegationRequestProjection<Schema, Scope, Context>,
    resolved: &WorthQueryDelegationResolvedRequest,
) -> Result<
    (
        WorthQueryAuthorizationDecisionFact,
        worth_relational::facade::authorization::RelationalAuthorizationObservationEvidence,
    ),
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
{
    let observed = observe_capability(
        session,
        relational,
        snapshot.clone(),
        runtime.authorization.bridge(),
        installed,
        retained,
        sample,
        Some(resolved.parent),
        None,
    )?;
    let (parent_decision, observed_parent) = observed.into_parts();
    if observed_parent != resolved.parent {
        return Err(delegation_denial(installed));
    }
    let narrowing = observe_narrowing(relational, snapshot, installed, proposed, resolved)?;
    Ok((parent_decision, narrowing))
}

#[allow(clippy::too_many_arguments)]
fn resolve_activation_context<Schema>(
    relational: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphLayout,
    schema: &worth_query_installation::facade::WorthQueryInstalledApplicationSchema<Schema>,
    runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    installed: &super::super::capability_registry::WorthQueryInstalledCapabilityPlan,
    proposed: &[worth_query_declaration::facade::application_capability::ApplicationCapabilityRelatedEntitySelector<Schema>],
) -> Result<Vec<WorthQueryResolvedActivationContext>, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    let expected = installed
        .delegation
        .activation
        .as_ref()
        .ok_or_else(|| delegation_denial(installed))?
        .context_relations
        .as_slice();
    if proposed.len() != expected.len() {
        return Err(delegation_denial(installed));
    }
    let mut resolved = Vec::with_capacity(expected.len());
    let mut relation_kinds = std::collections::BTreeSet::new();
    for selected in proposed {
        let traversal = relation(
            layout,
            selected.relation(),
            worth_relational::facade::authorization::RelationalAuthorizationTraversalDirection::Forward,
        )?;
        if traversal.from_kind() != installed.grant_kind
            || !expected.iter().any(|candidate| candidate == &traversal)
            || !relation_kinds.insert(traversal.relation_kind())
        {
            return Err(delegation_denial(installed));
        }
        let entity = resolve_erased_selector(
            relational,
            snapshot,
            layout,
            schema,
            selected.selector(),
            runtime_authority,
        )?;
        if entity.entity_kind != traversal.to_kind() {
            return Err(delegation_denial(installed));
        }
        resolved.push(WorthQueryResolvedActivationContext {
            traversal,
            entity: entity.entity_id,
        });
    }
    resolved.sort_by_key(|context| context.traversal.relation_kind());
    Ok(resolved)
}
