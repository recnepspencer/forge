use super::super::evidence::{S0ArtifactKind, S0CanonicalArtifactSpec, S0StableDigest};
use super::validation::S0EvidenceBundleBuildRejection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0EvidenceProvenance {
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: S0StableDigest,
    pub(super) audit_input_manifest_digest: S0StableDigest,
    pub(super) upstream_artifact_digests: Vec<S0CanonicalArtifactSpec>,
}

impl S0EvidenceProvenance {
    pub fn artifact_digests(&self) -> &[S0CanonicalArtifactSpec] {
        &self.upstream_artifact_digests
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0RegenerationRequirement {
    pub(super) command: String,
}

impl S0RegenerationRequirement {
    pub fn new(command: impl Into<String>) -> Result<Self, S0EvidenceBundleBuildRejection> {
        let command = command.into();
        if command.trim().is_empty() {
            return Err(S0EvidenceBundleBuildRejection::EmptyRequiredField);
        }
        Ok(Self { command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0ArtifactStalenessReport {
    pub(super) stale_artifacts: Vec<S0ArtifactKind>,
    pub(super) manually_edited_artifacts: Vec<S0ArtifactKind>,
}

impl S0ArtifactStalenessReport {
    pub fn is_clean(&self) -> bool {
        self.stale_artifacts.is_empty() && self.manually_edited_artifacts.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0AcceptedEvidenceBundleWitness {
    pub(super) source_revision: String,
    pub(super) audit_input_manifest_digest: S0StableDigest,
    pub(super) evidence_bundle_digest: S0StableDigest,
}

impl S0AcceptedEvidenceBundleWitness {
    pub fn evidence_bundle_digest(&self) -> &S0StableDigest {
        &self.evidence_bundle_digest
    }
}
