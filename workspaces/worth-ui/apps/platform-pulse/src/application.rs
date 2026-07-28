use eframe::egui;
use worth_ui::facade::app::WorthUiApplicationPreparationDenial;
use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor,
    ComponentHitTestContract, ComponentHitTestOrder, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, ComponentStaticPaintContract, ComponentStaticPaintOrder,
    ComponentViewportInset, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, ThemeColorValue, ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily,
    ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
};
use worth_ui::facade::inspection::{
    UiVisualInspectionByteBudget, UiVisualInspectionCapacity, UiVisualInspectionPolicy,
    UiVisualInspectionRegionCapacity,
};
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher,
    WorthUiFilesystemWatcherDenial, WorthUiSourcePackageRevision,
    WorthUiWatchedCandidateSubmissionDenial,
};
use worth_ui_host_egui::WorthUiHostEgui;

use crate::launch_configuration::AdmittedPlatformPulseLaunchConfiguration;
use worth_ui_platform_pulse::visual_identity_pulse::PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES;

const COMPONENT: &str = "platform.pulse.component.seed";
const IDENTITY_TARGET_COMPONENT: &str = "platform.pulse.component.identity_target";
const SURFACE: &str = "platform.pulse.surface.main";
const FILL_TOKEN: &str = "theme.platform_pulse.fill";
const IDENTITY_TARGET_FILL_TOKEN: &str = "theme.platform_pulse.identity_target_fill";
const BLUE_TOKEN: &str = "theme.platform_pulse.blue";
const GREEN_TOKEN: &str = "theme.platform_pulse.green";
const YELLOW_TOKEN: &str = "theme.platform_pulse.yellow";

pub(crate) struct PreparedPlatformPulse {
    pub(crate) app: WorthUiApp,
    pub(crate) host: WorthUiHostEgui,
    pub(crate) watcher: WorthUiFilesystemSourceWatcher,
    pub(crate) initial_source: WorthUiSourcePackageRevision,
}

#[derive(Debug)]
pub(crate) enum PlatformPulsePreparationDenial {
    WatcherStart(WorthUiFilesystemWatcherDenial),
    InitialSourceSettlement(WorthUiFilesystemWatcherDenial),
    CapabilityApplication(WorthUiApplicationPreparationDenial),
    InitialSourceLowering(WorthUiWatchedCandidateSubmissionDenial),
    FileApplication(WorthUiApplicationPreparationDenial),
}

pub(crate) fn prepare(
    context: egui::Context,
    launch: &AdmittedPlatformPulseLaunchConfiguration,
) -> Result<PreparedPlatformPulse, PlatformPulsePreparationDenial> {
    let provider = WorthUiFilesystemSourceProvider::new(launch.source_root());
    let mut watcher = WorthUiFilesystemSourceWatcher::start(provider)
        .map_err(PlatformPulsePreparationDenial::WatcherStart)?;
    let result = (|| {
        let snapshot = watcher
            .take_initial_snapshot()
            .map_err(PlatformPulsePreparationDenial::InitialSourceSettlement)?;
        let initial_source = snapshot.source_revision().clone();
        let host = WorthUiHostEgui::new(context);
        let capability_app = builder(host.clone())
            .freeze()
            .map_err(PlatformPulsePreparationDenial::CapabilityApplication)?;
        let submission = snapshot
            .lower_to_candidate_submission(capability_app.capabilities())
            .map_err(PlatformPulsePreparationDenial::InitialSourceLowering)?;
        builder(host.clone())
            .with_candidate_submission(submission)
            .freeze()
            .map_err(PlatformPulsePreparationDenial::FileApplication)
            .map(|app| (app, host, initial_source))
    })();
    match result {
        Ok((app, host, initial_source)) => Ok(PreparedPlatformPulse {
            app,
            host,
            watcher,
            initial_source,
        }),
        Err(denial) => {
            let _ = watcher.shutdown();
            Err(denial)
        }
    }
}

fn builder(host: WorthUiHostEgui) -> WorthUiApplicationBuilder {
    let builder = register_pulse_structure(WorthUi::app().with_host(host));
    register_pulse_theme_tokens(builder).with_visual_inspection_policy(visual_inspection_policy())
}

fn visual_inspection_policy() -> UiVisualInspectionPolicy {
    UiVisualInspectionPolicy::bounded(
        worth_ui::facade::inspection::UiVisualInspectionDisclosure::local_development_unredacted(),
        UiVisualInspectionCapacity::bounded(1, 8, 16),
        UiVisualInspectionRegionCapacity::bounded(65_536, 65_536),
        UiVisualInspectionByteBudget::bounded(
            PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES,
            PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES,
            256 << 10,
            256 << 10,
        ),
    )
    .expect("the permanent pulse declares a valid bounded visual policy")
}

fn register_pulse_structure(builder: WorthUiApplicationBuilder) -> WorthUiApplicationBuilder {
    let background_allocation = ComponentAllocationMeasurementContract::fill_viewport();
    let target_allocation = ComponentAllocationMeasurementContract::viewport_inset(
        ComponentViewportInset::symmetric(48, 24),
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
                    ComponentHitTestOrder::front_to_back(1),
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
                    ComponentHitTestOrder::front_to_back(0),
                    target_allocation,
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

fn register_pulse_theme_tokens(builder: WorthUiApplicationBuilder) -> WorthUiApplicationBuilder {
    builder
        .register_theme_token(ThemeTokenDescriptor::define(
            token_id(YELLOW_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(
                ThemeColorValue::hex("#f2cc60").expect("valid pulse target color"),
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
            token_id(FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(BLUE_TOKEN)),
        ))
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
