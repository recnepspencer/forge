#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiContentSlotReport {
    diagnostics: Vec<WorthUiContentSlotDiagnostic>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiContentSlotDiagnostic {
    code: WorthUiContentSlotDiagnosticCode,
    page_name: String,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiContentSlotDiagnosticCode {
    MissingPreparedPageStructure,
    SlotMountCountMismatch,
    CanonicalMountOrderMismatch,
}

impl WorthUiContentSlotReport {
    pub(crate) fn new(diagnostics: Vec<WorthUiContentSlotDiagnostic>) -> Self {
        Self { diagnostics }
    }

    pub fn diagnostics(&self) -> &[WorthUiContentSlotDiagnostic] {
        &self.diagnostics
    }
}

impl WorthUiContentSlotDiagnostic {
    pub(crate) fn new(
        code: WorthUiContentSlotDiagnosticCode,
        page_name: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            page_name: page_name.into(),
            detail: detail.into(),
        }
    }

    pub fn code(&self) -> &WorthUiContentSlotDiagnosticCode {
        &self.code
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
