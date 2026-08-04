use super::{
    publish_exact, StoreErrorKind, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportRuntimeHandoffRequest, WORTHStoreBuilder,
};

#[test]
fn subscription_support_runtime_handoff_requires_distinct_runtime_owners() {
    let mut store = WORTHStoreBuilder::new().in_memory().build().unwrap();
    let artifact_id = publish_exact(&mut store, "basis:handoff", "cursor:1", "checkpoint:1");

    let error = SubscriptionSupportRuntimeHandoffRequest::new(
        SubscriptionSupportFamilyId::new("basis-bound-continuation-support").unwrap(),
        SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
        artifact_id,
        "runtime:same",
        "runtime:same",
    )
    .expect_err("handoff must not collapse source and target runtime owners");

    assert_eq!(
        error.kind(),
        &StoreErrorKind::SubscriptionSupportClassificationViolation
    );
}
