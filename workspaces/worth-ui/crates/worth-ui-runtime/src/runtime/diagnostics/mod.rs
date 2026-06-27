pub(crate) mod mapping;
mod projection;
mod reporting;
mod taxonomy;

pub use projection::WorthUiDiagnosticProjectionHook;
pub use reporting::{
    WorthUiDiagnosticMaterialization, WorthUiDiagnosticRichnessPolicy,
    WorthUiDiagnosticSupportReport, WorthUiRuntimeDiagnosticCounters,
    WorthUiRuntimeDiagnosticPolicy, WorthUiRuntimeDiagnosticReport, WorthUiSupportReportPolicy,
};
pub use taxonomy::{
    WorthUiDiagnosticRichnessTier, WorthUiDiagnosticSource, WorthUiPlanDiagnostic,
    WorthUiReloadDiagnostic, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily,
};

/// Phase 1 launch status for the active runtime authority boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeActivationStatus {
    Active,
    FailedActivationPreserved,
    Shutdown,
}
