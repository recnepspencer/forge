//! Capability-access progression into installed operation authority.

use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_schema::TypedMutationPreconditions,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};

use crate::domain_computation::authorization::graph_work_session::transition_capability_to_operation_graph_work;
use crate::domain_computation::authorization::operation_progression::authorization_revalidation::WorthQueryOperationAuthorizationRevalidation;
use crate::domain_computation::authorization::operation_progression::{
    WorthQueryAuthorizedCapabilityOperation, WorthQueryCapabilityOperationProgression,
    WorthQueryCapabilityTransitionPermit, WorthQueryOperationAuthorizationBasis,
};
use crate::domain_computation::authorization::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryOperationScopeBinding,
};
use crate::domain_computation::primary_graph::{
    WorthQueryBoundMutationPreconditions, WorthQueryPrimaryGraphApplicationRuntime,
};

mod authority_validation;
mod precondition_binding;
mod projection_equivalence;

use authority_validation::{
    validate_operation_graph_work_authority, validate_progression_authority,
};
use precondition_binding::bind_progression_preconditions;
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
        crate::domain_computation::authorization::operation_progression::progress_capability_operation(
            self,
            access,
            operation,
            preconditions,
            WorthQueryCapabilityOperationProgression::Ordinary,
        )
    }
}

pub(in crate::domain_computation::authorization) fn validate_capability_operation_authority<
    Schema,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    progression: WorthQueryCapabilityOperationProgression,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validate_progression_authority(runtime, access, operation, progression)?;
    validate_current_projection(access)
}

pub(in crate::domain_computation::authorization) fn bind_capability_operation_preconditions<
    Schema,
    Capability,
    Operation,
    Input,
>(
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
    bind_progression_preconditions(runtime, access, operation, preconditions)
}

pub(in crate::domain_computation::authorization) fn transition_capability_operation<
    Schema,
    Capability,
    Operation,
    Input,
>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    preconditions: WorthQueryBoundMutationPreconditions,
    permit: WorthQueryCapabilityTransitionPermit,
) -> Result<WorthQueryAuthorizedCapabilityOperation<Input>, WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let resource_binding_identity = access
        .operation_admission_identity
        .resource_binding_identity();
    let resource_entity_id = access.resource_entity_id();
    let resource_entity_kind = access.resource_entity_kind();
    let resource_entity_name = access.resource_entity_name().to_string();
    let graph_work = transition_capability_to_operation_graph_work(
        runtime,
        operation,
        &resource_binding_identity,
        resource_entity_id,
        access.graph_work,
    )?;
    validate_operation_graph_work_authority(&graph_work, operation)?;
    let (authorization, graph_work) = WorthQueryOperationAuthorizationRevalidation::bind(
        &permit,
        runtime,
        access.authorization,
        graph_work,
    )
    .revalidate(operation.operation())?;
    let governed_input_identity = access.governed_input_identity;
    let installation = permit.bind_installation(
        runtime.runtime.authority_identity(),
        operation.binding_identity().clone(),
        operation.operation().to_string(),
        operation.authority_identity().to_string(),
        operation.authority_identity_bytes(),
        operation.contracts().clone(),
    );
    let subject = permit.bind_subject(
        WorthQueryOperationScopeBinding::mint(
            runtime.runtime.authority_identity(),
            operation.binding_identity(),
            operation.authority_identity(),
            access.principal_entity_id,
            resource_entity_id,
        ),
        resource_entity_id,
        resource_entity_kind,
        resource_entity_name,
        access.authentication_valid_until,
        access.request_scope,
    );
    let evidence = permit.bind_evidence(
        access.operation_admission_identity,
        preconditions,
        access.canonical_work,
        authorization,
        governed_input_identity,
        WorthQueryOperationAuthorizationBasis::Capability {
            input: access.input,
        },
        graph_work,
    );
    Ok(permit.authorize(installation, subject, evidence))
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

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
