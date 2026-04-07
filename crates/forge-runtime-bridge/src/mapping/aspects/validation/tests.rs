use super::{
    canonical_aspect_registration_order, registration_rank_group, validate_registration_set,
    validate_registration_values,
};
use crate::mapping::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, MappingSelector, SliceFallbackPolicy,
    SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};

fn registration(
    id: &str,
    truth_scope: TruthPatchScope,
    truth_surface_kind: TruthDeltaSurfaceKind,
    subscription_slice_kind: SubscriptionSliceKind,
    fallback_policy: SliceFallbackPolicy,
) -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new(id),
        truth_scope,
        truth_surface_kind,
        subscription_slice_kind,
        fallback_policy,
    )
}

#[test]
fn freeze_accepts_empty_aspect_registry_for_incremental_rollout() {
    let registry = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(vec![])
        .expect("empty aspect registry should freeze");
    assert!(registry.registrations().is_empty());
}

#[test]
fn freeze_rejects_duplicate_registration_ids() {
    let truth_scope = TruthPatchScope::new(
        MappingSelector::exact("user"),
        MappingSelector::exact("profile"),
        MappingSelector::exact("name"),
    );
    let registrations = vec![
        registration(
            "id",
            truth_scope.clone(),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
        registration(
            "id",
            truth_scope,
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
    ];

    let error = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(registrations)
        .expect_err("expected duplicate registration id to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::DuplicateAspectRegistration
    );
}

#[test]
fn freeze_rejects_same_rank_overlap_for_same_surface_kind() {
    let registrations = vec![
        registration(
            "id-a",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::any(),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
        registration(
            "id-b",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
    ];

    let error = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(registrations)
        .expect_err("expected ambiguous registrations to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::AmbiguousAspectRegistration
    );
}

#[test]
fn freeze_allows_same_scope_for_different_surface_kinds() {
    let truth_scope = TruthPatchScope::new(
        MappingSelector::exact("user"),
        MappingSelector::exact("profile"),
        MappingSelector::exact("name"),
    );
    let registrations = vec![
        registration(
            "id-a",
            truth_scope.clone(),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
        registration(
            "id-b",
            truth_scope,
            TruthDeltaSurfaceKind::EntityFacet,
            SubscriptionSliceKind::SignalFacet,
            SliceFallbackPolicy::Disallow,
        ),
    ];

    let registry = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(registrations)
        .expect("expected distinct truth surface kinds to freeze");
    assert_eq!(registry.registrations().len(), 2);
}

#[test]
fn freeze_rejects_invalid_entity_fallback_target() {
    let registrations = vec![registration(
        "id-a",
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::RegisteredEntityCoarseFallback,
    )];

    let error = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(registrations)
        .expect_err("expected invalid fallback target to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::InvalidFineGrainedFallbackPolicy
    );
}

#[test]
fn freeze_canonicalizes_aspect_registration_order() {
    let mut registrations = vec![
        registration(
            "id-a",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::any(),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
        registration(
            "id-b",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
        registration(
            "id-c",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::any(),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
    ];
    let expected = registrations.clone();
    registrations.sort_by(canonical_aspect_registration_order);
    assert_eq!(registrations, expected);
}

#[test]
fn registration_rank_group_defines_fallback_rank() {
    let registration = registration(
        "id-a",
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::RegisteredEntityCoarseFallback,
    );

    assert_eq!(
        registration_rank_group(&registration),
        (
            TruthDeltaSurfaceKind::EntityField,
            SliceFallbackPolicy::RegisteredEntityCoarseFallback
        )
    );
}

#[test]
fn validate_registration_values_rejects_empty_selectors() {
    let registrations = vec![registration(
        "id-a",
        TruthPatchScope::new(
            MappingSelector::exact(""),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )];

    let error =
        validate_registration_values(&registrations).expect_err("expected empty selector to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::MissingMappingRegistrations
    );
}

#[test]
fn validate_registration_values_rejects_empty_identity_bearing_fields() {
    let registrations = vec![registration(
        "",
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )];

    let error = validate_registration_values(&registrations)
        .expect_err("expected empty registration id to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::MissingMappingRegistrations
    );
}

#[test]
fn validate_registration_set_rejects_semantic_duplicates() {
    let registrations = vec![
        registration(
            "id-a",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
        registration(
            "id-b",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceFallbackPolicy::Disallow,
        ),
    ];

    let error = validate_registration_set(&registrations)
        .expect_err("expected semantic duplicates to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::DuplicateAspectRegistration
    );
}
