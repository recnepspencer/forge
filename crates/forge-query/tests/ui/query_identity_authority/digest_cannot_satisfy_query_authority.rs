use forge_query::facade::identity_authority::{
    QueryDigestIdentityEvidence, QueryFeederDigestIdentityBasis, QuerySubscriptionAuthority,
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

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
        ForgeQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
) {
}

fn main() {
    require_subscription_authority(digest_evidence());
}
