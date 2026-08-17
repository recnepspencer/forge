use eframe::egui;
use worth_ui::facade::app::WorthUiApplicationPreparationDenial;
use worth_ui::facade::app::{
    UiChangeProfileInstalled, UiIntentWiringSatisfied, WorthUi, WorthUiApp,
    WorthUiApplicationBuilder,
};
use worth_ui::facade::intent::{
    UiIntentApplicationFactRegistrationError, UiIntentDefinitionRegistrationError,
    UiIntentExecutionBindingPreparationDenial,
};
use worth_ui::facade::query_binding::{
    WorthUiInstalledQueryView, WorthUiProjectionRegistrationError,
    WorthUiQueryViewRegistrationError,
};
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
use worth_ui_platform_pulse::intent::{
    platform_pulse_action_confirmation_fact, platform_pulse_action_definition,
    platform_pulse_action_mutability_fact, platform_pulse_action_policy_fact,
    platform_pulse_action_readiness_fact, platform_pulse_action_revision_fact,
    PlatformPulseActionPortOwner, PlatformPulseActionProvider, PlatformPulseExecutorGate,
    PlatformPulseIntentInputInstallation, PlatformPulseIntentInputRecord,
    PlatformPulseIntentInputWatch, PlatformPulseIntentInputWatchDenial,
};
mod presentation;

use presentation::{register_structure, register_theme_tokens, visual_inspection_policy};

pub(crate) struct PreparedPlatformPulse {
    pub(crate) app: WorthUiApp,
    pub(crate) host: WorthUiHostEgui,
    pub(crate) watcher: WorthUiFilesystemSourceWatcher,
    pub(crate) initial_source: WorthUiSourcePackageRevision,
    pub(crate) query_lifecycle: PlatformPulseQueryLifecycle,
    pub(crate) query_watcher: PlatformPulseExternalValueWatch,
    pub(crate) intent_watcher: PlatformPulseIntentInputWatch,
    pub(crate) intent_gate: PlatformPulseExecutorGate,
    pub(crate) intent_action_owner: PlatformPulseActionPortOwner,
}

#[derive(Debug)]
pub(crate) enum PlatformPulsePreparationDenial {
    WatcherStart(WorthUiFilesystemWatcherDenial),
    InitialSourceSettlement(WorthUiFilesystemWatcherDenial),
    CapabilityApplication(WorthUiApplicationPreparationDenial),
    InitialSourceLowering(UiSourceRebindAttemptFailure),
    FileApplication(WorthUiApplicationPreparationDenial),
    QueryInstallation(Box<PlatformPulseQueryInstallationDenial>),
    QueryRegistration(WorthUiProjectionRegistrationError),
    QueryViewRegistration(WorthUiQueryViewRegistrationError),
    IntentInput(PlatformPulseIntentInputWatchDenial),
    IntentFact(UiIntentApplicationFactRegistrationError),
    IntentDefinition(UiIntentDefinitionRegistrationError),
    IntentProvider(UiIntentExecutionBindingPreparationDenial),
}

pub(crate) fn prepare(
    context: egui::Context,
    launch: &AdmittedPlatformPulseLaunchConfiguration,
) -> Result<PreparedPlatformPulse, PlatformPulsePreparationDenial> {
    let query = crate::query_source::install(launch.query_source_root())
        .map_err(|denial| PlatformPulsePreparationDenial::QueryInstallation(Box::new(denial)))?;
    let intent = match PlatformPulseIntentInputInstallation::open(launch.intent_source_root()) {
        Ok(intent) => intent,
        Err(denial) => {
            query.shutdown();
            return Err(PlatformPulsePreparationDenial::IntentInput(denial));
        }
    };
    let (intent_initial, intent_watcher) = intent.into_parts();
    let intent_gate =
        PlatformPulseExecutorGate::at(intent_initial.revision(), intent_initial.executor_held());
    let (intent_port, intent_action_owner) =
        worth_ui_platform_pulse::intent::PlatformPulseActionPort::bounded();
    let intent_provider = PlatformPulseActionProvider::new(intent_port, intent_gate.clone());
    let provider = WorthUiFilesystemSourceProvider::new(launch.source_root());
    let mut watcher = match WorthUiFilesystemSourceWatcher::start(provider) {
        Ok(watcher) => watcher,
        Err(denial) => {
            query.shutdown();
            let _ = intent_watcher.shutdown();
            return Err(PlatformPulsePreparationDenial::WatcherStart(denial));
        }
    };
    let InstalledPlatformPulseQuery {
        registration,
        action_view,
        lifecycle: query_lifecycle,
        watcher: query_watcher,
    } = query;
    let result = (|| {
        let snapshot = watcher
            .take_initial_snapshot()
            .map_err(PlatformPulsePreparationDenial::InitialSourceSettlement)?;
        let initial_source = snapshot.source_revision().clone();
        let host = WorthUiHostEgui::new(context);
        let capability_app = builder(
            registration.clone(),
            action_view.clone(),
            &intent_initial,
            intent_provider.clone(),
        )?
        .freeze()
        .map(|application| {
            worth_ui::facade::app::WorthUiLegacyEguiApplicationTransition::activate(
                application,
                host.clone(),
            )
        })
        .map_err(PlatformPulsePreparationDenial::CapabilityApplication)?;
        let submission = snapshot
            .attempt_source_rebind(capability_app.capabilities())
            .into_candidate_submission()
            .map_err(PlatformPulsePreparationDenial::InitialSourceLowering)?;
        builder(registration, action_view, &intent_initial, intent_provider)?
            .with_candidate_submission(submission)
            .freeze()
            .map(|application| {
                worth_ui::facade::app::WorthUiLegacyEguiApplicationTransition::activate(
                    application,
                    host.clone(),
                )
            })
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
            intent_watcher,
            intent_gate,
            intent_action_owner,
        }),
        Err(denial) => {
            let _ = watcher.shutdown();
            let _ = query_watcher.shutdown();
            let _ = query_lifecycle.close();
            let _ = intent_watcher.shutdown();
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
            Self::QueryViewRegistration(denial) => {
                write!(formatter, "Query view registration: {denial:?}")
            }
            Self::IntentInput(denial) => write!(formatter, "intent input: {denial}"),
            Self::IntentFact(denial) => write!(formatter, "intent fact: {denial:?}"),
            Self::IntentDefinition(denial) => write!(formatter, "intent definition: {denial:?}"),
            Self::IntentProvider(denial) => write!(formatter, "intent provider: {denial:?}"),
        }
    }
}

fn builder(
    registration: worth_ui::facade::query_binding::UiScalarProjectionRegistration,
    action_view: WorthUiInstalledQueryView,
    intent: &PlatformPulseIntentInputRecord,
    provider: PlatformPulseActionProvider,
) -> Result<
    WorthUiApplicationBuilder<UiChangeProfileInstalled, UiIntentWiringSatisfied>,
    PlatformPulsePreparationDenial,
> {
    let builder = register_structure(
        WorthUi::app()
            .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse()),
    );
    let builder = register_theme_tokens(builder)
        .register_intent_boolean_fact(platform_pulse_action_mutability_fact(), intent.mutable())
        .map_err(PlatformPulsePreparationDenial::IntentFact)?
        .register_intent_boolean_fact(platform_pulse_action_readiness_fact(), intent.ready())
        .map_err(PlatformPulsePreparationDenial::IntentFact)?
        .register_intent_boolean_fact(platform_pulse_action_policy_fact(), intent.policy_allowed())
        .map_err(PlatformPulsePreparationDenial::IntentFact)?
        .register_intent_boolean_fact(
            platform_pulse_action_confirmation_fact(),
            intent.confirmation_required(),
        )
        .map_err(PlatformPulsePreparationDenial::IntentFact)?
        .register_intent_unsigned64_fact(platform_pulse_action_revision_fact(), intent.revision())
        .map_err(PlatformPulsePreparationDenial::IntentFact)?
        .register_query_view(action_view)
        .map_err(PlatformPulsePreparationDenial::QueryViewRegistration)?
        .register_intent_definition(platform_pulse_action_definition())
        .map_err(PlatformPulsePreparationDenial::IntentDefinition)?
        .register_intent_provider(provider)
        .map_err(PlatformPulsePreparationDenial::IntentProvider)?;
    builder
        .register_scalar_projection(registration)
        .map(|builder| builder.with_visual_inspection_policy(visual_inspection_policy()))
        .map_err(PlatformPulsePreparationDenial::QueryRegistration)
}
