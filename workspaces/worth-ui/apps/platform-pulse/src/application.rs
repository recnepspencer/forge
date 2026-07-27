use eframe::egui;
use worth_ui::facade::app::WorthUiApplicationPreparationDenial;
use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};
use worth_ui::facade::declaration::{
    ComponentAllocationMeasurementContract, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, SurfaceDescriptor, SurfaceId, SurfaceKind,
    SurfacePlacementClass, SurfaceStateClass, ThemeColorValue, ThemeTokenAlias,
    ThemeTokenDescriptor, ThemeTokenFamily, ThemeTokenId, ThemeTokenSource, ThemeTokenValue,
};
use worth_ui::facade::source::{
    WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher,
    WorthUiFilesystemWatcherDenial, WorthUiSourcePackageRevision,
    WorthUiWatchedCandidateSubmissionDenial,
};
use worth_ui_host_egui::WorthUiHostEgui;

use crate::launch_configuration::AdmittedPlatformPulseLaunchConfiguration;

const COMPONENT: &str = "platform.pulse.component.seed";
const SURFACE: &str = "platform.pulse.surface.main";
const FILL_TOKEN: &str = "theme.platform_pulse.fill";
const BLUE_TOKEN: &str = "theme.platform_pulse.blue";
const GREEN_TOKEN: &str = "theme.platform_pulse.green";

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
    WorthUi::app()
        .with_host(host)
        .register_component(
            component()
                .with_theme_token_dependency(token_id(FILL_TOKEN))
                .with_allocation_measurement_contract(
                    ComponentAllocationMeasurementContract::fill_viewport(),
                ),
        )
        .register_surface(SurfaceDescriptor::new(
            SurfaceId::new(SURFACE).expect("valid pulse surface id"),
            SurfaceKind::primary_content(),
            ComponentId::new(COMPONENT).expect("valid pulse component id"),
            SurfacePlacementClass::primary_region(),
            SurfaceStateClass::ephemeral(),
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
            token_id(FILL_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(BLUE_TOKEN)),
        ))
}

fn component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(COMPONENT).expect("valid pulse component id"),
        ComponentPropSchema::named("platform.pulse.component.seed.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn token_id(text: &str) -> ThemeTokenId {
    ThemeTokenId::new(text).expect("valid pulse theme token id")
}
