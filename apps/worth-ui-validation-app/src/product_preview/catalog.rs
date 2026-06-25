use worth_ui::facade::{
    ComponentChildPolicy, ComponentDescriptor, ComponentExecutionLane, ComponentFocusSupport,
    ComponentId, ComponentPropSchema, ComponentStateOwnership, IconDescriptor, IconFamily, IconId,
    IconSourceDescriptor, ImageAssetDescriptor, ImageAssetId, SurfaceDescriptor, SurfaceId,
    SurfaceKind, SurfacePlacementClass, SurfaceStateClass, WorthUiAppBuilder,
};

pub(crate) const PREVIEW_DEFAULT_PAGE: &str = "HeaderProofPage";

const PREVIEW_COMPONENT_IDS: &[(&str, bool)] = &[
    ("worth.component.button", true),
    ("worth.component.primitive_proof", false),
];

const PREVIEW_ICONS: &[(&str, &str, &str)] = &[
    ("worth.icon.action.plus", "toolbar", "plus"),
    ("worth.icon.action.check", "toolbar", "check"),
    ("worth.icon.navigation.search", "navigation", "search"),
    ("worth.icon.navigation.layers", "navigation", "layers-3"),
    ("worth.icon.navigation.grid", "navigation", "grid-2x2"),
    ("worth.icon.status.info", "status", "info"),
    ("worth.icon.status.warning", "status", "warning"),
    ("worth.icon.toolbar.edit", "toolbar", "pencil-line"),
    ("worth.icon.toolbar.delete", "toolbar", "trash-2"),
];

const PREVIEW_IMAGES: &[(&str, &str, u16, u16)] =
    &[("worth.image.logo", "validation-logo.png", 64, 40)];

const PREVIEW_SURFACES: &[(&str, &str)] = &[
    (
        "worth.surface.preview.button.proof",
        "worth.component.button",
    ),
    (
        "worth.surface.preview.primitive.proof",
        "worth.component.primitive_proof",
    ),
    (
        "worth.surface.preview.primitive.inner",
        "worth.component.primitive_proof",
    ),
];

pub(crate) fn register_preview_component_capabilities(
    mut builder: WorthUiAppBuilder,
) -> WorthUiAppBuilder {
    for (id, interactive) in PREVIEW_COMPONENT_IDS {
        let descriptor = preview_component_descriptor(id, *interactive);
        builder = builder.register_component(descriptor);
    }
    builder
}

pub(crate) fn register_preview_icon_capabilities(
    mut builder: WorthUiAppBuilder,
) -> WorthUiAppBuilder {
    for (id, family, source_key) in PREVIEW_ICONS {
        builder = builder.register_icon(IconDescriptor::new(
            IconId::new(*id).expect("valid preview icon id"),
            preview_icon_family(family),
            IconSourceDescriptor::symbol(*source_key),
        ));
    }
    builder
}

pub(crate) fn register_preview_image_asset_capabilities(
    mut builder: WorthUiAppBuilder,
) -> WorthUiAppBuilder {
    for (id, source_key, width, height) in PREVIEW_IMAGES {
        builder = builder.register_image_asset(ImageAssetDescriptor::local_static(
            ImageAssetId::new(*id).expect("valid preview image asset id"),
            *source_key,
            *width,
            *height,
        ));
    }
    builder
}

pub(crate) fn register_preview_surface_capabilities(
    mut builder: WorthUiAppBuilder,
) -> WorthUiAppBuilder {
    for (surface_id, component_id) in PREVIEW_SURFACES {
        builder = builder.register_surface(SurfaceDescriptor::new(
            SurfaceId::new(*surface_id).expect("valid preview surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new(*component_id).expect("valid preview component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::restorable(),
        ));
    }
    builder
}

fn preview_component_descriptor(id: &str, interactive: bool) -> ComponentDescriptor {
    let descriptor = ComponentDescriptor::new(
        ComponentId::new(id).expect("valid preview component id"),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    );
    if interactive {
        descriptor
            .with_focus(ComponentFocusSupport::focusable())
            .with_execution_lane(ComponentExecutionLane::Interactive)
    } else {
        descriptor
    }
}

fn preview_icon_family(family: &str) -> IconFamily {
    match family {
        "navigation" => IconFamily::navigation(),
        "status" => IconFamily::status(),
        "toolbar" => IconFamily::toolbar(),
        other => panic!("unknown preview icon family: {other}"),
    }
}
