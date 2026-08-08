//! Capability-access progression into installed operation authority.

use std::time::Instant;

use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_schema::TypedMutationPreconditions,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};

use super::admission::admit_request;
use super::admitted_operation::{
    WorthQueryAdmittedApplicationOperationInput, WorthQueryOperationAuthorizationBasis,
};
use super::graph_work_session::transition_capability_to_operation_graph_work;
use super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryOperationScopeBinding, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::{
    bind_mutation_preconditions, WorthQueryBoundMutationPreconditions,
    WorthQueryPrimaryGraphApplicationRuntime,
};

mod projection_equivalence;

use projection_equivalence::same_projection;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_capability_operation<Capability, Operation, Input>(
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
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        progress_capability_operation(
            self,
            access,
            operation,
            preconditions,
            WorthQueryCapabilityOperationProgression::Ordinary,
        )
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) enum WorthQueryCapabilityOperationProgression {
    Ordinary,
    DelegationActivation,
    CapabilityRevocation,
    ElevationLifecycle,
}

pub(super) fn progress_capability_operation<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    mut access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    preconditions: TypedMutationPreconditions<
        Schema,
        Operation,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
    progression: WorthQueryCapabilityOperationProgression,
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
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validate_progression_authority(runtime, &access, operation, progression)?;
    validate_current_projection(&access)?;
    let preconditions = bind_progression_preconditions(runtime, &access, operation, preconditions)?;
    validate_access_lifecycle(&access)?;
    let resource_binding_identity = access
        .operation_admission_identity
        .resource_binding_identity();
    let mut graph_work = transition_capability_to_operation_graph_work(
        runtime,
        operation,
        &resource_binding_identity,
        access.resolved.resource.entity_id(),
        access.graph_work,
    )?;
    validate_operation_graph_work_authority(&graph_work, operation)?;
    runtime
        .refresh_capability_authorization_for_graph_work(&mut access.authorization, &graph_work)?;
    graph_work.set_retained_decision_facts(access.authorization.exact_fact_count());
    let session_identity = graph_work.identity();
    let authorization =
        WorthQueryRetainedAuthorizationDecisionFacts::capability(access.authorization);
    if !authorization.belongs_to_session(session_identity) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            operation.operation(),
        ));
    }
    let governed_input_identity = access.governed_input_identity;
    Ok(WorthQueryAdmittedApplicationOperation::mint(
        WorthQueryAdmittedApplicationOperationInput {
            admission_identity: access.operation_admission_identity,
            runtime_authority: runtime.runtime.authority_identity(),
            binding_identity: operation.binding_identity().clone(),
            operation: operation.operation().to_string(),
            operation_authority_identity: operation.authority_identity().to_string(),
            operation_authority_identity_bytes: operation.authority_identity_bytes(),
            operation_scope_binding: WorthQueryOperationScopeBinding::mint(
                runtime.runtime.authority_identity(),
                operation.binding_identity(),
                operation.authority_identity(),
                access.principal_entity_id,
                access.resolved.resource.entity_id(),
            ),
            scope_entity_id: access.resolved.resource.entity_id(),
            scope_entity_kind: access.resolved.resource.entity_kind(),
            scope_entity_name: access.resolved.resource.entity_name().to_string(),
            authentication_valid_until: access.authentication_valid_until,
            request_scope: access.request_scope,
            contracts: operation.contracts().clone(),
            mutation_preconditions: preconditions,
            authorization_admission_work: access.canonical_work,
            authorization,
            governed_input_identity,
            authorization_basis: WorthQueryOperationAuthorizationBasis::Capability {
                input: access.input,
            },
            graph_work,
        },
    ))
}

fn validate_progression_authority<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    progression: WorthQueryCapabilityOperationProgression,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validate_access_lifecycle(access)?;
    validate_installed_operation_identity(runtime, access, operation, progression)?;
    validate_capability_graph_work_authority(access, operation)
}

fn validate_installed_operation_identity<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    progression: WorthQueryCapabilityOperationProgression,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let lifecycle = runtime
        .authorization
        .elevation_lifecycle_operation::<Operation, Input>(operation.operation())
        .map_err(|()| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                operation.operation(),
            )
        })?;
    if lifecycle.is_some()
        && progression != WorthQueryCapabilityOperationProgression::ElevationLifecycle
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationTransitionRequired,
            operation.operation(),
        ));
    }
    if operation
        .execution_posture()
        .requires_delegation_activation()
        && progression != WorthQueryCapabilityOperationProgression::DelegationActivation
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::DelegationTransitionRequired,
            operation.operation(),
        ));
    }
    if operation
        .execution_posture()
        .requires_capability_revocation()
        && progression != WorthQueryCapabilityOperationProgression::CapabilityRevocation
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::DelegationTransitionRequired,
            operation.operation(),
        ));
    }
    if access.runtime_authority != runtime.runtime.authority_identity() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            access.operation.as_ref(),
        ));
    }
    if access.binding_identity != *operation.binding_identity()
        || runtime.installed_schema.binding_identity() != *operation.binding_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema,
            access.operation.as_ref(),
        ));
    }
    if access.operation.as_ref() != operation.operation() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
            access.operation.as_ref(),
        ));
    }
    runtime
        .runtime
        .installed_packages()
        .validate_application_operation(operation)
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                operation.operation(),
            )
        })
}

fn validate_capability_graph_work_authority<Schema, Capability, Operation, Input>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    if access.graph_work.runtime_authority() != access.runtime_authority
        || access.graph_work.binding() != &access.binding_identity
        || access.graph_work.principal() != access.principal_entity_id
        || access.graph_work.capability_access_context()
            != Some(access.authorization.installed_capability_identity())
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            operation.operation(),
        ));
    }
    if !operation.contracts().authorization().requires_capability() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityNotRequired,
            operation.operation(),
        ));
    }
    if access.authorization.exact_fact_count() != access.graph_work.retained_decision_facts() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            operation.operation(),
        ));
    }
    Ok(())
}

fn validate_operation_graph_work_authority<Schema, Operation, Input>(
    graph_work: &crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    if graph_work.binding() != operation.binding_identity()
        || graph_work.obligation() != operation.graph_obligations().identity()
        || graph_work.subject_authority() != operation.authority_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            operation.operation(),
        ));
    }
    Ok(())
}

fn validate_current_projection<Schema, Capability, Operation, Input>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let current = access.input.capability_request().map_err(|rejection| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
            rejection.subject(),
        )
    })?;
    if same_projection(&access.projection, &current) {
        Ok(())
    } else {
        Err(denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
            access.operation.as_ref(),
        ))
    }
}

fn bind_progression_preconditions<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    preconditions: TypedMutationPreconditions<
        Schema,
        Operation,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
) -> Result<WorthQueryBoundMutationPreconditions, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            access.operation.as_ref(),
        )
    })?;
    bind_mutation_preconditions(
        preconditions,
        operation.contracts(),
        access.resolved.resource.entity_name(),
        access.resolved.resource.entity_id(),
        graph.layout(),
    )
    .map_err(|()| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::MutationPreconditionRejected,
            operation.operation(),
        )
    })
}

fn validate_access_lifecycle<Schema, Capability, Operation, Input>(
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    admit_request(&access.request_scope, &access.operation)?;
    if Instant::now() >= access.authentication_valid_until {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
            access.operation.as_ref(),
        ));
    }
    Ok(())
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
