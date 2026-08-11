use super::super::{
    classification_error, SubscriptionResumeClassification, SubscriptionSupportArtifactId,
};
use super::operational_basis::{
    require_non_empty, SubscriptionSupportActionOrigin, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ExactResumePreservationWitness {
    basis: SubscriptionSupportOperationalBasis,
}

impl ExactResumePreservationWitness {
    pub(crate) fn new(basis: SubscriptionSupportOperationalBasis) -> Result<Self, StoreError> {
        if basis.action_origin() == SubscriptionSupportActionOrigin::TierRecall {
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
                artifact_id: witness.basis.artifact_id().clone(),
            },
            Self::Degraded(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::Degraded,
                operational_verdict: SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
                artifact_id: witness.basis.artifact_id().clone(),
            },
            Self::Rebuild(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::RebuildRequired,
                operational_verdict: SubscriptionSupportOperationalVerdict::RebuildRequired,
                artifact_id: witness.basis.artifact_id().clone(),
            },
            Self::NotResumable(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::NotResumable,
                operational_verdict: SubscriptionSupportOperationalVerdict::NotResumable,
                artifact_id: witness.basis.artifact_id().clone(),
            },
            Self::PolicyRejected(witness) => PostActionResumeClassificationInput {
                classification: SubscriptionResumeClassification::NotResumable,
                operational_verdict: SubscriptionSupportOperationalVerdict::RejectedByPolicy,
                artifact_id: witness.basis.artifact_id().clone(),
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
