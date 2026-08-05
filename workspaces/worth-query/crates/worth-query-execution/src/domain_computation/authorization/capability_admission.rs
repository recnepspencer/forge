//! Current capability request admission.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityRequest, ApplicationCapabilityRequestProjection,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryCanonicalWorkEvidence, WorthQueryInstalledApplicationCapability,
};

use super::admission::admit_request;
use super::admitted_capability_access::WorthQueryAdmittedApplicationCapabilityAccessInput;
use super::admitted_operation::WorthQueryOperationAdmissionIdentity;
use super::capability_elevation_projection::validate_elevation_projection;
use super::capability_observation_admission::{
    observe_current_capability, WorthQueryCapabilityObservationContext,
};
use super::capability_registry::WorthQueryInstalledCapabilityPlan;
use super::graph_work_session::start_capability_graph_work;
use super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAuthorizationTimeSample,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApprovedElevation, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::WorthQueryGraphWorkAccessContextAffinity;

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn admit_capability_access<Principal, PrincipalIdentity, Capability, Operation, Input>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        self.admit_capability_access_inner(principal, capability, input, request, None)
    }

    pub fn admit_approved_elevation_access<
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >(
        &self,
        approved: &WorthQueryApprovedElevation,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
    ) -> Result<
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        self.admit_capability_access_inner(principal, capability, input, request, Some(approved))
    }

    fn admit_capability_access_inner<Principal, PrincipalIdentity, Capability, Operation, Input>(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
        input: Input,
        request: &WorthQueryRequestScope,
        approved: Option<&WorthQueryApprovedElevation>,
    ) -> Result<
        WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
        WorthQueryOperationAuthorizationDenial,
    >
    where
        Input: ApplicationCapabilityRequest<Schema, Capability>,
    {
        admit_request(request, capability.contract().operation())?;
        let governed_input_identity = input.governed_input_identity();
        let WorthQueryPreparedCapabilityAdmission {
            installed,
            projection,
            sample,
            operation_admission_identity,
            mut graph_work,
        } = prepare_capability_admission(self, principal, capability, &input, approved)?;
        let observed = observe_current_capability(
            self,
            WorthQueryCapabilityObservationContext {
                principal,
                capability,
                installed,
                projection: &projection,
                approved,
                graph_work: &graph_work,
                sample: &sample,
            },
        )?;
        admit_request(request, capability.contract().operation())?;
        if principal.is_expired() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        graph_work.record_decision_facts(2);
        Ok(WorthQueryAdmittedApplicationCapabilityAccess::mint(
            WorthQueryAdmittedApplicationCapabilityAccessInput {
                runtime_authority: self.runtime.authority_identity(),
                binding_identity: capability.binding_identity().clone(),
                capability: capability.contract().name().into(),
                capability_type: std::any::type_name::<Capability>().into(),
                operation: capability.contract().operation().into(),
                principal_entity_id: principal.principal_entity_id(),
                input,
                governed_input_identity: governed_input_identity.map(|binding| binding.identity()),
                projection,
                resolved: observed.resolved,
                authentication_valid_until: principal.valid_until(),
                request_scope: request.clone(),
                canonical_work: capability.lookup_evidence().canonical_work().combine(
                    governed_input_identity
                        .and_then(|binding| binding.canonical_work())
                        .map(WorthQueryCanonicalWorkEvidence::one_digest)
                        .unwrap_or_else(WorthQueryCanonicalWorkEvidence::zero),
                ),
                authorization: observed.authorization,
                operation_admission_identity,
                graph_work,
                _operation: std::marker::PhantomData,
            },
        ))
    }
}

struct WorthQueryPreparedCapabilityAdmission<'a, Schema, Capability, Input>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    installed: &'a WorthQueryInstalledCapabilityPlan,
    projection: ApplicationCapabilityRequestProjection<
        Schema,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        <Input as ApplicationCapabilityRequest<Schema, Capability>>::Context,
    >,
    sample: WorthQueryAuthorizationTimeSample,
    operation_admission_identity: WorthQueryOperationAdmissionIdentity,
    graph_work: crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
}

fn prepare_capability_admission<
    'a,
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
    runtime: &'a WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
    capability: &WorthQueryInstalledApplicationCapability<Schema, Capability, Operation, Input>,
    input: &Input,
    approved: Option<&WorthQueryApprovedElevation>,
) -> Result<
    WorthQueryPreparedCapabilityAdmission<'a, Schema, Capability, Input>,
    WorthQueryOperationAuthorizationDenial,
>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validate_capability_static_authority(runtime, principal, capability)?;
    let installed = runtime
        .authorization
        .capability_plan(capability)
        .ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                capability.contract().name(),
            )
        })?;
    if installed.elevation.is_none() && approved.is_some() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationNotApplicable,
            capability.contract().name(),
        ));
    }
    if !runtime.authorization.bridge().matches_installed_policy(
        installed.correspondence,
        &super::bridge_authorization_binding_identity(capability.binding_identity()),
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
    let projection = input.capability_request().map_err(|projection_denial| {
        denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
            projection_denial.subject(),
        )
    })?;
    validate_elevation_projection(&installed.contract, &projection)?;
    if installed.elevation.is_some() && approved.is_none() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ElevationTransitionRequired,
            capability.contract().name(),
        ));
    }
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
        &operation_admission_identity.resource_binding_identity(),
        principal.principal_entity_id(),
        WorthQueryGraphWorkAccessContextAffinity::installed_capability(
            *capability.identity().bytes(),
        ),
    )?;
    Ok(WorthQueryPreparedCapabilityAdmission {
        installed,
        projection,
        sample,
        operation_admission_identity,
        graph_work,
    })
}

fn validate_capability_static_authority<
    Schema,
    Principal,
    PrincipalIdentity,
    Capability,
    Operation,
    Input,
>(
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
        .map_err(|installation_denial| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                installation_denial.subject(),
            )
        })
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
