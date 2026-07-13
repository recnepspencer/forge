use worth_query::facade::runtime::WorthQueryEvidenceScope;
use worth_query::facade::WorthQueryEvidenceIdentityEncoder;

fn main() {
    let _encoder = WorthQueryEvidenceIdentityEncoder::for_scope(
        WorthQueryEvidenceScope::SubscriptionActivationReceipt,
    );
}
