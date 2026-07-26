pub use crate::admission::{
    UiAdmissionAggregation, UiAdmissionBoundary, UiAdmissionDecision, UiAdmissionFamily,
    UiAdmissionHostCapability, UiAdmissionOutcome, UiAdmissionQueryBasis, UiAdmissionReport,
    UiAdmissionSelectionBudget, UiAdmissionStaleEvidence, UiAdmissionTarget, UiAdmissionWorld,
    UiLegalityDecision, UiLegalityPosture, UiLegalityReason, UiSupportPosture, UiSupportReason,
    UiSupportSnapshot,
};

/// Named admission audience for prepared and active application generations.
pub trait WorthUiAdmissionExt {
    fn admission(&self) -> UiAdmissionBoundary<'_>;
}

impl WorthUiAdmissionExt for crate::facade::WorthUiApp {
    fn admission(&self) -> UiAdmissionBoundary<'_> {
        crate::facade::WorthUiApp::admission(self)
    }
}

impl WorthUiAdmissionExt for crate::facade::WorthUiActiveApplicationSession {
    fn admission(&self) -> UiAdmissionBoundary<'_> {
        crate::facade::WorthUiActiveApplicationSession::admission(self)
    }
}
