use crate::capability::CapabilitySnapshotDigest;
use crate::runtime::candidate::worth_ui_candidate_dependency_metadata_digest::digest_dependency_report;
use crate::runtime::candidate::WorthUiCandidateLoweringBasis;
use crate::runtime::candidate::{
    WorthUiCandidateDependencyMetadata, WorthUiReplacementCandidateBasis,
    WorthUiReplacementCandidateDenial,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDependencyDeriver, WorthUiArtifactDigest,
    WorthUiArtifactDigestReport, WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthUiCandidateArtifactBundle {
    artifact: WorthUiArtifact,
    artifact_digest: WorthUiArtifactDigest,
    artifact_digest_report: WorthUiArtifactDigestReport,
    dependency_metadata: WorthUiCandidateDependencyMetadata,
    lowering_basis: WorthUiCandidateLoweringBasis,
}

impl WorthUiCandidateArtifactBundle {
    pub(crate) fn derive_and_seal(
        artifact: WorthUiArtifact,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Result<Self, WorthUiReplacementCandidateDenial> {
        let (artifact_digest, artifact_digest_report) = digest_artifact(&artifact);
        let dependency_report = WorthUiArtifactDependencyDeriver::derive_with_report(&artifact);
        let dependency_metadata = WorthUiCandidateDependencyMetadata::from_derived_report(
            artifact_digest,
            dependency_report,
        );
        let lowering_basis = WorthUiCandidateLoweringBasis::from_snapshot_and_dependency_metadata(
            snapshot_digest,
            &dependency_metadata,
        );
        Ok(Self {
            artifact,
            artifact_digest,
            artifact_digest_report,
            dependency_metadata,
            lowering_basis,
        })
    }

    pub(crate) fn seal(
        artifact: WorthUiArtifact,
        dependency_metadata: WorthUiCandidateDependencyMetadata,
        snapshot_digest: CapabilitySnapshotDigest,
    ) -> Result<Self, WorthUiReplacementCandidateDenial> {
        let (artifact_digest, artifact_digest_report) = digest_artifact(&artifact);
        if artifact_digest != dependency_metadata.artifact_digest() {
            return Err(
                WorthUiReplacementCandidateDenial::DependencyMetadataArtifactDigestMismatch,
            );
        }
        reject_stale_dependency_metadata(&artifact, &dependency_metadata)?;
        let lowering_basis = WorthUiCandidateLoweringBasis::from_snapshot_and_dependency_metadata(
            snapshot_digest,
            &dependency_metadata,
        );
        Ok(Self {
            artifact,
            artifact_digest,
            artifact_digest_report,
            dependency_metadata,
            lowering_basis,
        })
    }

    pub(crate) fn artifact(&self) -> &WorthUiArtifact {
        &self.artifact
    }

    pub(crate) fn artifact_digest(&self) -> WorthUiArtifactDigest {
        self.artifact_digest
    }

    pub(crate) fn artifact_digest_report(&self) -> WorthUiArtifactDigestReport {
        self.artifact_digest_report
    }

    pub(crate) fn dependency_metadata(&self) -> &WorthUiCandidateDependencyMetadata {
        &self.dependency_metadata
    }

    pub(crate) fn lowering_basis(&self) -> WorthUiCandidateLoweringBasis {
        self.lowering_basis
    }

    pub(crate) fn basis(&self) -> WorthUiReplacementCandidateBasis {
        WorthUiReplacementCandidateBasis::new(
            self.artifact_digest,
            self.dependency_metadata.dependency_metadata_digest(),
            self.lowering_basis.basis_digest(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_optional_parts_for_test(
        artifact: WorthUiArtifact,
        artifact_digest: Option<WorthUiArtifactDigest>,
        dependency_metadata: Option<WorthUiCandidateDependencyMetadata>,
        lowering_basis: Option<WorthUiCandidateLoweringBasis>,
    ) -> Result<Self, WorthUiReplacementCandidateDenial> {
        let artifact_digest =
            artifact_digest.ok_or(WorthUiReplacementCandidateDenial::MissingArtifactDigest)?;
        let dependency_metadata = dependency_metadata
            .ok_or(WorthUiReplacementCandidateDenial::MissingDependencyMetadata)?;
        let lowering_basis =
            lowering_basis.ok_or(WorthUiReplacementCandidateDenial::MissingLoweringBasis)?;
        let (_, artifact_digest_report) = digest_artifact(&artifact);
        if artifact_digest != dependency_metadata.artifact_digest() {
            return Err(
                WorthUiReplacementCandidateDenial::DependencyMetadataArtifactDigestMismatch,
            );
        }
        reject_stale_dependency_metadata(&artifact, &dependency_metadata)?;
        Ok(Self {
            artifact,
            artifact_digest,
            artifact_digest_report,
            dependency_metadata,
            lowering_basis,
        })
    }
}

fn digest_artifact(
    artifact: &WorthUiArtifact,
) -> (WorthUiArtifactDigest, WorthUiArtifactDigestReport) {
    WorthUiArtifactDigestor::digest_with_report(
        artifact,
        WorthUiArtifactEquivalenceBasis::semantic(),
    )
}

fn reject_stale_dependency_metadata(
    artifact: &WorthUiArtifact,
    dependency_metadata: &WorthUiCandidateDependencyMetadata,
) -> Result<(), WorthUiReplacementCandidateDenial> {
    let expected_report = WorthUiArtifactDependencyDeriver::derive_with_report(artifact);
    let expected_digest = digest_dependency_report(&expected_report);
    if expected_digest == dependency_metadata.dependency_metadata_digest() {
        Ok(())
    } else {
        Err(WorthUiReplacementCandidateDenial::StaleDependencyMetadata)
    }
}
