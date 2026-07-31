use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_declaration::facade::{
    application_capability::ApplicationCapabilityRequest,
    application_schema::TypedMutationPreconditions,
};
use worth_query_installation::facade::{
    ApplicationSchema, TypedApplicationValue, WorthQueryInstalledApplicationCapability,
    WorthQueryInstalledApplicationOperation,
};

use super::admission::{admit_request, operation_scope_binding, validate_static_authority};
use super::capability_currentness::WorthQueryCapabilityCurrentnessAuthority;
use super::capability_observation::observe_capability;
use super::capability_request_resolution::resolve_capability_request;
use super::{
    WorthQueryAdmittedApplicationOperation, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind,
};
use crate::domain_computation::primary_graph::{
    resolution::validate_freshness_at_snapshot, WorthQueryAuthenticatedPrincipal,
    WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_capability_operation<
        Principal,
        PrincipalIdentity,
        Capability,
        Operation,
        Input,
    >(
        &self,
        principal: &WorthQueryAuthenticatedPrincipal<Schema, Principal, PrincipalIdentity>,
        capability: &WorthQueryInstalledApplicationCapability<
            Schema,
            Capability,
            Operation,
            Input,
        >,
        operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
        input: Input,
        preconditions: TypedMutationPreconditions<
            Schema,
            Operation,
            <Input as ApplicationCapabilityRequest<Schema, Capability>>::Scope,
        >,
        request: &WorthQueryRequestScope,
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
        admit_request(request, operation.operation())?;
        self.installed_schema
            .validate_installed_capability(capability)
            .map_err(|denial| {
                WorthQueryOperationAuthorizationDenial::new(
                    WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                    denial.subject(),
                )
            })?;
        self.runtime
            .installed_packages()
            .validate_application_operation(operation)
            .map_err(|_| {
                WorthQueryOperationAuthorizationDenial::new(
                    WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                    operation.operation(),
                )
            })?;
        let installed = self
            .authorization
            .capability_plan(capability)
            .ok_or_else(|| {
                WorthQueryOperationAuthorizationDenial::new(
                    WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                    capability.contract().name(),
                )
            })?;
        if operation.operation() != installed.contract.operation()
            || !self.authorization.bridge().matches_installed_policy(
                installed.correspondence,
                operation.binding_identity(),
                installed.contract.name(),
                &installed.request.resource_entity,
                installed.contract.operation(),
                &installed.bridge_rules,
            )
        {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                capability.contract().name(),
            ));
        }
        let projection = input.capability_request().map_err(|denial| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
                denial.subject(),
            )
        })?;
        let sample = self
            .authorization_clock
            .sample(installed.request.timeline)
            .map_err(|_| {
                WorthQueryOperationAuthorizationDenial::new(
                    WorthQueryOperationAuthorizationDenialKind::TrustedTimeUnavailable,
                    capability.contract().name(),
                )
            })?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                operation.operation(),
            )
        })?;
        let principal_layout = graph
            .layout
            .principal_binding(principal.binding())
            .cloned()
            .ok_or_else(|| {
                WorthQueryOperationAuthorizationDenial::new(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    principal.binding(),
                )
            })?;
        let expected_external_identity = principal
            .external_identity()
            .clone()
            .into_foundational_value();
        let (resolved, authorization) = graph.integration_handle().with_runtime_mut(|runtime| {
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
                    WorthQueryOperationAuthorizationDenial::new(
                        WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                        principal.binding(),
                    )
                })?;
                let resolved = resolve_capability_request(
                    runtime,
                    &snapshot,
                    &graph.layout,
                    &self.installed_schema,
                    &projection,
                    self.runtime.authority_identity(),
                )?;
                let authorization = observe_capability(
                    runtime,
                    snapshot.clone(),
                    self.authorization.bridge(),
                    installed,
                    principal.principal_entity_id(),
                    &projection,
                    &resolved,
                    &sample,
                )?;
                Ok((resolved, authorization))
            })();
            runtime.snapshots().release_snapshot(&snapshot);
            result
        })?;
        validate_static_authority(self, principal, &resolved.resource, operation)?;
        let preconditions =
            super::super::application_attempt::precondition_binding::bind_mutation_preconditions(
                preconditions,
                operation.contracts(),
                resolved.resource.entity_name(),
                resolved.resource.entity_id(),
                &graph.layout,
            )
            .map_err(|()| {
                WorthQueryOperationAuthorizationDenial::new(
                    WorthQueryOperationAuthorizationDenialKind::MutationPreconditionRejected,
                    operation.operation(),
                )
            })?;
        admit_request(request, operation.operation())?;
        if principal.is_expired() {
            return Err(WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
                principal.binding(),
            ));
        }
        let admission = WorthQueryAdmittedApplicationOperation::mint(
            self.runtime.authority_identity(),
            operation.binding_identity().clone(),
            operation.operation().to_string(),
            operation.authority_identity().to_string(),
            operation_scope_binding(self, principal, &resolved.resource, operation),
            resolved.resource.entity_id(),
            resolved.resource.entity_kind(),
            resolved.resource.entity_name().to_string(),
            principal.valid_until(),
            request.clone(),
            operation.contracts().clone(),
            preconditions,
            vec![authorization],
        )
        .map_err(|_| {
            WorthQueryOperationAuthorizationDenial::new(
                WorthQueryOperationAuthorizationDenialKind::AdmissionIdentityExhausted,
                operation.operation(),
            )
        })?;
        Ok(admission.bind_capability_authority(
            input,
            WorthQueryCapabilityCurrentnessAuthority::new(
                installed.capability_authority_identity.clone(),
                sample.timeline(),
                sample.value().clone(),
            ),
        ))
    }
}
