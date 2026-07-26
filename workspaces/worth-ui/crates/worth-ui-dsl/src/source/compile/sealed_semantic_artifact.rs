use super::WorthUiSemanticProvenanceRef;
use crate::source::WorthUiSemanticArtifactDeclaration;

/// One rich semantic declaration after compiler admission.
///
/// The authored declaration remains immutable, while provenance is represented
/// only by the package-local reference minted during sealing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSealedSemanticArtifact {
    declaration: WorthUiSemanticArtifactDeclaration,
    provenance_ref: WorthUiSemanticProvenanceRef,
}

impl WorthUiSealedSemanticArtifact {
    pub(super) fn new(
        declaration: WorthUiSemanticArtifactDeclaration,
        provenance_ref: WorthUiSemanticProvenanceRef,
    ) -> Self {
        Self {
            declaration,
            provenance_ref,
        }
    }

    pub fn declaration(&self) -> &WorthUiSemanticArtifactDeclaration {
        &self.declaration
    }

    pub fn provenance_ref(&self) -> WorthUiSemanticProvenanceRef {
        self.provenance_ref
    }
}
