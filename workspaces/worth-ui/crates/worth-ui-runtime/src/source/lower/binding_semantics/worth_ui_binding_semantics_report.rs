use crate::source::{WorthUiBindingDiagnostic, WorthUiBindingSemanticsMetrics};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiBindingSemanticsReport {
    diagnostics: Vec<WorthUiBindingDiagnostic>,
    metrics: WorthUiBindingSemanticsMetrics,
}

impl WorthUiBindingSemanticsReport {
    pub(crate) fn new(
        mut diagnostics: Vec<WorthUiBindingDiagnostic>,
        metrics: WorthUiBindingSemanticsMetrics,
    ) -> Self {
        diagnostics.sort_by(|left, right| left.stable_cmp(right));
        Self {
            diagnostics,
            metrics,
        }
    }

    pub(crate) fn diagnostics(&self) -> &[WorthUiBindingDiagnostic] {
        &self.diagnostics
    }

    pub(crate) fn metrics(&self) -> WorthUiBindingSemanticsMetrics {
        self.metrics
    }
}
