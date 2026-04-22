use crate::mapping::SubscriptionSliceKind;

use super::super::{
    BridgeSubscriptionDeclaration, BridgeSubscriptionDeliveryIntentClass,
    NormalizedSubscriptionSliceIntent, NormalizedSubscriptionSliceIntentErrorKind,
};
use crate::subscription::{
    phase_one_subscription_families, BridgeSubscriptionDeclarationFamilyKind,
    FrozenSubscriptionFamilyRegistry,
};

#[test]
fn same_inputs_produce_identical_declaration_digest() {
    let registry = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let family = registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::DetailExact);
    let left = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![NormalizedSubscriptionSliceIntent::try_new(
            "entity-1",
            "profile",
            "name",
            SubscriptionSliceKind::SignalField,
        )
        .expect("slice intent should validate")],
        family,
    )
    .expect("declaration should normalize");
    let right = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![NormalizedSubscriptionSliceIntent::try_new(
            "entity-1",
            "profile",
            "name",
            SubscriptionSliceKind::SignalField,
        )
        .expect("slice intent should validate")],
        family,
    )
    .expect("declaration should normalize");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn slice_order_normalizes_canonically() {
    let registry = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let family =
        registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::CollectionMembership);
    let left = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![
            NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate"),
            NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west-partition",
                SubscriptionSliceKind::SignalPartition,
            )
            .expect("slice intent should validate"),
        ],
        family,
    )
    .expect("declaration should normalize");
    let right = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![
            NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west-partition",
                SubscriptionSliceKind::SignalPartition,
            )
            .expect("slice intent should validate"),
            NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate"),
        ],
        family,
    )
    .expect("declaration should normalize");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn duplicate_slice_intents_collapse_canonically() {
    let registry = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let family = registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::DetailExact);
    let declaration = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![
            NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate"),
            NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate"),
        ],
        family,
    )
    .expect("declaration should normalize");

    assert_eq!(declaration.normalized_slice_intent_count(), 1);
    assert_eq!(
        declaration
            .counters()
            .declaration_deduplicated_slice_intent_count(),
        1
    );
}

#[test]
fn wrong_slice_kind_for_family_rejects_deterministically() {
    let registry = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let family = registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::DetailExact);
    let error = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![NormalizedSubscriptionSliceIntent::try_new(
            "entity-1",
            "profile",
            "west",
            SubscriptionSliceKind::SignalRegion,
        )
        .expect("slice intent should validate")],
        family,
    )
    .expect_err("region slices should not be admitted for detail family");

    assert_eq!(
        error.rejection_kind(),
        crate::subscription::BridgeSubscriptionDeclarationRejectionKind::UnsupportedSliceKindForFamily
    );
    assert_eq!(error.counters().declaration_rejection_count(), 1);
}

#[test]
fn non_identity_delivery_intent_is_canonicalized_away() {
    let registry = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let family =
        registry.family_for_kind(BridgeSubscriptionDeclarationFamilyKind::CollectionMembership);
    let declaration = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
        BridgeSubscriptionDeliveryIntentClass::CanonicalMeaningfulChange,
        vec![NormalizedSubscriptionSliceIntent::try_new(
            "entity-1",
            "profile",
            "west",
            SubscriptionSliceKind::SignalRegion,
        )
        .expect("slice intent should validate")],
        family,
    )
    .expect("declaration should normalize");

    assert_eq!(
        declaration.delivery_intent_class(),
        BridgeSubscriptionDeliveryIntentClass::None
    );
}

#[test]
fn slice_intent_rejects_empty_identity_bearing_fields() {
    let error = NormalizedSubscriptionSliceIntent::try_new(
        "",
        "profile",
        "name",
        SubscriptionSliceKind::SignalField,
    )
    .expect_err("empty entity identity should reject");

    assert_eq!(
        error.kind(),
        NormalizedSubscriptionSliceIntentErrorKind::EmptyEntityIdentity
    );
}
