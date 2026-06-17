use forge_query::facade::identity_authority::{
    QueryExternalIdentityToken, QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

fn external_token() -> QueryExternalIdentityToken<ForgeQueryEvidenceIdentity, QuerySubscriptionIdentityKind> {
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
    require_subscription_authority(external_token());
}
