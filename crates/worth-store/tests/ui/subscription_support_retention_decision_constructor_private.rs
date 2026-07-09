use worth_store::SubscriptionSupportRetentionDecision;

fn main() {
    let _decision = SubscriptionSupportRetentionDecision::reclaim_with_rebuild(
        "retained-rebuild-basis:external",
        "maintenance-admission:external",
    )
    .unwrap();
}
