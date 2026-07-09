use worth_query::facade::{SubscriptionContinuationClass, SubscriptionContinuationEvidence};

fn main() {
    let _evidence = SubscriptionContinuationEvidence {
        active_lane_digest: todo!(),
        continuation_class: SubscriptionContinuationClass::IdentityRemap,
        source_identity: todo!(),
        target_identity: todo!(),
        basis_digest: String::new(),
        authority_digest: String::new(),
        remap_width: todo!(),
        continuation_digest: String::new(),
    };
}
