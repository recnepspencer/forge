use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::artifact_validation::{require_non_empty, S0ArtifactBuildRejection};

pub const S0_ARTIFACT_SCHEMA_VERSION: &str = "storage-foundation-s0/v1";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0NondeterministicMetadata {
    generated_at_policy: String,
    local_path_hint: Option<String>,
    host_hint: Option<String>,
}

impl S0NondeterministicMetadata {
    pub fn excluded(
        generated_at_policy: impl Into<String>,
        local_path_hint: Option<impl Into<String>>,
        host_hint: Option<impl Into<String>>,
    ) -> Result<Self, S0ArtifactBuildRejection> {
        Ok(Self {
            generated_at_policy: require_non_empty("generated_at_policy", generated_at_policy)?,
            local_path_hint: local_path_hint.map(Into::into),
            host_hint: host_hint.map(Into::into),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ArtifactValidationCostSurface {
    artifact_byte_count: u64,
    row_count: u64,
    canonicalized_row_byte_count: u64,
    sort_row_count: u64,
}

impl S0ArtifactValidationCostSurface {
    pub(crate) fn new(
        artifact_byte_count: u64,
        row_count: u64,
        canonicalized_row_byte_count: u64,
        sort_row_count: u64,
    ) -> Self {
        Self {
            artifact_byte_count,
            row_count,
            canonicalized_row_byte_count,
            sort_row_count,
        }
    }

    pub fn artifact_byte_count(&self) -> u64 {
        self.artifact_byte_count
    }

    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    pub fn canonicalized_row_byte_count(&self) -> u64 {
        self.canonicalized_row_byte_count
    }

    pub fn sort_row_count(&self) -> u64 {
        self.sort_row_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ArtifactEnvelopeMetadata {
    schema_version: String,
    artifact_kind: S0ArtifactKind,
    source_revision: String,
    roadmap_parent_digest: S0StableDigest,
    generated_by: String,
    deterministic_digest: S0StableDigest,
    nondeterministic_metadata: S0NondeterministicMetadata,
}

impl S0ArtifactEnvelopeMetadata {
    pub(crate) fn new(
        artifact_kind: S0ArtifactKind,
        source_revision: String,
        roadmap_parent_digest: S0StableDigest,
        generated_by: String,
        deterministic_digest: S0StableDigest,
        nondeterministic_metadata: S0NondeterministicMetadata,
    ) -> Self {
        Self {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION.to_string(),
            artifact_kind,
            source_revision,
            roadmap_parent_digest,
            generated_by,
            deterministic_digest,
            nondeterministic_metadata,
        }
    }

    pub fn deterministic_digest(&self) -> &S0StableDigest {
        &self.deterministic_digest
    }

    pub fn artifact_kind(&self) -> S0ArtifactKind {
        self.artifact_kind
    }

    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn roadmap_parent_digest(&self) -> &S0StableDigest {
        &self.roadmap_parent_digest
    }
}
