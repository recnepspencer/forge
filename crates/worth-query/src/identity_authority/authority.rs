use worth_proof::{AuthorityMarker, AuthorityWitness};

pub struct QueryReceiptAdmissionAuthority;
pub struct QueryRuntimeBackendAuthority;
pub struct QueryFeederAuthority;
pub struct QueryDownstreamAdapterAuthority;
pub struct QueryEvidenceAuthority;
pub struct QueryIntentAuthority;
pub struct QuerySignalInvalidationAuthority;
pub struct QueryWorkflowAuthority;
pub struct QueryDomainCapabilityAuthority;
pub struct QueryMaterializationAuthority;
pub struct QueryEffectLifecycleAuthority;
pub struct QueryCausalInspectionAuthority;
pub struct QuerySubscriptionAuthority;
pub struct QueryTruthIdentityAdmissionAuthority;

impl AuthorityMarker for QueryReceiptAdmissionAuthority {}
impl AuthorityMarker for QueryRuntimeBackendAuthority {}
impl AuthorityMarker for QueryFeederAuthority {}
impl AuthorityMarker for QueryDownstreamAdapterAuthority {}
impl AuthorityMarker for QueryEvidenceAuthority {}
impl AuthorityMarker for QueryIntentAuthority {}
impl AuthorityMarker for QuerySignalInvalidationAuthority {}
impl AuthorityMarker for QueryWorkflowAuthority {}
impl AuthorityMarker for QueryDomainCapabilityAuthority {}
impl AuthorityMarker for QueryMaterializationAuthority {}
impl AuthorityMarker for QueryEffectLifecycleAuthority {}
impl AuthorityMarker for QueryCausalInspectionAuthority {}
impl AuthorityMarker for QuerySubscriptionAuthority {}
impl AuthorityMarker for QueryTruthIdentityAdmissionAuthority {}

pub fn query_receipt_admission_authority() -> AuthorityWitness<QueryReceiptAdmissionAuthority> {
    AuthorityWitness::from_authority_marker(QueryReceiptAdmissionAuthority)
}

pub fn query_runtime_backend_authority() -> AuthorityWitness<QueryRuntimeBackendAuthority> {
    AuthorityWitness::from_authority_marker(QueryRuntimeBackendAuthority)
}

pub fn query_feeder_authority() -> AuthorityWitness<QueryFeederAuthority> {
    AuthorityWitness::from_authority_marker(QueryFeederAuthority)
}

pub fn query_downstream_adapter_authority() -> AuthorityWitness<QueryDownstreamAdapterAuthority> {
    AuthorityWitness::from_authority_marker(QueryDownstreamAdapterAuthority)
}

pub fn query_evidence_authority() -> AuthorityWitness<QueryEvidenceAuthority> {
    AuthorityWitness::from_authority_marker(QueryEvidenceAuthority)
}

pub fn query_intent_authority() -> AuthorityWitness<QueryIntentAuthority> {
    AuthorityWitness::from_authority_marker(QueryIntentAuthority)
}

pub fn query_signal_invalidation_authority() -> AuthorityWitness<QuerySignalInvalidationAuthority> {
    AuthorityWitness::from_authority_marker(QuerySignalInvalidationAuthority)
}

pub fn query_workflow_authority() -> AuthorityWitness<QueryWorkflowAuthority> {
    AuthorityWitness::from_authority_marker(QueryWorkflowAuthority)
}

pub fn query_domain_capability_authority() -> AuthorityWitness<QueryDomainCapabilityAuthority> {
    AuthorityWitness::from_authority_marker(QueryDomainCapabilityAuthority)
}

pub fn query_materialization_authority() -> AuthorityWitness<QueryMaterializationAuthority> {
    AuthorityWitness::from_authority_marker(QueryMaterializationAuthority)
}

pub fn query_effect_lifecycle_authority() -> AuthorityWitness<QueryEffectLifecycleAuthority> {
    AuthorityWitness::from_authority_marker(QueryEffectLifecycleAuthority)
}

pub fn query_causal_inspection_authority() -> AuthorityWitness<QueryCausalInspectionAuthority> {
    AuthorityWitness::from_authority_marker(QueryCausalInspectionAuthority)
}

pub fn query_subscription_authority() -> AuthorityWitness<QuerySubscriptionAuthority> {
    AuthorityWitness::from_authority_marker(QuerySubscriptionAuthority)
}

pub fn query_truth_identity_admission_authority(
) -> AuthorityWitness<QueryTruthIdentityAdmissionAuthority> {
    AuthorityWitness::from_authority_marker(QueryTruthIdentityAdmissionAuthority)
}
