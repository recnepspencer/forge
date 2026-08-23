use worth_ui::facade::app::{
    UiChangeProfileInstalled, UiIntentWiringSatisfied, WorthUiApplicationBuilder,
};
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor,
    ComponentHitTestContract, ComponentHitTestOrder, ComponentId, ComponentPropSchema,
    ComponentSemanticTextContract, ComponentStateOwnership, ComponentStaticPaintContract,
    ComponentStaticPaintOrder, ComponentViewportInset, SurfaceDescriptor, SurfaceId, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass, ThemeColorValue, ThemeTokenAlias,
    ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
};
use worth_ui::facade::inspection::{
    UiVisualInspectionByteBudget, UiVisualInspectionCapacity, UiVisualInspectionPolicy,
    UiVisualInspectionRegionCapacity,
};

use worth_ui_platform_pulse::visual_identity_pulse::PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES;

const COMPONENT: &str = "platform.pulse.component.seed";
const IDENTITY_TARGET_COMPONENT: &str = "platform.pulse.component.identity_target";
const PROJECTED_STATUS_COMPONENT: &str = "platform.pulse.component.projected_status";
const SURFACE: &str = "platform.pulse.surface.main";
const FILL_TOKEN: &str = "theme.platform_pulse.fill";
const IDENTITY_TARGET_FILL_TOKEN: &str = "theme.platform_pulse.identity_target_fill";
const CONFIRMATION_FILL_TOKEN: &str = "theme.platform_pulse.confirmation_fill";
const BLUE_TOKEN: &str = "theme.platform_pulse.blue";
const GREEN_TOKEN: &str = "theme.platform_pulse.green";
const YELLOW_TOKEN: &str = "theme.platform_pulse.yellow";
const WHITE_TOKEN: &str = "theme.platform_pulse.white";
const PURPLE_TOKEN: &str = "theme.platform_pulse.purple";
const TEXT_TOKEN: &str = "theme.platform_pulse.projected_status.text";
const PLATFORM_PULSE_RETAINED_PIXEL_BYTES: u64 = 2 * PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES;
const PLATFORM_PULSE_STRUCTURAL_BYTES_PER_RECEIPT: u64 = 256 << 10;
const PLATFORM_PULSE_RETAINED_STRUCTURAL_BYTES: u64 =
    2 * PLATFORM_PULSE_STRUCTURAL_BYTES_PER_RECEIPT;

pub(super) fn register_structure(
    builder: WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied>,
) -> WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied> {
    let background_allocation = ComponentAllocationMeasurementContract::fill_viewport();
    let target_allocation = ComponentAllocationMeasurementContract::viewport_inset(
        ComponentViewportInset::symmetric(48, 24),
    );
    let confirmation_allocation = ComponentAllocationMeasurementContract::viewport_inset(
        ComponentViewportInset::symmetric(64, 36),
    );
    builder
        .register_component(
            component()
                .with_static_paint(
                    ComponentStaticPaintContract::opaque_fill(
                        token_id(FILL_TOKEN),
                        ComponentStaticPaintOrder::back_to_front(0),
                    ),
                    background_allocation,
                )
                .with_hit_test(ComponentHitTestContract::allocation_bounds(
                    ComponentHitTestOrder::front_to_back(2),
                    background_allocation,
                )),
        )
        .register_component(
            component_named(IDENTITY_TARGET_COMPONENT)
                .with_static_paint(
                    ComponentStaticPaintContract::opaque_fill(
                        token_id(IDENTITY_TARGET_FILL_TOKEN),
                        ComponentStaticPaintOrder::back_to_front(1),
                    ),
                    target_allocation,
                )
                .with_hit_test(ComponentHitTestContract::allocation_bounds(
                    ComponentHitTestOrder::front_to_back(1),
                    target_allocation,
                ))
                .with_semantic_text(ComponentSemanticTextContract::body_default(
                    token_id(TEXT_TOKEN),
                    1,
                )),
        )
        .register_component(
            component_named(PROJECTED_STATUS_COMPONENT)
                .with_static_paint(
                    ComponentStaticPaintContract::opaque_fill(
                        token_id(CONFIRMATION_FILL_TOKEN),
                        ComponentStaticPaintOrder::back_to_front(2),
                    ),
                    confirmation_allocation,
                )
                .with_semantic_text(ComponentSemanticTextContract::body_default(
                    token_id(TEXT_TOKEN),
                    2,
                ))
                .with_hit_test(ComponentHitTestContract::allocation_bounds(
                    ComponentHitTestOrder::front_to_back(0),
                    confirmation_allocation,
                )),
        )
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new(SURFACE).expect("valid pulse surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new(COMPONENT).expect("valid pulse component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::ephemeral(),
        ))
}

pub(super) fn register_theme_tokens(
    builder: WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied>,
) -> WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied> {
    builder
        .register_theme_token(ThemeTokenDescriptor::define(
            token_id(WHITE_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(
                ThemeColorValue::hex("#ffffff").expect("valid Pulse text color"),
            ),
        ))
        .register_theme_token(ThemeTokenDescriptor::define(
            token_id(YELLOW_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(
                ThemeColorValue::hex("#f2cc60").expect("valid pulse target color"),
            ),
        ))
        .register_theme_token(ThemeTokenDescriptor::define(
            token_id(PURPLE_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(
                ThemeColorValue::hex("#6e40c9").expect("valid Pulse confirmation color"),
            ),
        ))
        .register_theme_token(ThemeTokenDescriptor::define(
            token_id(BLUE_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex("#2f81f7").expect("valid pulse blue")),
        ))
        .register_theme_token(ThemeTokenDescriptor::define(
            token_id(GREEN_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex("#3fb950").expect("valid pulse green")),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(IDENTITY_TARGET_FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(YELLOW_TOKEN)),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(CONFIRMATION_FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(PURPLE_TOKEN)),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(BLUE_TOKEN)),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(TEXT_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(WHITE_TOKEN)),
        ))
}

pub(super) fn visual_inspection_policy() -> UiVisualInspectionPolicy {
    UiVisualInspectionPolicy::bounded(
        worth_ui::facade::inspection::UiVisualInspectionDisclosure::local_development_unredacted(),
        UiVisualInspectionCapacity::bounded(2, 8, 16),
        UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
        UiVisualInspectionByteBudget::bounded(
            PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES,
            PLATFORM_PULSE_RETAINED_PIXEL_BYTES,
            PLATFORM_PULSE_STRUCTURAL_BYTES_PER_RECEIPT,
            PLATFORM_PULSE_RETAINED_STRUCTURAL_BYTES,
        ),
    )
    .expect("the permanent pulse declares a valid bounded visual policy")
}

fn component() -> ComponentDescriptor {
    component_named(COMPONENT)
}

fn component_named(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).expect("valid pulse component id"),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn token_id(text: &str) -> ThemeTokenId {
    ThemeTokenId::new(text).expect("valid pulse theme token id")
}
