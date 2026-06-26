use crate::capability::{CapabilityRegistrationReport, CapabilitySnapshot};
use crate::runtime::{WorthUiRuntimeHost, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial};

/// Worth UI application after capability registration has frozen.
pub struct WorthUiApp {
    capability_snapshot: CapabilitySnapshot,
}

impl WorthUiApp {
    pub(crate) fn from_registration_report(report: CapabilityRegistrationReport) -> Self {
        Self {
            capability_snapshot: report.into_accepted_snapshot(),
        }
    }

    /// Inspect the immutable capability snapshot owned by this app.
    pub fn capabilities(&self) -> &CapabilitySnapshot {
        &self.capability_snapshot
    }

    /// Launch the Worth UI runtime host from an admitted runtime launch artifact.
    pub fn launch_runtime(
        &self,
        launch: WorthUiRuntimeLaunch,
    ) -> Result<WorthUiRuntimeHost, WorthUiRuntimeLaunchDenial> {
        WorthUiRuntimeHost::launch(launch, &self.capability_snapshot)
    }
}
