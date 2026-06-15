use crate::runtime::diagnostics::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeDiagnostic {
    family: WorthUiRuntimeDiagnosticFamily,
    code: WorthUiRuntimeDiagnosticCode,
    source: WorthUiDiagnosticSource,
    phase_reference_digest: Option<u64>,
}

pub type WorthUiReloadDiagnostic = WorthUiRuntimeDiagnostic;
pub type WorthUiPlanDiagnostic = WorthUiRuntimeDiagnostic;

impl WorthUiRuntimeDiagnostic {
    pub(crate) fn new(
        family: WorthUiRuntimeDiagnosticFamily,
        code: WorthUiRuntimeDiagnosticCode,
        source: WorthUiDiagnosticSource,
        phase_reference_digest: Option<u64>,
    ) -> Self {
        Self {
            family,
            code,
            source,
            phase_reference_digest,
        }
    }

    pub fn family(&self) -> WorthUiRuntimeDiagnosticFamily {
        self.family
    }

    pub fn code(&self) -> WorthUiRuntimeDiagnosticCode {
        self.code
    }

    pub fn source(&self) -> WorthUiDiagnosticSource {
        self.source
    }

    pub fn phase_reference_digest(&self) -> Option<u64> {
        self.phase_reference_digest
    }
}
