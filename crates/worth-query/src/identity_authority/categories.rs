use worth_foundational::facade::{
    FoundationalAuthorityIdentity, FoundationalBoundaryBridgedIdentity,
    FoundationalDigestIdentityEvidence, FoundationalExternalIdentityToken,
    FoundationalProjectionIdentity,
};

use super::authority::{
    QueryCausalInspectionAuthority, QueryDomainCapabilityAuthority,
    QueryDownstreamAdapterAuthority, QueryEffectLifecycleAuthority, QueryEvidenceAuthority,
    QueryFeederAuthority, QueryIntentAuthority, QueryMaterializationAuthority,
    QueryReceiptAdmissionAuthority, QueryRuntimeBackendAuthority, QuerySignalInvalidationAuthority,
    QuerySubscriptionAuthority, QueryTruthIdentityAdmissionAuthority, QueryWorkflowAuthority,
};

pub type QueryAuthorityIdentity<Value, Authority, Kind> =
    FoundationalAuthorityIdentity<Value, Authority, Kind>;

pub type QueryBoundaryBridgedIdentity<Value, Authority, Kind> =
    FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>;

pub type QueryExternalIdentityToken<Value, Kind> = FoundationalExternalIdentityToken<Value, Kind>;

pub type QueryProjectionIdentity<Label, Kind> = FoundationalProjectionIdentity<Label, Kind>;

pub type QueryDigestIdentityEvidence<Basis, Authority, Kind> =
    FoundationalDigestIdentityEvidence<Basis, Authority, Kind>;

pub type QueryReceiptAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryReceiptAdmissionAuthority, Kind>;

pub type QueryRuntimeBackendAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryRuntimeBackendAuthority, Kind>;

pub type QueryFeederAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryFeederAuthority, Kind>;

pub type QueryDownstreamAdapterAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryDownstreamAdapterAuthority, Kind>;

pub type QueryEvidenceAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryEvidenceAuthority, Kind>;

pub type QueryIntentAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryIntentAuthority, Kind>;

pub type QuerySignalInvalidationAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QuerySignalInvalidationAuthority, Kind>;

pub type QueryWorkflowAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryWorkflowAuthority, Kind>;

pub type QueryDomainCapabilityAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryDomainCapabilityAuthority, Kind>;

pub type QueryMaterializationAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryMaterializationAuthority, Kind>;

pub type QueryEffectLifecycleAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryEffectLifecycleAuthority, Kind>;

pub type QueryCausalInspectionAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryCausalInspectionAuthority, Kind>;

pub type QuerySubscriptionAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QuerySubscriptionAuthority, Kind>;

pub type QueryTruthIdentityAdmissionAuthorityIdentity<Value, Kind> =
    QueryAuthorityIdentity<Value, QueryTruthIdentityAdmissionAuthority, Kind>;
