use super::support::*;

#[test]
fn runtime_admits_subscription_delivery_cost_profiles() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let sparse = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            1,
            1,
        )
        .expect("sparse profile should admit");
    let coalesced = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow,
            4,
            4,
            1,
        )
        .expect("coalesced profile should admit");

    assert_eq!(
        sparse
            .counters()
            .subscription_delivery_cost_profile_selection_count(),
        1
    );
    assert_eq!(
        sparse
            .counters()
            .subscription_delivery_density_sparse_count(),
        1
    );
    assert_eq!(
        coalesced
            .counters()
            .subscription_delivery_density_coalesced_count(),
        1
    );
}

#[test]
fn runtime_rejects_over_budget_delivery_profile_before_delivery() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::RejectedOverBudget,
            4,
            1,
            1,
        )
        .expect_err("over-budget posture should reject");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionDeliveryCostProfileRejectionKind::OverBudgetPostureRejected
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_delivery_over_budget_rejection_count(),
        1
    );
}

#[test]
fn runtime_rejects_zero_fanout_width_cost_profile_before_activation() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            1,
            0,
        )
        .expect_err("active subscription needs at least one admitted consumer slot");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionDeliveryCostProfileRejectionKind::EmptyFanoutBudget
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_delivery_cost_profile_rejection_count(),
        1
    );
}

#[test]
fn delivery_window_rejects_member_count_over_cost_profile_before_projection() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        1,
    );
    let open = runtime.open_subscription_delivery_window(
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
    );

    let rejection = runtime
        .seal_subscription_delivery_window(
            open,
            vec![
                BridgeSubscriptionDeliveryMemberInput::payload_digest(
                    "slice:entity-1/profile/name",
                    "routing:fixture:1",
                    BridgeSubscriptionDeliveryMemberClass::Update,
                    "payload:fixture:1",
                ),
                BridgeSubscriptionDeliveryMemberInput::payload_digest(
                    "slice:entity-1/profile/name",
                    "routing:fixture:2",
                    BridgeSubscriptionDeliveryMemberClass::Update,
                    "payload:fixture:2",
                ),
                BridgeSubscriptionDeliveryMemberInput::payload_digest(
                    "slice:entity-1/profile/name",
                    "routing:fixture:3",
                    BridgeSubscriptionDeliveryMemberClass::Update,
                    "payload:fixture:3",
                ),
                BridgeSubscriptionDeliveryMemberInput::payload_digest(
                    "slice:entity-1/profile/name",
                    "routing:fixture:4",
                    BridgeSubscriptionDeliveryMemberClass::Update,
                    "payload:fixture:4",
                ),
                BridgeSubscriptionDeliveryMemberInput::payload_digest(
                    "slice:entity-1/profile/name",
                    "routing:fixture:5",
                    BridgeSubscriptionDeliveryMemberClass::Update,
                    "payload:fixture:5",
                ),
            ],
        )
        .expect_err("delivery window should reject before constructing records");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionDeliveryWindowRejectionKind::MemberCountExceedsCostProfile
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_delivery_over_budget_rejection_count(),
        1
    );
    assert_eq!(rejection.counters().subscription_delivery_record_count(), 0);
    assert_eq!(
        rejection
            .counters()
            .subscription_rich_diagnostics_hot_path_materialization_count(),
        0
    );
}

#[test]
fn runtime_admits_consumer_contract_without_callback_identity() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let left = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("consumer contract should admit");
    let right = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("same semantic contract should admit");

    assert_eq!(left, right);
    assert_eq!(
        left.consumer_contract_identity(),
        right.consumer_contract_identity()
    );
    assert_eq!(
        left.counters()
            .subscription_consumer_contract_admission_count(),
        1
    );
    assert_eq!(
        left.sharing_eligibility().digest(),
        right.sharing_eligibility().digest()
    );
}

#[test]
fn runtime_activates_subscription_delivery_from_activation_ready() {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            1,
            1,
        )
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);

    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);

    assert_eq!(active.counters().subscription_activation_count(), 1);
    assert_eq!(
        active
            .buffer_plan()
            .counters()
            .subscription_delivery_buffer_reuse_count(),
        1
    );
    assert_eq!(
        active
            .buffer_plan()
            .counters()
            .subscription_delivery_arena_reset_count(),
        1
    );
}

#[test]
fn runtime_emits_stable_canonical_subscription_delivery_records() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);

    let left = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let right = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.members().len(), 1);
    assert_eq!(left.members()[0].canonical_sequence(), 0);
    assert_eq!(
        left.members()[0].route_or_slice_identity(),
        "slice:entity-1/profile/name"
    );
    assert_eq!(left.counters().subscription_delivery_record_count(), 1);
    assert_eq!(left.counters().subscription_delivery_member_count(), 1);
    assert_eq!(
        left.counters()
            .subscription_rich_diagnostics_hot_path_materialization_count(),
        0
    );
}

#[test]
fn delivery_window_identity_changes_with_canonical_member_truth() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left_open = runtime.open_subscription_delivery_window(
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
    );
    let right_open = runtime.open_subscription_delivery_window(
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
    );

    let left = runtime
        .seal_subscription_delivery_window(
            left_open,
            vec![BridgeSubscriptionDeliveryMemberInput::payload_digest(
                "slice:entity-1/profile/name",
                "routing:fixture",
                BridgeSubscriptionDeliveryMemberClass::Update,
                "payload:left",
            )],
        )
        .expect("left delivery window should seal");
    let right = runtime
        .seal_subscription_delivery_window(
            right_open,
            vec![BridgeSubscriptionDeliveryMemberInput::payload_digest(
                "slice:entity-1/profile/name",
                "routing:fixture",
                BridgeSubscriptionDeliveryMemberClass::Update,
                "payload:right",
            )],
        )
        .expect("right delivery window should seal");

    assert_ne!(
        left.delivery_window_identity(),
        right.delivery_window_identity()
    );
    assert_ne!(
        left.members()[0].delivery_window_identity(),
        right.members()[0].delivery_window_identity()
    );
}

#[test]
fn delivery_window_identity_changes_with_occurrence_sequence() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let left_open = runtime.open_subscription_delivery_window(
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        41,
    );
    let right_open = runtime.open_subscription_delivery_window(
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        42,
    );

    assert_ne!(
        left_open.delivery_window_open_identity(),
        right_open.delivery_window_open_identity()
    );

    let member = || {
        BridgeSubscriptionDeliveryMemberInput::payload_digest(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            "payload:same",
        )
    };
    let left = runtime
        .seal_subscription_delivery_window(left_open, vec![member()])
        .expect("left delivery window should seal");
    let right = runtime
        .seal_subscription_delivery_window(right_open, vec![member()])
        .expect("right delivery window should seal");

    assert_ne!(
        left.delivery_window_identity(),
        right.delivery_window_identity()
    );
    assert_ne!(left.members()[0].digest(), right.members()[0].digest());
}

#[test]
fn coalesced_delivery_reconstructs_the_same_canonical_member_truth() {
    let (runtime, active) = active_detail_subscription(
        BridgeSubscriptionDeliveryDensityPosture::BoundedCoalescedWindow,
    );

    let canonical = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );
    let coalesced = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::AdmittedCoalesced,
    );

    assert_eq!(canonical.members().len(), coalesced.members().len());
    assert_eq!(
        canonical.members()[0].route_or_slice_identity(),
        coalesced.members()[0].route_or_slice_identity()
    );
    assert_eq!(
        canonical.members()[0].member_class(),
        coalesced.members()[0].member_class()
    );
}

#[test]
fn retained_delivery_seed_binds_window_sequence_and_member_truth() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);

    let first = sealed_window_with_member(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        7,
        BridgeSubscriptionDeliveryMemberInput::payload_digest(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            "payload:same",
        ),
    );
    let second_sequence = sealed_window_with_member(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        8,
        BridgeSubscriptionDeliveryMemberInput::payload_digest(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            "payload:same",
        ),
    );
    let second_truth = sealed_window_with_member(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        7,
        BridgeSubscriptionDeliveryMemberInput::payload_digest(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            "payload:different",
        ),
    );

    let first_seed = runtime.retain_subscription_delivery_window_seed(&first);
    let second_sequence_seed = runtime.retain_subscription_delivery_window_seed(&second_sequence);
    let second_truth_seed = runtime.retain_subscription_delivery_window_seed(&second_truth);

    assert_eq!(
        first_seed.delivery_window_identity(),
        first.delivery_window_identity()
    );
    assert_eq!(
        first_seed.active_subscription_identity(),
        first.active_subscription_identity()
    );
    assert_eq!(
        first_seed.admitted_subscription_identity(),
        first.admitted_subscription_identity()
    );
    assert_eq!(first_seed.basis_identity(), first.basis_identity());
    assert_eq!(first_seed.delivery_window_sequence(), 7);
    assert_eq!(
        first_seed.canonical_member_digest_basis(),
        first.members()[0].digest()
    );
    assert_eq!(
        first_seed.replay_readiness_class(),
        crate::facade::BridgeSubscriptionDeliveryReplayReadinessClass::CanonicalMemberReplayReady
    );
    assert_ne!(first_seed.digest(), second_sequence_seed.digest());
    assert_ne!(first_seed.digest(), second_truth_seed.digest());
    assert_eq!(
        first_seed
            .counters()
            .subscription_delivery_window_seed_retention_count(),
        1
    );
}

#[test]
fn replay_readiness_blocks_omitted_payload_for_canonical_replay() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window_with_member(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
        BridgeSubscriptionDeliveryMemberInput::omitted_payload(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            BridgeSubscriptionPayloadOmissionReason::PayloadDigestOnly,
        ),
    );

    let readiness = runtime.inspect_subscription_delivery_replay_readiness(&sealed);

    assert_eq!(
        readiness.readiness_class(),
        crate::facade::BridgeSubscriptionDeliveryReplayReadinessClass::ReplayBlockedByOmittedPayload
    );
    assert_eq!(
        readiness
            .counters()
            .subscription_delivery_replay_readiness_inspection_count(),
        1
    );
    assert_eq!(
        readiness
            .counters()
            .subscription_rich_diagnostics_hot_path_materialization_count(),
        0
    );
}

#[test]
fn descriptor_replay_readiness_ignores_payload_omission() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window_with_member(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor,
        0,
        BridgeSubscriptionDeliveryMemberInput::omitted_payload(
            "route:entity-1",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            BridgeSubscriptionPayloadOmissionReason::RouteFocusedDelivery,
        ),
    );

    let readiness = runtime.inspect_subscription_delivery_replay_readiness(&sealed);

    assert_eq!(
        readiness.readiness_class(),
        crate::facade::BridgeSubscriptionDeliveryReplayReadinessClass::DescriptorOnlyReplayReady
    );
}

#[test]
fn descriptor_family_retains_descriptor_replay_seed_without_reconstruction() {
    let (runtime, active) = active_detail_subscription_with_fanout(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
        2,
    );
    let plan = runtime
        .plan_shared_subscription_fanout(&active, vec![canonical_consumer_contract(&runtime)])
        .expect("equivalent consumers should share");
    let layout = runtime.build_subscription_fanout_layout(
        plan,
        BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor,
    );
    let sealed = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::RouteFocusedDescriptor,
    );
    let projection_set = runtime
        .project_subscription_delivery_to_fanout(&layout, &sealed)
        .expect("descriptor projection should match layout");

    let replay_seed = runtime.retain_subscription_fanout_projection_seed(&projection_set);
    let readiness = runtime.inspect_subscription_delivery_replay_readiness(&sealed);

    assert_eq!(
        readiness.readiness_class(),
        crate::facade::BridgeSubscriptionDeliveryReplayReadinessClass::DescriptorOnlyReplayReady
    );
    assert_eq!(
        replay_seed.canonical_member_digest_basis(),
        projection_set.canonical_member_digest_basis()
    );
    assert_eq!(
        replay_seed.fanout_projection_set_identity(),
        projection_set.fanout_delivery_projection_set_identity()
    );
    assert_eq!(
        replay_seed.delivery_window_identity(),
        projection_set.delivery_window_identity()
    );
    assert_eq!(
        replay_seed
            .counters()
            .subscription_delivery_replay_seed_retention_count(),
        1
    );
}

#[test]
fn diagnostics_reference_emits_without_rich_hot_path_materialization() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    let reference = runtime.inspect_subscription_delivery_reference(&sealed);

    assert_eq!(
        reference
            .counters()
            .subscription_diagnostics_reference_emit_count(),
        1
    );
    assert_eq!(
        sealed
            .counters()
            .subscription_rich_diagnostics_hot_path_materialization_count(),
        0
    );
}

#[test]
fn detail_and_collection_families_both_deliver_through_phase_one_path() {
    let (runtime, detail_active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let detail = sealed_window(
        &runtime,
        &detail_active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    let (runtime, collection_active) = active_collection_subscription(
        BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
    );
    let collection = sealed_window(
        &runtime,
        &collection_active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    assert_eq!(detail.members().len(), 1);
    assert_eq!(collection.members().len(), 1);
    assert_ne!(detail.digest(), collection.digest());
}
