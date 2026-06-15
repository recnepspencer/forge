use crate::source::WorthUiSourceSpan;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiAuthoringEntryDiagnosticCode {
    DuplicateDeclarationName,
    MultipleAppDeclarations,
    MissingAppDeclaration,
    MissingWorkspaceReference,
    DuplicateWorkspaceReference,
    MissingThemeReference,
    DuplicateThemeReference,
    UnknownWorkspaceReference,
    UnknownThemeReference,
    UnownedWorkspaceDeclaration,
    MissingWorkspaceShell,
    DuplicateWorkspaceShell,
    MissingWorkspaceShellSlot,
    DuplicateWorkspaceShellSlot,
    InvalidWorkspaceShellEntry,
    UnknownPageReference,
    UnownedPageDeclaration,
    StaticPageCannotDeclareSignature,
    StaticPageReferencesTemplate,
    DynamicPageRequiresSignature,
    InvalidDynamicPageSignature,
    DynamicPageSignatureMismatch,
    MissingPageSection,
    DuplicatePageSection,
    MixedPageSectionDefinition,
    InvalidLayoutTopology,
    UnknownRuntimeReference,
    UnknownLayoutReference,
    UnknownContentReference,
    UnknownAppearanceReference,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiAuthoringEntryDiagnostic {
    code: WorthUiAuthoringEntryDiagnosticCode,
    message: String,
    span: WorthUiSourceSpan,
}

impl WorthUiAuthoringEntryDiagnostic {
    pub(crate) fn new(
        code: WorthUiAuthoringEntryDiagnosticCode,
        message: impl Into<String>,
        span: WorthUiSourceSpan,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            span,
        }
    }

    pub(crate) fn code(&self) -> WorthUiAuthoringEntryDiagnosticCode {
        self.code
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn span(&self) -> &WorthUiSourceSpan {
        &self.span
    }
}
