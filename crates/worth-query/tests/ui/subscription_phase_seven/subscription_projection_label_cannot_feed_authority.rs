use worth_query::facade::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use worth_query::facade::{WorthQueryEvidenceIdentity, TerminalProjectionLabel};

fn require_subscription_authority(
    _identity: QuerySubscriptionAuthorityIdentity<
        WorthQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
) {
}

fn main() {
    let label: TerminalProjectionLabel = todo!();
    require_subscription_authority(label);
}
