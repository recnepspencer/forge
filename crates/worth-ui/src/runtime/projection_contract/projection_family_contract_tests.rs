use crate::facade::{
    AppearanceTokenId, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface,
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, DensityTokenId, MosaicChildRule, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    ThemeTokenValue, WorthUi, WorthUiApp, WorthUiAppearanceFamily,
    WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource, WorthUiAppearanceValue,
    WorthUiBorderWidthValue, WorthUiDensityFamily, WorthUiDensityTokenDescriptor,
    WorthUiDensityValue, WorthUiFontSizeValue, WorthUiLengthValue, WorthUiPaddingValue,
    WorthUiRuntimeSourceModule, WorthUiShadowValue, WorthUiSpacingValue,
};
use crate::runtime::{
    WorthUiDropdownAppearanceRequest, WorthUiHeaderAppearanceRequest, WorthUiHeaderFramePlan,
    WorthUiHeaderMenuPlan, WorthUiHeaderMenuProjectionRequest, WorthUiHeaderThemePlan,
    WorthUiHeaderThemeTokenRequest, WorthUiPageHostPlan, WorthUiPageHostRequest,
    WorthUiProjectionEquivalenceBasisKind, WorthUiProjectionFamily, WorthUiRuntimeFactId,
    WorthUiRuntimeHost, WorthUiRuntimeInstanceWitness,
};

#[test]
fn runtime_host_admission_binds_projection_plan_to_host_runtime_witness() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let app = projection_family_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let plan = WorthUiHeaderMenuPlan::from_snapshot(
        app.capabilities(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "File",
            projection_id.clone(),
            ComponentId::new("validation.component.sample").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
        )],
        dropdown_appearance_request(),
    )
    .expect("header menu plan builds");

    let admitted = runtime
        .admit_projection_plan(plan)
        .expect("runtime admits projection plan");

    assert_eq!(
        admitted.runtime_instance(),
        WorthUiRuntimeInstanceWitness::from_raw(runtime.instance_id().raw())
    );
    assert_eq!(
        admitted.proof().runtime_instance(),
        WorthUiRuntimeInstanceWitness::from_raw(runtime.instance_id().raw())
    );
    assert!(admitted
        .dependencies()
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::command_projection(&projection_id)));
}

#[test]
fn header_theme_projection_admission_declares_theme_fact_contract() {
    let app = projection_family_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let admitted = runtime
        .admit_projection_plan(header_theme_plan(&app))
        .expect("runtime admits theme plan");

    assert_eq!(
        admitted.dependencies().family(),
        WorthUiProjectionFamily::HeaderTheme
    );
    assert_eq!(
        admitted.equivalence_basis().kind(),
        WorthUiProjectionEquivalenceBasisKind::ThemeDigest
    );
    assert!(admitted.dependencies().dependencies().contains_exact(
        &WorthUiRuntimeFactId::theme_token(&ThemeTokenId::new("theme.header.panel").unwrap())
    ));
}

#[test]
fn header_frame_projection_admission_merges_menu_and_theme_dependencies() {
    let projection_id = CommandProjectionId::new("workspace.header.file").unwrap();
    let app = projection_family_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let plan = WorthUiHeaderFramePlan::from_snapshot(
        app.capabilities(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "File",
            projection_id.clone(),
            ComponentId::new("validation.component.sample").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
        )],
        header_theme_token_request(),
        header_appearance_request(),
    )
    .expect("header frame plan builds from menu and theme plans");
    let admitted = runtime
        .admit_projection_plan(plan)
        .expect("runtime admits frame plan");

    assert_eq!(
        admitted.dependencies().family(),
        WorthUiProjectionFamily::HeaderFrame
    );
    assert_eq!(
        admitted.equivalence_basis().kind(),
        WorthUiProjectionEquivalenceBasisKind::FrameDigest
    );
    assert!(admitted
        .dependencies()
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::command_projection(&projection_id)));
    assert!(admitted.dependencies().dependencies().contains_exact(
        &WorthUiRuntimeFactId::theme_token(&ThemeTokenId::new("theme.header.text").unwrap())
    ));
}

#[test]
fn page_host_projection_admission_declares_layout_slot_and_surface_contract() {
    let app = projection_family_app("Save");
    let runtime = runtime_for_source_authored_page_host(&app);
    let plan =
        WorthUiPageHostPlan::from_runtime(&runtime, WorthUiPageHostRequest::new("ProductsPage"))
            .expect("page host plan builds from source-authored runtime");
    let admitted = runtime
        .admit_projection_plan(plan)
        .expect("runtime admits page host projection");

    assert_eq!(
        admitted.dependencies().family(),
        WorthUiProjectionFamily::PageHost
    );
    assert_eq!(
        admitted.equivalence_basis().kind(),
        WorthUiProjectionEquivalenceBasisKind::FrameDigest
    );
    assert!(admitted
        .dependencies()
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::layout_topology("ProductsPage")));
    assert!(admitted
        .dependencies()
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::layout_gap("ProductsPage")));
    assert!(admitted
        .dependencies()
        .dependencies()
        .contains_exact(&WorthUiRuntimeFactId::layout_padding("ProductsPage")));
    assert!(admitted.dependencies().dependencies().contains_exact(
        &WorthUiRuntimeFactId::content_mount("ProductsPage.collection")
    ));
    assert!(admitted.dependencies().dependencies().contains_exact(
        &WorthUiRuntimeFactId::page_content_slot(
            &crate::runtime::WorthUiPageTemplateId::new("ProductsPage").unwrap(),
            &crate::runtime::WorthUiContentSlotId::new("collection").unwrap(),
        )
    ));
    assert!(admitted.dependencies().dependencies().contains_exact(
        &WorthUiRuntimeFactId::surface_mount(
            &SurfaceId::new("validation.surface.products.collection").unwrap(),
        )
    ));
}

fn header_theme_plan(app: &WorthUiApp) -> WorthUiHeaderThemePlan {
    WorthUiHeaderThemePlan::from_snapshot(app.capabilities(), header_theme_token_request())
        .expect("theme plan resolves header tokens")
}

fn dropdown_appearance_request() -> WorthUiDropdownAppearanceRequest {
    WorthUiDropdownAppearanceRequest::new(
        AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
        DensityTokenId::new("density.header.row_padding").unwrap(),
        DensityTokenId::new("density.header.control_spacing").unwrap(),
    )
}

fn projection_family_app(label: &str) -> WorthUiApp {
    let command_id = CommandId::new("workspace.command.save").unwrap();
    WorthUi::app()
        .register_command(CommandDescriptor::new(command_id.clone(), label))
        .register_command_projection(
            CommandProjectionDescriptor::new(
                CommandProjectionId::new("workspace.header.file").unwrap(),
                CommandProjectionSurface::menu_bar(),
            )
            .with_command_reference(CommandProjectionCommandReference::command(command_id)),
        )
        .register_theme_token(theme_token("theme.header.panel", "#1e1e1e"))
        .register_theme_token(theme_token("theme.header.menu", "#252526"))
        .register_theme_token(theme_token("theme.header.menu.hover", "#2a2d2e"))
        .register_theme_token(theme_token("theme.header.menu.active", "#094771"))
        .register_theme_token(theme_token("theme.header.text", "#cccccc"))
        .register_theme_token(theme_token("theme.header.border", "#3c3c3c"))
        .register_appearance_token(appearance_font_size_token())
        .register_appearance_token(appearance_menu_width_token())
        .register_appearance_token(appearance_border_width_token())
        .register_appearance_token(appearance_shadow_token())
        .register_density_token(density_row_padding_token())
        .register_density_token(density_container_padding_token())
        .register_density_token(density_control_spacing_token())
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

fn runtime_for_source_authored_page_host(app: &WorthUiApp) -> WorthUiRuntimeHost {
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new(
            "app/main.wui",
            page_host_source_text(),
        ))
        .prepare_authoring_for(app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}

fn page_host_source_text() -> &'static str {
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
    "#
}

fn header_theme_token_request() -> WorthUiHeaderThemeTokenRequest {
    WorthUiHeaderThemeTokenRequest::new(
        ThemeTokenId::new("theme.header.panel").unwrap(),
        ThemeTokenId::new("theme.header.menu").unwrap(),
        ThemeTokenId::new("theme.header.menu.hover").unwrap(),
        ThemeTokenId::new("theme.header.menu.active").unwrap(),
        ThemeTokenId::new("theme.header.text").unwrap(),
        ThemeTokenId::new("theme.header.border").unwrap(),
    )
}

fn header_appearance_request() -> WorthUiHeaderAppearanceRequest {
    WorthUiHeaderAppearanceRequest::new(
        AppearanceTokenId::new("appearance.header.font_size").unwrap(),
        AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
        AppearanceTokenId::new("appearance.header.border_width").unwrap(),
        AppearanceTokenId::new("appearance.header.panel_shadow").unwrap(),
        DensityTokenId::new("density.header.row_padding").unwrap(),
        DensityTokenId::new("density.header.container_padding").unwrap(),
        DensityTokenId::new("density.header.control_spacing").unwrap(),
    )
}

fn theme_token(id: &str, color: &str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        ThemeTokenId::new(id).unwrap(),
        ThemeTokenFamily::surface(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(ThemeColorValue::hex(color).unwrap()),
    )
}

fn appearance_font_size_token() -> WorthUiAppearanceTokenDescriptor {
    WorthUiAppearanceTokenDescriptor::define(
        AppearanceTokenId::new("appearance.header.font_size").unwrap(),
        WorthUiAppearanceFamily::Typography,
        WorthUiAppearanceTokenSource::Application,
        WorthUiAppearanceValue::FontSize(WorthUiFontSizeValue::from_px("13px").unwrap()),
    )
}

fn appearance_menu_width_token() -> WorthUiAppearanceTokenDescriptor {
    WorthUiAppearanceTokenDescriptor::define(
        AppearanceTokenId::new("appearance.header.menu_min_width").unwrap(),
        WorthUiAppearanceFamily::Layout,
        WorthUiAppearanceTokenSource::Application,
        WorthUiAppearanceValue::Length(WorthUiLengthValue::from_px("220px").unwrap()),
    )
}

fn appearance_border_width_token() -> WorthUiAppearanceTokenDescriptor {
    WorthUiAppearanceTokenDescriptor::define(
        AppearanceTokenId::new("appearance.header.border_width").unwrap(),
        WorthUiAppearanceFamily::Border,
        WorthUiAppearanceTokenSource::Application,
        WorthUiAppearanceValue::BorderWidth(WorthUiBorderWidthValue::from_px("1px").unwrap()),
    )
}

fn appearance_shadow_token() -> WorthUiAppearanceTokenDescriptor {
    WorthUiAppearanceTokenDescriptor::define(
        AppearanceTokenId::new("appearance.header.panel_shadow").unwrap(),
        WorthUiAppearanceFamily::Elevation,
        WorthUiAppearanceTokenSource::Application,
        WorthUiAppearanceValue::Shadow(
            WorthUiShadowValue::from_authored_parts(
                ThemeColorValue::hex("#00000066").unwrap(),
                "0px",
                "1px",
                "3px",
                "0px",
            )
            .unwrap(),
        ),
    )
}

fn density_row_padding_token() -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new("density.header.row_padding").unwrap(),
        WorthUiDensityFamily::RowPadding,
        WorthUiDensityValue::Padding(WorthUiPaddingValue::from_shorthand_px("1px 6px").unwrap()),
    )
}

fn density_container_padding_token() -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new("density.header.container_padding").unwrap(),
        WorthUiDensityFamily::ContainerPadding,
        WorthUiDensityValue::Padding(WorthUiPaddingValue::from_shorthand_px("4px 8px").unwrap()),
    )
}

fn density_control_spacing_token() -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new("density.header.control_spacing").unwrap(),
        WorthUiDensityFamily::ControlSpacing,
        WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px("8px").unwrap()),
    )
}

fn validation_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.sample").unwrap(),
        ComponentPropSchema::named("validation.sample.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn validation_surface(surface_id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(surface_id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new("validation.component.sample").unwrap(),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn layout_region(
    region_id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(MosaicRegionKindId::new(region_id).unwrap(), role)
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}
