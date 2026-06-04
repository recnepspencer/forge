use super::FrozenMappingRegistry;
use crate::error::{BridgeBuildErrorKind, BridgeMappingFreezeContext};
use crate::mapping::lookup::BridgeMappingLookupKey;
use crate::mapping::{
    AspectKeySelector, BridgeMappingId, BridgeMappingRegistration, BridgeMappingWideningClass,
    CoarseRoutingMode, MappingSelector, SignalInvalidationScope, TruthPatchScope,
    TruthPatchTargetSelector,
};
use forge_foundational::facade::{AspectKey, FieldKey, ScalarAspectType};
mod registration_identity;
mod target_selector_basis;

fn registration(
    mapping_id: &str,
    entity: MappingSelector,
    aspect: AspectKeySelector,
    target: TruthPatchTargetSelector,
    signal_scope: &str,
) -> BridgeMappingRegistration {
    let snapshot_read_contract = declared_read_contract(&aspect);
    BridgeMappingRegistration::new(
        BridgeMappingId::new(mapping_id),
        TruthPatchScope::new(entity, aspect, target),
        snapshot_read_contract,
        SignalInvalidationScope::new(signal_scope),
        CoarseRoutingMode::Direct,
    )
}

fn declared_read_contract(aspect: &AspectKeySelector) -> crate::snapshot::SnapshotReadContract {
    let AspectKeySelector::Exact(aspect_key) = aspect else {
        panic!("mapping registrations must declare an exact aspect read contract")
    };
    crate::snapshot::SnapshotReadContract::scalar(aspect_key.clone(), ScalarAspectType::String)
}

fn aspect(value: &str) -> AspectKeySelector {
    AspectKeySelector::exact(AspectKey::new(value).expect("valid native aspect key"))
}

fn field(value: &str) -> TruthPatchTargetSelector {
    TruthPatchTargetSelector::entity_field(
        FieldKey::new(value.to_owned()).expect("valid native field key"),
    )
}

fn any_target() -> TruthPatchTargetSelector {
    TruthPatchTargetSelector::any()
}

fn lookup_key<'a>(
    entity: &'a str,
    aspect_key: &'a AspectKey,
    target: &'a TruthPatchTargetSelector,
) -> BridgeMappingLookupKey<'a> {
    match target {
        TruthPatchTargetSelector::EntityField(path) => BridgeMappingLookupKey::new(
            entity,
            aspect_key,
            Some(path),
            crate::mapping::TruthDeltaSurfaceKind::EntityField,
        ),
        TruthPatchTargetSelector::EntityRegion => BridgeMappingLookupKey::new(
            entity,
            aspect_key,
            None,
            crate::mapping::TruthDeltaSurfaceKind::EntityRegion,
        ),
        TruthPatchTargetSelector::EntityPartition => BridgeMappingLookupKey::new(
            entity,
            aspect_key,
            None,
            crate::mapping::TruthDeltaSurfaceKind::EntityPartition,
        ),
        TruthPatchTargetSelector::EntityRelationEndpoint => BridgeMappingLookupKey::new(
            entity,
            aspect_key,
            None,
            crate::mapping::TruthDeltaSurfaceKind::EntityRelationEndpoint,
        ),
        TruthPatchTargetSelector::EntityFacet => BridgeMappingLookupKey::new(
            entity,
            aspect_key,
            None,
            crate::mapping::TruthDeltaSurfaceKind::EntityFacet,
        ),
        TruthPatchTargetSelector::Any => panic!("lookup keys must use exact native targets"),
    }
}

fn freeze_context(error: &crate::error::BridgeBuildError) -> &BridgeMappingFreezeContext {
    error
        .context()
        .mapping_freeze_context()
        .expect("mapping freeze denial should retain typed freeze context")
}

fn sorted_context_mapping_id_assertion_pair(context: &BridgeMappingFreezeContext) -> [&str; 2] {
    let mut mapping_ids = [
        context
            .mapping_id()
            .expect("primary mapping id should be retained")
            .as_str(),
        context
            .conflicting_mapping_id()
            .expect("conflicting mapping id should be retained")
            .as_str(),
    ];
    mapping_ids.sort_unstable();
    mapping_ids
}

#[test]
fn freeze_rejects_missing_registrations() {
    let error = FrozenMappingRegistry::freeze(vec![])
        .expect_err("registry freeze should fail without registrations");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::MissingMappingRegistrations
    );
}

#[test]
fn freeze_rejects_duplicate_semantic_registrations() {
    let error = FrozenMappingRegistry::freeze(vec![
        registration(
            "alpha",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.user.profile",
        ),
        registration(
            "beta",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.user.profile",
        ),
    ])
    .expect_err("duplicate semantic registrations must fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::DuplicateMappingRegistration
    );
    let context = freeze_context(&error);
    assert_eq!(
        sorted_context_mapping_id_assertion_pair(context),
        ["alpha", "beta"]
    );
}

#[test]
fn freeze_rejects_duplicate_mapping_ids_even_when_scopes_differ() {
    let error = FrozenMappingRegistry::freeze(vec![
        registration(
            "shared-id",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.user.profile.name",
        ),
        registration(
            "shared-id",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("avatar"),
            "signal.user.profile.avatar",
        ),
    ])
    .expect_err("duplicate mapping ids must fail even when scopes differ");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::DuplicateMappingRegistration
    );
    let context = freeze_context(&error);
    assert_eq!(
        context
            .mapping_id()
            .expect("primary mapping id should be retained")
            .as_str(),
        "shared-id"
    );
    assert_eq!(
        context
            .conflicting_mapping_id()
            .expect("conflicting mapping id should be retained")
            .as_str(),
        "shared-id"
    );
}

#[test]
fn freeze_rejects_same_rank_overlap() {
    let error = FrozenMappingRegistry::freeze(vec![
        registration(
            "entity-wide",
            MappingSelector::exact("user"),
            aspect("profile"),
            any_target(),
            "signal.entity-wide",
        ),
        registration(
            "aspect-wide",
            MappingSelector::any(),
            aspect("profile"),
            field("name"),
            "signal.aspect-wide",
        ),
    ])
    .expect_err("same-rank overlapping registrations must fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::AmbiguousMappingRegistration
    );
    let context = freeze_context(&error);
    assert_eq!(
        sorted_context_mapping_id_assertion_pair(context),
        ["aspect-wide", "entity-wide"]
    );
}

#[test]
fn freeze_allows_identical_truth_scope_for_distinct_signal_targets() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "steel-bicycle",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            field("usd"),
            "price:bicycle",
        ),
        registration(
            "steel-scooter",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            field("usd"),
            "price:scooter",
        ),
    ])
    .expect("identical truth scopes should fan out when signal targets differ");

    let ordered_ids: Vec<_> = registry
        .registrations()
        .iter()
        .map(|registration| registration.mapping_id().as_str())
        .collect();
    assert_eq!(ordered_ids, vec!["steel-bicycle", "steel-scooter"]);
}

#[test]
fn freeze_canonicalizes_iteration_order() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "widening",
            MappingSelector::exact("user"),
            aspect("profile"),
            any_target(),
            "signal.surface-widening",
        ),
        registration(
            "exact",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.exact",
        ),
        registration(
            "broad",
            MappingSelector::any(),
            aspect("profile"),
            any_target(),
            "signal.broad",
        ),
    ])
    .expect("registry freeze should succeed");

    let ordered_ids: Vec<_> = registry
        .registrations()
        .iter()
        .map(|registration| registration.mapping_id().as_str())
        .collect();

    assert_eq!(ordered_ids, vec!["exact", "widening", "broad"]);
}

#[test]
fn lookup_prefers_more_specific_match_before_widening() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "widening",
            MappingSelector::exact("user"),
            aspect("profile"),
            any_target(),
            "signal.surface-widening",
        ),
        registration(
            "exact",
            MappingSelector::exact("user"),
            aspect("profile"),
            field("name"),
            "signal.exact",
        ),
    ])
    .expect("registry freeze should succeed");

    let profile = AspectKey::new("profile").expect("valid native aspect key");
    match registry.lookup(lookup_key("user", &profile, &field("name"))) {
        crate::mapping::BridgeMappingLookup::Exact { resolved } => {
            let matched_ids = resolved
                .registrations()
                .map(|registration| registration.mapping_id().as_str())
                .collect::<Vec<_>>();
            assert_eq!(matched_ids, vec!["exact"]);
        }
        other => panic!("expected exact match, found {other:?}"),
    }

    match registry.lookup(lookup_key("user", &profile, &field("avatar"))) {
        crate::mapping::BridgeMappingLookup::Widening { resolved } => {
            let matched = resolved.registrations().collect::<Vec<_>>();
            assert_eq!(matched.len(), 1);
            assert_eq!(matched[0].mapping_id().as_str(), "widening");
            assert_eq!(
                matched[0].widening_class(),
                Some(BridgeMappingWideningClass::Surface)
            );
        }
        other => panic!("expected widening match, found {other:?}"),
    }
}

#[test]
fn lookup_returns_all_fanout_matches_at_the_most_specific_scope() {
    let registry = FrozenMappingRegistry::freeze(vec![
        registration(
            "widening",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            any_target(),
            "price:widening",
        ),
        registration(
            "bicycle",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            field("usd"),
            "price:bicycle",
        ),
        registration(
            "wheelbarrow",
            MappingSelector::exact("component:steel"),
            aspect("cost"),
            field("usd"),
            "price:wheelbarrow",
        ),
    ])
    .expect("fanout and specificity should coexist");

    let cost = AspectKey::new("cost").expect("valid native aspect key");
    match registry.lookup(lookup_key("component:steel", &cost, &field("usd"))) {
        crate::mapping::BridgeMappingLookup::Exact { resolved } => {
            let matched_ids = resolved
                .registrations()
                .map(|registration| registration.mapping_id().as_str())
                .collect::<Vec<_>>();
            assert_eq!(matched_ids, vec!["bicycle", "wheelbarrow"]);
        }
        other => panic!("expected exact fanout match, found {other:?}"),
    }
}

#[test]
fn freeze_rejects_empty_mapping_values() {
    let error = FrozenMappingRegistry::freeze(vec![registration(
        " ",
        MappingSelector::exact("user"),
        aspect("profile"),
        field("name"),
        "signal.valid",
    )])
    .expect_err("empty mapping identifiers must fail");

    assert_eq!(
        error.kind(),
        BridgeBuildErrorKind::AmbiguousMappingRegistration
    );
    let context = freeze_context(&error);
    assert_eq!(
        context
            .mapping_id()
            .expect("invalid mapping id should be retained")
            .as_str(),
        " "
    );
    assert_eq!(context.invalid_field(), Some("mapping id"));
}
