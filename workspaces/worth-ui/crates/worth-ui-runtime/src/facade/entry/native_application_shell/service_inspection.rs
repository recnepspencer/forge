#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiNativeReducedMotionPosture {
    NoPreference,
    Reduce,
    Unavailable,
}

impl super::WorthUiNativeApplicationShell {
    /// Bounded explanation of the latest command-routing winner observed by
    /// this native application. The command owner remains the truth source.
    pub fn why_command_won(&self) -> Option<worth_ui_inspection::UiCommandWonInspectionSummary> {
        self.session.why_command_won()
    }

    /// Bounded explanation of the latest Portal close observed by this
    /// native application. This is evidence, not Portal mutation authority.
    pub fn why_portal_closed(
        &self,
    ) -> Option<worth_ui_inspection::UiPortalClosedInspectionSummary> {
        self.session.why_portal_closed()
    }

    /// Bounded explanation of the latest semantic-focus transition.
    pub fn why_focus_moved(&self) -> Option<worth_ui_inspection::UiFocusMovedInspectionSummary> {
        self.session.why_focus_moved()
    }

    /// Bounded explanation of the latest motion interruption.
    pub fn why_motion_interrupted(
        &self,
    ) -> Option<worth_ui_inspection::UiMotionInterruptedInspectionSummary> {
        self.session.why_motion_interrupted()
    }

    /// Exact live resource census across the 3.15 runtime-service families.
    /// This exposes no mutation authority and is suitable for bounded product
    /// status and shutdown evidence.
    pub fn runtime_service_resource_census(
        &self,
    ) -> worth_ui_inspection::UiRuntimeServiceResourceCensus {
        self.session.runtime_service_resource_census()
    }

    /// Latest operating-system reduced-motion posture admitted by the native
    /// motion sampler. `Unavailable` is preserved rather than guessed.
    pub const fn native_reduced_motion_posture(&self) -> WorthUiNativeReducedMotionPosture {
        self.reduced_motion_posture
    }
}
