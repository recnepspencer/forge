use worth_query::facade::runtime::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope};

fn main() {
    let _identity = WorthQueryEvidenceIdentity::compose(
        WorthQueryEvidenceScope::SubscriptionActivationReceipt,
    );
}
