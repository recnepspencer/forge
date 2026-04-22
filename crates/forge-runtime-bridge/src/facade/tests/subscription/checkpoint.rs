use super::support::*;

#[test]
fn acknowledgement_frontier_binds_member_identity_digest_and_prefix() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window_with_members(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        7,
        fixture_members(2),
    );
    let acknowledged = &sealed.members()[1];

    let frontier = runtime
        .admit_subscription_acknowledgement_frontier(
            &sealed,
            1,
            acknowledged.delivery_member_identity(),
            acknowledged.digest(),
        )
        .expect("acknowledgement frontier should admit");

    assert_eq!(frontier.delivery_window_sequence(), 7);
    assert_eq!(frontier.acknowledged_canonical_sequence(), 1);
    assert_eq!(frontier.acknowledged_member_digest(), acknowledged.digest());
    assert_eq!(
        frontier
            .counters()
            .subscription_acknowledgement_frontier_admission_count(),
        1
    );
}

#[test]
fn acknowledgement_frontier_rejects_mismatched_member_digest() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
    );

    let rejection = runtime
        .admit_subscription_acknowledgement_frontier(
            &sealed,
            0,
            sealed.members()[0].delivery_member_identity(),
            "wrong-digest",
        )
        .expect_err("wrong member digest should reject");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionAcknowledgementFrontierRejectionKind::AcknowledgedMemberDigestMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_acknowledgement_frontier_rejection_count(),
        1
    );
}

#[test]
fn checkpoint_publication_binds_active_cost_consumer_and_duplicate_policy() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window_with_members(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::CanonicalMember,
        0,
        fixture_members(2),
    );
    let checkpoint = checkpoint_from_sealed(
        &runtime,
        &active,
        &sealed,
        1,
        crate::facade::BridgeSubscriptionDuplicateReplayPolicyKind::SuppressAcknowledgedMembers,
    );

    assert_eq!(
        checkpoint.active_subscription_identity(),
        active.active_subscription_identity()
    );
    assert_eq!(
        checkpoint.cost_profile_identity(),
        active.cost_profile().cost_profile_identity()
    );
    assert_eq!(
        checkpoint.consumer_contract_identity(),
        active.consumer_contract().consumer_contract_identity()
    );
    assert_eq!(
        checkpoint.duplicate_replay_policy().policy_kind(),
        crate::facade::BridgeSubscriptionDuplicateReplayPolicyKind::SuppressAcknowledgedMembers
    );
    assert_eq!(
        checkpoint
            .counters()
            .subscription_checkpoint_publication_count(),
        1
    );
}

#[test]
fn descriptor_only_family_cannot_publish_canonical_checkpoint_frontier() {
    let (runtime, active) =
        active_detail_subscription(BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery);
    let sealed = sealed_window(
        &runtime,
        &active,
        BridgeSubscriptionDeliveryFamilyKind::ReplayAuditDescriptor,
    );

    let rejection = runtime
        .admit_subscription_acknowledgement_frontier(
            &sealed,
            0,
            sealed.members()[0].delivery_member_identity(),
            sealed.members()[0].digest(),
        )
        .expect_err("descriptor-only delivery cannot publish canonical checkpoint frontier");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionAcknowledgementFrontierRejectionKind::DescriptorOnlyFamilyCannotPublishCanonicalCheckpoint
    );
}
