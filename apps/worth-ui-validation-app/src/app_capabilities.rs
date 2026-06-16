use worth_ui::facade::{
    CommandProjectionId, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, MosaicChildRule, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    ThemeTokenValue, WorthUi, WorthUiApp, WorthUiAppBuilder, WorthUiHeaderFrameRebindRequest,
    WorthUiHeaderMenuProjectionRequest, WorthUiHeaderThemeTokenRequest,
};

use crate::header::register_header_command_capabilities;

pub fn validation_worth_ui_app() -> WorthUiApp {
    validation_app_builder().freeze()
}

fn validation_app_builder() -> WorthUiAppBuilder {
    register_header_command_capabilities(register_header_runtime_surface(register_header_regions(
        register_header_theme_tokens(WorthUi::app().register_component(header_component())),
    )))
}

fn register_header_regions(builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    builder
        .register_mosaic_region_kind(header_region(
            "worth.ui.layout.column",
            MosaicRegionRole::stack(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(header_region(
            "worth.ui.layout.row",
            MosaicRegionRole::split(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(header_region(
            "worth.ui.layout.slot",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
}

fn header_region(
    id: &str,
    role: MosaicRegionRole,
    child_rule: MosaicChildRule,
) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(MosaicRegionKindId::new(id).expect("valid region id"), role)
        .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
        .with_scroll_ownership(MosaicScrollOwnership::region_owned())
        .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
        .with_child_rule(child_rule)
        .with_allowed_surface_class(SurfacePlacementClass::primary_region())
        .with_persistence(MosaicRegionPersistence::restorable())
        .with_clipping(MosaicClippingPosture::clip_to_region())
        .with_hit_test(MosaicHitTestPosture::participates())
}

fn register_header_runtime_surface(builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    builder
        .register_surface(header_surface("validation.surface.header.proof"))
        .register_surface(header_surface("validation.surface.header.proof.alt"))
}

fn header_surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).expect("valid surface id"),
        SurfaceKind::primary_content(),
        ComponentId::new("validation.component.header.proof").expect("valid component id"),
        SurfacePlacementClass::primary_region(),
        SurfaceStateClass::restorable(),
    )
}

fn register_header_theme_tokens(mut builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    for (id, family, color) in HEADER_THEME_TOKENS {
        builder = builder.register_theme_token(ThemeTokenDescriptor::define(
            ThemeTokenId::new(id).expect("valid theme token id"),
            family.clone(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex(*color).expect("valid theme color")),
        ));
    }
    builder
}

fn header_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.header.proof").expect("valid component id"),
        ComponentPropSchema::named("validation.header.proof.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

pub(crate) fn validation_header_menu_requests() -> Vec<WorthUiHeaderMenuProjectionRequest> {
    [
        ("File", "validation.header.menu.file"),
        ("Edit", "validation.header.menu.edit"),
        ("Terminal", "validation.header.menu.terminal"),
        ("Help", "validation.header.menu.help"),
    ]
    .into_iter()
    .map(|(title, projection_id)| {
        WorthUiHeaderMenuProjectionRequest::new(
            title,
            CommandProjectionId::new(projection_id).expect("valid header projection id"),
        )
    })
    .collect()
}

pub(crate) fn validation_header_theme_request() -> WorthUiHeaderThemeTokenRequest {
    WorthUiHeaderThemeTokenRequest::new(
        ThemeTokenId::new("validation.theme.header.panel").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.menu").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.menu.hover").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.menu.active").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.text").expect("valid theme token id"),
        ThemeTokenId::new("validation.theme.header.border").expect("valid theme token id"),
    )
}

pub(crate) fn validation_header_frame_rebind_request() -> WorthUiHeaderFrameRebindRequest {
    WorthUiHeaderFrameRebindRequest::new(
        validation_header_menu_requests(),
        validation_header_theme_request(),
    )
}

const HEADER_THEME_TOKENS: &[(&str, ThemeTokenFamily, &str)] = &[
    (
        "validation.theme.header.panel",
        ThemeTokenFamily::Surface,
        "#1E1E1E",
    ),
    (
        "validation.theme.header.menu",
        ThemeTokenFamily::ElevatedSurface,
        "#252526",
    ),
    (
        "validation.theme.header.menu.hover",
        ThemeTokenFamily::Selection,
        "#3E3E42",
    ),
    (
        "validation.theme.header.menu.active",
        ThemeTokenFamily::Accent,
        "#007ACC",
    ),
    (
        "validation.theme.header.text",
        ThemeTokenFamily::Text,
        "#CCCCCC",
    ),
    (
        "validation.theme.header.border",
        ThemeTokenFamily::Border,
        "#3F3F46",
    ),
];
