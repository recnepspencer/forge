use worth_query::facade::{WorthQueryEvidenceIdentityEncoder, WorthQueryEvidenceScope};

fn main() {
    let _encoder = WorthQueryEvidenceIdentityEncoder::for_scope(
        WorthQueryEvidenceScope::SubscriptionActivationReceipt,
    );
}
