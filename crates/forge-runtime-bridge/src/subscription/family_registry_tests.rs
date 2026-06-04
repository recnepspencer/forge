use crate::mapping::SubscriptionSliceKind;

use super::{
    phase_one_subscription_families, BridgeSubscriptionDeclarationFamily,
    BridgeSubscriptionDeclarationFamilyIdentity, BridgeSubscriptionDeclarationFamilyKind,
    FrozenSubscriptionFamilyRegistry,
};

#[test]
fn frozen_registry_order_is_canonical_and_stable() {
    let left = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let mut reversed = phase_one_subscription_families().expect("phase 1 families should build");
    reversed.reverse();
    let right = FrozenSubscriptionFamilyRegistry::freeze(reversed)
        .expect("reversed families should freeze");

    assert_eq!(left, right);
    assert_eq!(left.registry_identity(), right.registry_identity());
}

#[test]
fn duplicate_family_metadata_is_rejected() {
    let duplicate = BridgeSubscriptionDeclarationFamily::new(
        BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-duplicate"),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        "detail_duplicate",
        vec![SubscriptionSliceKind::SignalField],
        false,
    )
    .expect("duplicate family should build");
    let existing_detail = phase_one_subscription_families()
        .expect("phase 1 families should build")
        .into_iter()
        .find(|family| family.family_kind() == BridgeSubscriptionDeclarationFamilyKind::DetailExact)
        .expect("detail family should exist");
    let error = FrozenSubscriptionFamilyRegistry::freeze(vec![existing_detail, duplicate])
        .expect_err("duplicate family kind should be rejected");

    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::BuilderConfigurationConflict
    );
}

#[test]
fn registry_identity_changes_when_family_semantics_change() {
    let baseline = FrozenSubscriptionFamilyRegistry::freeze(
        phase_one_subscription_families().expect("phase 1 families should build"),
    )
    .expect("phase 1 families should freeze");
    let modified = FrozenSubscriptionFamilyRegistry::freeze(vec![
        BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new(
                "subscription-family:collection-membership",
            ),
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            "collection_membership",
            vec![
                SubscriptionSliceKind::SignalPartition,
                SubscriptionSliceKind::SignalRegion,
            ],
            false,
        )
        .expect("collection family should build"),
        BridgeSubscriptionDeclarationFamily::new(
            BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            "detail_exact",
            vec![
                SubscriptionSliceKind::SignalField,
                SubscriptionSliceKind::SignalLens,
                SubscriptionSliceKind::SignalFacet,
            ],
            false,
        )
        .expect("detail family should build"),
    ])
    .expect("modified families should freeze");

    assert_ne!(baseline.registry_identity(), modified.registry_identity());
}

#[test]
fn family_constructor_canonicalizes_slice_kind_order() {
    let left = BridgeSubscriptionDeclarationFamily::new(
        BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        "detail_exact",
        vec![
            SubscriptionSliceKind::SignalLens,
            SubscriptionSliceKind::SignalField,
        ],
        false,
    )
    .expect("family should build");
    let right = BridgeSubscriptionDeclarationFamily::new(
        BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        "detail_exact",
        vec![
            SubscriptionSliceKind::SignalField,
            SubscriptionSliceKind::SignalLens,
        ],
        false,
    )
    .expect("family should build");

    assert_eq!(left.supported_slice_kinds(), right.supported_slice_kinds());
}

#[test]
fn family_constructor_rejects_duplicate_slice_kinds() {
    let error = BridgeSubscriptionDeclarationFamily::new(
        BridgeSubscriptionDeclarationFamilyIdentity::new("subscription-family:detail-exact"),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        "detail_exact",
        vec![
            SubscriptionSliceKind::SignalField,
            SubscriptionSliceKind::SignalField,
        ],
        false,
    )
    .expect_err("duplicate slice kinds should reject");

    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::BuilderConfigurationConflict
    );
}
