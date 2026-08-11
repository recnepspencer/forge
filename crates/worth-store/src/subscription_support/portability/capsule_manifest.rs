use super::super::{
    classification_error, stable_digest, SubscriptionSupportArtifactId, SupportAffectedSetDigest,
};
use super::{
    affected_set::SupportPortabilityAffectedSet, manifest_budget::SupportPortabilityManifestBudget,
    scope_footprint::SupportPortabilityScopeFootprint,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CapsuleSupportManifest {
    affected_set_digest: SupportAffectedSetDigest,
    footprint: SupportPortabilityScopeFootprint,
    manifest_entry_count: u64,
    manifest_header_bytes: u64,
    required_basis_digests: Vec<String>,
    manifest_digest: String,
}

impl CapsuleSupportManifest {
    pub(crate) fn new(
        affected_set: &SupportPortabilityAffectedSet,
        footprint: SupportPortabilityScopeFootprint,
        budget: SupportPortabilityManifestBudget,
        manifest_header_bytes: u64,
        basis_artifact_ids: &[SubscriptionSupportArtifactId],
    ) -> Result<Self, StoreError> {
        let manifest_entry_count = footprint.included_support_count();
        if !budget.admits(manifest_entry_count, manifest_header_bytes) {
            return Err(classification_error(
                "subscription-support capsule manifest exceeds portability manifest budget before materialization",
            ));
        }
        let required_basis_digests =
            affected_set.basis_digests_for_artifact_ids(basis_artifact_ids)?;
        if required_basis_digests.len() as u64 != footprint.required_basis_count() {
            return Err(classification_error(
                "subscription-support capsule manifest required-basis accounting drift",
            ));
        }
        let manifest_digest = stable_digest(&(
            affected_set.affected_set_digest(),
            footprint.footprint_digest(),
            manifest_entry_count,
            manifest_header_bytes,
            &required_basis_digests,
        ))?;
        Ok(Self {
            affected_set_digest: affected_set.affected_set_digest().clone(),
            footprint,
            manifest_entry_count,
            manifest_header_bytes,
            required_basis_digests,
            manifest_digest,
        })
    }

    pub fn affected_set_digest(&self) -> &SupportAffectedSetDigest {
        &self.affected_set_digest
    }

    pub fn footprint(&self) -> &SupportPortabilityScopeFootprint {
        &self.footprint
    }

    pub fn manifest_entry_count(&self) -> u64 {
        self.manifest_entry_count
    }

    pub fn manifest_header_bytes(&self) -> u64 {
        self.manifest_header_bytes
    }

    pub fn required_basis_count(&self) -> u64 {
        self.footprint.required_basis_count()
    }

    pub fn omitted_support_count(&self) -> u64 {
        self.footprint.omitted_support_count()
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }
}
