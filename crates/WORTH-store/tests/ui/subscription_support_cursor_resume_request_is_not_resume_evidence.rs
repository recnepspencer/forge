use worth_store::{DurableCursorResumeRequest, SubscriptionSupportResumeEvidence};

fn main() {
    require_subscription_support_evidence(cursor_only_resume_request());
}

fn require_subscription_support_evidence(_: SubscriptionSupportResumeEvidence) {}

fn cursor_only_resume_request() -> DurableCursorResumeRequest {
    todo!()
}
