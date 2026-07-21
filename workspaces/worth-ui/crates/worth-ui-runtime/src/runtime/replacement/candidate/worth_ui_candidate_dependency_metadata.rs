#[cfg(test)]
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDependencyDeriver, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalenceBasis,
};
use crate::source::{
    WorthUiArtifactDependencyReport, WorthUiArtifactDigest, WorthUiIncrementalInvalidationBasis,
};
use std::rc::Rc;

use super::worth_ui_candidate_dependency_metadata_digest::digest_dependency_report;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateDependencyMetadata {
    artifact_digest: WorthUiArtifactDigest,
    dependency_report: Rc<WorthUiArtifactDependencyReport>,
    dependency_metadata_digest: u64,
}

impl WorthUiCandidateDependencyMetadata {
    #[cfg(test)]
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
            dependency_report: Rc::new(dependency_report),
            dependency_metadata_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn artifact_digest(&self) -> WorthUiArtifactDigest {
        self.artifact_digest
    }

    pub(crate) fn invalidation_basis(&self) -> &WorthUiIncrementalInvalidationBasis {
        self.dependency_report.basis()
    }

    pub(crate) fn dependency_report_authority(&self) -> Rc<WorthUiArtifactDependencyReport> {
        Rc::clone(&self.dependency_report)
    }

    #[cfg(test)]
    pub(crate) fn dependency_report(&self) -> &WorthUiArtifactDependencyReport {
        &self.dependency_report
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
