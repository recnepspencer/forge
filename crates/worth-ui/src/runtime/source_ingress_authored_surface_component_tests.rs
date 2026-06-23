use super::source_ingress_authored_delta_test_support::{
    declaration_rows, prepare_validation_reload, runtime_for_source, semantic_fact_family_rows,
};
use crate::runtime::{
    WorthUiAuthoredDeclarationKind, WorthUiAuthoredDeltaChangePosture, WorthUiSemanticSliceId,
    WorthUiValidationReloadStatus,
};

#[test]
fn surface_component_edit_emits_authored_mount_component_changed_fact_for_same_surface() {
    let app = authored_surface_test_app();
    let runtime = runtime_for_source(&app, baseline_source());
    let prepared = prepare_validation_reload(&runtime, changed_surface_component_source());
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("surface component edit should emit changed-fact proof");

    assert_eq!(
        prepared.evidence().status(),
        WorthUiValidationReloadStatus::ReadyForFrameBoundary
    );
    assert_eq!(
        declaration_rows(receipt.authored_delta_summary()),
        std::collections::BTreeSet::from([(
            WorthUiAuthoredDeclarationKind::Surface,
            "validation.surface.products.collection".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
        )])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        std::collections::BTreeSet::from([(
            WorthUiSemanticSliceId::AuthoredMountComponentSelection,
            "surface:validation.surface.products.collection".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            vec![crate::runtime::WorthUiRuntimeFactFamily::AuthoredMountComponentSelection],
        )])
    );
}

#[test]
fn surface_prop_edit_emits_authored_surface_props_changed_fact_for_same_surface() {
    let app = authored_surface_test_app();
    let runtime = runtime_for_source(&app, baseline_source());
    let prepared = prepare_validation_reload(&runtime, changed_surface_props_source());
    let receipt = prepared
        .changed_fact_mapping_receipt()
        .expect("surface prop edit should emit changed-fact proof");

    assert_eq!(
        declaration_rows(receipt.authored_delta_summary()),
        std::collections::BTreeSet::from([(
            WorthUiAuthoredDeclarationKind::Surface,
            "validation.surface.products.collection".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
        )])
    );
    assert_eq!(
        semantic_fact_family_rows(receipt),
        std::collections::BTreeSet::from([(
            WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
            "surface:validation.surface.products.collection".to_owned(),
            WorthUiAuthoredDeltaChangePosture::Changed,
            vec![crate::runtime::WorthUiRuntimeFactFamily::AuthoredSurfaceProps],
        )])
    );
}

#[test]
fn active_authoring_snapshot_preserves_surface_component_selection() {
    let app = authored_surface_test_app();
    let runtime = runtime_for_source(&app, changed_surface_component_source());
    let component_id = runtime
        .active_authoring_snapshot()
        .expect("source-authored runtime exposes active authoring snapshot")
        .authored_surfaces()
        .component_id_for_surface("validation.surface.products.collection");

    assert_eq!(component_id, Some("workspace.component.product_list.alt"));
}

#[test]
fn active_authoring_snapshot_preserves_surface_props() {
    let app = authored_surface_test_app();
    let runtime = runtime_for_source(&app, changed_surface_props_source());
    let snapshot = runtime
        .active_authoring_snapshot()
        .expect("source-authored runtime exposes active authoring snapshot");

    assert_eq!(
        snapshot
            .authored_surface_props()
            .string_prop("validation.surface.products.collection", "title"),
        Some("Updated collection")
    );
}

fn authored_surface_test_app() -> crate::facade::WorthUiApp {
    crate::facade::WorthUi::app()
        .register_component(component("workspace.component.product_list"))
        .register_component(component("workspace.component.product_list.alt"))
        .register_surface(surface("validation.surface.products.collection"))
        .register_surface(surface("validation.surface.orders.collection"))
        .register_mosaic_region_kind(region(
            "worth.ui.layout.column",
            crate::facade::MosaicRegionRole::stack(),
            crate::facade::MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(region(
            "worth.ui.layout.row",
            crate::facade::MosaicRegionRole::split(),
            crate::facade::MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(region(
            "worth.ui.layout.slot",
            crate::facade::MosaicRegionRole::primary(),
            crate::facade::MosaicChildRule::accepts_surfaces(),
        ))
        .freeze()
}

fn component(id: &str) -> crate::facade::ComponentDescriptor {
    crate::facade::ComponentDescriptor::new(
        crate::facade::ComponentId::new(id).unwrap(),
        crate::facade::ComponentPropSchema::named("workspace.props"),
        crate::facade::ComponentChildPolicy::no_children(),
        crate::facade::ComponentStateOwnership::runtime_owned(),
    )
}

fn surface(id: &str) -> crate::facade::SurfaceDescriptor {
    crate::facade::SurfaceDescriptor::new(
        crate::facade::SurfaceId::new(id).unwrap(),
        crate::facade::SurfaceKind::primary_content(),
        crate::facade::ComponentId::new("workspace.component.product_list").unwrap(),
        crate::facade::SurfacePlacementClass::primary_region(),
        crate::facade::SurfaceStateClass::restorable(),
    )
}

fn region(
    id: &str,
    role: crate::facade::MosaicRegionRole,
    child_rule: crate::facade::MosaicChildRule,
) -> crate::facade::MosaicRegionKindDescriptor {
    crate::facade::MosaicRegionKindDescriptor::new(
        crate::facade::MosaicRegionKindId::new(id).unwrap(),
        role,
    )
    .with_persistence(crate::facade::MosaicRegionPersistence::restorable())
    .with_sizing_behavior(crate::facade::MosaicSizingBehavior::fills_available_space())
    .with_focus_scope(crate::facade::MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(child_rule)
    .with_allowed_surface_class(crate::facade::SurfacePlacementClass::primary_region())
    .with_scroll_ownership(crate::facade::MosaicScrollOwnership::region_owned())
    .with_clipping(crate::facade::MosaicClippingPosture::clip_to_region())
    .with_hit_test(crate::facade::MosaicHitTestPosture::participates())
}

fn baseline_source() -> &'static str {
    r#"
    app ShopifyAdminApp {
        theme ShopifyAdminTheme
        workspace AdminWorkspace
    }

    workspace AdminWorkspace {
        shell {
            topbar AdminTopbar
            rail AdminPrimaryRail
            page_host AdminPageHost
            inspector AdminInspectorDock
            status AdminStatusBar
            overlays [CommandPaletteOverlay]
            toasts AdminToastCenter
        }
        pages [ProductsPage]
    }

    page ProductsPage {
        runtime ProductsRuntime
        layout ProductsLayout
        content ProductsContent
    }

    runtime ProductsRuntime {}
    appearance ShopifyAdminTheme {}

    surface validation.surface.products.collection {
        component workspace.component.product_list
        title "Products collection"
    }

    layout ProductsLayout {
        column {
            row height fill scroll_owner { slot collection }
        }
    }

    content ProductsContent {
        collection -> validation.surface.products.collection
    }
    "#
}

fn changed_surface_component_source() -> &'static str {
    r#"
    app ShopifyAdminApp {
        theme ShopifyAdminTheme
        workspace AdminWorkspace
    }

    workspace AdminWorkspace {
        shell {
            topbar AdminTopbar
            rail AdminPrimaryRail
            page_host AdminPageHost
            inspector AdminInspectorDock
            status AdminStatusBar
            overlays [CommandPaletteOverlay]
            toasts AdminToastCenter
        }
        pages [ProductsPage]
    }

    page ProductsPage {
        runtime ProductsRuntime
        layout ProductsLayout
        content ProductsContent
    }

    runtime ProductsRuntime {}
    appearance ShopifyAdminTheme {}

    surface validation.surface.products.collection {
        component workspace.component.product_list.alt
        title "Products collection"
    }

    layout ProductsLayout {
        column {
            row height fill scroll_owner { slot collection }
        }
    }

    content ProductsContent {
        collection -> validation.surface.products.collection
    }
    "#
}

fn changed_surface_props_source() -> &'static str {
    r#"
    app ShopifyAdminApp {
        theme ShopifyAdminTheme
        workspace AdminWorkspace
    }

    workspace AdminWorkspace {
        shell {
            topbar AdminTopbar
            rail AdminPrimaryRail
            page_host AdminPageHost
            inspector AdminInspectorDock
            status AdminStatusBar
            overlays [CommandPaletteOverlay]
            toasts AdminToastCenter
        }
        pages [ProductsPage]
    }

    page ProductsPage {
        runtime ProductsRuntime
        layout ProductsLayout
        content ProductsContent
    }

    runtime ProductsRuntime {}
    appearance ShopifyAdminTheme {}

    surface validation.surface.products.collection {
        component workspace.component.product_list
        title "Updated collection"
        summary "Runtime-owned authored props"
    }

    layout ProductsLayout {
        column {
            row height fill scroll_owner { slot collection }
        }
    }

    content ProductsContent {
        collection -> validation.surface.products.collection
    }
    "#
}
