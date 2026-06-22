use forge_query::facade::identity_authority::{
    QueryFeederIdentityKind, QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

fn wrong_kind_identity(
) -> QuerySubscriptionAuthorityIdentity<ForgeQueryEvidenceIdentity, QueryFeederIdentityKind> {
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
    require_subscription_authority(wrong_kind_identity());
}
