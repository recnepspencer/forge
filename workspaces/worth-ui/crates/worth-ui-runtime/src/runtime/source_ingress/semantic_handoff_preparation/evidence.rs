use worth_ui_dsl::{
    WorthUiAuthoredMode, WorthUiDslProtocolIdentity, WorthUiSealedSemanticPackage,
    WorthUiSemanticPackageIdentity,
};

/// Read-only evidence identifying the exact DSL package presented at the
/// authored-to-runtime ownership transition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSemanticHandoffEvidence {
    identity: WorthUiSemanticPackageIdentity,
    protocol: WorthUiDslProtocolIdentity,
    authored_mode: WorthUiAuthoredMode,
}

impl WorthUiSemanticHandoffEvidence {
    pub(super) fn from_package(package: &WorthUiSealedSemanticPackage) -> Self {
        Self {
            identity: package.identity().clone(),
            protocol: package.protocol(),
            authored_mode: package.authored_mode(),
        }
    }

    pub fn identity(&self) -> &WorthUiSemanticPackageIdentity {
        &self.identity
    }

    pub fn protocol(&self) -> WorthUiDslProtocolIdentity {
        self.protocol
    }

    pub fn authored_mode(&self) -> WorthUiAuthoredMode {
        self.authored_mode
    }
}
