//! Move-only installed operation admission authority.

use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
    WorthQueryCanonicalWorkPhases, WorthQueryCompiledApplicationOperationContracts,
};

use super::WorthQueryRetainedAuthorizationDecisionFacts;
use crate::domain_computation::primary_graph::WorthQueryBoundMutationPreconditions;

mod access;
mod authorization_basis;
mod capability_revocation;
mod delegation_activation;
mod elevation_approval;
mod elevation_close;
mod elevation_request;
mod graph_work_inspection;
mod idempotency_binding;
mod input;
mod mandatory_review;

pub(super) use authorization_basis::WorthQueryOperationAuthorizationBasis;
pub(super) use input::WorthQueryAdmittedApplicationOperationInput;

static NEXT_OPERATION_ADMISSION_IDENTITY: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Eq, PartialEq)]
pub(in crate::domain_computation) struct WorthQueryOperationAdmissionIdentity(u64);

impl WorthQueryOperationAdmissionIdentity {
    pub(super) fn mint() -> Option<Self> {
        Self::mint_from(&NEXT_OPERATION_ADMISSION_IDENTITY)
    }

    pub(super) fn resource_binding_identity(self) -> Arc<str> {
        Arc::from(format!("worth-query-application-admission:{}", self.0))
    }

    fn mint_from(counter: &AtomicU64) -> Option<Self> {
        counter
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .ok()
            .map(Self)
    }
}

use super::WorthQueryOperationScopeBinding;

/// Query-owned proof that one installed operation was authorized for one exact
/// current principal and typed scope.
///
/// The proof is move-only, has private fields, and is not deserializable.
/// Descriptive identities, token claims, or decision enums cannot mint it.
///
/// ```compile_fail
/// use worth_query_execution::facade::primary_graph::WorthQueryAdmittedApplicationOperation;
///
/// let _: WorthQueryAdmittedApplicationOperation<(), (), (), ()> =
///     serde_json::from_str("{}").unwrap();
/// ```
pub struct WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope> {
    runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    binding_identity: ApplicationSchemaBindingIdentity,
    operation: String,
    operation_authority_identity: Arc<str>,
    operation_authority_identity_bytes: [u8; 32],
    admission_identity: WorthQueryOperationAdmissionIdentity,
    resource_binding_identity: Arc<str>,
    operation_scope_binding: WorthQueryOperationScopeBinding,
    canonical_work: WorthQueryCanonicalWorkPhases,
    scope_entity_id: worth_relational::facade::identity::EntityId,
    scope_entity_kind: worth_relational::facade::identity::KindId,
    scope_entity_name: String,
    authentication_valid_until: Instant,
    request_scope: worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope,
    contracts: WorthQueryCompiledApplicationOperationContracts,
    mutation_preconditions: WorthQueryBoundMutationPreconditions,
    authorization: Option<WorthQueryRetainedAuthorizationDecisionFacts>,
    governed_input_identity: Option<[u8; 32]>,
    authorization_basis: WorthQueryOperationAuthorizationBasis<Input>,
    graph_work: crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
    _marker: PhantomData<fn(Input) -> (Schema, Operation, Scope)>,
}

impl<Schema, Operation, Input, Scope>
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(super) fn mint(input: WorthQueryAdmittedApplicationOperationInput<Input>) -> Self {
        let WorthQueryAdmittedApplicationOperationInput {
            admission_identity,
            runtime_authority,
            binding_identity,
            operation,
            operation_authority_identity,
            operation_authority_identity_bytes,
            operation_scope_binding,
            scope_entity_id,
            scope_entity_kind,
            scope_entity_name,
            authentication_valid_until,
            request_scope,
            contracts,
            mutation_preconditions,
            authorization_admission_work,
            authorization,
            governed_input_identity,
            authorization_basis,
            graph_work,
        } = input;
        let resource_binding_identity = admission_identity.resource_binding_identity();
        let canonical_work = WorthQueryCanonicalWorkPhases::new(
            contracts.canonical_work(),
            mutation_preconditions
                .canonical_work()
                .combine(authorization_admission_work),
            WorthQueryCanonicalWorkEvidence::zero(),
            WorthQueryCanonicalWorkEvidence::zero(),
            WorthQueryCanonicalWorkEvidence::zero(),
        );
        Self {
            runtime_authority,
            binding_identity,
            operation,
            operation_authority_identity: operation_authority_identity.into(),
            operation_authority_identity_bytes,
            admission_identity,
            resource_binding_identity,
            operation_scope_binding,
            canonical_work,
            scope_entity_id,
            scope_entity_kind,
            scope_entity_name,
            authentication_valid_until,
            request_scope,
            contracts,
            mutation_preconditions,
            authorization: Some(authorization),
            governed_input_identity,
            authorization_basis,
            graph_work,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicU64;

    use super::WorthQueryOperationAdmissionIdentity;

    #[test]
    fn operation_admission_identity_exhaustion_cannot_wrap() {
        let counter = AtomicU64::new(u64::MAX);
        assert!(WorthQueryOperationAdmissionIdentity::mint_from(&counter).is_none());
        assert_eq!(counter.load(std::sync::atomic::Ordering::Relaxed), u64::MAX);
    }
}
