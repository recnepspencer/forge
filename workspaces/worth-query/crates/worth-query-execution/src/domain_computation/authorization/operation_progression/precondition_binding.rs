//! Operation-precondition binding phase owner.

use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_schema::TypedMutationPreconditions,
};
use worth_query_installation::facade::ApplicationSchema;

use crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial;
use crate::domain_computation::primary_graph::WorthQueryBoundMutationPreconditions;

use super::authority_validation::{ValidatedCapabilityOperation, ValidatedConventionalOperation};

pub(super) struct PreconditionBoundConventionalOperation<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Operation,
    Input,
    Scope,
> {
    validated: ValidatedConventionalOperation<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >,
    preconditions: WorthQueryBoundMutationPreconditions,
}

pub(super) fn bind_conventional_preconditions<
    Schema,
    Principal,
    PrincipalIdentity,
    Operation,
    Input,
    Scope,
>(
    validated: ValidatedConventionalOperation<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >,
    preconditions: TypedMutationPreconditions<Schema, Operation, Scope>,
) -> Result<
    PreconditionBoundConventionalOperation<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
{
    let (runtime, scope, operation) = validated.context();
    let graph = runtime.runtime.primary_graph().ok_or_else(|| {
        WorthQueryOperationAuthorizationDenial::new(
            crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            operation.operation(),
        )
    })?;
    let preconditions = crate::domain_computation::primary_graph::bind_mutation_preconditions(
        preconditions,
        operation.contracts(),
        scope.entity_name(),
        scope.entity_id(),
        graph.layout(),
    )
    .map_err(|()| {
        WorthQueryOperationAuthorizationDenial::new(
            crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenialKind::MutationPreconditionRejected,
            operation.operation(),
        )
    })?;
    Ok(PreconditionBoundConventionalOperation {
        validated,
        preconditions,
    })
}

impl<'a, Schema, Principal, PrincipalIdentity, Operation, Input, Scope>
    PreconditionBoundConventionalOperation<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >
{
    pub(super) fn into_parts(
        self,
    ) -> (
        ValidatedConventionalOperation<
            'a,
            Schema,
            Principal,
            PrincipalIdentity,
            Operation,
            Input,
            Scope,
        >,
        WorthQueryBoundMutationPreconditions,
    ) {
        (self.validated, self.preconditions)
    }
}

pub(super) struct PreconditionBoundCapabilityOperation<'a, Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validated: ValidatedCapabilityOperation<'a, Schema, Capability, Operation, Input>,
    preconditions: WorthQueryBoundMutationPreconditions,
}

pub(super) fn bind_capability_preconditions<Schema, Capability, Operation, Input>(
    validated: ValidatedCapabilityOperation<'_, Schema, Capability, Operation, Input>,
    preconditions: TypedMutationPreconditions<
        Schema,
        Operation,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
    >,
) -> Result<
    PreconditionBoundCapabilityOperation<'_, Schema, Capability, Operation, Input>,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let (runtime, access, operation) = validated.context();
    let preconditions = access.bind_operation_preconditions(runtime, operation, preconditions)?;
    Ok(PreconditionBoundCapabilityOperation {
        validated,
        preconditions,
    })
}

impl<'a, Schema, Capability, Operation, Input>
    PreconditionBoundCapabilityOperation<'a, Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    pub(super) fn into_parts(
        self,
    ) -> (
        ValidatedCapabilityOperation<'a, Schema, Capability, Operation, Input>,
        WorthQueryBoundMutationPreconditions,
    ) {
        (self.validated, self.preconditions)
    }
}
