use worth_proof::{AuthorityMarker, AuthorityWitness};

macro_rules! owner_authority {
    ($($authority:ident),+ $(,)?) => {
        $(pub struct $authority { _owner_seal: () })+
    };
}

owner_authority! {
    QueryReceiptAdmissionAuthority,
    QueryRuntimeBackendAuthority,
    QueryFeederAuthority,
    QueryDownstreamAdapterAuthority,
    QueryEvidenceAuthority,
    QueryIntentAuthority,
    QuerySignalInvalidationAuthority,
    QueryWorkflowAuthority,
    QueryDomainCapabilityAuthority,
    QueryMaterializationAuthority,
    QueryEffectLifecycleAuthority,
    QueryCausalInspectionAuthority,
    QuerySubscriptionAuthority,
    QueryTruthIdentityAdmissionAuthority,
    QueryOperationProgressionAuthority,
}

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
impl AuthorityMarker for QueryOperationProgressionAuthority {}

pub(crate) fn query_receipt_admission_authority() -> AuthorityWitness<QueryReceiptAdmissionAuthority>
{
    AuthorityWitness::from_authority_marker(QueryReceiptAdmissionAuthority { _owner_seal: () })
}

pub(crate) fn query_runtime_backend_authority() -> AuthorityWitness<QueryRuntimeBackendAuthority> {
    AuthorityWitness::from_authority_marker(QueryRuntimeBackendAuthority { _owner_seal: () })
}

pub(crate) fn query_signal_invalidation_authority(
) -> AuthorityWitness<QuerySignalInvalidationAuthority> {
    AuthorityWitness::from_authority_marker(QuerySignalInvalidationAuthority { _owner_seal: () })
}

pub(crate) fn query_domain_capability_authority() -> AuthorityWitness<QueryDomainCapabilityAuthority>
{
    AuthorityWitness::from_authority_marker(QueryDomainCapabilityAuthority { _owner_seal: () })
}

pub(crate) fn query_causal_inspection_authority() -> AuthorityWitness<QueryCausalInspectionAuthority>
{
    AuthorityWitness::from_authority_marker(QueryCausalInspectionAuthority { _owner_seal: () })
}

pub(crate) fn query_subscription_authority() -> AuthorityWitness<QuerySubscriptionAuthority> {
    AuthorityWitness::from_authority_marker(QuerySubscriptionAuthority { _owner_seal: () })
}

pub(crate) fn query_truth_identity_admission_authority(
) -> AuthorityWitness<QueryTruthIdentityAdmissionAuthority> {
    AuthorityWitness::from_authority_marker(QueryTruthIdentityAdmissionAuthority {
        _owner_seal: (),
    })
}

pub(crate) fn query_operation_progression_authority(
) -> AuthorityWitness<QueryOperationProgressionAuthority> {
    AuthorityWitness::from_authority_marker(QueryOperationProgressionAuthority { _owner_seal: () })
}
