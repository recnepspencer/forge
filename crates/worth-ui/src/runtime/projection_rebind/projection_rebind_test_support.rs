use crate::capability::{
    AppearanceTokenId, CommandDescriptor, CommandId, CommandProjectionCommandReference,
    CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface,
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, DensityTokenId, MosaicChildRule, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    ThemeTokenValue, WorthUiAppearanceFamily, WorthUiAppearanceTokenDescriptor,
    WorthUiAppearanceTokenSource, WorthUiAppearanceValue, WorthUiBorderWidthValue,
    WorthUiDensityFamily, WorthUiDensityTokenDescriptor, WorthUiDensityValue, WorthUiFontSizeValue,
    WorthUiLengthValue, WorthUiPaddingValue, WorthUiShadowValue, WorthUiSpacingValue,
};
use crate::facade::{WorthUi, WorthUiApp, WorthUiRuntimeSourceModule};
use crate::runtime::{
    WorthUiAdmittedRuntimeChangeEvidence, WorthUiCapabilityReloadEvidence,
    WorthUiCapabilityReloadRequest, WorthUiCommandProjectionReloadPackage,
    WorthUiHeaderAppearanceRequest, WorthUiHeaderFramePlan, WorthUiHeaderFrameRebindRequest,
    WorthUiHeaderMenuProjectionRequest, WorthUiHeaderThemeTokenRequest, WorthUiPageHostPlan,
    WorthUiPageHostRequest, WorthUiRuntimeHost, WorthUiThemeTokenReloadPackage,
    WorthUiValidationReloadEvidence, WorthUiValidationReloadRequest,
};

pub(crate) fn prepare_projection_rebind_plan<P: crate::runtime::WorthUiProjectionPlanContract>(
    runtime: &WorthUiRuntimeHost,
    plan: P,
    evidence: &WorthUiAdmittedRuntimeChangeEvidence,
) -> crate::runtime::WorthUiProjectionRebindPlan<P> {
    let admitted = runtime.admit_projection_plan(plan).unwrap();
    runtime
        .prepare_projection_rebind(evidence, admitted)
        .unwrap()
}

pub(crate) fn rebuilt_header_batch(
    runtime: &mut WorthUiRuntimeHost,
    plan: WorthUiHeaderFramePlan,
    evidence: &WorthUiCapabilityReloadEvidence,
) -> crate::runtime::WorthUiProjectionRebindBatchReceipt {
    runtime
        .rebind_header_frame_after_capability_reload(&plan, header_rebind_request(), evidence)
        .unwrap()
        .1
        .projection_rebind_batch()
        .clone()
}

pub(crate) fn validation_ready(runtime: &WorthUiRuntimeHost) -> WorthUiValidationReloadEvidence {
    runtime
        .prepare_validation_reload(
            runtime.active_capability_snapshot(),
            WorthUiValidationReloadRequest::from_source_module(
                "app/main.wui",
                page_host_source_text().replace(
                    "collection -> validation.surface.products.collection",
                    "collection -> validation.surface.products.toolbar",
                ),
            ),
        )
        .evidence()
        .clone()
}

pub(crate) fn validation_activated(
    runtime: &mut WorthUiRuntimeHost,
) -> WorthUiValidationReloadEvidence {
    validation_activated_for_source(
        runtime,
        page_host_source_text().replace(
            "collection -> validation.surface.products.collection",
            "collection -> validation.surface.products.toolbar",
        ),
    )
}

pub(crate) fn validation_activated_for_source(
    runtime: &mut WorthUiRuntimeHost,
    source_text: String,
) -> WorthUiValidationReloadEvidence {
    let prepared = runtime.prepare_validation_reload(
        runtime.active_capability_snapshot(),
        WorthUiValidationReloadRequest::from_source_module("app/main.wui", source_text),
    );
    assert!(
        prepared.is_ready(),
        "changed source package should prepare for activation"
    );
    prepared
        .activate(runtime)
        .expect("prepared source reload should activate")
}

pub(crate) fn capability_equivalent(
    runtime: &WorthUiRuntimeHost,
) -> WorthUiCapabilityReloadEvidence {
    runtime
        .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_theme_tokens(
            theme_reload_package("theme.header.text", "#cccccc"),
        ))
        .evidence()
        .clone()
}

pub(crate) fn capability_denied(runtime: &WorthUiRuntimeHost) -> WorthUiCapabilityReloadEvidence {
    runtime
        .prepare_capability_reload(WorthUiCapabilityReloadRequest::from_theme_tokens(
            theme_reload_package("theme.header.unknown", "#102030"),
        ))
        .evidence()
        .clone()
}

pub(crate) fn capability_activated(
    runtime: &mut WorthUiRuntimeHost,
    token_id: &str,
    color: &str,
) -> WorthUiCapabilityReloadEvidence {
    let prepared = runtime.prepare_capability_reload(
        WorthUiCapabilityReloadRequest::from_theme_tokens(theme_reload_package(token_id, color)),
    );
    assert!(
        prepared.is_ready(),
        "changed theme token should prepare for activation"
    );
    prepared
        .activate(runtime)
        .expect("prepared theme token reload should activate")
}

pub(crate) fn command_projection_activated(
    runtime: &mut WorthUiRuntimeHost,
    source_text: &str,
) -> WorthUiCapabilityReloadEvidence {
    let prepared = runtime.prepare_capability_reload(
        WorthUiCapabilityReloadRequest::from_command_projections(
            WorthUiCommandProjectionReloadPackage::from_source(
                "tests/projection-rebind/header.projections",
                source_text,
            ),
        ),
    );
    assert!(
        prepared.is_ready(),
        "changed projection policy should prepare for activation"
    );
    prepared
        .activate(runtime)
        .expect("prepared projection reload should activate")
}

pub(crate) fn header_frame_plan(app: &WorthUiApp) -> WorthUiHeaderFramePlan {
    WorthUiHeaderFramePlan::from_snapshot(
        app.capabilities(),
        [WorthUiHeaderMenuProjectionRequest::new(
            "File",
            CommandProjectionId::new("workspace.header.file").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
        )],
        header_theme_token_request(),
        header_appearance_request(),
    )
    .expect("header frame plan builds")
}

pub(crate) fn page_host_plan(runtime: &WorthUiRuntimeHost) -> WorthUiPageHostPlan {
    WorthUiPageHostPlan::from_runtime(runtime, WorthUiPageHostRequest::new("ProductsPage"))
        .expect("page host plan builds")
}

pub(crate) fn header_rebind_request() -> WorthUiHeaderFrameRebindRequest {
    WorthUiHeaderFrameRebindRequest::new(
        vec![WorthUiHeaderMenuProjectionRequest::new(
            "File",
            CommandProjectionId::new("workspace.header.file").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
            ComponentId::new("validation.component.sample").unwrap(),
        )],
        header_theme_token_request(),
        header_appearance_request(),
    )
}

pub(crate) fn projection_rebind_app(label: &str) -> WorthUiApp {
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

pub(crate) fn runtime_for_source_authored_page_host(app: &WorthUiApp) -> WorthUiRuntimeHost {
    let module = WorthUiRuntimeSourceModule::new("app/main.wui", page_host_source_text());
    let prepared = WorthUi::runtime_launch()
        .from_source_module(module)
        .prepare_authoring_for(app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}

pub(crate) fn page_host_source_text() -> &'static str {
    r#"
    app ShopifyAdminApp { theme ShopifyAdminTheme workspace AdminWorkspace }
    workspace AdminWorkspace { shell {
        topbar AdminTopbar rail AdminPrimaryRail page_host AdminPageHost
        inspector AdminInspectorDock status AdminStatusBar
        overlays [CommandPaletteOverlay] toasts AdminToastCenter
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

fn theme_reload_package(token_id: &str, color: &str) -> WorthUiThemeTokenReloadPackage {
    WorthUiThemeTokenReloadPackage::from_source(
        "tests/projection-rebind/theme.header",
        format!("{token_id} = {color}"),
    )
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
