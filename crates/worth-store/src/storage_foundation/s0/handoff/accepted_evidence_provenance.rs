use super::super::evidence::S0StableDigest;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0AcceptedEvidenceProvenance {
    pub(super) source_revision: String,
    pub(super) roadmap_parent_digest: S0StableDigest,
    pub(super) audit_input_manifest_digest: S0StableDigest,
}

impl S0AcceptedEvidenceProvenance {
    pub(super) fn from_parts(
        source_revision: String,
        roadmap_parent_digest: S0StableDigest,
        audit_input_manifest_digest: S0StableDigest,
    ) -> Self {
        Self {
            source_revision,
            roadmap_parent_digest,
            audit_input_manifest_digest,
        }
    }

    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn roadmap_parent_digest(&self) -> &S0StableDigest {
        &self.roadmap_parent_digest
    }

    pub fn audit_input_manifest_digest(&self) -> &S0StableDigest {
        &self.audit_input_manifest_digest
    }
}
