use super::support::*;

#[test]
fn runtime_exposes_stable_subscription_family_registry_identity() {
    let left = runtime(BridgeRuntimePolicy::development());
    let right = runtime(BridgeRuntimePolicy::development());

    assert_eq!(
        left.subscription_family_registry_identity(),
        right.subscription_family_registry_identity()
    );
}

#[test]
fn runtime_declares_detail_exact_subscription() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native subscription field key"),
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("detail subscription should declare");

    assert_eq!(
        declaration.requested_family_kind(),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact
    );
    assert_eq!(declaration.normalized_slice_intent_count(), 1);
    assert_eq!(declaration.counters().declaration_count(), 1);
}

#[test]
fn runtime_declares_collection_membership_subscription() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_region(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("collection subscription should declare");

    assert_eq!(
        declaration.requested_family_kind(),
        BridgeSubscriptionDeclarationFamilyKind::CollectionMembership
    );
    assert_eq!(declaration.normalized_slice_intent_count(), 1);
}

#[test]
fn runtime_rejects_unsupported_family_slice_combinations() {
    let runtime = runtime(BridgeRuntimePolicy::development());

    let rejection = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_region(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect_err("detail subscriptions must reject collection slice kinds");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionDeclarationRejectionKind::UnsupportedSliceKindForFamily
    );
    assert_eq!(rejection.counters().declaration_rejection_count(), 1);
}

#[test]
fn runtime_declares_equivalent_subscriptions_canonically() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let left = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![
                NormalizedSubscriptionSliceIntent::try_new_entity_field(
                    "entity-1",
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid native subscription aspect key"),
                    forge_foundational::facade::FieldKey::new("name".to_owned())
                        .expect("valid native subscription field key"),
                    SubscriptionSliceKind::SignalField,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new_entity_field(
                    "entity-1",
                    forge_foundational::facade::AspectKey::new("profile")
                        .expect("valid native subscription aspect key"),
                    forge_foundational::facade::FieldKey::new("name".to_owned())
                        .expect("valid native subscription field key"),
                    SubscriptionSliceKind::SignalField,
                )
                .expect("slice intent should validate"),
            ],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("first subscription should declare");
    let right = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native subscription field key"),
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("second subscription should declare");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn runtime_exposes_subscription_registry_counters() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let counters = runtime.subscription_family_registry_counters();

    assert_eq!(counters.family_registry_freeze_count(), 1);
    assert_eq!(counters.family_count(), 2);
    assert_eq!(counters.family_supported_slice_kind_count(), 4);
}
