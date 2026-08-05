//! Current capability request admission.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryCanonicalWorkEvidence,
    WorthQueryInstalledApplicationCapability,
};

use super::admission::admit_request;
use super::admitted_operation::WorthQueryOperationAdmissionIdentity;
use super::capability_elevation_projection::validate_elevation_projection;
use super::capability_request_resolution::resolve_capability_request;
use super::delegation_admission::observe_capability;
use super::graph_work_session::start_capability_graph_work;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryPrincipalCurrentnessDependency,
    WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::{
    validate_freshness_at_snapshot, WorthQueryApprovedElevation, WorthQueryAuthenticatedPrincipal,
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
        validate_capability_static_authority(self, principal, capability)?;
        let installed = self
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
        if !self.authorization.bridge().matches_installed_policy(
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
        let governed_input_identity = input.governed_input_identity();
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
        let sample = self
            .authorization_clock
            .sample(installed.request.timeline)
            .map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                    capability.contract().name(),
                )
            })?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                capability.contract().operation(),
            )
        })?;
        let operation = self
            .installed_schema
            .installed_operation_for_capability(capability)
            .map_err(|_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                    capability.contract().operation(),
                )
            })?;
        let operation_admission_identity = WorthQueryOperationAdmissionIdentity::mint()
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::AdmissionIdentityExhausted,
                    capability.contract().operation(),
                )
            })?;
        let resource_binding_identity = operation_admission_identity.resource_binding_identity();
        let mut graph_work = start_capability_graph_work(
            self,
            &operation,
            &resource_binding_identity,
            principal.principal_entity_id(),
            WorthQueryGraphWorkAccessContextAffinity::installed_capability(
                *capability.identity().bytes(),
            ),
        )?;
        if approved.is_some_and(|approved| {
            !approved.belongs_to_lifecycle(
                self.runtime.authority_identity(),
                graph_work.branch().relational(),
                *capability.identity().bytes(),
                &installed.capability_authority_identity,
            )
        }) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
                capability.contract().name(),
            ));
        }
        let session_identity = graph_work.identity();
        let principal_layout = graph
            .layout()
            .principal_binding(principal.binding())
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    principal.binding(),
                )
            })?;
        let expected_external_identity = principal
            .external_identity()
            .clone()
            .into_foundational_value();
        let principal_currentness = WorthQueryPrincipalCurrentnessDependency::capture(
            session_identity,
            principal,
            &principal_layout,
        );
        let snapshot = graph_work
            .mutation_snapshot()
            .expect("a capability session owns its admitted snapshot")
            .clone();
        let handle = graph_work
            .mutation_handle()
            .expect("a capability session owns its graph handle")
            .clone();
        let (resolved, revalidation, authorization) = handle.with_runtime_mut(|runtime| {
            (|| {
                validate_freshness_at_snapshot(
                    runtime,
                    &snapshot,
                    principal,
                    &principal_layout,
                    &expected_external_identity,
                )
                .map_err(|_| {
                    denial(
                        WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                        principal.binding(),
                    )
                })?;
                let resolved = resolve_capability_request(
                    runtime,
                    &snapshot,
                    graph.layout(),
                    &self.installed_schema,
                    &projection,
                    self.runtime.authority_identity(),
                )?;
                let revalidation = WorthQueryRetainedCapabilityRequest::capture(
                    *capability.identity().bytes(),
                    principal.principal_entity_id(),
                    &projection,
                    &resolved,
                );
                if approved.is_some_and(|approved| {
                    !approved.support_remains_current_in(
                        runtime,
                        &snapshot,
                        self.authorization.bridge(),
                    )
                }) {
                    return Err(denial(
                        WorthQueryOperationAuthorizationDenialKind::StaleAuthorization,
                        capability.contract().name(),
                    ));
                }
                let authorization = observe_capability(
                    session_identity,
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    &revalidation,
                    &sample,
                    None,
                    approved.map(WorthQueryApprovedElevation::support_decision),
                )?;
                Ok((resolved, revalidation, authorization))
            })()
        })?;
        let (authorization, grant) = authorization.into_parts();
        if let Some(approved) = approved {
            let elevation = resolved.elevation.ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::ElevationProjectionRejected,
                    capability.contract().name(),
                )
            })?;
            if !approved.admits_active_use(
                self.runtime.authority_identity(),
                graph_work.branch().relational(),
                *capability.identity().bytes(),
                &installed.capability_authority_identity,
                &revalidation,
                elevation,
                grant,
            ) {
                return Err(denial(
                    WorthQueryOperationAuthorizationDenialKind::ElevationApprovalRejected,
                    capability.contract().name(),
                ));
            }
        }
        admit_request(request, capability.contract().operation())?;
        if principal.is_expired() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        graph_work.record_decision_facts(2);
        Ok(WorthQueryAdmittedApplicationCapabilityAccess::mint(
            self.runtime.authority_identity(),
            capability.binding_identity().clone(),
            capability.contract().name(),
            std::any::type_name::<Capability>(),
            capability.contract().operation(),
            principal.principal_entity_id(),
            input,
            governed_input_identity.map(|binding| binding.identity()),
            projection,
            resolved,
            principal.valid_until(),
            request.clone(),
            capability.lookup_evidence().canonical_work().combine(
                governed_input_identity
                    .and_then(|binding| binding.canonical_work())
                    .map(WorthQueryCanonicalWorkEvidence::one_digest)
                    .unwrap_or_else(WorthQueryCanonicalWorkEvidence::zero),
            ),
            WorthQueryRetainedCapabilityAuthorization::new(
                principal_currentness,
                authorization,
                installed.capability_authority_identity.clone(),
                grant,
                revalidation,
                sample,
            ),
            operation_admission_identity,
            graph_work,
        ))
    }
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
