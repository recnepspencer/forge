use worth_query::facade::identity_authority::{
    QueryBoundaryBridgedIdentity, QuerySubscriptionAuthority, QuerySubscriptionAuthorityIdentity,
    QuerySubscriptionIdentityKind,
};
use worth_query::facade::WorthQueryEvidenceIdentity;

fn bridged_identity(
) -> QueryBoundaryBridgedIdentity<
    WorthQueryEvidenceIdentity,
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
    require_subscription_authority(bridged_identity());
}
