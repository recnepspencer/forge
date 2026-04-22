use super::super::support::*;

#[test]
fn fanout_projection_validation_accepts_matching_layout_and_rejects_layout_drift() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![canonical_consumer_contract(&runtime)])
        .expect("equivalent consumers should share");
    let layout = runtime.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let sealed = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let projection_set = runtime
        .project_subscription_delivery_to_fanout(&layout, &sealed)
        .expect("projection should match");

    let validation = runtime
        .validate_subscription_fanout_projection(&layout, &projection_set)
        .expect("matching projection set should validate");
    assert_eq!(
        validation
            .counters()
            .subscription_fanout_projection_validation_count(),
        1
    );
    assert_eq!(
        validation
            .counters()
            .subscription_fanout_per_member_consumer_scan_count(),
        0
    );

    let drift_plan = runtime
        .plan_shared_subscription_fanout(&active, vec![canonical_consumer_contract(&runtime)])
        .expect("equivalent consumers should share");
    let drift_layout = runtime.build_subscription_fanout_layout(
        drift_plan,
        BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor,
    );
    let rejection = runtime
        .validate_subscription_fanout_projection(&drift_layout, &projection_set)
        .expect_err("different layout identity should reject");
    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutProjectionValidationRejectionKind::LayoutIdentityMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_fanout_projection_validation_rejection_count(),
        1
    );
}

#[test]
fn fanout_projection_validation_rejects_tampered_member_and_binding_bases() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![canonical_consumer_contract(&runtime)])
        .expect("equivalent consumers should share");
    let layout = runtime.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let sealed = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let projection_set = runtime
        .project_subscription_delivery_to_fanout(&layout, &sealed)
        .expect("projection should match");

    let tampered_member_basis =
        projection_set.with_canonical_member_digest_basis_for_test("tampered-member-basis");
    let member_rejection = runtime
        .validate_subscription_fanout_projection(&layout, &tampered_member_basis)
        .expect_err("tampered member basis should reject");
    assert_eq!(
        member_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutProjectionValidationRejectionKind::CanonicalMemberDigestMismatch
    );
    assert_eq!(
        member_rejection.fanout_layout_identity(),
        layout.fanout_layout_identity()
    );
    assert_eq!(
        member_rejection.projection_set_identity(),
        tampered_member_basis.fanout_delivery_projection_set_identity()
    );
    assert_eq!(member_rejection.rejected_projection_index(), Some(0));

    let tampered_binding_basis =
        projection_set.with_consumer_binding_digest_basis_for_test("tampered-binding-basis");
    let binding_rejection = runtime
        .validate_subscription_fanout_projection(&layout, &tampered_binding_basis)
        .expect_err("tampered binding basis should reject");
    assert_eq!(
        binding_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutProjectionValidationRejectionKind::ConsumerBindingOrderMismatch
    );
    assert_eq!(binding_rejection.rejected_projection_index(), None);
}
