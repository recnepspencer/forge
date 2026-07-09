use worth_query::facade::identity_authority::{
    QueryProjectionIdentity, QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::WorthQueryEvidenceIdentity;

fn projection_identity() -> QueryProjectionIdentity<String, QuerySubscriptionIdentityKind> {
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
    require_subscription_authority(projection_identity());
}
