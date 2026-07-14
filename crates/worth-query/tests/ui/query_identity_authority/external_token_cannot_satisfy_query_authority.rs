use worth_query::facade::identity_authority::{
    QueryExternalIdentityToken, QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

fn external_token() -> QueryExternalIdentityToken<WorthQueryEvidenceIdentity, QuerySubscriptionIdentityKind> {
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
    require_subscription_authority(external_token());
}
