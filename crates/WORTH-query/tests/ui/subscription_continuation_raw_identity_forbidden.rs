use worth_query::facade::{
    admit_subscription_continuation_evidence, ContinuationRemapWidth,
    SubscriptionContinuationClass,
};

fn main() {
    let _evidence = admit_subscription_continuation_evidence(
        todo!(),
        SubscriptionContinuationClass::IdentityRemap,
        "employee:old",
        "employee:new",
        "basis:current",
        "identity-evolution-authority",
        ContinuationRemapWidth::measured(1),
    )
    .unwrap();
}
