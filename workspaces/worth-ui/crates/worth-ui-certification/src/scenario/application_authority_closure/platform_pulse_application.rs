use worth_ui::facade::app::{WorthUi, WorthUiApplicationBuilder};
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, ComponentStaticPaintContract,
    ComponentStaticPaintOrder, ComponentViewportInset, SurfaceDescriptor, SurfaceId, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass, ThemeColorValue, ThemeTokenAlias,
    ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
};
use worth_ui_runtime::facade::host::WorthUiOperationalHostAdapter;

pub(crate) const PLATFORM_PULSE_BACKGROUND_COMPONENT: &str = "platform.pulse.component.seed";
pub(crate) const PLATFORM_PULSE_IDENTITY_TARGET_COMPONENT: &str =
    "platform.pulse.component.identity_target";
pub(crate) const PLATFORM_PULSE_SURFACE: &str = "platform.pulse.surface.main";
pub(crate) const PLATFORM_PULSE_FILL_TOKEN: &str = "theme.platform_pulse.fill";
pub(crate) const PLATFORM_PULSE_IDENTITY_TARGET_FILL_TOKEN: &str =
    "theme.platform_pulse.identity_target_fill";
pub(crate) const PLATFORM_PULSE_BLUE_TOKEN: &str = "theme.platform_pulse.blue";
pub(crate) const PLATFORM_PULSE_GREEN_TOKEN: &str = "theme.platform_pulse.green";
pub(crate) const PLATFORM_PULSE_YELLOW_TOKEN: &str = "theme.platform_pulse.yellow";
pub(crate) const PLATFORM_PULSE_TARGET_HORIZONTAL_INSET: u16 = 48;
pub(crate) const PLATFORM_PULSE_TARGET_VERTICAL_INSET: u16 = 24;

pub(crate) fn platform_pulse_application_builder_with_host<Host>(
    host: Host,
) -> WorthUiApplicationBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    WorthUi::app()
        .with_host(host)
        .register_component(
            component(PLATFORM_PULSE_BACKGROUND_COMPONENT).with_static_paint(
                ComponentStaticPaintContract::opaque_fill(
                    token_id(PLATFORM_PULSE_FILL_TOKEN),
                    ComponentStaticPaintOrder::back_to_front(0),
                ),
                ComponentAllocationMeasurementContract::fill_viewport(),
            ),
        )
        .register_component(
            component(PLATFORM_PULSE_IDENTITY_TARGET_COMPONENT).with_static_paint(
                ComponentStaticPaintContract::opaque_fill(
                    token_id(PLATFORM_PULSE_IDENTITY_TARGET_FILL_TOKEN),
                    ComponentStaticPaintOrder::back_to_front(1),
                ),
                ComponentAllocationMeasurementContract::viewport_inset(
                    ComponentViewportInset::symmetric(
                        PLATFORM_PULSE_TARGET_HORIZONTAL_INSET,
                        PLATFORM_PULSE_TARGET_VERTICAL_INSET,
                    ),
                ),
            ),
        )
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new(PLATFORM_PULSE_SURFACE).expect("valid platform pulse surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new(PLATFORM_PULSE_BACKGROUND_COMPONENT)
                .expect("valid platform pulse background component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::ephemeral(),
        ))
        .register_theme_token(color_token(PLATFORM_PULSE_YELLOW_TOKEN, "#f2cc60"))
        .register_theme_token(color_token(PLATFORM_PULSE_BLUE_TOKEN, "#2f81f7"))
        .register_theme_token(color_token(PLATFORM_PULSE_GREEN_TOKEN, "#3fb950"))
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(PLATFORM_PULSE_IDENTITY_TARGET_FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(PLATFORM_PULSE_YELLOW_TOKEN)),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(PLATFORM_PULSE_FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(PLATFORM_PULSE_BLUE_TOKEN)),
        ))
}

fn component(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).expect("valid platform pulse component id"),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn color_token(id: &str, color: &str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        token_id(id),
        ThemeTokenFamily::surface(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(
            ThemeColorValue::hex(color).expect("valid platform pulse theme color"),
        ),
    )
}

fn token_id(id: &str) -> ThemeTokenId {
    ThemeTokenId::new(id).expect("valid platform pulse theme token id")
}
