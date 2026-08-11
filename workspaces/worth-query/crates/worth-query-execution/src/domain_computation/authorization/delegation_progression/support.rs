//! Delegation progression consumes one access-bound exact support observation.

use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityDelegationRequest, ApplicationCapabilityRequest,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
};

pub(in crate::domain_computation::authorization) use super::super::capability_admission::WorthQueryDelegationResolvedRequest;
use super::super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryRetainedCapabilitySupport,
};
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphApplicationRuntime;

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
    access
        .with_exact_observation(runtime, |observation| {
            observation.authorize_delegation_support(target_capability, proposed)
        })
        .ok_or_else(|| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                access.operation(),
            )
        })?
}
