//! Static capability admission and graph-work preparation.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityRequest, ApplicationCapabilityRequestProjection,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationCapability,
};

use super::super::capability_elevation_projection::validate_elevation_projection;
use super::super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::super::graph_work_session::start_capability_graph_work;
use super::super::{
    bridge_authorization_binding_identity, WorthQueryOperationAdmissionIdentity,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryRuntimeTimeSample,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApprovedElevation, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::{
    WorthQueryGraphWorkAccessContextAffinity, WorthQueryManagedGraphWorkSession,
};

mod observation;

pub use observation::WorthQueryAdmittedApplicationCapabilityAccess;
pub(in crate::domain_computation::authorization) use observation::{
    progress_capability_operation, WorthQueryCapabilityOperationProgression,
    WorthQueryDelegationResolvedRequest,
};
pub(in crate::domain_computation::authorization) use observation::{
    WorthQueryCapabilityContextKey, WorthQueryResolvedCapabilityRequest,
};

pub(super) fn complete_capability_admission<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
    prepared: PreparedCapabilityAdmission<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >,
) -> Result<
    WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    let observed = observation::observe_current_capability(prepared)?;
    Ok(observation::admit_observed_capability(observed))
}

pub(super) struct PreparedCapabilityAdmission<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
> where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    capability: &'a WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    approved: Option<&'a WorthQueryApprovedElevation>,
    input: Input,
    request_scope: WorthQueryRequestScope,
    installed: &'a WorthQueryInstalledCapabilityPlan,
    projection: ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
    sample: WorthQueryRuntimeTimeSample,
    operation_admission_identity: WorthQueryOperationAdmissionIdentity,
    graph_work: WorthQueryManagedGraphWorkSession,
    _seal: PreparedSeal,
}

struct PreparedSeal;

impl<'a, Schema, Principal, PrincipalIdentity, Capability, Operation, Input>
    PreparedCapabilityAdmission<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    const fn runtime(&self) -> &'a WorthQueryPrimaryGraphApplicationRuntime<Schema> {
        self.runtime
    }

    const fn principal(
        &self,
    ) -> &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity> {
        self.principal
    }

    const fn capability(
        &self,
    ) -> &'a WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input> {
        self.capability
    }

    const fn approved(&self) -> Option<&'a WorthQueryApprovedElevation> {
        self.approved
    }

    const fn installed(&self) -> &'a WorthQueryInstalledCapabilityPlan {
        self.installed
    }

    const fn projection(
        &self,
    ) -> &ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    > {
        &self.projection
    }

    const fn sample(&self) -> &WorthQueryRuntimeTimeSample {
        &self.sample
    }

    const fn graph_work(&self) -> &WorthQueryManagedGraphWorkSession {
        &self.graph_work
    }

    fn record_admission_decisions(&mut self) {
        self.graph_work.record_decision_facts(2);
    }
}

pub(super) fn prepare_capability_admission<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &'a WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    capability: &'a WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    input: Input,
    request: &WorthQueryRequestScope,
    approved: Option<&'a WorthQueryApprovedElevation>,
) -> Result<
    PreparedCapabilityAdmission<
        'a,
        Schema,
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validate_static_authority(runtime, principal, capability)?;
    let installed = runtime
        .authorization
        .capability_plan(capability)
        .ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                capability.contract().name(),
            )
        })?;
    validate_approved_applicability(installed, capability, approved)?;
    if !runtime.authorization.bridge().matches_installed_policy(
        installed.correspondence,
        &bridge_authorization_binding_identity(capability.binding_identity()),
        installed.contract.name(),
        &installed.request.resource_entity,
        installed.contract.operation(),
        &installed.bridge_rules,
    ) {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
            capability.contract().name(),
        ));
    }
    let projection = input.capability_request().map_err(|rejection| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
            rejection.subject(),
        )
    })?;
    validate_elevation_projection(&installed.contract, &projection)?;
    require_approved_transition(installed, capability, approved)?;
    let sample = runtime
        .authorization_clock
        .sample(installed.request.timeline)
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                capability.contract().name(),
            )
        })?;
    let operation = runtime
        .installed_schema
        .installed_operation_for_capability(capability)
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                capability.contract().operation(),
            )
        })?;
    let operation_admission_identity =
        WorthQueryOperationAdmissionIdentity::mint().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::AdmissionIdentityExhausted,
                capability.contract().operation(),
            )
        })?;
    let graph_work = start_capability_graph_work(
        runtime,
        &operation,
        principal.principal_entity_id(),
        WorthQueryGraphWorkAccessContextAffinity::installed_capability(
            *capability.identity().bytes(),
        ),
    )?;
    Ok(PreparedCapabilityAdmission {
        runtime,
        principal,
        capability,
        approved,
        input,
        request_scope: request.clone(),
        installed,
        projection,
        sample,
        operation_admission_identity,
        graph_work,
        _seal: PreparedSeal,
    })
}

fn validate_approved_applicability<Schema, Capability, Operation, Input>(
    installed: &WorthQueryInstalledCapabilityPlan,
    capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    approved: Option<&WorthQueryApprovedElevation>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if installed.elevation.is_none() && approved.is_some() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationNotApplicable,
            capability.contract().name(),
        ));
    }
    Ok(())
}

fn require_approved_transition<Schema, Capability, Operation, Input>(
    installed: &WorthQueryInstalledCapabilityPlan,
    capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    approved: Option<&WorthQueryApprovedElevation>,
) -> Result<(), WorthQueryOperationAuthorizationDenial> {
    if installed.elevation.is_some() && approved.is_none() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationTransitionRequired,
            capability.contract().name(),
        ));
    }
    Ok(())
}

fn validate_static_authority<Schema, Principal, PrincipalIdentity, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
{
    if principal.is_expired() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
            principal.binding(),
        ));
    }
    if principal.runtime_authority() != runtime.runtime.authority_identity() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            capability.contract().name(),
        ));
    }
    if principal.binding_identity() != capability.binding_identity()
        || runtime.installed_schema.binding_identity() != *capability.binding_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema,
            capability.contract().name(),
        ));
    }
    runtime
        .installed_schema
        .validate_installed_capability(capability)
        .map_err(|rejection| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                rejection.subject(),
            )
        })
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
