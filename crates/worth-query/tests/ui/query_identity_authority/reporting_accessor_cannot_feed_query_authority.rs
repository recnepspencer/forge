use worth_query::facade::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::runtime::WorthQueryEvidenceIdentity;

fn evidence_identity() -> WorthQueryEvidenceIdentity {
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
    let projected = evidence_identity().as_str();
    require_subscription_authority(projected);
}
