use worth_store::{LiveQueryContinuationSessionEvidence, SubscriptionSupportResumeEvidence};

fn main() {
    require_subscription_support_evidence(session_evidence());
}

fn require_subscription_support_evidence(_: SubscriptionSupportResumeEvidence) {}

fn session_evidence() -> LiveQueryContinuationSessionEvidence {
    todo!()
}
