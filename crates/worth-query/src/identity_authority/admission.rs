use worth_foundational::facade::admit_foundational_authority_identity;

use crate::WorthQueryEvidenceIdentity;

use super::authority::{
    query_causal_inspection_authority, query_feeder_authority, query_subscription_authority,
};
use super::categories::{
    QueryCausalInspectionAuthorityIdentity, QueryFeederAuthorityIdentity,
    QuerySubscriptionAuthorityIdentity,
};
use super::kinds::{
    QueryCausalInspectionIdentityKind, QueryFeederIdentityKind, QuerySubscriptionIdentityKind,
};

pub(crate) fn admit_query_subscription_authority_identity(
    identity: WorthQueryEvidenceIdentity,
) -> QuerySubscriptionAuthorityIdentity<WorthQueryEvidenceIdentity, QuerySubscriptionIdentityKind> {
    admit_foundational_authority_identity(identity, query_subscription_authority())
}

pub(crate) fn admit_query_causal_inspection_authority_identity(
    identity: WorthQueryEvidenceIdentity,
) -> QueryCausalInspectionAuthorityIdentity<
    WorthQueryEvidenceIdentity,
    QueryCausalInspectionIdentityKind,
> {
    admit_foundational_authority_identity(identity, query_causal_inspection_authority())
}

pub(crate) fn admit_query_feeder_authority_identity(
    identity: WorthQueryEvidenceIdentity,
) -> QueryFeederAuthorityIdentity<WorthQueryEvidenceIdentity, QueryFeederIdentityKind> {
    admit_foundational_authority_identity(identity, query_feeder_authority())
}
