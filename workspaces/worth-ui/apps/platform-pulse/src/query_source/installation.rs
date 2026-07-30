use std::path::Path;

use worth_ui::facade::query_binding::{
    UiScalarProjectionRegistration, WorthUiScalarProjectionHostPlan,
    WorthUiScalarProjectionInstallationError,
};

use super::{PlatformPulseExternalValueWatch, PlatformPulseExternalValueWatchDenial};
use super::PlatformPulseQueryLifecycle;

pub(crate) struct InstalledPlatformPulseQuery {
    pub(crate) registration: UiScalarProjectionRegistration,
    pub(crate) lifecycle: PlatformPulseQueryLifecycle,
    pub(crate) watcher: PlatformPulseExternalValueWatch,
}

#[derive(Debug)]
pub(crate) enum PlatformPulseQueryInstallationDenial {
    Plan(WorthUiScalarProjectionInstallationError),
    Host(String),
    Completion(WorthUiScalarProjectionInstallationError),
    Watch(PlatformPulseExternalValueWatchDenial),
}

impl std::fmt::Display for PlatformPulseQueryInstallationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plan(denial) => write!(formatter, "plan: {denial:?}"),
            Self::Host(detail) => write!(formatter, "host: {detail}"),
            Self::Completion(denial) => write!(formatter, "completion: {denial:?}"),
            Self::Watch(denial) => write!(formatter, "source watch: {denial}"),
        }
    }
}

pub(crate) fn install(
    source_root: &Path,
) -> Result<InstalledPlatformPulseQuery, PlatformPulseQueryInstallationDenial> {
    let plan = WorthUiScalarProjectionHostPlan::prepare()
        .map_err(PlatformPulseQueryInstallationDenial::Plan)?;
    let (request, completion) = plan.into_parts();
    let installation =
        worth_query_host::facade::runtime::WorthQueryExecutionRuntimeInstaller::new()
            .install(request.generation(), request.into_packages())
            .map_err(|error| PlatformPulseQueryInstallationDenial::Host(format!("{error:?}")))?;
    let installed = completion
        .complete(installation)
        .map_err(PlatformPulseQueryInstallationDenial::Completion)?;
    let (registration, initial) = installed.into_parts();
    let watcher = PlatformPulseExternalValueWatch::spawn(source_root)
        .map_err(PlatformPulseQueryInstallationDenial::Watch)?;
    Ok(InstalledPlatformPulseQuery {
        registration,
        lifecycle: PlatformPulseQueryLifecycle::new(initial),
        watcher,
    })
}
