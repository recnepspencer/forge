use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiSourcePackageDiagnosticCode {
    InvalidModulePath,
    DuplicateModuleIdentity,
    UnknownImportTarget,
    CyclicModuleImport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiSourcePackageDiagnostic {
    code: WorthUiSourcePackageDiagnosticCode,
    message: String,
    module_path: Option<PathBuf>,
    module_id_text: Option<String>,
    related_module_id_text: Option<String>,
}

impl WorthUiSourcePackageDiagnostic {
    pub(crate) fn new(
        code: WorthUiSourcePackageDiagnosticCode,
        message: impl Into<String>,
        module_path: Option<PathBuf>,
        module_id_text: Option<String>,
        related_module_id_text: Option<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            module_path,
            module_id_text,
            related_module_id_text,
        }
    }

    #[cfg(test)]
    pub(crate) fn code(&self) -> WorthUiSourcePackageDiagnosticCode {
        self.code
    }

    #[cfg(test)]
    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    #[cfg(test)]
    pub(crate) fn module_path(&self) -> Option<&PathBuf> {
        self.module_path.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn module_id_text(&self) -> Option<&str> {
        self.module_id_text.as_deref()
    }

    #[cfg(test)]
    pub(crate) fn related_module_id_text(&self) -> Option<&str> {
        self.related_module_id_text.as_deref()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct WorthUiSourcePackageReport {
    diagnostics: Vec<WorthUiSourcePackageDiagnostic>,
}

impl WorthUiSourcePackageReport {
    pub(crate) fn new(diagnostics: Vec<WorthUiSourcePackageDiagnostic>) -> Self {
        Self { diagnostics }
    }

    #[cfg(test)]
    pub(crate) fn diagnostics(&self) -> &[WorthUiSourcePackageDiagnostic] {
        &self.diagnostics
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}
