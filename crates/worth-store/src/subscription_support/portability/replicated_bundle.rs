use super::super::SubscriptionSupportArtifactId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReplicatedSupportBundle {
    manifest_digest: String,
    source_identity_digest: String,
    target_identity_digest: String,
    preserved_artifact_ids: Vec<SubscriptionSupportArtifactId>,
    identity_preservation_digest: String,
}

impl ReplicatedSupportBundle {
    pub(super) fn new(
        manifest_digest: String,
        source_identity_digest: String,
        target_identity_digest: String,
        preserved_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        identity_preservation_digest: String,
    ) -> Self {
        Self {
            manifest_digest,
            source_identity_digest,
            target_identity_digest,
            preserved_artifact_ids,
            identity_preservation_digest,
        }
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn identity_preservation_digest(&self) -> &str {
        &self.identity_preservation_digest
    }

    pub fn preserved_count(&self) -> u64 {
        self.preserved_artifact_ids.len() as u64
    }
}
