use forge_query::facade::{ForgeQueryEvidenceIdentityEncoder, ForgeQueryEvidenceScope};

fn main() {
    let _encoder = ForgeQueryEvidenceIdentityEncoder::for_scope(
        ForgeQueryEvidenceScope::SubscriptionActivationReceipt,
    );
}
