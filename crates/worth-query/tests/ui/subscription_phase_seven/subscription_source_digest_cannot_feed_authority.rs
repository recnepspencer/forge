use worth_query::facade::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;
use worth_query::facade::runtime::QuerySubscriptionSupportProfile;

fn require_subscription_authority(
    _identity: QuerySubscriptionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
) {
}

fn main() {
    let profile: QuerySubscriptionSupportProfile = todo!();
    require_subscription_authority(profile.source_projection().label());
}
