use forge_query::facade::identity_authority::{
    QuerySubscriptionAuthorityIdentity, QuerySubscriptionIdentityKind,
};
use forge_query::facade::{ForgeQueryEvidenceIdentity, TerminalProjectionLabel};

fn require_subscription_authority(
    _identity: QuerySubscriptionAuthorityIdentity<
        ForgeQueryEvidenceIdentity,
        QuerySubscriptionIdentityKind,
    >,
) {
}

fn main() {
    let label: TerminalProjectionLabel = todo!();
    require_subscription_authority(label);
}
