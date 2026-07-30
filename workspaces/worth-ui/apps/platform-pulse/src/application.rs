use eframe::egui;
use worth_ui::facade::app::WorthUiApplicationPreparationDenial;
use worth_ui::facade::app::{WorthUi, WorthUiApp, WorthUiApplicationBuilder};
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
use worth_ui::facade::query_binding::WorthUiProjectionRegistrationError;
use worth_ui::facade::source::{
    UiSourceRebindAttemptFailure, WorthUiFilesystemSourceProvider, WorthUiFilesystemSourceWatcher,
    WorthUiFilesystemWatcherDenial, WorthUiSourcePackageRevision,
};
use worth_ui_host_egui::WorthUiHostEgui;

use crate::launch_configuration::AdmittedPlatformPulseLaunchConfiguration;
use crate::query_source::{
    InstalledPlatformPulseQuery, PlatformPulseExternalValueWatch,
    PlatformPulseQueryInstallationDenial, PlatformPulseQueryLifecycle,
};
use worth_ui_platform_pulse::visual_identity_pulse::PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES;

const COMPONENT: &str = "platform.pulse.component.seed";
const IDENTITY_TARGET_COMPONENT: &str = "platform.pulse.component.identity_target";
const PROJECTED_STATUS_COMPONENT: &str = "platform.pulse.component.projected_status";
const SURFACE: &str = "platform.pulse.surface.main";
const FILL_TOKEN: &str = "theme.platform_pulse.fill";
const IDENTITY_TARGET_FILL_TOKEN: &str = "theme.platform_pulse.identity_target_fill";
const BLUE_TOKEN: &str = "theme.platform_pulse.blue";
const GREEN_TOKEN: &str = "theme.platform_pulse.green";
const YELLOW_TOKEN: &str = "theme.platform_pulse.yellow";
const WHITE_TOKEN: &str = "theme.platform_pulse.white";
const TEXT_TOKEN: &str = "theme.platform_pulse.projected_status.text";
const PLATFORM_PULSE_RETAINED_PIXEL_BYTES: u64 = 2 * PLATFORM_PULSE_MAXIMUM_PIXEL_BYTES;
const PLATFORM_PULSE_STRUCTURAL_BYTES_PER_RECEIPT: u64 = 256 << 10;
const PLATFORM_PULSE_RETAINED_STRUCTURAL_BYTES: u64 =
    2 * PLATFORM_PULSE_STRUCTURAL_BYTES_PER_RECEIPT;

pub(crate) struct PreparedPlatformPulse {
    pub(crate) app: WorthUiApp,
    pub(crate) host: WorthUiHostEgui,
    pub(crate) watcher: WorthUiFilesystemSourceWatcher,
    pub(crate) initial_source: WorthUiSourcePackageRevision,
    pub(crate) query_lifecycle: PlatformPulseQueryLifecycle,
    pub(crate) query_watcher: PlatformPulseExternalValueWatch,
}

#[derive(Debug)]
pub(crate) enum PlatformPulsePreparationDenial {
    WatcherStart(WorthUiFilesystemWatcherDenial),
    InitialSourceSettlement(WorthUiFilesystemWatcherDenial),
    CapabilityApplication(WorthUiApplicationPreparationDenial),
    InitialSourceLowering(UiSourceRebindAttemptFailure),
    FileApplication(WorthUiApplicationPreparationDenial),
    QueryInstallation(PlatformPulseQueryInstallationDenial),
    QueryRegistration(WorthUiProjectionRegistrationError),
}

pub(crate) fn prepare(
    context: egui::Context,
    launch: &AdmittedPlatformPulseLaunchConfiguration,
) -> Result<PreparedPlatformPulse, PlatformPulsePreparationDenial> {
    let query = crate::query_source::install(launch.query_source_root())
        .map_err(PlatformPulsePreparationDenial::QueryInstallation)?;
    let provider = WorthUiFilesystemSourceProvider::new(launch.source_root());
    let mut watcher = match WorthUiFilesystemSourceWatcher::start(provider) {
        Ok(watcher) => watcher,
        Err(denial) => {
            let _ = query.watcher.shutdown();
            return Err(PlatformPulsePreparationDenial::WatcherStart(denial));
        }
    };
    let InstalledPlatformPulseQuery {
        registration,
        lifecycle: query_lifecycle,
        watcher: query_watcher,
    } = query;
    let result = (|| {
        let snapshot = watcher
            .take_initial_snapshot()
            .map_err(PlatformPulsePreparationDenial::InitialSourceSettlement)?;
        let initial_source = snapshot.source_revision().clone();
        let host = WorthUiHostEgui::new(context);
        let capability_app = builder(host.clone(), registration.clone())?
            .freeze()
            .map_err(PlatformPulsePreparationDenial::CapabilityApplication)?;
        let submission = snapshot
            .attempt_source_rebind(capability_app.capabilities())
            .into_candidate_submission()
            .map_err(PlatformPulsePreparationDenial::InitialSourceLowering)?;
        builder(host.clone(), registration)?
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
            query_lifecycle,
            query_watcher,
        }),
        Err(denial) => {
            let _ = watcher.shutdown();
            let _ = query_watcher.shutdown();
            Err(denial)
        }
    }
}

impl std::fmt::Display for PlatformPulsePreparationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WatcherStart(denial) => write!(formatter, "watcher start: {denial:?}"),
            Self::InitialSourceSettlement(denial) => {
                write!(formatter, "initial source settlement: {denial:?}")
            }
            Self::CapabilityApplication(denial) => {
                write!(formatter, "capability application: {denial:?}")
            }
            Self::InitialSourceLowering(denial) => {
                write!(formatter, "initial source lowering: {denial:?}")
            }
            Self::FileApplication(denial) => {
                write!(formatter, "file application: {denial:?}")
            }
            Self::QueryInstallation(denial) => {
                write!(formatter, "Query installation: {denial}")
            }
            Self::QueryRegistration(denial) => {
                write!(formatter, "Query registration: {denial:?}")
            }
        }
    }
}

fn builder(
    host: WorthUiHostEgui,
    registration: worth_ui::facade::query_binding::UiScalarProjectionRegistration,
) -> Result<WorthUiApplicationBuilder, PlatformPulsePreparationDenial> {
    let builder = register_pulse_structure(
        WorthUi::app()
            .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
            .with_host(host),
    );
    register_pulse_theme_tokens(builder)
        .register_scalar_projection(registration)
        .map(|builder| builder.with_visual_inspection_policy(visual_inspection_policy()))
        .map_err(PlatformPulsePreparationDenial::QueryRegistration)
}

fn visual_inspection_policy() -> UiVisualInspectionPolicy {
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
        .register_component(
            component_named(PROJECTED_STATUS_COMPONENT)
                .with_allocation_measurement_contract(
                    ComponentAllocationMeasurementContract::viewport_inset(
                        ComponentViewportInset::symmetric(12, 12),
                    ),
                )
                .with_semantic_text(ComponentSemanticTextContract::body_default(
                    token_id(TEXT_TOKEN),
                    2,
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
        .register_theme_token(ThemeTokenDescriptor::alias(
            token_id(TEXT_TOKEN),
            ThemeTokenFamily::surface(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(token_id(WHITE_TOKEN)),
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
