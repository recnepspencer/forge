use worth_ui_dsl::WorthUiSourceModuleId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiIdentitySeedingDiagnosticCode {
    DuplicateAuthoredIdentitySeed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiIdentitySeedingDiagnostic {
    code: WorthUiIdentitySeedingDiagnosticCode,
    module_id: WorthUiSourceModuleId,
    semantic_locus: String,
    authored_identity: String,
    conflicting_locus: String,
}

impl WorthUiIdentitySeedingDiagnostic {
    pub(crate) fn duplicate_authored_identity_seed(
        module_id: WorthUiSourceModuleId,
        semantic_locus: impl Into<String>,
        authored_identity: impl Into<String>,
        conflicting_locus: impl Into<String>,
    ) -> Self {
        Self {
            code: WorthUiIdentitySeedingDiagnosticCode::DuplicateAuthoredIdentitySeed,
            module_id,
            semantic_locus: semantic_locus.into(),
            authored_identity: authored_identity.into(),
            conflicting_locus: conflicting_locus.into(),
        }
    }

    pub(crate) fn code(&self) -> WorthUiIdentitySeedingDiagnosticCode {
        self.code
    }

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn semantic_locus(&self) -> &str {
        &self.semantic_locus
    }

    pub(crate) fn authored_identity(&self) -> &str {
        &self.authored_identity
    }

    pub(crate) fn conflicting_locus(&self) -> &str {
        &self.conflicting_locus
    }
}
