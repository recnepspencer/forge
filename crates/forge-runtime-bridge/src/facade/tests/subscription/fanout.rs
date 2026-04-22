use super::support::*;

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

#[test]
fn equivalent_detail_consumers_share_one_active_subscription() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let additional = canonical_consumer_contract(&runtime);

    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![additional])
        .expect("equivalent consumers should share");

    assert_eq!(
        plan.counters().subscription_fanout_plan_admission_count(),
        1
    );
    assert_eq!(plan.consumer_contract_identity_count(), 2);
    assert_eq!(
        plan.active_subscription_identity(),
        active.active_subscription_identity()
    );
    assert_eq!(
        plan.sharing_eligibility_digest(),
        active.consumer_contract().sharing_eligibility().digest()
    );
}

#[test]
fn equivalent_collection_consumers_share_one_active_subscription() {
    let (runtime, active) = active_collection_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let additional = canonical_consumer_contract(&runtime);

    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![additional])
        .expect("equivalent collection consumers should share");

    assert_eq!(plan.consumer_contract_identity_count(), 2);
    assert_eq!(
        plan.cost_profile_identity(),
        active.cost_profile().cost_profile_identity()
    );
}

#[test]
fn shared_and_separate_equivalent_consumers_preserve_canonical_delivery_truth() {
    let (runtime, shared_active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let additional = canonical_consumer_contract(&runtime);
    let plan = runtime
        .plan_shared_subscription_fanout(&shared_active, vec![additional])
        .expect("equivalent consumers should share");
    let layout = runtime.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let shared_window = sealed_window(
        &runtime,
        &shared_active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let projections = runtime
        .project_subscription_delivery_to_fanout(&layout, &shared_window)
        .expect("projection should match layout");

    let (separate_runtime, separate_active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let separate_window = sealed_window(
        &separate_runtime,
        &separate_active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    assert_eq!(
        shared_window.members()[0].digest(),
        separate_window.members()[0].digest()
    );
    assert_eq!(projections.len(), 2);
    assert_eq!(
        projections
            .counters()
            .subscription_fanout_delivery_projection_count(),
        2
    );
    assert!(projections
        .iter()
        .all(|projection| projection.canonical_member_digest_basis()
            == shared_window.members()[0].digest()));
}

#[test]
fn incompatible_replay_audit_consumer_rejects_shared_fanout() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let replay_audit = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::ReplayAudit,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::RetainedDetail,
        )
        .expect("replay/audit consumer should admit with retained diagnostics");

    let rejection = runtime
        .plan_shared_subscription_fanout(&active, vec![replay_audit])
        .expect_err("replay/audit consumer should not share canonical active delivery");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutPlanRejectionKind::ContractFamilyMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_fanout_plan_rejection_count(),
        1
    );
}

#[test]
fn mismatched_coalescing_rejects_shared_fanout() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let non_coalescing = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            false,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("non-coalescing consumer should admit");

    let rejection = runtime
        .plan_shared_subscription_fanout(&active, vec![non_coalescing])
        .expect_err("coalescing mismatch should reject sharing");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutPlanRejectionKind::CoalescingMismatch
    );
}

#[test]
fn mismatched_pacing_and_backpressure_reject_shared_fanout() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let lag_bounded = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::LagBounded,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("lag-bounded consumer should admit");

    let pacing_rejection = runtime
        .plan_shared_subscription_fanout(&active, vec![lag_bounded])
        .expect_err("pacing mismatch should reject sharing");

    assert_eq!(
        pacing_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutPlanRejectionKind::PacingCapabilityMismatch
    );

    let primary_runtime = runtime(BridgeRuntimePolicy::development());
    let lag_bounded_primary = primary_runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::LagBounded,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("lag-bounded primary consumer should admit");
    let (runtime, active) = active_detail_subscription_with_consumer(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
        lag_bounded_primary,
    );
    let independent_cursor = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::LagBounded,
            BridgeSubscriptionConsumerBackpressurePosture::IndependentCursorRequired,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("independent cursor consumer should admit with lag-bounded pacing");

    let backpressure_rejection = runtime
        .plan_shared_subscription_fanout(&active, vec![independent_cursor])
        .expect_err("backpressure mismatch should reject sharing");

    assert_eq!(
        backpressure_rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutPlanRejectionKind::BackpressurePostureMismatch
    );
}

#[test]
fn fanout_width_over_cost_profile_rejects_before_layout() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let additional = canonical_consumer_contract(&runtime);

    let rejection = runtime
        .plan_shared_subscription_fanout(&active, vec![additional])
        .expect_err("cost profile max fanout width is one");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutPlanRejectionKind::FanoutWidthExceedsCostProfile
    );
}

#[test]
fn fanout_layout_binds_ordered_consumer_slots_deterministically() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        3,
    );
    let second = canonical_consumer_contract(&runtime);
    let third = canonical_consumer_contract(&runtime);
    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![second, third])
        .expect("equivalent consumers should share");

    let left = runtime.build_subscription_fanout_layout(
        plan.clone(),
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let right = runtime.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.consumer_bindings().len(), 3);
    assert_eq!(left.consumer_bindings()[0].slot_index(), 0);
    assert_eq!(left.consumer_bindings()[1].slot_index(), 1);
    assert_eq!(left.consumer_bindings()[2].slot_index(), 2);
    assert_eq!(left.consumer_bindings()[0].frontier_slot_index(), 0);
    assert_eq!(left.consumer_bindings()[1].frontier_slot_index(), 1);
    assert_eq!(left.consumer_bindings()[2].frontier_slot_index(), 2);
    assert_eq!(
        left.consumer_bindings()[0].acknowledgement_policy_class(),
        crate::facade::BridgeSubscriptionFanoutAcknowledgementPolicyClass::CanonicalMemberAcknowledgement
    );
    assert_eq!(
        left.consumer_bindings()[0].diagnostics_policy_class(),
        crate::facade::BridgeSubscriptionFanoutDiagnosticsPolicyClass::MinimalReferenceOnly
    );
    assert_eq!(left.counters().subscription_fanout_layout_build_count(), 1);
    assert_eq!(
        left.counters().subscription_fanout_consumer_binding_count(),
        3
    );
}

#[test]
fn delivery_projection_preserves_canonical_truth_without_rich_materialization() {
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

    let projections = runtime
        .project_subscription_delivery_to_fanout(&layout, &sealed)
        .expect("sealed window should project through matching layout");

    assert_eq!(projections.len(), 2);
    assert_eq!(
        projections.canonical_member_digest_basis(),
        sealed.members()[0].digest()
    );
    assert_eq!(
        projections
            .counters()
            .subscription_fanout_delivery_projection_count(),
        2
    );
    for projection in projections.iter() {
        assert_eq!(
            projection.delivery_window_identity(),
            sealed.delivery_window_identity()
        );
        assert_eq!(projection.canonical_member_count(), sealed.members().len());
        assert_eq!(
            projection.canonical_member_digest_basis(),
            sealed.members()[0].digest()
        );
        assert_eq!(
            projection
                .counters()
                .subscription_fanout_delivery_projection_count(),
            0
        );
        assert_eq!(
            projection
                .counters()
                .subscription_rich_diagnostics_hot_path_materialization_count(),
            0
        );
    }
}

#[test]
fn fanout_counters_prove_zero_hot_path_scans() {
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

    assert_eq!(
        layout
            .counters()
            .subscription_callback_identity_scan_count(),
        0
    );
    assert_eq!(
        layout.counters().subscription_active_registry_scan_count(),
        0
    );
    assert_eq!(
        layout
            .counters()
            .subscription_fanout_per_member_consumer_scan_count(),
        0
    );
}

#[test]
fn fanout_projection_rejects_delivery_family_drift() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow,
        2,
    );
    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![canonical_consumer_contract(&runtime)])
        .expect("equivalent consumers should share");
    let layout = runtime.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let coalesced = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
    );

    let rejection = runtime
        .project_subscription_delivery_to_fanout(&layout, &coalesced)
        .expect_err("family drift should reject projection");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionFanoutProjectionRejectionKind::DeliveryFamilyMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_fanout_delivery_projection_rejection_count(),
        1
    );
}
