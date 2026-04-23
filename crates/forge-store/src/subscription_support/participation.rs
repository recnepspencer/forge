use super::{
    classification_error, SubscriptionResumeClassification, SubscriptionSupportArtifactId,
    SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind, SubscriptionSupportRole,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportActionOrigin {
    Retention,
    Compatibility,
    ReplicationExport,
    ReplicationImport,
    Maintenance,
    RestartRecovery,
    TierRecall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SubscriptionSupportOperationalVerdict {
    ExactResumePreserved,
    DegradedResumePreserved,
    RebuildRequired,
    NotResumable,
    RejectedByPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportOperationalBasis {
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    artifact_id: SubscriptionSupportArtifactId,
    basis_digest: String,
    cursor_digest: String,
    checkpoint_digest: String,
    compatibility_digest: String,
    portability_digest: String,
    action_origin: SubscriptionSupportActionOrigin,
}

impl SubscriptionSupportOperationalBasis {
    pub fn new(
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        artifact_id: SubscriptionSupportArtifactId,
        basis_digest: impl Into<String>,
        cursor_digest: impl Into<String>,
        checkpoint_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
        portability_digest: impl Into<String>,
        action_origin: SubscriptionSupportActionOrigin,
    ) -> Result<Self, StoreError> {
        let basis_digest = require_non_empty("basis", basis_digest)?;
        let cursor_digest = require_non_empty("cursor", cursor_digest)?;
        let checkpoint_digest = require_non_empty("checkpoint", checkpoint_digest)?;
        let compatibility_digest = require_non_empty("compatibility", compatibility_digest)?;
        let portability_digest = require_non_empty("portability", portability_digest)?;
        Ok(Self {
            family_id,
            family_kind,
            support_role,
            artifact_id,
            basis_digest,
            cursor_digest,
            checkpoint_digest,
            compatibility_digest,
            portability_digest,
            action_origin,
        })
    }

    pub fn family_id(&self) -> &SubscriptionSupportFamilyId {
        &self.family_id
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn cursor_digest(&self) -> &str {
        &self.cursor_digest
    }

    pub fn checkpoint_digest(&self) -> &str {
        &self.checkpoint_digest
    }

    pub fn compatibility_digest(&self) -> &str {
        &self.compatibility_digest
    }

    pub fn portability_digest(&self) -> &str {
        &self.portability_digest
    }

    pub fn action_origin(&self) -> SubscriptionSupportActionOrigin {
        self.action_origin
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactResumePreservationWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl ExactResumePreservationWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Result<Self, StoreError> {
        if basis.action_origin == SubscriptionSupportActionOrigin::TierRecall {
            return Err(classification_error(
                "tier recall may change support access cost but cannot prove exact operational preservation",
            ));
        }
        Ok(Self { basis })
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DegradedResumePreservationWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl DegradedResumePreservationWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Self {
        Self { basis }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportRebuildAdmissionWitness {
    basis: SubscriptionSupportOperationalBasis,
    maintenance_admission_key: String,
}

impl SupportRebuildAdmissionWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let maintenance_admission_key =
            require_non_empty("maintenance-admission", maintenance_admission_key)?;
        Ok(Self {
            basis,
            maintenance_admission_key,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportNonResumableWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl SupportNonResumableWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Self {
        Self { basis }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportPolicyRejectionWitness {
    basis: SubscriptionSupportOperationalBasis,
    policy_reason: String,
}

impl SupportPolicyRejectionWitness {
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        policy_reason: impl Into<String>,
    ) -> Result<Self, StoreError> {
        let policy_reason = require_non_empty("policy rejection", policy_reason)?;
        Ok(Self {
            basis,
            policy_reason,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum ResumeClassificationTranslationPlan {
    Exact(ExactResumePreservationWitness),
    Degraded(DegradedResumePreservationWitness),
    Rebuild(SupportRebuildAdmissionWitness),
    NotResumable(SupportNonResumableWitness),
    PolicyRejected(SupportPolicyRejectionWitness),
}

impl ResumeClassificationTranslationPlan {
    pub(crate) fn from_operational_verdict(
        verdict: SubscriptionSupportOperationalVerdict,
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: Option<String>,
        policy_reason: Option<String>,
    ) -> Result<Self, StoreError> {
        match verdict {
            SubscriptionSupportOperationalVerdict::ExactResumePreserved => {
                ExactResumePreservationWitness::new(basis).map(Self::Exact)
            }
            SubscriptionSupportOperationalVerdict::DegradedResumePreserved => Ok(Self::Degraded(
                DegradedResumePreservationWitness::new(basis),
            )),
            SubscriptionSupportOperationalVerdict::RebuildRequired => {
                let key = maintenance_admission_key.ok_or_else(|| {
                    classification_error(
                        "rebuild-required support verdicts require maintenance admission proof",
                    )
                })?;
                SupportRebuildAdmissionWitness::new(basis, key).map(Self::Rebuild)
            }
            SubscriptionSupportOperationalVerdict::NotResumable => {
                Ok(Self::NotResumable(SupportNonResumableWitness::new(basis)))
            }
            SubscriptionSupportOperationalVerdict::RejectedByPolicy => {
                let reason = policy_reason.ok_or_else(|| {
                    classification_error("policy-rejected support verdicts require a policy reason")
                })?;
                SupportPolicyRejectionWitness::new(basis, reason).map(Self::PolicyRejected)
            }
        }
    }

    pub(crate) fn lower(self) -> PostActionResumeClassificationInput {
        match self {
            Self::Exact(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::Exact,
                operational_verdict: SubscriptionSupportOperationalVerdict::ExactResumePreserved,
                artifact_id: witness.basis.artifact_id,
            },
            Self::Degraded(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::Degraded,
                operational_verdict: SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                artifact_id: witness.basis.artifact_id,
            },
            Self::Rebuild(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::RebuildRequired,
                operational_verdict: SubscriptionSupportOperationalVerdict::RebuildRequired,
                artifact_id: witness.basis.artifact_id,
            },
            Self::NotResumable(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::NotResumable,
                operational_verdict: SubscriptionSupportOperationalVerdict::NotResumable,
                artifact_id: witness.basis.artifact_id,
            },
            Self::PolicyRejected(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::NotResumable,
                operational_verdict: SubscriptionSupportOperationalVerdict::RejectedByPolicy,
                artifact_id: witness.basis.artifact_id,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PostActionResumeClassificationInput {
    classification: SubscriptionResumeClassification,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    artifact_id: SubscriptionSupportArtifactId,
}

impl PostActionResumeClassificationInput {
    pub fn classification(&self) -> SubscriptionResumeClassification {
        self.classification
    }

    pub fn operational_verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.operational_verdict
    }

    pub fn artifact_id(&self) -> &SubscriptionSupportArtifactId {
        &self.artifact_id
    }
}

fn require_non_empty(label: &'static str, value: impl Into<String>) -> Result<String, StoreError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(classification_error(format!(
            "subscription-support operational {label} evidence must be non-empty"
        )));
    }
    Ok(value)
}
