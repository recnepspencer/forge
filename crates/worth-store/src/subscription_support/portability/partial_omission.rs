use super::super::SubscriptionSupportArtifactId;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PartialSupportOmissionReport {
    manifest_digest: String,
    omission_reason: String,
    omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
}

impl PartialSupportOmissionReport {
    pub(super) fn new(
        manifest_digest: String,
        omission_reason: String,
        omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
    ) -> Self {
        Self {
            manifest_digest,
            omission_reason,
            omitted_artifact_ids,
        }
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }

    pub fn omission_reason(&self) -> &str {
        &self.omission_reason
    }

    pub fn omitted_count(&self) -> u64 {
        self.omitted_artifact_ids.len() as u64
    }
}
