use crate::capability::{CapabilityRegistrationReport, CapabilitySnapshot};

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
}
