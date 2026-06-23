use forge_query::facade::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;
use forge_query::facade::QuerySubscriptionSupportProfile;

fn require_subscription_authority(
    _identity: QuerySubscriptionAuthorityIdentity<
        ForgeQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
) {
}

fn main() {
    let profile: QuerySubscriptionSupportProfile = todo!();
    require_subscription_authority(profile.source_projection().label());
}
