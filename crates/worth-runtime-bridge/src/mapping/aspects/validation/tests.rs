use super::{
    canonical_aspect_registration_order, registration_rank_group, validate_registration_set,
    validate_registration_values,
};
use crate::mapping::{
    AspectKeySelector, BridgeAspectRegistration, BridgeAspectRegistrationId, MappingSelector,
    SliceWideningPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind, TruthPatchScope,
};
use worth_foundational::facade::ScalarAspectType;

fn registration(
    id: &str,
    truth_scope: TruthPatchScope,
    truth_surface_kind: TruthDeltaSurfaceKind,
    subscription_slice_kind: SubscriptionSliceKind,
    widening_policy: SliceWideningPolicy,
) -> BridgeAspectRegistration {
    let snapshot_read_contract = declared_read_contract(truth_scope.aspect_selector());
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::admit_bridge_owned(id),
        truth_scope,
        snapshot_read_contract,
        truth_surface_kind,
        subscription_slice_kind,
        widening_policy,
    )
}

fn declared_read_contract(
    aspect_selector: &AspectKeySelector,
) -> crate::snapshot::SnapshotReadContract {
    let AspectKeySelector::Exact(aspect_key) = aspect_selector else {
        panic!("aspect registrations must declare an exact aspect read contract")
    };
    crate::snapshot::SnapshotReadContract::scalar(aspect_key.clone(), ScalarAspectType::String)
}

#[test]
fn freeze_accepts_empty_aspect_registry_for_incremental_rollout() {
    let registry = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(vec![])
        .expect("empty aspect registry should freeze");
    assert!(registry.registrations().is_empty());
}

#[test]
fn freeze_rejects_duplicate_registration_ids() {
    let truth_scope = TruthPatchScope::for_entity_field(
        MappingSelector::exact("user"),
        worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid native field key"),
    );
    let registrations = vec![
        registration(
            "id",
            truth_scope.clone(),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        registration(
            "id",
            truth_scope,
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
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
            TruthPatchScope::for_target(
                MappingSelector::exact("user"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                crate::facade::TruthPatchTargetSelector::any(),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        registration(
            "id-b",
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("user"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
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
    let truth_scope = TruthPatchScope::for_entity_field(
        MappingSelector::exact("user"),
        worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
        worth_foundational::facade::FieldKey::new("name".to_owned())
            .expect("valid native field key"),
    );
    let registrations = vec![
        registration(
            "id-a",
            truth_scope.clone(),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        registration(
            "id-b",
            truth_scope,
            TruthDeltaSurfaceKind::EntityFacet,
            SubscriptionSliceKind::SignalFacet,
            SliceWideningPolicy::Disallow,
        ),
    ];

    let registry = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(registrations)
        .expect("expected distinct truth surface kinds to freeze");
    assert_eq!(registry.registrations().len(), 2);
}

#[test]
fn freeze_rejects_invalid_entity_widening_target() {
    let registrations = vec![registration(
        "id-a",
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("user"),
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::RegisteredEntityCoarseWidening,
    )];

    let error = crate::mapping::aspects::FrozenAspectMappingRegistry::freeze(registrations)
        .expect_err("expected invalid widening target to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::InvalidFineGrainedWideningPolicy
    );
}

#[test]
fn freeze_canonicalizes_aspect_registration_order() {
    let mut registrations = vec![
        registration(
            "id-a",
            TruthPatchScope::new(
                MappingSelector::exact("user"),
                crate::facade::AspectKeySelector::exact(
                    worth_foundational::facade::AspectKey::new("profile")
                        .expect("valid native aspect key"),
                ),
                crate::facade::TruthPatchTargetSelector::any(),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        registration(
            "id-b",
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("user"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        registration(
            "id-c",
            TruthPatchScope::for_target(
                MappingSelector::exact("user"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                crate::facade::TruthPatchTargetSelector::any(),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
    ];
    let expected = vec![
        registrations[1].clone(),
        registrations[0].clone(),
        registrations[2].clone(),
    ];
    registrations.sort_by(canonical_aspect_registration_order);
    assert_eq!(registrations, expected);
}

#[test]
fn registration_rank_group_defines_widening_rank() {
    let registration = registration(
        "id-a",
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("user"),
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::RegisteredEntityCoarseWidening,
    );

    assert_eq!(
        registration_rank_group(&registration),
        (
            TruthDeltaSurfaceKind::EntityField,
            SliceWideningPolicy::RegisteredEntityCoarseWidening
        )
    );
}

#[test]
fn validate_registration_values_rejects_empty_selectors() {
    let registrations = vec![registration(
        "id-a",
        TruthPatchScope::for_entity_field(
            MappingSelector::exact(""),
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::Disallow,
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
        TruthPatchScope::for_entity_field(
            MappingSelector::exact("user"),
            worth_foundational::facade::AspectKey::new("profile").expect("valid native aspect key"),
            worth_foundational::facade::FieldKey::new("name".to_owned())
                .expect("valid native field key"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceWideningPolicy::Disallow,
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
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("user"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        registration(
            "id-b",
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("user"),
                worth_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                worth_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
    ];

    let error = validate_registration_set(&registrations)
        .expect_err("expected semantic duplicates to fail");
    assert_eq!(
        error.kind(),
        crate::error::BridgeBuildErrorKind::DuplicateAspectRegistration
    );
}
