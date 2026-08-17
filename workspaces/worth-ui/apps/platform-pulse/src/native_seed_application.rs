use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, ComponentStaticPaintContract,
    ComponentStaticPaintOrder, ComponentViewportInset, ThemeColorValue, ThemeTokenDescriptor,
    ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativeApplicationProgram,
};

const COMPONENT: &str = "platform.pulse.native_seed.rectangle";
const TOKEN: &str = "theme.platform_pulse.native_seed.blue";

/// Phase 2's text-free public-composition seed.
///
/// It proves the first real native vertical without pretending that the later
/// text, Query, intent, and cumulative parity phases already exist.
pub struct PlatformPulseNativeSeedApplication;

impl PlatformPulseNativeSeedApplication {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for PlatformPulseNativeSeedApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl UiNativeApplicationDefinition for PlatformPulseNativeSeedApplication {
    fn prepare(
        self,
        mut preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        let result = (|| {
            let mut builder = preparation.builder();
            builder
                .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())?;
            builder.register_theme_token(theme_token())?;
            builder.register_component(component())?;
            builder.with_rust_authored_input(authored_input())?;
            drop(builder);
            preparation.install_frame_program(
                UiNativeApplicationProgram::single_frame().remain_open_until_external_close(),
            )
        })();
        match result {
            Ok(()) => preparation.complete(),
            Err(cause) => preparation.deny(cause),
        }
    }
}

fn theme_token() -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        ThemeTokenId::new(TOKEN).expect("valid native seed token"),
        ThemeTokenFamily::surface(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(ThemeColorValue::hex("#2f81f7").expect("qualified blue")),
    )
}

fn component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(COMPONENT).expect("valid native seed component"),
        ComponentPropSchema::named("platform.pulse.native_seed.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_static_paint(
        ComponentStaticPaintContract::opaque_fill(
            ThemeTokenId::new(TOKEN).expect("valid native seed token"),
            ComponentStaticPaintOrder::back_to_front(0),
        ),
        ComponentAllocationMeasurementContract::viewport_inset(ComponentViewportInset::symmetric(
            16, 12,
        )),
    )
}

fn authored_input() -> WorthUiRustAuthoredArtifactInput {
    WorthUiRustAuthoredArtifactInput::from_modules([WorthUiRustAuthoredArtifactInputModule::new(
        "app/native_seed.wui",
    )
    .with_token(TOKEN, "#2f81f7")
    .with_component_authored_identity(COMPONENT, "platform-pulse-native-seed")])
}
