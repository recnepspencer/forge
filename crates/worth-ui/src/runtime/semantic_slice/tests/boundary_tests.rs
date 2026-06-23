use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiProjectionFamily, WorthUiQueryOwnedSemanticSliceInventory, WorthUiRuntimeFactFamily,
    WorthUiSemanticCompileBoundary, WorthUiSemanticMeaningClass, WorthUiSemanticSliceFactMapping,
    WorthUiSemanticSliceId, WorthUiSemanticSliceInventory, WorthUiSemanticSliceOwner,
};

#[test]
fn query_owned_inventory_preserves_exact_runtime_fact_granularity() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let query_inventory = WorthUiQueryOwnedSemanticSliceInventory::current();

    assert!(query_inventory.audit_against_inventory(&inventory));

    let exact_families: BTreeSet<_> = query_inventory
        .slice_ids()
        .iter()
        .filter_map(|id| {
            inventory
                .slice(*id)
                .expect("query-owned semantic slice is registered")
                .runtime_fact_mapping()
                .exact_family()
        })
        .collect();
    let expected: BTreeSet<_> = [
        WorthUiRuntimeFactFamily::QueryBinding,
        WorthUiRuntimeFactFamily::LiveViewBinding,
        WorthUiRuntimeFactFamily::QueryResultPosture,
        WorthUiRuntimeFactFamily::QueryProjectionFact,
        WorthUiRuntimeFactFamily::QueryStateSnapshot,
        WorthUiRuntimeFactFamily::QueryEffectPosture,
        WorthUiRuntimeFactFamily::QueryRecoveryPosture,
        WorthUiRuntimeFactFamily::QueryInspectionTarget,
        WorthUiRuntimeFactFamily::VirtualizedDataFrame,
    ]
    .into_iter()
    .collect();

    assert_eq!(exact_families, expected);
    assert!(query_inventory.slice_ids().iter().all(|id| {
        inventory
            .slice(*id)
            .is_some_and(|descriptor| descriptor.must_preserve_upstream_granularity())
    }));

    for posture_slice in [
        WorthUiSemanticSliceId::QueryBindingPreservationPosture,
        WorthUiSemanticSliceId::QueryBindingRebindPosture,
        WorthUiSemanticSliceId::QueryBindingRetirementPosture,
    ] {
        let descriptor = inventory
            .slice(posture_slice)
            .expect("Query rebind posture slice is registered");
        assert_eq!(
            descriptor.owner(),
            WorthUiSemanticSliceOwner::QueryAuthority
        );
        assert_eq!(
            descriptor.runtime_fact_mapping(),
            WorthUiSemanticSliceFactMapping::Gap
        );
        assert!(descriptor.must_preserve_upstream_granularity());
        assert!(descriptor
            .consumers()
            .contains(WorthUiProjectionFamily::QueryProjectionConsumption));
    }
}

#[test]
fn platform_meaning_slices_remain_outside_authored_hot_reload_boundary() {
    let boundary = WorthUiSemanticCompileBoundary::current();

    for slice in [
        WorthUiSemanticSliceId::NewRustComponentImplementation,
        WorthUiSemanticSliceId::NewCapabilityFamilyDefinition,
        WorthUiSemanticSliceId::RuntimeSubsystemBehavior,
    ] {
        assert!(boundary.is_compile_required_platform_slice(slice));
        assert!(!boundary.is_hot_reloadable_product_slice(slice));
    }
}

#[test]
fn dropdown_mode_and_selected_state_are_independent_semantic_slices() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let mode = inventory
        .slice(WorthUiSemanticSliceId::CommandProjectionSelectionMode)
        .expect("selection-mode slice is registered");
    let selected = inventory
        .slice(WorthUiSemanticSliceId::DropdownSelectedState)
        .expect("selected-state slice is registered");

    assert_ne!(mode.id(), selected.id());
    assert_eq!(mode.owner(), WorthUiSemanticSliceOwner::CapabilityAuthority);
    assert_eq!(
        mode.runtime_fact_mapping(),
        WorthUiSemanticSliceFactMapping::Exact(WorthUiRuntimeFactFamily::InteractionPolicy)
    );
    assert_eq!(
        selected.owner(),
        WorthUiSemanticSliceOwner::RuntimeInteractionState
    );
    assert_eq!(
        selected.runtime_fact_mapping(),
        WorthUiSemanticSliceFactMapping::Exact(WorthUiRuntimeFactFamily::DropdownSelectionState)
    );
}

#[test]
fn content_slot_assignment_declares_narrow_composite_mapping() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let descriptor = inventory
        .slice(WorthUiSemanticSliceId::ContentSlotAssignment)
        .expect("content-slot slice is registered");

    assert_eq!(
        descriptor.owner(),
        WorthUiSemanticSliceOwner::AuthoredSource
    );
    assert_eq!(
        descriptor.meaning(),
        WorthUiSemanticMeaningClass::ProductMeaning
    );
    assert!(descriptor
        .runtime_fact_mapping()
        .contains_family(WorthUiRuntimeFactFamily::PageContentSlot));
    assert!(descriptor
        .runtime_fact_mapping()
        .contains_family(WorthUiRuntimeFactFamily::ContentMount));
    assert!(!descriptor
        .runtime_fact_mapping()
        .contains_family(WorthUiRuntimeFactFamily::ActiveArtifact));
    assert!(descriptor
        .consumers()
        .contains(WorthUiProjectionFamily::PageHost));
}

#[test]
fn authored_query_binding_shape_is_first_class_semantic_runtime_fact() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let descriptor = inventory
        .slice(WorthUiSemanticSliceId::AuthoredQueryBindingShape)
        .expect("authored Query-binding shape slice is registered");

    assert_eq!(
        descriptor.owner(),
        WorthUiSemanticSliceOwner::AuthoredSource
    );
    assert_eq!(
        descriptor.runtime_fact_mapping(),
        WorthUiSemanticSliceFactMapping::Exact(WorthUiRuntimeFactFamily::AuthoredQueryBindingShape)
    );
    assert!(descriptor
        .consumers()
        .contains(WorthUiProjectionFamily::QueryProjectionConsumption));
}

#[test]
fn product_meaning_surface_mount_differs_from_platform_meaning_component_implementation() {
    let inventory = WorthUiSemanticSliceInventory::current();
    let boundary = WorthUiSemanticCompileBoundary::current();
    let mount = inventory
        .slice(WorthUiSemanticSliceId::SurfaceMountTarget)
        .expect("surface-mount slice is registered");
    let component_impl = inventory
        .slice(WorthUiSemanticSliceId::NewRustComponentImplementation)
        .expect("new-Rust-component slice is registered");

    assert_eq!(mount.meaning(), WorthUiSemanticMeaningClass::ProductMeaning);
    assert_eq!(
        mount.runtime_fact_mapping(),
        WorthUiSemanticSliceFactMapping::Exact(WorthUiRuntimeFactFamily::SurfaceMount)
    );
    assert!(boundary.is_hot_reloadable_product_slice(mount.id()));
    assert_eq!(
        component_impl.meaning(),
        WorthUiSemanticMeaningClass::PlatformMeaning
    );
    assert!(boundary.is_compile_required_platform_slice(component_impl.id()));
}

#[test]
fn semantic_slice_inventory_declares_projection_consumers_for_runtime_rebind() {
    let inventory = WorthUiSemanticSliceInventory::current();

    let theme = inventory
        .slice(WorthUiSemanticSliceId::ThemeTokenValue)
        .expect("theme slice is registered");
    assert!(theme
        .consumers()
        .contains(WorthUiProjectionFamily::HeaderTheme));
    assert!(theme
        .consumers()
        .contains(WorthUiProjectionFamily::HeaderFrame));

    let appearance = inventory
        .slice(WorthUiSemanticSliceId::AppearanceField)
        .expect("appearance slice is registered");
    assert!(appearance
        .consumers()
        .contains(WorthUiProjectionFamily::HeaderAppearance));
    assert!(appearance
        .consumers()
        .contains(WorthUiProjectionFamily::Dropdown));

    let query_shape = inventory
        .slice(WorthUiSemanticSliceId::AuthoredQueryBindingShape)
        .expect("authored Query binding shape slice is registered");
    assert!(query_shape
        .consumers()
        .contains(WorthUiProjectionFamily::QueryProjectionConsumption));
}
