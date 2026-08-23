use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentSemanticTextContract, ComponentStateOwnership,
    ComponentStaticPaintContract, ComponentStaticPaintOrder, ComponentViewportInset,
    SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass,
    ThemeColorValue, ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource,
    ThemeTokenValue, WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_native_platform::{
    UiNativeApplicationDefinition, UiNativeApplicationFrame, UiNativeApplicationPreparation,
    UiNativeApplicationPreparationOutcome, UiNativeApplicationProgram,
    UiNativeComponentSemanticTextChange, UiNativeThemeTokenValueChange,
};

const ROOT: &str = "phase.f.root";
const TEXT: &str = "phase.f.text";
const SURFACE: &str = "phase.f.surface";
const ROOT_TOKEN: &str = "theme.phase_f.root";
const TEXT_TOKEN: &str = "theme.phase_f.text";

pub(crate) struct PlatformPulseNativePhaseFApplication {
    presentation_async: worth_ui::facade::query_binding::WorthUiPresentationAsyncInstallation,
    remain_open_until_external_close: bool,
    program_mode: PhaseFProgramMode,
}

#[derive(Clone, Copy)]
enum PhaseFProgramMode {
    TransitionCourtroom,
    SingleCurrent,
    ReconstructCurrent,
    PartialEffectsCancellation,
}

impl PlatformPulseNativePhaseFApplication {
    pub(crate) fn new(
        presentation_async: worth_ui::facade::query_binding::WorthUiPresentationAsyncInstallation,
    ) -> Self {
        Self {
            presentation_async,
            remain_open_until_external_close: false,
            program_mode: PhaseFProgramMode::TransitionCourtroom,
        }
    }

    pub(crate) fn single_current_frame(mut self) -> Self {
        self.program_mode = PhaseFProgramMode::SingleCurrent;
        self
    }

    pub(crate) fn reconstruct_current_frame(mut self) -> Self {
        self.program_mode = PhaseFProgramMode::ReconstructCurrent;
        self
    }

    pub(crate) fn cancel_after_external_submission(mut self) -> Self {
        self.program_mode = PhaseFProgramMode::PartialEffectsCancellation;
        self
    }

    pub(crate) fn remain_open_until_external_close(mut self) -> Self {
        self.remain_open_until_external_close = true;
        self
    }
}

impl UiNativeApplicationDefinition for PlatformPulseNativePhaseFApplication {
    fn prepare(
        self,
        mut preparation: UiNativeApplicationPreparation,
    ) -> UiNativeApplicationPreparationOutcome {
        let result = (|| {
            preparation.install_presentation_async(self.presentation_async)?;
            preparation.install_frame_program(phase_f_program(
                self.remain_open_until_external_close,
                self.program_mode,
            ))?;
            let mut builder = preparation.builder();
            builder
                .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())?;
            builder.register_theme_token(root_token())?;
            builder.register_theme_token(text_token())?;
            builder.register_component(root_component())?;
            builder.register_component(text_component())?;
            builder.register_surface(SurfaceDescriptor::new(
                SurfaceId::new(SURFACE).expect("Phase F surface identity"),
                SurfaceKind::primary_content(),
                ComponentId::new(ROOT).expect("Phase F root identity"),
                SurfacePlacementClass::primary_region(),
                SurfaceStateClass::ephemeral(),
            ))?;
            builder.with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([
                WorthUiRustAuthoredArtifactInputModule::new("app/phase_f_async.wui")
                    .with_token(ROOT_TOKEN, "#17202a")
                    .with_token(TEXT_TOKEN, "#ffffff")
                    .with_component_authored_identity(ROOT, "phase-f-root")
                    .with_component_authored_identity(TEXT, "phase-f-text")
                    .with_surface_authored_identity(SURFACE, "phase-f-surface"),
            ]))
        })();
        match result {
            Ok(()) => preparation.complete(),
            Err(cause) => preparation.deny(cause),
        }
    }
}

fn phase_f_program(
    remain_open_until_external_close: bool,
    program_mode: PhaseFProgramMode,
) -> UiNativeApplicationProgram {
    let frames = match program_mode {
        PhaseFProgramMode::SingleCurrent => vec![text_frame("ASYNC-CURRENT")],
        PhaseFProgramMode::ReconstructCurrent => {
            vec![
                text_frame("RECONSTRUCT-CURRENT"),
                UiNativeApplicationFrame::present_current(),
                text_frame("RECONSTRUCT-CURRENT!"),
            ]
        }
        PhaseFProgramMode::PartialEffectsCancellation => vec![
            text_frame("CANCELLATION-PREDECESSOR"),
            text_frame("CANCEL-AFTER-EXTERNAL-SUBMISSION").cancel_after_external_submission(),
        ],
        PhaseFProgramMode::TransitionCourtroom => vec![
            text_frame("ASYNC-A"),
            text_paint_frame("#d8e8ff", 0).superseding_pending(),
            text_paint_frame("#ffffff", 1),
        ],
    };
    let program =
        UiNativeApplicationProgram::new(frames).expect("Phase F transition program is bounded");
    if remain_open_until_external_close {
        program.remain_open_until_external_close()
    } else {
        program
    }
}

fn text_frame(text: &str) -> UiNativeApplicationFrame {
    UiNativeApplicationFrame::with_semantic_text([UiNativeComponentSemanticTextChange::new(
        format!("component:{TEXT}"),
        format!("{text} \u{1f469}\u{200d}\u{1f4bb}"),
    )
    .expect("Phase F text change")])
    .expect("Phase F text frame")
}

fn text_paint_frame(color: &str, expected_revision: u64) -> UiNativeApplicationFrame {
    UiNativeApplicationFrame::with_theme_token_values([UiNativeThemeTokenValueChange::successor(
        ThemeTokenId::new(TEXT_TOKEN).expect("Phase F text token"),
        expected_revision,
        ThemeTokenValue::color(ThemeColorValue::hex(color).expect("Phase F text paint value")),
    )
    .expect("Phase F text paint change")])
    .expect("Phase F text paint frame")
}

fn root_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(ROOT).expect("Phase F root identity"),
        ComponentPropSchema::named("phase.f.root.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_static_paint(
        ComponentStaticPaintContract::opaque_fill(
            ThemeTokenId::new(ROOT_TOKEN).expect("Phase F root token"),
            ComponentStaticPaintOrder::back_to_front(0),
        ),
        ComponentAllocationMeasurementContract::fill_viewport(),
    )
}

fn text_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(TEXT).expect("Phase F text identity"),
        ComponentPropSchema::named("phase.f.text.props"),
        ComponentChildPolicy::text_children(),
        ComponentStateOwnership::runtime_owned(),
    )
    .with_allocation_measurement_contract(ComponentAllocationMeasurementContract::viewport_inset(
        ComponentViewportInset::symmetric(16, 16),
    ))
    .with_semantic_text(ComponentSemanticTextContract::body_default(
        ThemeTokenId::new(TEXT_TOKEN).expect("Phase F text token"),
        1,
    ))
}

fn root_token() -> ThemeTokenDescriptor {
    color_token(ROOT_TOKEN, "#17202a")
}

fn text_token() -> ThemeTokenDescriptor {
    color_token(TEXT_TOKEN, "#ffffff")
}

fn color_token(identity: &str, value: &str) -> ThemeTokenDescriptor {
    ThemeTokenDescriptor::define(
        ThemeTokenId::new(identity).expect("Phase F token identity"),
        ThemeTokenFamily::surface(),
        ThemeTokenSource::application(),
        ThemeTokenValue::color(ThemeColorValue::hex(value).expect("Phase F token color")),
    )
}
