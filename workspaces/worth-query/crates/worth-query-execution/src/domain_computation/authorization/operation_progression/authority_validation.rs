//! Exact operation-authority validation phase owner.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};

use crate::domain_computation::authorization::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryOperationAuthorizationDenial,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationEntityIdentity, WorthQueryAuthenticatedPrincipal,
};

use super::WorthQueryCapabilityOperationProgression;

pub(super) struct ValidatedConventionalOperation<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Operation,
    Input,
    Scope,
> {
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    scope: &'a WorthQueryApplicationEntityIdentity<Schema, Scope>,
    operation: &'a WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    request: &'a WorthQueryRequestScope,
}

pub(super) fn validate_conventional_operation<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Operation,
    Input,
    Scope,
>(
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    scope: &'a WorthQueryApplicationEntityIdentity<Schema, Scope>,
    operation: &'a WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    request: &'a WorthQueryRequestScope,
) -> Result<
    ValidatedConventionalOperation<
        'a,
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
    crate::domain_computation::authorization::admission::admit_request(
        request,
        operation.operation(),
    )?;
    if operation.contracts().authorization().requires_capability() {
        return Err(WorthQueryOperationAuthorizationDenial::new(
            crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenialKind::CapabilityRequired,
            operation.operation(),
        ));
    }
    crate::domain_computation::authorization::admission::validate_static_authority(
        runtime, principal, scope, operation,
    )?;
    Ok(ValidatedConventionalOperation {
        runtime,
        principal,
        scope,
        operation,
        request,
    })
}

impl<'a, Schema, Principal, PrincipalIdentity, Operation, Input, Scope>
    ValidatedConventionalOperation<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >
{
    pub(super) const fn context(
        &self,
    ) -> (
        &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        &WorthQueryApplicationEntityIdentity<Schema, Scope>,
        &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    ) {
        (self.runtime, self.scope, self.operation)
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        &'a WorthQueryApplicationEntityIdentity<Schema, Scope>,
        &'a WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        &'a WorthQueryRequestScope,
    ) {
        (
            self.runtime,
            self.principal,
            self.scope,
            self.operation,
            self.request,
        )
    }
}

pub(super) struct ValidatedCapabilityOperation<'a, Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &'a WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
}

pub(super) fn validate_capability_operation<'a, Schema, Capability, Operation, Input>(
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &'a WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    progression: WorthQueryCapabilityOperationProgression,
) -> Result<
    ValidatedCapabilityOperation<'a, Schema, Capability, Operation, Input>,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    access.validate_operation_authority(runtime, operation, progression)?;
    Ok(ValidatedCapabilityOperation {
        runtime,
        access,
        operation,
    })
}

impl<'a, Schema, Capability, Operation, Input>
    ValidatedCapabilityOperation<'a, Schema, Capability, Operation, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    pub(super) const fn context(
        &self,
    ) -> (
        &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    ) {
        (self.runtime, &self.access, self.operation)
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        &'a WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
    ) {
        (self.runtime, self.access, self.operation)
    }
}
