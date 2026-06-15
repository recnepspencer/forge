#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLayoutTopologyDiagnosticCode {
    MissingLayoutRoot,
    InvalidLayoutRoot,
    InvalidLayoutNode,
    InvalidLayoutSizing,
    InvalidLayoutModifier,
    InvalidResizePersistence,
    DuplicateLayoutSlot,
    InvalidPageLayoutReference,
    UnknownLayoutReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutTopologyDiagnostic {
    code: WorthUiLayoutTopologyDiagnosticCode,
    locus: String,
    message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLayoutTopologyReport {
    diagnostics: Vec<WorthUiLayoutTopologyDiagnostic>,
}

impl WorthUiLayoutTopologyDiagnostic {
    pub fn new(
        code: WorthUiLayoutTopologyDiagnosticCode,
        locus: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            locus: locus.into(),
            message: message.into(),
        }
    }

    pub fn code(&self) -> WorthUiLayoutTopologyDiagnosticCode {
        self.code
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl WorthUiLayoutTopologyReport {
    pub fn new(diagnostics: Vec<WorthUiLayoutTopologyDiagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn diagnostics(&self) -> &[WorthUiLayoutTopologyDiagnostic] {
        &self.diagnostics
    }
}
