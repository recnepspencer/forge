use super::worth_ui_authoring_entry_diagnostic::WorthUiAuthoringEntryDiagnostic;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiAuthoringEntryReport {
    diagnostics: Vec<WorthUiAuthoringEntryDiagnostic>,
}

impl WorthUiAuthoringEntryReport {
    pub(crate) fn new(mut diagnostics: Vec<WorthUiAuthoringEntryDiagnostic>) -> Self {
        diagnostics.sort_by(|left, right| {
            (
                left.span().module_id().as_str(),
                left.span().start_byte(),
                left.span().end_byte(),
                format!("{:?}", left.code()),
                left.message(),
            )
                .cmp(&(
                    right.span().module_id().as_str(),
                    right.span().start_byte(),
                    right.span().end_byte(),
                    format!("{:?}", right.code()),
                    right.message(),
                ))
        });
        diagnostics.dedup();
        Self { diagnostics }
    }

    pub(crate) fn diagnostics(&self) -> &[WorthUiAuthoringEntryDiagnostic] {
        &self.diagnostics
    }
}
