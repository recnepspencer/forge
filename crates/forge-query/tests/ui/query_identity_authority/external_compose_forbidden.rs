use forge_query::facade::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope};

fn main() {
    let _identity = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
    );
}
