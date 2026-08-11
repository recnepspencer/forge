use std::time::Instant;

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
    WorthQueryCompiledApplicationOperationContracts,
};

use crate::domain_computation::authorization::{
    WorthQueryOperationScopeBinding, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::WorthQueryBoundMutationPreconditions;
use crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession;

use super::precondition_binding::{
    PreconditionBoundCapabilityOperation, PreconditionBoundConventionalOperation,
};

mod admitted_operation;

pub use admitted_operation::WorthQueryAdmittedApplicationOperation;
pub(in crate::domain_computation) use admitted_operation::WorthQueryOperationAdmissionIdentity;
pub(in crate::domain_computation::authorization) use admitted_operation::WorthQueryOperationAuthorizationBasis;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(in crate::domain_computation::authorization) enum WorthQueryCapabilityOperationProgression {
    Ordinary,
    DelegationActivation,
    CapabilityRevocation,
    ElevationLifecycle,
}

pub(in crate::domain_computation::authorization) struct WorthQueryCapabilityTransitionPermit {
    _private: (),
}

impl WorthQueryCapabilityTransitionPermit {
    const fn mint() -> Self {
        Self { _private: () }
    }

    pub(in crate::domain_computation::authorization) fn bind_installation(
        &self,
        runtime_authority: crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
        binding_identity: ApplicationSchemaBindingIdentity,
        operation: String,
        operation_authority_identity: String,
        operation_authority_identity_bytes: [u8; 32],
        contracts: WorthQueryCompiledApplicationOperationContracts,
    ) -> WorthQueryAuthorizedOperationInstallation {
        WorthQueryAuthorizedOperationInstallation {
            runtime_authority,
            binding_identity,
            operation,
            operation_authority_identity,
            operation_authority_identity_bytes,
            contracts,
        }
    }

    pub(in crate::domain_computation::authorization) fn bind_subject(
        &self,
        operation_scope_binding: WorthQueryOperationScopeBinding,
        scope_entity_id: worth_relational::facade::identity::EntityId,
        scope_entity_kind: worth_relational::facade::identity::KindId,
        scope_entity_name: String,
        authentication_valid_until: Instant,
        request_scope: WorthQueryRequestScope,
    ) -> WorthQueryAuthorizedOperationSubject {
        WorthQueryAuthorizedOperationSubject {
            operation_scope_binding,
            scope_entity_id,
            scope_entity_kind,
            scope_entity_name,
            authentication_valid_until,
            request_scope,
        }
    }

    pub(in crate::domain_computation::authorization) fn bind_evidence<Input>(
        &self,
        admission_identity: WorthQueryOperationAdmissionIdentity,
        mutation_preconditions: WorthQueryBoundMutationPreconditions,
        authorization_admission_work: WorthQueryCanonicalWorkEvidence,
        authorization: WorthQueryRetainedAuthorizationDecisionFacts,
        governed_input_identity: Option<[u8; 32]>,
        authorization_basis: WorthQueryOperationAuthorizationBasis<Input>,
        graph_work: WorthQueryManagedGraphWorkSession,
    ) -> WorthQueryAuthorizedOperationEvidence<Input> {
        WorthQueryAuthorizedOperationEvidence {
            admission_identity,
            mutation_preconditions,
            authorization_admission_work,
            authorization,
            governed_input_identity,
            authorization_basis,
            graph_work,
        }
    }

    pub(in crate::domain_computation::authorization) fn authorize<Input>(
        self,
        installation: WorthQueryAuthorizedOperationInstallation,
        subject: WorthQueryAuthorizedOperationSubject,
        evidence: WorthQueryAuthorizedOperationEvidence<Input>,
    ) -> WorthQueryAuthorizedCapabilityOperation<Input> {
        WorthQueryAuthorizedCapabilityOperation {
            installation,
            subject,
            evidence,
        }
    }
}

pub(in crate::domain_computation::authorization) struct WorthQueryAuthorizedCapabilityOperation<
    Input,
> {
    installation: WorthQueryAuthorizedOperationInstallation,
    subject: WorthQueryAuthorizedOperationSubject,
    evidence: WorthQueryAuthorizedOperationEvidence<Input>,
}

pub(in crate::domain_computation::authorization) struct WorthQueryAuthorizedConventionalOperation<
    Input,
> {
    installation: WorthQueryAuthorizedOperationInstallation,
    subject: WorthQueryAuthorizedOperationSubject,
    evidence: WorthQueryAuthorizedOperationEvidence<Input>,
}

pub(in crate::domain_computation::authorization) enum WorthQueryAuthorizedOperationTransition<Input>
{
    Conventional(WorthQueryAuthorizedConventionalOperation<Input>),
    Capability(WorthQueryAuthorizedCapabilityOperation<Input>),
}

impl<Input> From<WorthQueryAuthorizedCapabilityOperation<Input>>
    for WorthQueryAuthorizedOperationTransition<Input>
{
    fn from(value: WorthQueryAuthorizedCapabilityOperation<Input>) -> Self {
        Self::Capability(value)
    }
}

impl<Input> From<WorthQueryAuthorizedConventionalOperation<Input>>
    for WorthQueryAuthorizedOperationTransition<Input>
{
    fn from(value: WorthQueryAuthorizedConventionalOperation<Input>) -> Self {
        Self::Conventional(value)
    }
}

pub(in crate::domain_computation::authorization) struct WorthQueryAuthorizedOperationInstallation {
    runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    binding_identity: ApplicationSchemaBindingIdentity,
    operation: String,
    operation_authority_identity: String,
    operation_authority_identity_bytes: [u8; 32],
    contracts: WorthQueryCompiledApplicationOperationContracts,
}

pub(in crate::domain_computation::authorization) struct WorthQueryAuthorizedOperationSubject {
    operation_scope_binding: WorthQueryOperationScopeBinding,
    scope_entity_id: worth_relational::facade::identity::EntityId,
    scope_entity_kind: worth_relational::facade::identity::KindId,
    scope_entity_name: String,
    authentication_valid_until: Instant,
    request_scope: WorthQueryRequestScope,
}

pub(in crate::domain_computation::authorization) struct WorthQueryAuthorizedOperationEvidence<Input>
{
    admission_identity: WorthQueryOperationAdmissionIdentity,
    mutation_preconditions: WorthQueryBoundMutationPreconditions,
    authorization_admission_work: WorthQueryCanonicalWorkEvidence,
    authorization: WorthQueryRetainedAuthorizationDecisionFacts,
    governed_input_identity: Option<[u8; 32]>,
    authorization_basis: WorthQueryOperationAuthorizationBasis<Input>,
    graph_work: WorthQueryManagedGraphWorkSession,
}

pub(super) fn transition_capability_operation<Schema, Capability, Operation, Input>(
    bound: PreconditionBoundCapabilityOperation<'_, Schema, Capability, Operation, Input>,
) -> Result<
    WorthQueryAuthorizedCapabilityOperation<Input>,
    crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial,
>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
    Input: worth_query_declaration::facade::application_capability::ApplicationCapabilityRequest<
        Schema,
        Capability,
    >,
{
    let (validated, preconditions) = bound.into_parts();
    let (runtime, access, operation) = validated.into_parts();
    access.transition_operation(
        runtime,
        operation,
        preconditions,
        WorthQueryCapabilityTransitionPermit::mint(),
    )
}

pub(super) fn transition_conventional_operation<
    Schema,
    Principal,
    PrincipalIdentity,
    Operation,
    Input,
    Scope,
>(
    bound: PreconditionBoundConventionalOperation<
        '_,
        Schema,
        Principal,
        PrincipalIdentity,
        Operation,
        Input,
        Scope,
    >,
) -> Result<
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>,
    crate::domain_computation::authorization::WorthQueryOperationAuthorizationDenial,
>
where
    Schema: worth_query_installation::facade::ApplicationSchema,
{
    use crate::domain_computation::authorization::admission::{
        admit_request, operation_scope_binding,
    };
    use crate::domain_computation::authorization::graph_work_session::start_operation_graph_work;
    use crate::domain_computation::authorization::{
        WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
    };
    use crate::domain_computation::provider_session::WorthQueryGraphWorkAccessContextAffinity;

    let (validated, preconditions) = bound.into_parts();
    let (runtime, principal, scope, operation, request) = validated.into_parts();
    let admission_identity = WorthQueryOperationAdmissionIdentity::mint().ok_or_else(|| {
        WorthQueryOperationAuthorizationDenial::new(
            WorthQueryOperationAuthorizationDenialKind::AdmissionIdentityExhausted,
            operation.operation(),
        )
    })?;
    let resource_binding_identity = admission_identity.resource_binding_identity();
    let mut graph_work = start_operation_graph_work(
        runtime,
        operation,
        &resource_binding_identity,
        principal.principal_entity_id(),
        WorthQueryGraphWorkAccessContextAffinity::entity(scope.entity_id()),
    )?;
    let authorization =
        runtime.observe_operation_authorization(principal, scope, operation, &graph_work)?;
    admit_request(request, operation.operation())?;
    if principal.is_expired() {
        return Err(WorthQueryOperationAuthorizationDenial::new(
            WorthQueryOperationAuthorizationDenialKind::ExpiredAuthentication,
            principal.binding(),
        ));
    }
    graph_work.record_decision_facts(authorization.exact_fact_count());
    let transition = WorthQueryAuthorizedConventionalOperation {
        installation: WorthQueryAuthorizedOperationInstallation {
            runtime_authority: runtime.runtime.authority_identity(),
            binding_identity: operation.binding_identity().clone(),
            operation: operation.operation().to_string(),
            operation_authority_identity: operation.authority_identity().to_string(),
            operation_authority_identity_bytes: operation.authority_identity_bytes(),
            contracts: operation.contracts().clone(),
        },
        subject: WorthQueryAuthorizedOperationSubject {
            operation_scope_binding: operation_scope_binding(runtime, principal, scope, operation),
            scope_entity_id: scope.entity_id(),
            scope_entity_kind: scope.entity_kind(),
            scope_entity_name: scope.entity_name().to_string(),
            authentication_valid_until: principal.valid_until(),
            request_scope: request.clone(),
        },
        evidence: WorthQueryAuthorizedOperationEvidence {
            admission_identity,
            mutation_preconditions: preconditions,
            authorization_admission_work: WorthQueryCanonicalWorkEvidence::zero(),
            authorization,
            governed_input_identity: None,
            authorization_basis: WorthQueryOperationAuthorizationBasis::Conventional,
            graph_work,
        },
    };
    Ok(WorthQueryAdmittedApplicationOperation::from_authorized_transition(transition.into()))
}
