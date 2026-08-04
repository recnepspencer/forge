//! Current capability request admission.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryInstalledApplicationCapability,
};

use super::admission::admit_request;
use super::admitted_operation::WorthQueryOperationAdmissionIdentity;
use super::capability_observation::observe_capability;
use super::capability_request_resolution::resolve_capability_request;
use super::graph_work_session::start_capability_graph_work;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryPrincipalCurrentnessDependency,
    WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::{
    validate_freshness_at_snapshot, WorthQueryAuthenticatedPrincipal,
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
        let projection = input.capability_request().map_err(|projection_denial| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
                projection_denial.subject(),
            )
        })?;
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
                let authorization = observe_capability(
                    session_identity,
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    &revalidation,
                    &sample,
                    None,
                )?;
                Ok((resolved, revalidation, authorization))
            })()
        })?;
        let (authorization, grant) = authorization.into_parts();
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
            projection,
            resolved,
            principal.valid_until(),
            request.clone(),
            capability.lookup_evidence().canonical_work(),
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
