use crate::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence,
    MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfaceDescriptor, SurfaceId,
    SurfaceKind, SurfacePlacementClass, SurfaceStateClass, WorthUi, WorthUiApp,
    WorthUiRuntimeSourceModule,
};
use crate::runtime::{
    WorthUiRuntimeHost, WorthUiValidationReloadRequest, WorthUiValidationReloadStatus,
};

#[test]
fn source_declaration_reorder_preserves_authoring_snapshot_meaning() {
    let app = snapshot_test_app();
    let canonical = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let reordered = runtime_for_source(
        &app,
        reordered_source_text("validation.surface.products.collection"),
    );

    assert_eq!(
        active_snapshot_digest(&canonical),
        active_snapshot_digest(&reordered),
        "authoring snapshot digest must ignore source declaration order noise"
    );
}

#[test]
fn invalid_validation_reload_preserves_active_authoring_snapshot() {
    let app = snapshot_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let before = active_authoring_truth(&runtime);
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module("app/main.wui", "import ;"),
    );

    assert_eq!(
        prepared.evidence().status(),
        WorthUiValidationReloadStatus::Denied(
            crate::runtime::WorthUiValidationReloadStage::CandidateSubmission
        )
    );
    assert_eq!(
        prepared
            .evidence()
            .active_authoring_snapshot_digest_before(),
        Some(before.authoring_snapshot_digest)
    );
    assert_eq!(
        prepared.evidence().active_authoring_snapshot_digest_after(),
        Some(before.authoring_snapshot_digest)
    );
    assert_eq!(
        prepared.evidence().candidate_authoring_snapshot_digest(),
        None
    );
    assert_eq!(active_authoring_truth(&runtime), before);
}

#[test]
fn activated_validation_reload_swaps_authoring_snapshot_with_runtime_truth() {
    let app = snapshot_test_app();
    let mut runtime =
        runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let before = active_authoring_truth(&runtime);
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            source_text("validation.surface.orders.collection"),
        ),
    );

    assert!(
        prepared.is_ready(),
        "changed source should prepare activation"
    );
    let evidence = prepared.activate(&mut runtime).expect("reload activates");

    assert_eq!(evidence.status(), WorthUiValidationReloadStatus::Activated);
    assert_eq!(
        evidence.active_authoring_snapshot_digest_before(),
        Some(before.authoring_snapshot_digest)
    );
    assert_eq!(
        evidence.candidate_authoring_snapshot_digest(),
        evidence.active_authoring_snapshot_digest_after()
    );
    let after = active_authoring_truth(&runtime);
    assert_eq!(
        evidence.active_artifact_digest_after(),
        after.artifact_digest
    );
    assert_eq!(
        evidence.active_plan_digest_after(),
        after.active_plan_digest
    );
    assert_eq!(
        evidence.active_authoring_snapshot_digest_after(),
        Some(after.authoring_snapshot_digest)
    );
    assert_ne!(after, before, "activation must swap all active truths");
    assert!(runtime
        .active_authoring_snapshot()
        .expect("active source snapshot")
        .content_slots()
        .page("ProductsPage")
        .expect("products page content")
        .assignment_for_slot("collection")
        .is_some_and(
            |assignment| assignment.surface_id() == "validation.surface.orders.collection"
        ));
}

#[test]
fn equivalent_validation_reload_keeps_active_authoring_snapshot_and_records_candidate_meaning() {
    let app = snapshot_test_app();
    let runtime = runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let before = active_authoring_truth(&runtime);
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            reordered_source_text("validation.surface.products.collection"),
        ),
    );

    assert_eq!(
        prepared.evidence().status(),
        WorthUiValidationReloadStatus::EquivalentNoOp
    );
    assert!(
        !prepared.is_ready(),
        "equivalent source must not carry an activatable replacement"
    );
    assert_eq!(
        prepared
            .evidence()
            .active_authoring_snapshot_digest_before(),
        Some(before.authoring_snapshot_digest)
    );
    assert_eq!(
        prepared.evidence().candidate_authoring_snapshot_digest(),
        Some(before.authoring_snapshot_digest),
        "equivalent reload still records the lowered candidate meaning"
    );
    assert_eq!(
        prepared.evidence().active_authoring_snapshot_digest_after(),
        Some(before.authoring_snapshot_digest)
    );
    assert_eq!(active_authoring_truth(&runtime), before);
}

#[test]
fn stale_prepared_reload_cannot_promote_old_candidate_authoring_snapshot() {
    let app = snapshot_test_app();
    let mut runtime =
        runtime_for_source(&app, source_text("validation.surface.products.collection"));
    let stale_prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            source_text("validation.surface.orders.collection"),
        ),
    );
    let current_prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module(
            "app/main.wui",
            source_text("validation.surface.customers.collection"),
        ),
    );

    current_prepared
        .activate(&mut runtime)
        .expect("current reload activates");
    let current_truth = active_authoring_truth(&runtime);
    let stale_result = stale_prepared.activate(&mut runtime);

    assert_eq!(
        stale_result,
        Err(crate::runtime::WorthUiValidationReloadStage::PlanSwap)
    );
    assert_eq!(
        active_authoring_truth(&runtime),
        current_truth,
        "failed stale activation must not promote its old candidate snapshot"
    );
    assert!(runtime
        .active_authoring_snapshot()
        .expect("active source snapshot")
        .content_slots()
        .page("ProductsPage")
        .expect("products page content")
        .assignment_for_slot("collection")
        .is_some_and(
            |assignment| assignment.surface_id() == "validation.surface.customers.collection"
        ));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ActiveAuthoringTruth {
    artifact_digest: u64,
    active_plan_digest: u64,
    authoring_snapshot_digest: u64,
}

fn active_authoring_truth(runtime: &WorthUiRuntimeHost) -> ActiveAuthoringTruth {
    let active = runtime.inspect_active();
    ActiveAuthoringTruth {
        artifact_digest: active.artifact_digest(),
        active_plan_digest: active.active_plan_digest(),
        authoring_snapshot_digest: active_snapshot_digest(runtime),
    }
}

fn active_snapshot_digest(runtime: &WorthUiRuntimeHost) -> u64 {
    runtime
        .active_authoring_snapshot()
        .expect("source-authored runtime exposes active authoring snapshot")
        .digest()
        .as_u64()
}

fn runtime_for_source(app: &WorthUiApp, source: impl Into<String>) -> WorthUiRuntimeHost {
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new("app/main.wui", source))
        .prepare_authoring_for(app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}

fn snapshot_test_app() -> WorthUiApp {
    WorthUi::app()
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.product_list").unwrap(),
            ComponentPropSchema::named("workspace.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_surface(validation_surface("validation.surface.products.collection"))
        .register_surface(validation_surface("validation.surface.orders.collection"))
        .register_surface(validation_surface(
            "validation.surface.customers.collection",
        ))
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

fn validation_surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new("workspace.component.product_list").unwrap(),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn layout_region(
    id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(MosaicRegionKindId::new(id).unwrap(), role)
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}

fn source_text(collection_surface: &str) -> String {
    format!(
        r#"
        app ShopifyAdminApp {{
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }}

        workspace AdminWorkspace {{
            shell {{
                topbar AdminTopbar
                rail AdminPrimaryRail
                page_host AdminPageHost
                inspector AdminInspectorDock
                status AdminStatusBar
                overlays [CommandPaletteOverlay]
                toasts AdminToastCenter
            }}
            pages [ProductsPage]
        }}

        page ProductsPage {{
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }}

        runtime ProductsRuntime {{}}
        appearance ShopifyAdminTheme {{}}

        layout ProductsLayout {{
            column {{
                row height fill scroll_owner {{ slot collection }}
            }}
        }}

        content ProductsContent {{
            collection -> {collection_surface}
        }}
        "#
    )
}

fn reordered_source_text(collection_surface: &str) -> String {
    format!(
        r#"
        runtime ProductsRuntime {{}}
        appearance ShopifyAdminTheme {{}}

        content ProductsContent {{
            collection -> {collection_surface}
        }}

        layout ProductsLayout {{
            column {{
                row height fill scroll_owner {{ slot collection }}
            }}
        }}

        page ProductsPage {{
            runtime ProductsRuntime
            layout ProductsLayout
            content ProductsContent
        }}

        workspace AdminWorkspace {{
            shell {{
                topbar AdminTopbar
                rail AdminPrimaryRail
                page_host AdminPageHost
                inspector AdminInspectorDock
                status AdminStatusBar
                overlays [CommandPaletteOverlay]
                toasts AdminToastCenter
            }}
            pages [ProductsPage]
        }}

        app ShopifyAdminApp {{
            theme ShopifyAdminTheme
            workspace AdminWorkspace
        }}
        "#
    )
}
