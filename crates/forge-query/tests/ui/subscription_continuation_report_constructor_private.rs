use forge_query::facade::{SubscriptionContinuationClass, SubscriptionContinuationReport};

fn main() {
    let _report = SubscriptionContinuationReport {
        active_lane_digest: todo!(),
        continuation_class: SubscriptionContinuationClass::IdentityRemap,
        continuation_digest: String::new(),
        remap_width: 1,
        report_digest: String::new(),
    };
}
