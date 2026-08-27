use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, ComponentStaticPaintContract,
    ComponentStaticPaintOrder, ComponentViewportInset, ThemeColorValue, ThemeTokenDescriptor,
    ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationFrame, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativeApplicationProgram,
    UiNativeThemeTokenValueChange,
};

const COMPONENT: &str = "platform.pulse.native_seed.rectangle";
const TOKEN: &str = "theme.platform_pulse.native_seed.blue";
const POST_RESTORE_COLOR: &str = "#3fb950";

/// Phase 2's text-free public-composition seed.
///
/// It proves the first real native vertical without pretending that the later
/// text, Query, intent, and cumulative parity phases already exist.
pub struct PlatformPulseNativeSeedApplication {
    program: NativeSeedProgram,
}

#[derive(Clone, Copy)]
enum NativeSeedProgram {
    Ordinary,
    CaptureInitial,
    CaptureAfterSurfaceSuccessor,
}

impl PlatformPulseNativeSeedApplication {
    pub const fn new() -> Self {
        Self {
            program: NativeSeedProgram::Ordinary,
        }
    }

    pub const fn with_presented_source_capture(mut self) -> Self {
        self.program = NativeSeedProgram::CaptureInitial;
        self
    }

    pub const fn with_surface_successor_capture(mut self) -> Self {
        self.program = NativeSeedProgram::CaptureAfterSurfaceSuccessor;
        self
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
            if !matches!(self.program, NativeSeedProgram::Ordinary) {
                builder.with_visual_inspection_policy(visual_inspection_policy())?;
            }
            builder.register_theme_token(theme_token())?;
            builder.register_component(component())?;
            builder.with_rust_authored_input(authored_input())?;
            drop(builder);
            let program = match self.program {
                NativeSeedProgram::Ordinary => UiNativeApplicationProgram::single_frame(),
                NativeSeedProgram::CaptureInitial => {
                    UiNativeApplicationProgram::new([UiNativeApplicationFrame::present_current()
                        .capture_presented_source_pixels()])
                    .expect("the seed admits one bounded capture")
                }
                NativeSeedProgram::CaptureAfterSurfaceSuccessor => {
                    UiNativeApplicationProgram::new([
                        UiNativeApplicationFrame::present_current(),
                        UiNativeApplicationFrame::present_current()
                            .after_host_surface_basis_successor()
                            .capture_presented_source_pixels(),
                        post_restore_frame(),
                    ])
                    .expect("the seed admits one capture across two surface successors")
                }
            };
            preparation.install_frame_program(program.remain_open_until_external_close())
        })();
        match result {
            Ok(()) => preparation.complete(),
            Err(cause) => preparation.deny(cause),
        }
    }
}

fn post_restore_frame() -> UiNativeApplicationFrame {
    UiNativeApplicationFrame::with_theme_token_values([UiNativeThemeTokenValueChange::new(
        ThemeTokenId::new(TOKEN).expect("valid native seed token"),
        ThemeTokenValue::color(
            ThemeColorValue::hex(POST_RESTORE_COLOR).expect("qualified post-restore color"),
        ),
    )
    .expect("the post-restore theme successor is valid")])
    .expect("the seed admits one bounded post-restore theme change")
    .after_host_surface_basis_successor()
}

fn visual_inspection_policy() -> worth_ui::facade::inspection::UiVisualInspectionPolicy {
    worth_ui::facade::inspection::UiVisualInspectionPolicy::bounded(
        worth_ui::facade::inspection::UiVisualInspectionDisclosure::local_development_unredacted(),
        worth_ui::facade::inspection::UiVisualInspectionCapacity::bounded(1, 1, 1),
        worth_ui::facade::inspection::UiVisualInspectionRegionCapacity::bounded(4, 4),
        worth_ui::facade::inspection::UiVisualInspectionByteBudget::bounded(
            16 * 1024 * 1024,
            16 * 1024 * 1024,
            64 * 1024,
            64 * 1024,
        ),
    )
    .expect("the seed declares a bounded visual inspection policy")
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
