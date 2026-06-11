use crate::source::WorthUiParseDiagnostic;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct WorthUiParseReport {
    diagnostics: Vec<WorthUiParseDiagnostic>,
}

impl WorthUiParseReport {
    pub(crate) fn new(diagnostics: Vec<WorthUiParseDiagnostic>) -> Self {
        Self { diagnostics }
    }

    pub(crate) fn diagnostics(&self) -> &[WorthUiParseDiagnostic] {
        &self.diagnostics
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}
