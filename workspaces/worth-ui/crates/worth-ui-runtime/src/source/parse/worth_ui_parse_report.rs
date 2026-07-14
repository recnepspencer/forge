use crate::source::WorthUiParseDiagnostic;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct WorthUiParseReport {
    diagnostics: Vec<WorthUiParseDiagnostic>,
}

impl WorthUiParseReport {
    pub(crate) fn new(diagnostics: Vec<WorthUiParseDiagnostic>) -> Self {
        Self { diagnostics }
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[WorthUiParseDiagnostic] {
        &self.diagnostics
    }
}
