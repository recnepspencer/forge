use crate::runtime::{
    WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReloadStatusSurface {
    active_artifact_digest: u64,
    active_plan_digest: u64,
    failures: Vec<WorthUiRuntimeDiagnostic>,
}

impl WorthUiReloadStatusSurface {
    pub(crate) fn from_runtime_rows(
        active_artifact_digest: u64,
        active_plan_digest: u64,
        rows: &[WorthUiRuntimeDiagnostic],
    ) -> Self {
        let failures = rows
            .iter()
            .filter(|row| {
                row.family() == WorthUiRuntimeDiagnosticFamily::Reload
                    && matches!(row.source(), WorthUiDiagnosticSource::ReloadFailure { .. })
            })
            .cloned()
            .collect();
        Self {
            active_artifact_digest,
            active_plan_digest,
            failures,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn failures(&self) -> &[WorthUiRuntimeDiagnostic] {
        &self.failures
    }

    pub fn latest_failure(&self) -> Option<&WorthUiRuntimeDiagnostic> {
        self.failures.last()
    }
}
