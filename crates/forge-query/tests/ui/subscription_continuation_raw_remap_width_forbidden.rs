use forge_query::facade::{
    admit_subscription_continuation_evidence, SubscriptionContinuationClass,
};

fn main() {
    let _evidence = admit_subscription_continuation_evidence(
        todo!(),
        SubscriptionContinuationClass::IdentityRemap,
        "employee:old",
        "employee:new",
        "basis:current",
        "identity-evolution-authority",
        1,
    )
    .unwrap();
}
