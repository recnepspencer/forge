use worth_query::facade::identity_authority::{
    QueryFeederIdentityKind, QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::WorthQueryEvidenceIdentity;

fn wrong_kind_identity(
) -> QuerySubscriptionAuthorityIdentity<WorthQueryEvidenceIdentity, QueryFeederIdentityKind> {
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
    require_subscription_authority(wrong_kind_identity());
}
