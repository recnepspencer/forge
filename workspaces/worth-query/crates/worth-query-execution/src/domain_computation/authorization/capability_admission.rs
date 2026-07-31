//! Current capability request admission.

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest;
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryInstalledApplicationCapability,
};

use super::admission::admit_request;
use super::capability_observation::observe_capability;
use super::capability_request_resolution::resolve_capability_request;
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
            capability.binding_identity(),
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
        let principal_currentness =
            WorthQueryPrincipalCurrentnessDependency::capture(principal, &principal_layout);
        let (resolved, revalidation, authorization) =
            graph.integration_handle().with_runtime_mut(|runtime| {
                let snapshot = runtime.snapshots().snapshot();
                let result = (|| {
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
                        runtime,
                        snapshot.clone(),
                        self.authorization.bridge(),
                        installed,
                        &revalidation,
                        &sample,
                    )?;
                    Ok((resolved, revalidation, authorization))
                })();
                runtime.snapshots().release_snapshot(&snapshot);
                result
            })?;
        admit_request(request, capability.contract().operation())?;
        if principal.is_expired() {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        Ok(WorthQueryAdmittedApplicationCapabilityAccess::mint(
            self.runtime.authority_identity(),
            capability.binding_identity().clone(),
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
                revalidation,
                sample,
            ),
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
