use std::path::Path;

use worth_ui::facade::query_binding::{
    UiScalarProjectionRegistration, WorthUiInstalledQueryView, WorthUiQueryViewDeclarationDenial,
    WorthUiScalarProjectionHostPlan, WorthUiScalarProjectionInstallationError,
};

use super::PlatformPulseQueryLifecycle;
use super::{PlatformPulseExternalValueWatch, PlatformPulseExternalValueWatchDenial};

pub(crate) struct InstalledPlatformPulseQuery {
    pub(crate) registration: UiScalarProjectionRegistration,
    pub(crate) action_view: WorthUiInstalledQueryView,
    pub(crate) lifecycle: PlatformPulseQueryLifecycle,
    pub(crate) watcher: PlatformPulseExternalValueWatch,
}

impl InstalledPlatformPulseQuery {
    pub(crate) fn shutdown(self) {
        let _ = self.lifecycle.close();
        let _ = self.watcher.shutdown();
    }
}

#[derive(Debug)]
pub(crate) enum PlatformPulseQueryInstallationDenial {
    Plan(Box<WorthUiScalarProjectionInstallationError>),
    Host(String),
    Completion(Box<WorthUiScalarProjectionInstallationError>),
    ActionView(WorthUiQueryViewDeclarationDenial),
    Watch(PlatformPulseExternalValueWatchDenial),
}

impl std::fmt::Display for PlatformPulseQueryInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plan(denial) => write!(formatter, "plan: {denial:?}"),
            Self::Host(detail) => write!(formatter, "host: {detail}"),
            Self::Completion(denial) => write!(formatter, "completion: {denial:?}"),
            Self::ActionView(denial) => write!(formatter, "action view: {denial:?}"),
            Self::Watch(denial) => write!(formatter, "source watch: {denial}"),
        }
    }
}

pub(crate) fn install(
    source_root: &Path,
) -> Result<InstalledPlatformPulseQuery, PlatformPulseQueryInstallationDenial> {
    let plan = WorthUiScalarProjectionHostPlan::prepare()
        .map_err(|denial| PlatformPulseQueryInstallationDenial::Plan(Box::new(denial)))?;
    let (request, completion) = plan.into_parts();
    let installation =
        worth_query_host::facade::runtime::WorthQueryExecutionRuntimeInstaller::new()
            .install(request.generation(), request.into_packages())
            .map_err(|error| PlatformPulseQueryInstallationDenial::Host(format!("{error:?}")))?;
    let installed = completion
        .complete(installation)
        .map_err(|denial| PlatformPulseQueryInstallationDenial::Completion(Box::new(denial)))?;
    let (registration, initial, action_view) = installed
        .into_action_installation()
        .into_parts_with_live_measurement_view(
            worth_ui_platform_pulse::intent::PLATFORM_PULSE_ACTION_QUERY_VIEW,
        )
        .map_err(PlatformPulseQueryInstallationDenial::ActionView)?;
    let action_view = WorthUiInstalledQueryView::from(action_view);
    let watcher = PlatformPulseExternalValueWatch::spawn(source_root)
        .map_err(PlatformPulseQueryInstallationDenial::Watch)?;
    Ok(InstalledPlatformPulseQuery {
        registration,
        action_view,
        lifecycle: PlatformPulseQueryLifecycle::new(initial),
        watcher,
    })
}
