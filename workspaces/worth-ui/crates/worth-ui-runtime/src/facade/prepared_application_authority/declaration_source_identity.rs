/// Comparison-safe identity of the declaration source admitted for one
/// prepared generation. Its construction basis remains private to preparation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiPreparedDeclarationSourceIdentity {
    semantic_package: worth_ui_dsl::WorthUiSemanticPackageIdentity,
}

impl WorthUiPreparedDeclarationSourceIdentity {
    pub(crate) fn from_semantic_package(
        identity: worth_ui_dsl::WorthUiSemanticPackageIdentity,
    ) -> Self {
        Self {
            semantic_package: identity,
        }
    }

    pub fn semantic_package_identity(&self) -> &worth_ui_dsl::WorthUiSemanticPackageIdentity {
        &self.semantic_package
    }
}
