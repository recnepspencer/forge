//! Owner-sealed progression from authorization into operation admission.

mod authority_validation;
pub(in crate::domain_computation::authorization) mod authorization_revalidation;
mod precondition_binding;
mod transition;

pub use transition::WorthQueryAdmittedApplicationOperation;
pub(in crate::domain_computation) use transition::WorthQueryOperationAdmissionIdentity;
pub(in crate::domain_computation::authorization) use transition::{
    WorthQueryAuthorizedCapabilityOperation, WorthQueryCapabilityOperationProgression,
    WorthQueryCapabilityTransitionPermit, WorthQueryOperationAuthorizationBasis,
};

pub(in crate::domain_computation::authorization) fn progress_capability_operation<
    Schema,
    Capability,
    Operation,
    Input,
>(
    runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: crate::domain_computation::authorization::WorthQueryAdmittedApplicationCapabilityAccess<
        Schema,
        Capability,
        Operation,
        Input,
    >,
    operation: &worth_query_installation::facade::WorthQueryInstalledApplicationOperation<
        Schema,
        Operation,
        Input,
    >,
    preconditions: worth_query_declaration::facade::application_schema::TypedMutationPreconditions<
        Schema,
        Operation,
        <Input as worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest<
            Schema,
            Capability,
        >>::Scope,
    >,
    progression: WorthQueryCapabilityOperationProgression,
) -> Result<
    crate::domain_computation::authorization::WorthQueryAdmittedApplicationOperation<
        Schema,
        Operation,
        Input,
        <Input as worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest<
            Schema,
            Capability,
        >>::Scope,
    >,
    crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial,
>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest<
        Schema,
        Capability,
    >,
{
    let validated = authority_validation::validate_capability_operation(
        runtime,
        access,
        operation,
        progression,
    )?;
    let bound = precondition_binding::bind_capability_preconditions(validated, preconditions)?;
    let transition = transition::transition_capability_operation(bound)?;
    Ok(crate::domain_computation::authorization::WorthQueryAdmittedApplicationOperation::from_authorized_transition(
        transition.into(),
    ))
}

pub(super) fn progress_conventional_operation<
    Schema,
    Principal,
    PrincipalIdentity,
    Operation,
    Input,
    Scope,
>(
    runtime: &crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime<
        Schema,
    >,
    principal: &crate::domain_computation::primary_graph::WorthQueryAuthenticatedPrincipal<
        Schema,
        Principal,
        PrincipalIdentity,
    >,
    scope: &crate::domain_computation::primary_graph::WorthQueryApplicationEntityIdentity<
        Schema,
        Scope,
    >,
    operation: &worth_query_installation::facade::WorthQueryInstalledApplicationOperation<
        Schema,
        Operation,
        Input,
    >,
    preconditions: worth_query_declaration::facade::application_schema::TypedMutationPreconditions<
        Schema,
        Operation,
        Scope,
    >,
    request: &worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
) -> Result<
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial,
>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    let validated = authority_validation::validate_conventional_operation(
        runtime, principal, scope, operation, request,
    )?;
    let bound = precondition_binding::bind_conventional_preconditions(validated, preconditions)?;
    transition::transition_conventional_operation(bound)
}
