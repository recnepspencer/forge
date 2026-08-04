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
use super::admitted_operation::WorthQueryOperationAuthorizationBasis;
use super::{
    WorthQueryAdmittedApplicationCapabilityAccess, WorthQueryAdmittedApplicationOperation,
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    WorthQueryOperationScopeBinding, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::{
    bind_mutation_preconditions, WorthQueryPrimaryGraphApplicationRuntime,
};

impl<Schema> WorthQueryPrimaryGraphApplicationRuntime<Schema>
where
    Schema: ApplicationSchema,
{
    pub fn authorize_capability_operation<Capability, Operation, Input>(
        &self,
        mut access: WorthQueryAdmittedApplicationCapabilityAccess<
            Schema,
            Capability,
            Operation,
            Input,
        >,
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
        self.refresh_capability_authorization_for_graph_work(
            &mut access.authorization,
            &access.graph_work,
        )?;
        let graph = self.runtime.primary_graph().ok_or_else(|| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::ForeignRuntime,
                access.operation.as_ref(),
            )
        })?;
        let preconditions = bind_mutation_preconditions(
            preconditions,
            operation.contracts(),
            access.resolved.resource.entity_name(),
            access.resolved.resource.entity_id(),
            graph.layout(),
        )
        .map_err(|()| {
            denial(
                WorthQueryOperationAuthorizationDenialKind::MutationPreconditionRejected,
                operation.operation(),
            )
        })?;
        validate_access_lifecycle(&access)?;
        let session_identity = access.graph_work.identity();
        let authorization =
            WorthQueryRetainedAuthorizationDecisionFacts::capability(access.authorization);
        if !authorization.belongs_to_session(session_identity) {
            return Err(denial(
                WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
                operation.operation(),
            ));
        }
        Ok(WorthQueryAdmittedApplicationOperation::mint(
            access.operation_admission_identity,
            self.runtime.authority_identity(),
            operation.binding_identity().clone(),
            operation.operation().to_string(),
            operation.authority_identity().to_string(),
            WorthQueryOperationScopeBinding::mint(
                self.runtime.authority_identity(),
                operation.binding_identity(),
                operation.authority_identity(),
                access.principal_entity_id,
                access.resolved.resource.entity_id(),
            ),
            access.resolved.resource.entity_id(),
            access.resolved.resource.entity_kind(),
            access.resolved.resource.entity_name().to_string(),
            access.authentication_valid_until,
            access.request_scope,
            operation.contracts().clone(),
            preconditions,
            access.canonical_work,
            authorization,
            WorthQueryOperationAuthorizationBasis::Capability {
                input: access.input,
            },
            access.graph_work,
        ))
    }
}

fn validate_progression_authority<Schema, Capability, Operation, Input>(
    runtime: &WorthQueryPrimaryGraphApplicationRuntime<Schema>,
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
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
    if access.graph_work.binding() != operation.binding_identity()
        || access.graph_work.obligation() != operation.graph_obligations().identity()
        || access.graph_work.subject_authority() != operation.authority_identity()
    {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
            operation.operation(),
        ));
    }
    if !operation.contracts().authorization().requires_capability() {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::CapabilityNotRequired,
            operation.operation(),
        ));
    }
    if access.authorization.exact_fact_count() != 2 {
        return Err(denial(
            WorthQueryOperationAuthorizationDenialKind::InconsistentDecision,
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
    access: &WorthQueryAdmittedApplicationCapabilityAccess<Schema, Capability, Operation, Input>,
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
        && expected.elevation_selector() == actual.elevation_selector()
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
