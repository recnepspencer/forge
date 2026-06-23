use forge_query::facade::identity_authority::{
    QueryProjectionIdentity, QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

fn projection_identity() -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
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
    require_subscription_authority(projection_identity());
}
