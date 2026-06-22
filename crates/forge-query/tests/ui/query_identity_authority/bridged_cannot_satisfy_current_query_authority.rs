use forge_query::facade::identity_authority::{
    QueryBoundaryBridgedIdentity, QuerySubscriptionAuthority, QuerySubscriptionAuthorityIdentity,
    QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

fn bridged_identity(
) -> QueryBoundaryBridgedIdentity<
    ForgeQueryEvidenceIdentity,
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
    require_subscription_authority(bridged_identity());
}
