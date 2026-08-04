use std::collections::BTreeSet;
use std::sync::Arc;

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityCardinalityDimension, ApplicationCapabilityElevationRequest,
    ApplicationCapabilityElevationRequestProjection, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRequest,
};
use worth_query_declaration::facade::application_schema::TypedMutationPreconditions;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};

use super::super::capability_operation_progression::progress_capability_operation;
use super::super::capability_registry::{
    WorthQueryElevationLifecycleOperationRole, WorthQueryInstalledCapabilityPlan,
};
use super::super::capability_request_resolution::{
    resolve_capability_request, resolve_erased_selector,
};
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use super::request_binding::WorthQueryElevationRequestBinding;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_elevation_request<Capability, Operation, Input>(
        &self,
        access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        preconditions: TypedMutationPreconditions<
            Schema,
            Operation,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        >,
    ) -> Result<
        WorthQueryAdmittedApplicationOperation<
            Schema,
            Operation,
            Input,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        >,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>
            + ApplicationCapabilityElevationRequest<
                Schema,
                Operation,
                Scope = <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
                Context = <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
            >,
    {
        let proposed = access.input.elevation_request().map_err(|rejection| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ElevationRequestRejected,
                rejection.subject(),
            )
        })?;
        let (capability_identity, installed) = installed_request_lifecycle(self, operation)?;
        validate_request_projection(installed, &access, &proposed)?;
        let binding = bind_request(self, capability_identity, installed, &access, &proposed)?;
        progress_capability_operation(self, access, operation, preconditions, true)?
            .bind_elevation_request(binding)
    }
}

fn installed_request_lifecycle<'runtime, Schema, Operation, Input>(
    runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<
    ([u8; 32], &'runtime WorthQueryInstalledCapabilityPlan),
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
{
    let Some((capability, role)) = runtime
        .authorization
        .elevation_lifecycle_operation::<Operation, Input>(operation.operation())
        .map_err(|()| stale_operation(operation.operation()))?
    else {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationLifecycleRoleMismatch,
            operation.operation(),
        ));
    };
    if role != WorthQueryElevationLifecycleOperationRole::Request {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationLifecycleRoleMismatch,
            operation.operation(),
        ));
    }
    let installed = runtime
        .authorization
        .capability_plan_by_identity(&capability)
        .filter(|plan| plan.elevation.is_some())
        .ok_or_else(|| stale_operation(operation.operation()))?;
    Ok((capability, installed))
}

fn validate_request_projection<Schema, Capability, Operation, Input>(
    installed: &WorthQueryInstalledCapabilityPlan,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    proposed: &ApplicationCapabilityElevationRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let target = proposed.target();
    let request = &installed.request;
    let elevation = installed
        .contract
        .elevation()
        .definition()
        .ok_or_else(|| projection_denial(installed.contract.name()))?;
    if target.elevation_selector().is_some()
        || target.action() != &request.action
        || target.purpose() != &request.purpose
        || target.resource().entity() != request.resource_entity
        || target.context_value().context() != request.context
        || target.context_value().context_type() != request.context_type
        || !cardinality_admitted(request.cardinality, target.cardinality_value())
        || target.field_value().is_some() != request.field.is_some()
        || target.amount_value().is_some() != request.amount.is_some()
        || !same_resource(target.resource(), access.projection.resource())
        || !relation_matches(installed, target)
        || !context_matches(installed, target)
        || proposed.grant().entity() != installed.contract.grant_entity()
        || proposed.elevation_identity().field() != elevation.identity()
        || proposed.review_identity().field() != elevation.review().identity()
        || proposed.reason().field() != elevation.reason()
    {
        return Err(projection_denial(installed.contract.name()));
    }
    let duration = proposed.duration();
    if duration.is_zero() || duration > elevation.maximum_duration() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationDurationExceeded,
            installed.contract.name(),
        ));
    }
    Ok(())
}

fn bind_request<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    capability_identity: [u8; 32],
    installed: &WorthQueryInstalledCapabilityPlan,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    proposed: &ApplicationCapabilityElevationRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
) -> Result<WorthQueryElevationRequestBinding, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            access.operation(),
        )
    })?;
    let snapshot = access
        .graph_work
        .mutation_snapshot()
        .ok_or_else(|| projection_denial(access.operation()))?
        .clone();
    let handle = access
        .graph_work
        .mutation_handle()
        .ok_or_else(|| projection_denial(access.operation()))?
        .clone();
    let (target, grant) = handle.with_runtime(|relational| {
        let target = resolve_capability_request(
            relational,
            &snapshot,
            graph.layout(),
            &runtime.installed_schema,
            proposed.target(),
            runtime.runtime.authority_identity(),
        )?;
        let grant = resolve_erased_selector(
            relational,
            &snapshot,
            graph.layout(),
            &runtime.installed_schema,
            proposed.grant(),
            runtime.runtime.authority_identity(),
        )?;
        Ok((target, grant))
    })?;
    if target.resource.entity_id() != access.resolved.resource.entity_id()
        || grant.entity_kind != installed.grant_kind
    {
        return Err(projection_denial(installed.contract.name()));
    }
    let elevation = installed
        .elevation
        .as_ref()
        .ok_or_else(|| projection_denial(installed.contract.name()))?;
    let interval = runtime
        .authorization_clock
        .sample_interval(elevation.temporal.timeline, proposed.duration())
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                installed.contract.name(),
            )
        })?;
    let lifecycle = &elevation.lifecycle;
    Ok(WorthQueryElevationRequestBinding {
        capability_identity,
        capability_authority_identity: Arc::clone(&installed.capability_authority_identity),
        requester: access.principal_entity_id,
        resource: target.resource.entity_id(),
        grant: grant.entity_id,
        elevation_kind: elevation.elevation_kind,
        review_kind: lifecycle.review_kind,
        elevation_identity_field: lifecycle.identity.clone(),
        elevation_identity: proposed.elevation_identity().value().clone(),
        reason_field: lifecycle.reason.clone(),
        reason: proposed.reason().value().clone(),
        status_field: lifecycle.status.clone(),
        requested_status: lifecycle.requested.clone(),
        not_before_field: elevation.temporal.not_before.clone(),
        issued_at: interval.issued.value().clone(),
        not_after_field: elevation.temporal.not_after.clone(),
        expires_at: interval.expires,
        review_identity_field: lifecycle.review_identity.clone(),
        review_identity: proposed.review_identity().value().clone(),
        review_status_field: lifecycle.review_status.clone(),
        review_required_status: lifecycle.review_required.clone(),
        requester_relation: lifecycle.requester_relation,
        grant_relation: lifecycle.grant_relation,
        review_relation: lifecycle.review_relation,
    })
}

fn relation_matches<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &worth_query_declaration::facade::application_capability::ApplicationCapabilityRequestProjection<
        Schema,
        Scope,
        Context,
    >,
) -> bool {
    match (installed.contract.target().relation(), projection.related()) {
        (ApplicationCapabilityRelationDimension::NotApplicable, None) => true,
        (ApplicationCapabilityRelationDimension::Bound(expected), Some(actual)) => {
            expected == actual.relation()
        }
        _ => false,
    }
}

fn context_matches<Schema, Scope, Context>(
    installed: &WorthQueryInstalledCapabilityPlan,
    projection: &worth_query_declaration::facade::application_capability::ApplicationCapabilityRequestProjection<
        Schema,
        Scope,
        Context,
    >,
) -> bool {
    let expected = installed
        .paths
        .iter()
        .flat_map(|path| path.context_anchors.iter())
        .map(|anchor| {
            (
                anchor.context.as_str(),
                anchor.context_type.as_str(),
                anchor.slot.as_str(),
                anchor.slot_type.as_str(),
                anchor.entity.as_str(),
            )
        })
        .collect::<BTreeSet<_>>();
    let actual = projection
        .context_value()
        .entities()
        .iter()
        .map(|selected| {
            let slot = selected.slot();
            (
                slot.context(),
                slot.context_type(),
                slot.slot(),
                slot.slot_type(),
                slot.entity(),
            )
        })
        .collect::<BTreeSet<_>>();
    expected == actual
}

fn same_resource<Schema, Scope>(
    left: &worth_query_declaration::facade::application_capability::ApplicationCapabilityEntitySelector<
        Schema,
        Scope,
    >,
    right: &worth_query_declaration::facade::application_capability::ApplicationCapabilityEntitySelector<
        Schema,
        Scope,
    >,
) -> bool {
    left.entity() == right.entity()
        && left.aspect() == right.aspect()
        && left.field() == right.field()
        && left.scalar_family() == right.scalar_family()
        && left.value_type() == right.value_type()
        && left.value() == right.value()
}

const fn cardinality_admitted(
    expected: ApplicationCapabilityCardinalityDimension,
    actual: u32,
) -> bool {
    match expected {
        ApplicationCapabilityCardinalityDimension::One => actual == 1,
        ApplicationCapabilityCardinalityDimension::Many => actual > 0,
        ApplicationCapabilityCardinalityDimension::Bounded(limit) => actual > 0 && actual <= limit,
    }
}

fn projection_denial(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::ElevationRequestRejected,
        subject,
    )
}

fn stale_operation(subject: impl Into<String>) -> WorthQueryOperationAuthorizationDenial {
    denial(
        WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
        subject,
    )
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
