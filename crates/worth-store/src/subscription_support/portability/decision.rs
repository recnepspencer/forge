use super::super::{
    classification_error, SubscriptionSupportArtifactId, SubscriptionSupportOperationalVerdict,
};
use super::affected_set::SupportPortabilityAffectedSet;
use super::evidence_validation::require_non_empty;
use crate::failure::StoreError;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPortabilityDecision {
    evidence: SubscriptionSupportPortabilityDecisionEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
pub(super) enum SubscriptionSupportPortabilityDecisionEvidence {
    FullScopeReplication {
        source_identity_digest: String,
        target_identity_digest: String,
    },
    PartialScopeOmission {
        omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        omission_reason: String,
    },
    TargetImportAdmitted {
        target_admission_digest: String,
        source_identity_preservation_digest: String,
        imported_semantic_digest: String,
    },
    TargetImportMissingBasisNotResumable {
        target_admission_digest: String,
        basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        denial_reason: String,
    },
    UnsupportedFamilyRejected {
        rejection_reason: String,
    },
}

#[allow(dead_code)]
impl SubscriptionSupportPortabilityDecision {
    pub(crate) fn full_scope_replication(
        source_identity_digest: impl Into<String>,
        target_identity_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let source_identity_digest = require_non_empty("source identity", source_identity_digest)?;
        let target_identity_digest = require_non_empty("target identity", target_identity_digest)?;
        if source_identity_digest != target_identity_digest {
            return Err(classification_error(
                "full-scope subscription-support replication requires preserved source/target identity digests",
            ));
        }
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication {
                source_identity_digest,
                target_identity_digest,
            }
            .into(),
        )
    }

    pub(crate) fn partial_scope_omission(
        omitted_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        omission_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        if omitted_artifact_ids.is_empty() {
            return Err(classification_error(
                "partial subscription-support replication requires omitted artifact ids",
            ));
        }
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission {
                omitted_artifact_ids,
                omission_reason: require_non_empty("omission reason", omission_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn target_import_admitted(
        target_admission_digest: impl Into<String>,
        source_identity_preservation_digest: impl Into<String>,
        imported_semantic_digest: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted {
                target_admission_digest: require_non_empty(
                    "target admission",
                    target_admission_digest,
                )?,
                source_identity_preservation_digest: require_non_empty(
                    "source identity preservation",
                    source_identity_preservation_digest,
                )?,
                imported_semantic_digest: require_non_empty(
                    "imported semantic",
                    imported_semantic_digest,
                )?,
            }
            .into(),
        )
    }

    pub(crate) fn target_import_missing_basis_not_resumable(
        target_admission_digest: impl Into<String>,
        basis_artifact_ids: Vec<SubscriptionSupportArtifactId>,
        denial_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                target_admission_digest: require_non_empty(
                    "target admission",
                    target_admission_digest,
                )?,
                basis_artifact_ids,
                denial_reason: require_non_empty("missing basis denial", denial_reason)?,
            }
            .into(),
        )
    }

    pub(crate) fn unsupported_family_rejected(
        rejection_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        Ok(
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                rejection_reason: require_non_empty("portability rejection", rejection_reason)?,
            }
            .into(),
        )
    }

    pub fn verdict(&self) -> SubscriptionSupportOperationalVerdict {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. } => {
                SubscriptionSupportOperationalVerdict::ExactResumePreserved
            }
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission { .. } => {
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                ..
            } => SubscriptionSupportOperationalVerdict::NotResumable,
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                ..
            } => SubscriptionSupportOperationalVerdict::RejectedByPolicy,
        }
    }

    pub fn kind(&self) -> SubscriptionSupportPortabilityDecisionKind {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. } => {
                SubscriptionSupportPortabilityDecisionKind::FullScopeReplication
            }
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission { .. } => {
                SubscriptionSupportPortabilityDecisionKind::PartialScopeOmission
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. } => {
                SubscriptionSupportPortabilityDecisionKind::TargetImportAdmitted
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                ..
            } => SubscriptionSupportPortabilityDecisionKind::TargetImportMissingBasisNotResumable,
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                ..
            } => SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected,
        }
    }

    pub(super) fn evidence(&self) -> &SubscriptionSupportPortabilityDecisionEvidence {
        &self.evidence
    }

    pub(crate) fn omitted_artifact_ids_for_scope(
        &self,
        affected_set: &SupportPortabilityAffectedSet,
    ) -> Vec<SubscriptionSupportArtifactId> {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission {
                omitted_artifact_ids,
                ..
            } => omitted_artifact_ids.clone(),
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected {
                ..
            } => affected_set.all_artifacts_omitted(),
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                ..
            } => Vec::new(),
        }
    }

    pub(crate) fn basis_artifact_ids_for_scope(
        &self,
        affected_set: &SupportPortabilityAffectedSet,
    ) -> Vec<SubscriptionSupportArtifactId> {
        match &self.evidence {
            SubscriptionSupportPortabilityDecisionEvidence::UnsupportedFamilyRejected { .. } => {
                Vec::new()
            }
            SubscriptionSupportPortabilityDecisionEvidence::TargetImportMissingBasisNotResumable {
                basis_artifact_ids,
                ..
            } => basis_artifact_ids.clone(),
            SubscriptionSupportPortabilityDecisionEvidence::FullScopeReplication { .. }
            | SubscriptionSupportPortabilityDecisionEvidence::TargetImportAdmitted { .. } => {
                affected_set.affected_artifact_ids()
            }
            SubscriptionSupportPortabilityDecisionEvidence::PartialScopeOmission { .. } => {
                let omitted = self
                    .omitted_artifact_ids_for_scope(affected_set)
                    .into_iter()
                    .collect::<BTreeSet<_>>();
                affected_set
                    .affected_artifact_ids()
                    .into_iter()
                    .filter(|artifact_id| !omitted.contains(artifact_id))
                    .collect()
            }
        }
    }
}

impl From<SubscriptionSupportPortabilityDecisionEvidence>
    for SubscriptionSupportPortabilityDecision
{
    fn from(evidence: SubscriptionSupportPortabilityDecisionEvidence) -> Self {
        Self { evidence }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportPortabilityDecisionKind {
    FullScopeReplication,
    PartialScopeOmission,
    TargetImportAdmitted,
    TargetImportMissingBasisNotResumable,
    UnsupportedFamilyRejected,
}
