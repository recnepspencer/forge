use crate::source::{
    WorthUiArtifact, WorthUiArtifactDependencyDeriver, WorthUiArtifactDependencyReport,
    WorthUiArtifactDigest, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
    WorthUiIncrementalInvalidationBasis,
};

use super::worth_ui_candidate_dependency_metadata_digest::digest_dependency_report;

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiCandidateDependencyMetadata {
    artifact_digest: WorthUiArtifactDigest,
    dependency_report: WorthUiArtifactDependencyReport,
    dependency_metadata_digest: u64,
}

impl WorthUiCandidateDependencyMetadata {
    pub(crate) fn derive_for_artifact(artifact: &WorthUiArtifact) -> Self {
        let artifact_digest =
            WorthUiArtifactDigestor::digest(artifact, WorthUiArtifactEquivalenceBasis::semantic());
        let dependency_report = WorthUiArtifactDependencyDeriver::derive_with_report(artifact);
        Self::from_derived_report(artifact_digest, dependency_report)
    }

    pub(crate) fn from_derived_report(
        artifact_digest: WorthUiArtifactDigest,
        dependency_report: WorthUiArtifactDependencyReport,
    ) -> Self {
        let dependency_metadata_digest = digest_dependency_report(&dependency_report);
        Self {
            artifact_digest,
            dependency_report,
            dependency_metadata_digest,
        }
    }

    pub(crate) fn artifact_digest(&self) -> WorthUiArtifactDigest {
        self.artifact_digest
    }

    pub(crate) fn dependency_report(&self) -> &WorthUiArtifactDependencyReport {
        &self.dependency_report
    }

    pub(crate) fn invalidation_basis(&self) -> &WorthUiIncrementalInvalidationBasis {
        self.dependency_report.basis()
    }

    pub(crate) fn dependency_metadata_digest(&self) -> u64 {
        self.dependency_metadata_digest
    }

    #[cfg(test)]
    pub(crate) fn with_artifact_digest_for_test(
        mut self,
        artifact_digest: WorthUiArtifactDigest,
    ) -> Self {
        self.artifact_digest = artifact_digest;
        self
    }
}
