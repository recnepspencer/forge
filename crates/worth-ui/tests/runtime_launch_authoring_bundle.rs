use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence,
    MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfaceDescriptor, SurfaceId,
    SurfaceKind, SurfacePlacementClass, SurfaceStateClass, WorthUi, WorthUiApp,
    WorthUiRuntimeSourceModule,
};

#[test]
fn prepare_authoring_for_returns_layout_topology_from_same_source_package_as_runtime_launch() {
    let app = public_facade_content_slot_app();
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
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
                title "Products"
                runtime ProductsRuntime
                layout ProductsLayout
                content ProductsContent
            }

            runtime ProductsRuntime {}
            appearance ShopifyAdminTheme {}

            layout ProductsLayout {
                column {
                    row height fit { slot toolbar }
                    row height fill scroll_owner { slot collection }
                }
            }

            content ProductsContent {
                collection -> validation.surface.products.collection
                toolbar -> validation.surface.products.toolbar
            }
            "#,
        ))
        .prepare_authoring_for(&app)
        .expect("authoring bundle should prepare");

    let products_page = prepared
        .layout_topology()
        .page("ProductsPage")
        .expect("prepared authoring should expose page topology");
    assert_eq!(products_page.layout_name(), "ProductsLayout");
    assert!(!products_page.dynamic_template());

    let content_slots = prepared
        .content_slots()
        .page("ProductsPage")
        .expect("prepared authoring should expose page content slots");
    let mounted: Vec<_> = content_slots
        .assignments()
        .iter()
        .map(|assignment| (assignment.slot_name(), assignment.surface_id()))
        .collect();
    assert_eq!(
        mounted,
        vec![
            ("toolbar", "validation.surface.products.toolbar"),
            ("collection", "validation.surface.products.collection"),
        ],
        "public facade content slots must follow layout order, not content declaration order"
    );

    let runtime = app
        .launch_runtime(prepared.into_runtime_launch())
        .expect("prepared authoring bundle should still launch a runtime");
    assert_ne!(runtime.inspect_active().artifact_digest(), 0);
    assert_ne!(runtime.inspect_active().active_plan_digest(), 0);
}

fn public_facade_content_slot_app() -> WorthUiApp {
    WorthUi::app()
        .register_component(validation_component())
        .register_surface(validation_surface("validation.surface.products.toolbar"))
        .register_surface(validation_surface("validation.surface.products.collection"))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.column",
            MosaicRegionRole::stack(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.row",
            MosaicRegionRole::split(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.slot",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
        .freeze()
}

fn validation_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.sample").expect("valid component id"),
        ComponentPropSchema::named("validation.sample.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn validation_surface(surface_id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(surface_id).expect("valid surface id"),
        SurfaceKind::primary_content(),
        ComponentId::new("validation.component.sample").expect("valid component id"),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn layout_region(
    region_id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new(region_id).expect("valid region id"),
        role,
    )
    .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(MosaicScrollOwnership::region_owned())
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(child_rule)
    .with_allowed_surface_class(SurfacePlacementClass::primary_region())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}
