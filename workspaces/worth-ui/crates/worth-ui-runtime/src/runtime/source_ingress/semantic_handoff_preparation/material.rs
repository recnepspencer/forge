use crate::source::WorthUiArtifact;

use super::{WorthUiPreparedDeclarationMaterial, WorthUiSemanticHandoffEvidence};

pub(in crate::runtime::source_ingress) struct WorthUiPreparedSemanticHandoffMaterial {
    artifact: WorthUiArtifact,
    declaration_material: WorthUiPreparedDeclarationMaterial,
    evidence: WorthUiSemanticHandoffEvidence,
}

impl WorthUiPreparedSemanticHandoffMaterial {
    pub(in crate::runtime::source_ingress) fn new(
        artifact: WorthUiArtifact,
        declaration_material: WorthUiPreparedDeclarationMaterial,
        evidence: WorthUiSemanticHandoffEvidence,
    ) -> Self {
        Self {
            artifact,
            declaration_material,
            evidence,
        }
    }

    pub(in crate::runtime::source_ingress) fn into_parts(
        self,
    ) -> (
        WorthUiArtifact,
        WorthUiPreparedDeclarationMaterial,
        WorthUiSemanticHandoffEvidence,
    ) {
        (self.artifact, self.declaration_material, self.evidence)
    }
}
