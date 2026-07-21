use crate::source::WorthUiArtifactEquivalenceBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeEquivalenceBasis {
    artifact_equivalence_basis: WorthUiArtifactEquivalenceBasis,
}

impl WorthUiRuntimeEquivalenceBasis {
    pub fn semantic_artifact_meaning() -> Self {
        Self {
            artifact_equivalence_basis: WorthUiArtifactEquivalenceBasis::semantic(),
        }
    }

    pub(crate) fn artifact_equivalence_basis(self) -> WorthUiArtifactEquivalenceBasis {
        self.artifact_equivalence_basis
    }
}
