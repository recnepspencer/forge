use std::collections::BTreeSet;

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
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

mod binding;
use binding::bind_request;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_elevation_request<Capability, Operation, Input>(
        &self,
        mut access: WorthQueryAdmittedApplicationCapabilityAccess<
            Schema,
            Capability,
            Operation,
            Input,
        >,
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
        let (capability_identity, installed) =
            installed_request_lifecycle(self, &access, operation)?;
        validate_request_projection(installed, &access, &proposed)?;
        let (binding, supporting) =
            bind_request(self, capability_identity, installed, &access, &proposed)?;
        access
            .authorization
            .retain_supporting(supporting)
            .map_err(|()| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    installed.contract.name(),
                )
            })?;
        access.graph_work.record_decision_facts(1);
        progress_capability_operation(self, access, operation, preconditions, true)?
            .bind_elevation_request(binding)
    }
}

fn installed_request_lifecycle<'runtime, Schema, Capability, Operation, Input>(
    runtime: &'runtime WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<
    ([u8; 32], &'runtime WorthQueryInstalledCapabilityPlan),
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let Some((capability, command_capability, role)) = runtime
        .authorization
        .elevation_lifecycle_operation::<Operation, Input>(operation.operation())
        .map_err(|()| stale_operation(operation.operation()))?
    else {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationLifecycleRoleMismatch,
            operation.operation(),
        ));
    };
    if role != WorthQueryElevationLifecycleOperationRole::Request
        || access.authorization.installed_capability_identity() != command_capability
    {
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
    let maximum_duration = installed
        .elevation
        .as_ref()
        .ok_or_else(|| projection_denial(installed.contract.name()))?
        .lifecycle
        .maximum_duration;
    if duration.is_zero() || duration > maximum_duration {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationDurationExceeded,
            installed.contract.name(),
        ));
    }
    Ok(())
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
