use worth_ui::facade::{
    AppearanceTokenId, CommandProjectionId, ComponentChildPolicy, ComponentDescriptor,
    ComponentExecutionLane, ComponentFocusSupport, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, DensityTokenId, MeasurementConstraint, MeasurementValue,
    MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture,
    MosaicMeasurementAuthority, MosaicOverflowBehavior, MosaicParentGrowthBehavior,
    MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole,
    MosaicResizePermission, MosaicScrollOwnership, MosaicSizingBehavior,
    MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicViewportConstraint, NamedMeasurementDefinition,
    NamedMeasurementToken, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, ThemeColorValue, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId,
    ThemeTokenSource, ThemeTokenValue, WorthUi, WorthUiApp, WorthUiAppBuilder,
    WorthUiAppearanceFamily, WorthUiAppearanceTokenDescriptor, WorthUiAppearanceTokenSource,
    WorthUiAppearanceValue, WorthUiBorderWidthValue, WorthUiFontSizeValue,
    WorthUiHeaderAppearanceRequest, WorthUiHeaderFrameRebindRequest,
    WorthUiHeaderMenuProjectionRequest, WorthUiHeaderThemeTokenRequest, WorthUiLengthValue,
    WorthUiShadowValue,
};

use crate::app_capabilities_densities;
use crate::header::{register_header_command_capabilities, register_header_icon_capabilities};
use crate::product_preview::{
    register_preview_component_capabilities, register_preview_icon_capabilities,
    register_preview_surface_capabilities,
};

pub fn validation_worth_ui_app() -> WorthUiApp {
    validation_app_builder().freeze()
}

fn validation_app_builder() -> WorthUiAppBuilder {
    register_header_command_capabilities(register_header_runtime_surface(register_header_regions(
        register_header_density_tokens(register_header_appearance_tokens(
            register_header_theme_tokens(register_validation_components(
                register_header_icon_capabilities(register_preview_surface_capabilities(
                    register_preview_icon_capabilities(register_preview_component_capabilities(
                        register_preview_sizing_tokens(WorthUi::app()),
                    )),
                )),
            )),
        )),
    )))
}

fn register_validation_components(builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    builder
        .register_component(header_component())
        .register_component(header_proof_alt_component())
        .register_component(header_dropdown_component())
        .register_component(header_multi_select_dropdown_component())
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

fn register_preview_sizing_tokens(builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    preview_sizing_contracts()
        .into_iter()
        .fold(builder, |builder, descriptor| {
            builder.register_mosaic_sizing_contract(descriptor)
        })
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
        .register_surface(header_alt_surface("validation.surface.header.proof.alt"))
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

fn header_alt_surface(id: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).expect("valid surface id"),
        SurfaceKind::primary_content(),
        ComponentId::new("validation.component.header.proof.alt").expect("valid component id"),
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

fn register_header_appearance_tokens(mut builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    for descriptor in header_appearance_tokens() {
        builder = builder.register_appearance_token(descriptor);
    }
    builder
}

fn register_header_density_tokens(mut builder: WorthUiAppBuilder) -> WorthUiAppBuilder {
    for descriptor in app_capabilities_densities::header_density_tokens() {
        builder = builder.register_density_token(descriptor);
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

fn header_proof_alt_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.header.proof.alt").expect("valid component id"),
        ComponentPropSchema::named("validation.header.proof.alt.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn header_dropdown_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.header.dropdown").expect("valid component id"),
        ComponentPropSchema::named("validation.header.dropdown.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_focus(ComponentFocusSupport::focusable())
    .with_execution_lane(ComponentExecutionLane::Interactive)
}

fn header_multi_select_dropdown_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("validation.component.header.multi_select_dropdown")
            .expect("valid component id"),
        ComponentPropSchema::named("validation.header.multi_select_dropdown.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_focus(ComponentFocusSupport::focusable())
    .with_execution_lane(ComponentExecutionLane::Interactive)
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
            ComponentId::new("validation.component.header.dropdown").expect("valid component id"),
            ComponentId::new("validation.component.header.multi_select_dropdown")
                .expect("valid component id"),
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

pub(crate) fn validation_header_appearance_request() -> WorthUiHeaderAppearanceRequest {
    WorthUiHeaderAppearanceRequest::new(
        AppearanceTokenId::new("validation.appearance.header.font_size")
            .expect("valid appearance token id"),
        AppearanceTokenId::new("validation.appearance.header.menu_min_width")
            .expect("valid appearance token id"),
        AppearanceTokenId::new("validation.appearance.header.border_width")
            .expect("valid appearance token id"),
        AppearanceTokenId::new("validation.appearance.header.panel_shadow")
            .expect("valid appearance token id"),
        DensityTokenId::new("validation.density.header.row_padding").expect("valid density id"),
        DensityTokenId::new("validation.density.header.container_padding")
            .expect("valid density id"),
        DensityTokenId::new("validation.density.header.control_spacing").expect("valid density id"),
    )
}

pub(crate) fn validation_header_frame_rebind_request() -> WorthUiHeaderFrameRebindRequest {
    WorthUiHeaderFrameRebindRequest::new(
        validation_header_menu_requests(),
        validation_header_theme_request(),
        validation_header_appearance_request(),
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

fn header_appearance_tokens() -> Vec<WorthUiAppearanceTokenDescriptor> {
    vec![
        WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("validation.appearance.header.font_size")
                .expect("valid appearance token id"),
            WorthUiAppearanceFamily::Typography,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::FontSize(
                WorthUiFontSizeValue::from_px("13px").expect("valid font size"),
            ),
        ),
        WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("validation.appearance.header.menu_min_width")
                .expect("valid appearance token id"),
            WorthUiAppearanceFamily::Layout,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::Length(
                WorthUiLengthValue::from_px("220px").expect("valid menu width"),
            ),
        ),
        WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("validation.appearance.header.border_width")
                .expect("valid appearance token id"),
            WorthUiAppearanceFamily::Border,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::BorderWidth(
                WorthUiBorderWidthValue::from_px("1px").expect("valid border width"),
            ),
        ),
        WorthUiAppearanceTokenDescriptor::define(
            AppearanceTokenId::new("validation.appearance.header.panel_shadow")
                .expect("valid appearance token id"),
            WorthUiAppearanceFamily::Elevation,
            WorthUiAppearanceTokenSource::Application,
            WorthUiAppearanceValue::Shadow(
                WorthUiShadowValue::from_authored_parts(
                    ThemeColorValue::hex("#00000066").expect("valid shadow color"),
                    "0px",
                    "1px",
                    "3px",
                    "0px",
                )
                .expect("valid shadow"),
            ),
        ),
    ]
}

fn preview_sizing_contracts() -> Vec<MosaicSizingContractDescriptor> {
    vec![
        sizing_contract(
            "worth.preview.measurement.rail.md",
            "rail.md",
            220,
            180,
            280,
        ),
        sizing_contract(
            "worth.preview.measurement.rail.xl",
            "rail.xl",
            280,
            220,
            340,
        ),
        sizing_contract(
            "worth.preview.measurement.inspector.md",
            "inspector.md",
            320,
            280,
            420,
        ),
        sizing_contract(
            "worth.preview.measurement.inspector.xl",
            "inspector.xl",
            420,
            340,
            520,
        ),
        sizing_contract(
            "worth.preview.measurement.panel.lg",
            "panel.lg",
            180,
            140,
            240,
        ),
    ]
}

fn sizing_contract(
    contract_id: &str,
    token: &str,
    value: u32,
    min: u32,
    max: u32,
) -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new(contract_id).expect("valid preview sizing contract id"),
        MosaicSizingKind::fixed(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new(token).expect("valid preview measurement token"),
        MeasurementValue::logical_pixels(value),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(min),
            MeasurementValue::logical_pixels(max),
        ),
    ))
}
