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
        vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
            "entity-1",
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid native subscription aspect key"),
            forge_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native subscription field key"),
            SubscriptionSliceKind::SignalField,
        )
        .expect("slice intent should validate")],
        family,
    )
    .expect("declaration should normalize");
    let right = BridgeSubscriptionDeclaration::new(
        BridgeSubscriptionDeclarationFamilyKind::DetailExact,
        BridgeSubscriptionDeliveryIntentClass::None,
        vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
            "entity-1",
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid native subscription aspect key"),
            forge_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native subscription field key"),
            SubscriptionSliceKind::SignalField,
        )
        .expect("slice intent should validate")],
        family,
    )
    .expect("declaration should normalize");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert!(left
        .canonical_basis()
        .contains("subscription-slice-target:sha256:"));
    assert!(!left.canonical_basis().contains("committed-patch-target"));
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
            NormalizedSubscriptionSliceIntent::try_new_entity_region(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate"),
            NormalizedSubscriptionSliceIntent::try_new_entity_partition(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
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
            NormalizedSubscriptionSliceIntent::try_new_entity_partition(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalPartition,
            )
            .expect("slice intent should validate"),
            NormalizedSubscriptionSliceIntent::try_new_entity_region(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
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
        vec![NormalizedSubscriptionSliceIntent::try_new_entity_region(
            "entity-1",
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid native subscription aspect key"),
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
        vec![NormalizedSubscriptionSliceIntent::try_new_entity_region(
            "entity-1",
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid native subscription aspect key"),
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
    let error = NormalizedSubscriptionSliceIntent::try_new_entity_field(
        "",
        forge_foundational::facade::AspectKey::new("profile")
            .expect("valid native subscription aspect key"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid native subscription field key"),
        SubscriptionSliceKind::SignalField,
    )
    .expect_err("empty entity identity should reject");

    assert_eq!(
        error.kind(),
        NormalizedSubscriptionSliceIntentErrorKind::EmptyEntityIdentity
    );
}

#[test]
fn slice_intent_rejects_missing_field_locator_for_field_target() {
    let aspect_locator = forge_foundational::facade::AspectLocator::new(
        forge_foundational::facade::LocatorAuthority::Authoritative,
        forge_foundational::facade::AspectKey::new("profile")
            .expect("valid native subscription aspect key"),
    );
    let error = NormalizedSubscriptionSliceIntent::try_new_native(
        "entity-1",
        aspect_locator,
        None,
        forge_foundational::facade::AspectMask::whole_aspect(),
        crate::mapping::TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
    )
    .expect_err("field targets must carry a foundational field locator");

    assert_eq!(
        error.kind(),
        NormalizedSubscriptionSliceIntentErrorKind::MissingFieldLocator
    );
}

#[test]
fn slice_intent_rejects_projection_mask_that_omits_field_target() {
    let aspect_locator = forge_foundational::facade::AspectLocator::new(
        forge_foundational::facade::LocatorAuthority::Authoritative,
        forge_foundational::facade::AspectKey::new("profile.name")
            .expect("valid native subscription aspect key"),
    );
    let field_locator = forge_foundational::facade::AspectFieldLocator::from_aspect(
        aspect_locator.clone(),
        forge_foundational::facade::CanonicalFieldPath::single(
            forge_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native subscription field key"),
        ),
    );
    let error = NormalizedSubscriptionSliceIntent::try_new_native(
        "entity-1",
        aspect_locator,
        Some(field_locator),
        forge_foundational::facade::AspectMask::whole_aspect(),
        crate::mapping::TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
    )
    .expect_err("field targets must carry a matching projection mask");

    assert_eq!(
        error.kind(),
        NormalizedSubscriptionSliceIntentErrorKind::ProjectionMaskTargetMismatch
    );
}

#[test]
fn slice_intent_target_basis_uses_committed_patch_target_proof() {
    let intent = NormalizedSubscriptionSliceIntent::try_new_entity_field(
        "entity-1",
        forge_foundational::facade::AspectKey::new("profile.name")
            .expect("valid native subscription aspect key"),
        forge_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid native subscription field key"),
        SubscriptionSliceKind::SignalField,
    )
    .expect("valid aspect key should admit");

    assert_eq!(intent.aspect_key().as_str(), "profile.name");
    assert_eq!(intent.aspect_key().as_str(), "profile.name");
    assert!(intent.field_locator().is_some());
    assert!(!intent.projection_mask().is_whole_aspect());
    assert!(intent
        .slice_target_identity()
        .as_str()
        .starts_with("subscription-slice-target:sha256:"));
    assert!(!intent
        .slice_target_identity()
        .as_str()
        .contains("committed-patch-target"));
    assert!(!intent.canonical_basis().contains("committed-patch-target"));
    assert_eq!(
        intent.native_target_basis(),
        "committed-patch-target|locator=version=bridge.committed-patch-target.v1;domain=locator;entries=[locus=named:aspect_field.aspect_key,kind=locator,value=exact-text:profile.name;locus=named:aspect_field.authority,kind=locator,value=exact-text:authoritative;locus=named:aspect_field.field_path,kind=locator,value=exact-text:name;locus=named:aspect_field.kind,kind=locator,value=exact-text:aspect]|mutation-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.name.mutation.field.name,kind=mask,value=exact-text:name]|projection-mask=version=bridge.committed-patch-target.v1;domain=aspect-mask;entries=[locus=named:profile.name.projection.field.name,kind=mask,value=exact-text:name]|kind=entity-field"
    );
}

#[test]
fn slice_intent_named_whole_target_constructors_cover_native_matrix() {
    let cases = [
        (
            NormalizedSubscriptionSliceIntent::try_new_entity_relation_endpoint(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalLens,
            )
            .expect("relation-endpoint slice intent should validate"),
            crate::mapping::TruthDeltaSurfaceKind::EntityRelationEndpoint,
            "entity-relation-endpoint",
        ),
        (
            NormalizedSubscriptionSliceIntent::try_new_entity_region(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("region slice intent should validate"),
            crate::mapping::TruthDeltaSurfaceKind::EntityRegion,
            "entity-region",
        ),
        (
            NormalizedSubscriptionSliceIntent::try_new_entity_partition(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalPartition,
            )
            .expect("partition slice intent should validate"),
            crate::mapping::TruthDeltaSurfaceKind::EntityPartition,
            "entity-partition",
        ),
        (
            NormalizedSubscriptionSliceIntent::try_new_entity_facet(
                "entity-1",
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native subscription aspect key"),
                SubscriptionSliceKind::SignalFacet,
            )
            .expect("facet slice intent should validate"),
            crate::mapping::TruthDeltaSurfaceKind::EntityFacet,
            "entity-facet",
        ),
    ];

    for (intent, expected_surface_kind, expected_target_kind) in cases {
        assert_eq!(intent.field_locator(), None);
        assert!(intent.projection_mask().is_whole_aspect());
        assert_eq!(intent.surface_kind(), expected_surface_kind);
        assert!(intent
            .slice_target_identity()
            .as_str()
            .starts_with("subscription-slice-target:sha256:"));
        assert!(intent
            .native_target_basis()
            .contains(&format!("|kind={expected_target_kind}")));
        assert!(intent
            .native_target_basis()
            .contains("profile.projection.whole,kind=mask,value=exact-text:whole"));
    }
}
