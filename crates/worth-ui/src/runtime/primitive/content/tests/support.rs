use crate::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, DensityTokenId, IconDescriptor, IconFamily, IconId,
    IconSourceDescriptor, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence,
    MosaicRegionRole, MosaicScrollOwnership, MosaicSizingBehavior, SurfaceDescriptor, SurfaceId,
    SurfaceKind, SurfacePlacementClass, SurfaceStateClass, WorthUi, WorthUiApp,
    WorthUiDensityFamily, WorthUiDensityTokenDescriptor, WorthUiDensityValue, WorthUiPaddingValue,
    WorthUiRuntimeSourceModule, WorthUiSpacingValue,
};
use crate::runtime::WorthUiRuntimeHost;

pub(super) const SURFACE_ID: &str = "worth.surface.preview.primitive.proof";
pub(super) const ROW_SURFACE_ID: &str = "worth.surface.preview.primitive.row";
pub(super) const CARD_SURFACE_ID: &str = "worth.surface.preview.primitive.card";
const COMPONENT_ID: &str = "worth.component.primitive_proof";
const ROW_COMPONENT_ID: &str = "worth.component.primitive_row_proof";
const CARD_COMPONENT_ID: &str = "worth.component.primitive_card_proof";

pub(super) fn runtime_for_source(source: String) -> WorthUiRuntimeHost {
    let app = primitive_content_test_app();
    let prepared = WorthUi::runtime_launch()
        .from_source_module(WorthUiRuntimeSourceModule::new("app/main.wui", source))
        .prepare_authoring_for(&app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}

fn primitive_content_test_app() -> WorthUiApp {
    let mut builder = WorthUi::app()
        .register_component(primitive_component(COMPONENT_ID))
        .register_component(primitive_component(ROW_COMPONENT_ID))
        .register_component(primitive_component(CARD_COMPONENT_ID))
        .register_surface(primitive_surface(SURFACE_ID, COMPONENT_ID))
        .register_surface(primitive_surface(ROW_SURFACE_ID, ROW_COMPONENT_ID))
        .register_surface(primitive_surface(CARD_SURFACE_ID, CARD_COMPONENT_ID))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.column",
            MosaicRegionRole::stack(),
            MosaicChildRule::accepts_regions(),
        ))
        .register_mosaic_region_kind(layout_region(
            "worth.ui.layout.slot",
            MosaicRegionRole::primary(),
            MosaicChildRule::accepts_surfaces(),
        ))
        .register_icon(icon("worth.icon.action.plus", "plus"))
        .register_icon(icon("worth.icon.action.check", "check"))
        .register_icon(fallback_icon("worth.icon.action.fallback", "plus"));
    for token in density_tokens() {
        builder = builder.register_density_token(token);
    }
    builder.freeze()
}

fn primitive_component(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).unwrap(),
        ComponentPropSchema::named("worth.primitive.proof.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn primitive_surface(surface: &str, component: &str) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(surface).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new(component).unwrap(),
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

fn icon(id: &str, source_key: &str) -> IconDescriptor {
    IconDescriptor::new(
        IconId::new(id).unwrap(),
        IconFamily::toolbar(),
        IconSourceDescriptor::symbol(source_key),
    )
}

fn fallback_icon(id: &str, source_key: &str) -> IconDescriptor {
    IconDescriptor::new(
        IconId::new(id).unwrap(),
        IconFamily::toolbar(),
        IconSourceDescriptor::symbol(source_key).with_native_vector_support(
            crate::capability::IconNativeVectorSupport::unsupported_by_host(),
        ),
    )
}

fn density_tokens() -> Vec<WorthUiDensityTokenDescriptor> {
    vec![
        spacing_token("validation.density.primitive.radius", "8px"),
        spacing_token("validation.density.primitive.border.none", "0px"),
        spacing_token("validation.density.primitive.border.default", "2px"),
        spacing_token("validation.density.primitive.motion.fast", "120px"),
        padding_token("validation.density.primitive.padding", "16px 32px"),
        spacing_token("validation.density.primitive.flow.gap.compact", "6px"),
        spacing_token("validation.density.primitive.flow.gap.default", "8px"),
        spacing_token("validation.density.primitive.flow.padding.compact", "16px"),
        spacing_token("validation.density.primitive.flow.padding.default", "32px"),
        spacing_token("validation.density.primitive.event.hit_slop.default", "8px"),
        spacing_token("validation.density.primitive.content.text.default", "15px"),
        spacing_token("validation.density.primitive.content.icon.default", "24px"),
        spacing_token("validation.density.primitive.content.icon.large", "32px"),
        spacing_token(
            "validation.density.primitive.content.icon.stroke.thin",
            "1px",
        ),
        spacing_token(
            "validation.density.primitive.content.icon.stroke.default",
            "2px",
        ),
        spacing_token("validation.density.primitive.content.spacer.default", "8px"),
        spacing_token(
            "validation.density.primitive.content.divider.default",
            "1px",
        ),
    ]
}

fn spacing_token(id: &str, value: &str) -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new(id).unwrap(),
        WorthUiDensityFamily::ControlSpacing,
        WorthUiDensityValue::Spacing(WorthUiSpacingValue::from_px(value).unwrap()),
    )
}

fn padding_token(id: &str, value: &str) -> WorthUiDensityTokenDescriptor {
    WorthUiDensityTokenDescriptor::define(
        DensityTokenId::new(id).unwrap(),
        WorthUiDensityFamily::ContainerPadding,
        WorthUiDensityValue::Padding(WorthUiPaddingValue::from_shorthand_px(value).unwrap()),
    )
}

pub(super) fn surface_id() -> SurfaceId {
    SurfaceId::new(SURFACE_ID).unwrap()
}

pub(super) fn row_surface_id() -> SurfaceId {
    SurfaceId::new(ROW_SURFACE_ID).unwrap()
}

pub(super) fn card_surface_id() -> SurfaceId {
    SurfaceId::new(CARD_SURFACE_ID).unwrap()
}

pub(super) fn content_source(props: &[(&str, &str)]) -> String {
    let mut source = String::from(
        r##"
app TestApp {
    theme TestTheme
    workspace TestWorkspace
}

workspace TestWorkspace {
    shell {
        topbar TestTopbar
        rail TestRail
        page_host TestPageHost
        inspector TestInspector
        status TestStatus
        overlays [CommandPaletteOverlay]
        toasts TestToasts
    }
    pages [TestPage]
}

page TestPage {
    title "Test"
    runtime TestRuntime
    layout TestLayout
    content TestContent
}

runtime TestRuntime {}

surface worth.surface.preview.primitive.proof {
    component worth.component.primitive_proof
"##,
    );
    for (key, value) in props {
        source.push_str("    ");
        source.push_str(key);
        source.push(' ');
        source.push_str(value);
        source.push('\n');
    }
    source.push_str(
        r##"}

surface worth.surface.preview.primitive.row {
    component worth.component.primitive_row_proof
"##,
    );
    for (key, value) in props {
        source.push_str("    ");
        source.push_str(key);
        source.push(' ');
        source.push_str(value);
        source.push('\n');
    }
    source.push_str(
        r##"}

surface worth.surface.preview.primitive.card {
    component worth.component.primitive_card_proof
"##,
    );
    for (key, value) in props {
        source.push_str("    ");
        source.push_str(key);
        source.push(' ');
        source.push_str(value);
        source.push('\n');
    }
    source.push_str(
        r##"}

layout TestLayout {
    column gap(0) padding(0) {
        slot proof
    }
}

content TestContent {
    proof -> worth.surface.preview.primitive.proof
}

appearance TestTheme {}
"##,
    );
    source
}
