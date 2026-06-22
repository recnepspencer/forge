use forge_query::facade::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::ForgeQueryEvidenceIdentity;

fn evidence_identity() -> ForgeQueryEvidenceIdentity {
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
    let projected = evidence_identity().as_str();
    require_subscription_authority(projected);
}
