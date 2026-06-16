use crate::source::{
    WorthUiAuthoringEntryDiagnostic, WorthUiAuthoringEntryDiagnosticCode,
    WorthUiParsedPageDeclaration,
};

pub(super) fn page_content_slotting_diagnostic(
    code: WorthUiAuthoringEntryDiagnosticCode,
    page: &WorthUiParsedPageDeclaration,
    message: impl Into<String>,
) -> WorthUiAuthoringEntryDiagnostic {
    WorthUiAuthoringEntryDiagnostic::new(code, message, page.span().clone())
}
