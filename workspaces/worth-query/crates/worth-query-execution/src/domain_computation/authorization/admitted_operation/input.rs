use std::time::Instant;

use worth_query_admission::facade::authenticated_principal::WorthQueryRequestScope;
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
    WorthQueryCompiledApplicationOperationContracts,
};

use super::{WorthQueryOperationAdmissionIdentity, WorthQueryOperationAuthorizationBasis};
use crate::domain_computation::authorization::{
    WorthQueryOperationScopeBinding, WorthQueryRetainedAuthorizationDecisionFacts,
};
use crate::domain_computation::primary_graph::WorthQueryBoundMutationPreconditions;

pub(in crate::domain_computation::authorization) struct WorthQueryAdmittedApplicationOperationInput<
    Input,
> {
    pub(in crate::domain_computation::authorization) admission_identity:
        WorthQueryOperationAdmissionIdentity,
    pub(in crate::domain_computation::authorization) runtime_authority:
        crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity,
    pub(in crate::domain_computation::authorization) binding_identity:
        ApplicationSchemaBindingIdentity,
    pub(in crate::domain_computation::authorization) operation: String,
    pub(in crate::domain_computation::authorization) operation_authority_identity: String,
    pub(in crate::domain_computation::authorization) operation_authority_identity_bytes: [u8; 32],
    pub(in crate::domain_computation::authorization) operation_scope_binding:
        WorthQueryOperationScopeBinding,
    pub(in crate::domain_computation::authorization) scope_entity_id:
        worth_relational::facade::identity::EntityId,
    pub(in crate::domain_computation::authorization) scope_entity_kind:
        worth_relational::facade::identity::KindId,
    pub(in crate::domain_computation::authorization) scope_entity_name: String,
    pub(in crate::domain_computation::authorization) authentication_valid_until: Instant,
    pub(in crate::domain_computation::authorization) request_scope: WorthQueryRequestScope,
    pub(in crate::domain_computation::authorization) contracts:
        WorthQueryCompiledApplicationOperationContracts,
    pub(in crate::domain_computation::authorization) mutation_preconditions:
        WorthQueryBoundMutationPreconditions,
    pub(in crate::domain_computation::authorization) authorization_admission_work:
        WorthQueryCanonicalWorkEvidence,
    pub(in crate::domain_computation::authorization) authorization:
        WorthQueryRetainedAuthorizationDecisionFacts,
    pub(in crate::domain_computation::authorization) governed_input_identity: Option<[u8; 32]>,
    pub(in crate::domain_computation::authorization) authorization_basis:
        WorthQueryOperationAuthorizationBasis<Input>,
    pub(in crate::domain_computation::authorization) graph_work:
        crate::domain_computation::provider_session::WorthQueryManagedGraphWorkSession,
}
