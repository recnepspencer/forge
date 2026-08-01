//! Capability-access progression into installed operation authority.

use std::time::Instant;

use worth_query_declaration::facade::{
    application_capability::{
        ApplicationCapabilityRequest, ApplicationCapabilityRequestProjection,
    },
    application_schema::TypedMutationPreconditions,
};
use worth_query_installation::facade::{
    ApplicationSchema, WorthQueryInstalledApplicationOperation,
};

use super::admission::admit_request;
use super::admitted_operation::{
    WorthQueryOperationAdmissionIdentity, WorthQueryOperationAuthorizationBasis,
};
use super::capability_observation::observe_capability;
use super::capability_request_resolution::resolve_capability_request;
use super::graph_work_session::start_operation_graph_work;
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAdmittedApplicationOperation, WorthQueryOperationAuthorizationDenial,
    WorthQueryOperationAuthorizationDenialKind, WorthQueryOperationScopeBinding,
    WorthQueryPreparedApplicationCapabilityAccess, WorthQueryPrincipalCurrentnessDependency,
    WorthQueryRetainedAuthorizationDecisionFacts, WorthQueryRetainedCapabilityAuthorization,
};
use crate::domain_computation::primary_graph::{
    bind_mutation_preconditions, WorthQueryPrimaryGraphApplicationRuntime,
};
use crate::domain_computation::provider_session::{
    record_capability_authorization_completion, WorthQueryGraphWorkAccessContextAffinity,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_capability_operation<Capability, Operation, Input>(
        &self,
        access: WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
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
        validate_progression_authority(self, &access, operation)?;
        let current_projection = access.input.capability_request().map_err(|rejection| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
                rejection.subject(),
            )
        })?;
        if !same_projection(&access.projection, &current_projection) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::CapabilityProjectionRejected,
                access.operation.as_ref(),
            ));
        }
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                access.operation.as_ref(),
            )
        })?;
        let installed = self
            .authorization
            .capability_plan_by_identity(access.capability_identity.bytes())
            .filter(|installed| {
                installed.capability_authority_identity.as_ref()
                    == access.capability_authority_identity.as_ref()
            })
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::PolicyNotInstalled,
                    access.capability.as_ref(),
                )
            })?;
        let admission_identity = WorthQueryOperationAdmissionIdentity::mint().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::AdmissionIdentityExhausted,
                operation.operation(),
            )
        })?;
        let resource_binding_identity = admission_identity.resource_binding_identity();
        let mut graph_work = start_operation_graph_work(
            self,
            operation,
            &resource_binding_identity,
            access.principal_entity_id,
            WorthQueryGraphWorkAccessContextAffinity::installed_capability(
                access.capability_identity,
            ),
        )?;
        let principal_layout = graph
            .layout()
            .principal_binding(&access.principal_binding)
            .cloned()
            .ok_or_else(|| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                    access.principal_binding.as_ref(),
                )
            })?;
        let sample = self.sample_capability_time(installed)?;
        let (resolved, request, decision, grant) =
            graph.integration_handle().with_runtime_mut(|runtime| {
                let snapshot = graph_work.basis().snapshot().clone();
                if !access.principal_freshness.remains_current_in(
                    runtime,
                    &snapshot,
                    &principal_layout,
                    &access.principal_binding,
                ) {
                    return Err(denial(
                        WorthQueryOperationAuthorizationDenialKind::StalePrincipal,
                        access.principal_binding.as_ref(),
                    ));
                }
                let resolved = resolve_capability_request(
                    runtime,
                    &snapshot,
                    graph.layout(),
                    &self.installed_schema,
                    &access.projection,
                    self.runtime.authority_identity(),
                )?;
                let request = WorthQueryRetainedCapabilityRequest::capture(
                    *access.capability_identity.bytes(),
                    access.principal_entity_id,
                    &access.projection,
                    &resolved,
                );
                let observed = observe_capability(
                    *graph_work.identity(),
                    runtime,
                    snapshot,
                    self.authorization.bridge(),
                    installed,
                    &request,
                    &sample,
                    None,
                )?;
                let (decision, grant) = observed.into_parts();
                Ok((resolved, request, decision, grant))
            })?;
        let principal_currentness = WorthQueryPrincipalCurrentnessDependency::capture_retained(
            *graph_work.identity(),
            access.principal_binding.clone(),
            principal_layout,
            access.principal_freshness.clone(),
            graph_work.branch_affinity().relational_branch().clone(),
        );
        let authorization = WorthQueryRetainedCapabilityAuthorization::new(
            principal_currentness,
            decision,
            access.capability_authority_identity.clone(),
            grant,
            request,
            sample,
        );
        let preconditions = bind_mutation_preconditions(
            preconditions,
            operation.contracts(),
            resolved.resource.entity_name(),
            resolved.resource.entity_id(),
            graph.layout(),
        )
        .map_err(|()| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::MutationPreconditionRejected,
                operation.operation(),
            )
        })?;
        validate_access_lifecycle(&access)?;
        record_capability_authorization_completion(&mut graph_work, &authorization).map_err(
            |_| {
                denial(
                    WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                    operation.operation(),
                )
            },
        )?;
        Ok(WorthQueryAdmittedApplicationOperation::mint(
            admission_identity,
            self.runtime.authority_identity(),
            operation.binding_identity().clone(),
            operation.operation().to_string(),
            operation.authority_identity().to_string(),
            WorthQueryOperationScopeBinding::mint(
                self.runtime.authority_identity(),
                operation.binding_identity(),
                operation.authority_identity(),
                access.principal_entity_id,
                resolved.resource.entity_id(),
            ),
            resolved.resource.entity_id(),
            resolved.resource.entity_kind(),
            resolved.resource.entity_name().to_string(),
            access.authentication_valid_until,
            access.request_scope,
            operation.contracts().clone(),
            preconditions,
            access.canonical_work,
            WorthQueryRetainedAuthorizationDecisionFacts::capability(authorization),
            WorthQueryOperationAuthorizationBasis::Capability {
                input: access.input,
            },
            graph_work,
        ))
    }
}

fn validate_progression_authority<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
    operation: &WorthQueryInstalledApplicationOperation<Schema, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Schema: ApplicationSchema,
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    validate_access_lifecycle(access)?;
    if access.runtime_authority != runtime.runtime.authority_identity() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
            access.operation.as_ref(),
        ));
    }
    if access.binding_identity != *operation.binding_identity()
        || runtime.installed_schema.binding_identity() != *operation.binding_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledSchema,
            access.operation.as_ref(),
        ));
    }
    if access.operation.as_ref() != operation.operation() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
            access.operation.as_ref(),
        ));
    }
    if !operation.contracts().authorization().requires_capability() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityNotRequired,
            operation.operation(),
        ));
    }
    runtime
        .runtime
        .installed_packages()
        .validate_application_operation(operation)
        .map_err(|_| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::StaleInstalledOperation,
                operation.operation(),
            )
        })
}

fn validate_access_lifecycle<Schema, Capability, Operation, Input>(
    access: &WorthQueryPreparedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
) -> Result<(), WorthQueryOperationAuthorizationDenial>
where
    Input: ApplicationCapabilityRequest<Schema, Capability>,
{
    admit_request(&access.request_scope, &access.operation)?;
    if Instant::now() >= access.authentication_valid_until {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
            access.operation.as_ref(),
        ));
    }
    Ok(())
}

fn same_projection<Schema, Scope, Context>(
    expected: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    actual: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
) -> bool {
    same_resource(expected, actual)
        && expected.action() == actual.action()
        && expected.purpose() == actual.purpose()
        && same_related(expected, actual)
        && expected.field_value() == actual.field_value()
        && expected.amount_value() == actual.amount_value()
        && expected.cardinality_value() == actual.cardinality_value()
        && expected.context_value().context() == actual.context_value().context()
        && expected.context_value().context_type() == actual.context_value().context_type()
        && expected.context_value().entities() == actual.context_value().entities()
}

fn same_resource<Schema, Scope, Context>(
    expected: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    actual: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
) -> bool {
    expected.resource().entity() == actual.resource().entity()
        && expected.resource().aspect() == actual.resource().aspect()
        && expected.resource().field() == actual.resource().field()
        && expected.resource().scalar_family() == actual.resource().scalar_family()
        && expected.resource().value_type() == actual.resource().value_type()
        && expected.resource().value() == actual.resource().value()
}

fn same_related<Schema, Scope, Context>(
    expected: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
    actual: &ApplicationCapabilityRequestProjection<Schema, Scope, Context>,
) -> bool {
    match (expected.related(), actual.related()) {
        (None, None) => true,
        (Some(expected), Some(actual)) => {
            expected.relation() == actual.relation() && expected.selector() == actual.selector()
        }
        _ => false,
    }
}

fn denial(
    kind: WorthQueryOperationAuthorizationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryOperationAuthorizationDenial {
    WorthQueryOperationAuthorizationDenial::new(kind, subject)
}
