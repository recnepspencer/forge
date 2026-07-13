use worth_query::facade::identity_authority::{
    QueryDigestIdentityEvidence, QueryFeederDigestIdentityBasis, QuerySubscriptionAuthority,
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

fn digest_evidence(
) -> QueryDigestIdentityEvidence<
    QueryFeederDigestIdentityBasis,
    QuerySubscriptionAuthority,
    QuerySubscriptionIdentityKind,
> {
    unreachable!()
}

fn require_subscription_authority(
    _identity: QuerySubscriptionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
) {
}

fn main() {
    require_subscription_authority(digest_evidence());
}
