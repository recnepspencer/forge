use super::super::artifacts::S0ArtifactValidationCostSurface;
use super::aggregate::S0EvidenceBundle;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ValidatedEvidenceBundleArtifact {
    pub(super) bundle: S0EvidenceBundle,
    pub(super) validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedEvidenceBundleArtifact {
    pub fn bundle(&self) -> &S0EvidenceBundle {
        &self.bundle
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}
