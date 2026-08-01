use worth_ui::facade::app::{WorthUi, WorthUiApplicationBuilder};
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor,
    ComponentHitTestContract, ComponentHitTestInset, ComponentHitTestOrder, ComponentId,
    ComponentPropSchema, ComponentSemanticTextContract, ComponentStateOwnership,
    ComponentStaticPaintContract, ComponentStaticPaintOrder, ComponentViewportInset,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId,
    ThemeTokenSource, ThemeTokenValue,
};
use worth_ui_runtime::facade::host::WorthUiOperationalHostAdapter;

pub(crate) const VISUAL_PAINT_ONLY_COMPONENT: &str = "visual.identity.component.paint_only";
pub(crate) const VISUAL_HIT_ONLY_COMPONENT: &str = "visual.identity.component.hit_only";
pub(crate) const VISUAL_PAINT_AND_HIT_COMPONENT: &str = "visual.identity.component.paint_and_hit";
pub(crate) const VISUAL_NEITHER_COMPONENT: &str = "visual.identity.component.neither";
pub(crate) const VISUAL_IDENTITY_SURFACE: &str = "visual.identity.surface.main";

pub(crate) const VISUAL_PAINT_ONLY_TOKEN: &str = "theme.visual_identity.paint_only";
pub(crate) const VISUAL_PAINT_AND_HIT_TOKEN: &str = "theme.visual_identity.paint_and_hit";
pub(crate) const VISUAL_RED_TOKEN: &str = "theme.visual_identity.red";
pub(crate) const VISUAL_PURPLE_TOKEN: &str = "theme.visual_identity.purple";

pub(crate) fn visual_identity_application_builder_with_host<Host>(
    host: Host,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder(
        host,
        ComponentHitTestOrder::front_to_back(0),
        ComponentStaticPaintOrder::back_to_front(7),
        None,
        false,
    )
}

pub(crate) fn duplicate_hit_order_application_builder_with_host<Host>(
    host: Host,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder(
        host,
        ComponentHitTestOrder::front_to_back(1),
        ComponentStaticPaintOrder::back_to_front(7),
        None,
        false,
    )
}

pub(crate) fn region_identity_application_builder_with_host<Host>(
    host: Host,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder(
        host,
        ComponentHitTestOrder::front_to_back(0),
        ComponentStaticPaintOrder::back_to_front(1),
        None,
        false,
    )
}

pub(crate) fn clipped_visual_identity_application_builder_with_host<Host>(
    host: Host,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder(
        host,
        ComponentHitTestOrder::front_to_back(0),
        ComponentStaticPaintOrder::back_to_front(7),
        Some(ComponentHitTestInset::symmetric(12, 8)),
        false,
    )
}

pub(crate) fn clipped_semantic_text_action_application_builder_with_host<Host>(
    host: Host,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder(
        host,
        ComponentHitTestOrder::front_to_back(0),
        ComponentStaticPaintOrder::back_to_front(7),
        Some(ComponentHitTestInset::symmetric(12, 8)),
        true,
    )
}

pub(crate) fn clipped_semantic_text_action_application_builder_with_host_and_profile<Host>(
    host: Host,
    profile: worth_ui::facade::rebind::UiChangeProfile,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder_with_profile(
        host,
        ComponentHitTestOrder::front_to_back(0),
        ComponentStaticPaintOrder::back_to_front(7),
        Some(ComponentHitTestInset::symmetric(12, 8)),
        true,
        profile,
    )
}

fn visual_identity_builder<Host>(
    host: Host,
    paint_and_hit_order: ComponentHitTestOrder,
    paint_only_order: ComponentStaticPaintOrder,
    hit_only_clip: Option<ComponentHitTestInset>,
    paint_and_hit_semantic_text: bool,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    visual_identity_builder_with_profile(
        host,
        paint_and_hit_order,
        paint_only_order,
        hit_only_clip,
        paint_and_hit_semantic_text,
        worth_ui::facade::rebind::UiChangeProfile::platform_pulse(),
    )
}

fn visual_identity_builder_with_profile<Host>(
    host: Host,
    paint_and_hit_order: ComponentHitTestOrder,
    paint_only_order: ComponentStaticPaintOrder,
    hit_only_clip: Option<ComponentHitTestInset>,
    paint_and_hit_semantic_text: bool,
    profile: worth_ui::facade::rebind::UiChangeProfile,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    let hit_only_allocation = inset_allocation(8, 8);
    let paint_and_hit_allocation = inset_allocation(16, 12);
    let hit_only_contract = match hit_only_clip {
        Some(inset) => ComponentHitTestContract::allocation_bounds_clipped_by_inset(
            ComponentHitTestOrder::front_to_back(1),
            hit_only_allocation,
            inset,
        ),
        None => ComponentHitTestContract::allocation_bounds(
            ComponentHitTestOrder::front_to_back(1),
            hit_only_allocation,
        ),
    };
    let paint_and_hit = component(VISUAL_PAINT_AND_HIT_COMPONENT)
        .with_static_paint(
            ComponentStaticPaintContract::opaque_fill(
                token_id(VISUAL_PAINT_AND_HIT_TOKEN),
                ComponentStaticPaintOrder::back_to_front(3),
            ),
            paint_and_hit_allocation,
        )
        .with_hit_test(ComponentHitTestContract::allocation_bounds(
            paint_and_hit_order,
            paint_and_hit_allocation,
        ));
    let paint_and_hit = if paint_and_hit_semantic_text {
        paint_and_hit.with_semantic_text(ComponentSemanticTextContract::body_default(
            token_id(VISUAL_PAINT_AND_HIT_TOKEN),
            4,
        ))
    } else {
        paint_and_hit
    };
    WorthUi::app()
        .with_change_profile(profile)
        .with_host(host)
        .register_component(component(VISUAL_PAINT_ONLY_COMPONENT).with_static_paint(
            ComponentStaticPaintContract::opaque_fill(
                token_id(VISUAL_PAINT_ONLY_TOKEN),
                paint_only_order,
            ),
            ComponentAllocationMeasurementContract::fill_viewport(),
        ))
        .register_component(component(VISUAL_HIT_ONLY_COMPONENT).with_hit_test(hit_only_contract))
        .register_component(paint_and_hit)
        .register_component(component(VISUAL_NEITHER_COMPONENT))
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new(VISUAL_IDENTITY_SURFACE).expect("valid visual identity surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new(VISUAL_PAINT_ONLY_COMPONENT)
                .expect("valid visual identity root component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::ephemeral(),
        ))
        .register_theme_token(color_token(VISUAL_RED_TOKEN, "#cf222e"))
        .register_theme_token(color_token(VISUAL_PURPLE_TOKEN, "#8250df"))
        .register_theme_token(alias_token(VISUAL_PAINT_ONLY_TOKEN, VISUAL_RED_TOKEN))
        .register_theme_token(alias_token(VISUAL_PAINT_AND_HIT_TOKEN, VISUAL_PURPLE_TOKEN))
}

fn component(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).expect("valid visual identity component id"),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn inset_allocation(horizontal: u16, vertical: u16) -> ComponentAllocationMeasurementContract {
    ComponentAllocationMeasurementContract::viewport_inset(ComponentViewportInset::symmetric(
        horizontal, vertical,
    ))
}

fn color_token(id: &str, color: &str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        token_id(id),
        ThemeTokenFamily::surface(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(
            ThemeColorValue::hex(color).expect("valid visual identity theme color"),
        ),
    )
}

fn alias_token(id: &str, target: &str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::alias(
        token_id(id),
        ThemeTokenFamily::surface(),
        ThemeTokenSource::application(),
        ThemeTokenAlias::to(token_id(target)),
    )
}

fn token_id(id: &str) -> ThemeTokenId {
    ThemeTokenId::new(id).expect("valid visual identity theme token id")
}
