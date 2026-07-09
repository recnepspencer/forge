use worth_query::facade::{WorthQueryEvidenceIdentity, WorthQueryEvidenceScope};

fn main() {
    let _identity = WorthQueryEvidenceIdentity::compose(
        WorthQueryEvidenceScope::SubscriptionActivationReceipt,
    );
}
