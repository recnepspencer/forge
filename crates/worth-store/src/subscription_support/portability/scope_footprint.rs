use super::super::{
    classification_error, stable_digest, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportRole,
};
use super::affected_set::SupportPortabilityAffectedSet;
use super::evidence_validation::{validate_basis_artifact_ids, validate_omitted_artifact_ids};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPortabilityScopeFootprint {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    included_support_count: u64,
    required_basis_count: u64,
    omitted_support_count: u64,
    footprint_digest: String,
}

impl SupportPortabilityScopeFootprint {
    pub(crate) fn new(
        affected_set: &SupportPortabilityAffectedSet,
        included_support_count: u64,
        omitted_support_count: u64,
        omitted_artifact_ids: &[SubscriptionSupportArtifactId],
        basis_artifact_ids: &[SubscriptionSupportArtifactId],
    ) -> Result<Self, StoreError> {
        if included_support_count + omitted_support_count != affected_set.affected_count() {
            return Err(classification_error(
                "subscription-support portability footprint must account for every affected support artifact",
            ));
        }
        if omitted_artifact_ids.len() as u64 != omitted_support_count {
            return Err(classification_error(
                "subscription-support portability footprint omitted ids must match omitted count",
            ));
        }
        validate_omitted_artifact_ids(affected_set, omitted_artifact_ids)?;
        validate_basis_artifact_ids(affected_set, basis_artifact_ids, omitted_artifact_ids)?;
        let required_basis_count = basis_artifact_ids.len() as u64;
        let footprint_digest = stable_digest(&(
            affected_set.affected_set_digest(),
            affected_set.portability_digests(),
            omitted_artifact_ids,
            included_support_count,
            required_basis_count,
            omitted_support_count,
        ))?;
        Ok(Self {
            family_id: affected_set.family_id().clone(),
            family_kind: affected_set.family_kind(),
            support_role: affected_set.support_role(),
            included_support_count,
            required_basis_count,
            omitted_support_count,
            footprint_digest,
        })
    }

    pub fn included_support_count(&self) -> u64 {
        self.included_support_count
    }

    pub fn required_basis_count(&self) -> u64 {
        self.required_basis_count
    }

    pub fn omitted_support_count(&self) -> u64 {
        self.omitted_support_count
    }

    pub fn footprint_digest(&self) -> &str {
        &self.footprint_digest
    }
}
