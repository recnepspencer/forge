use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportExactTrustTranslation {
    basis: SubscriptionSupportOperationalBasis,
    resume_classification: SubscriptionResumeClassification,
    operational_verdict: SubscriptionSupportOperationalVerdict,
}

impl SupportExactTrustTranslation {
    #[allow(dead_code)]
    pub(crate) fn new(
        basis: SubscriptionSupportOperationalBasis,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
    ) -> Result<Self, SupportTrustFailure> {
        if resume_classification != SubscriptionResumeClassification::Exact {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustResumeClassificationMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "exact support trust requires an exact resume classification",
            ));
        }
        if operational_verdict != SubscriptionSupportOperationalVerdict::ExactResumePreserved {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustOperationalVerdictMismatch,
                SupportTrustRecoveryPosture::RetryWithFresherReceipts,
                "exact support trust requires an exact-preserved operational verdict",
            ));
        }
        Ok(Self {
            basis,
            resume_classification,
            operational_verdict,
        })
    }

    pub fn basis(&self) -> &SubscriptionSupportOperationalBasis {
        &self.basis
    }

    pub fn resume_classification(&self) -> SubscriptionResumeClassification {
        self.resume_classification
    }

    pub fn operational_verdict(&self) -> SubscriptionSupportOperationalVerdict {
        self.operational_verdict
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum SupportTrustTranslationPlan {
    Exact(SupportExactTrustTranslation),
    Degraded {
        basis: SubscriptionSupportOperationalBasis,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
    },
    RebuildDerived {
        basis: SubscriptionSupportOperationalBasis,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
    },
    Rejected {
        basis: SubscriptionSupportOperationalBasis,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
    },
}

impl SupportTrustTranslationPlan {
    #[allow(dead_code)]
    pub(crate) fn from_resume_and_operational(
        basis: SubscriptionSupportOperationalBasis,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
    ) -> Result<Self, SupportTrustFailure> {
        match (resume_classification, operational_verdict) {
            (
                SubscriptionResumeClassification::Exact,
                SubscriptionSupportOperationalVerdict::ExactResumePreserved,
            ) => {
                SupportExactTrustTranslation::new(basis, resume_classification, operational_verdict)
                    .map(Self::Exact)
            }
            (
                SubscriptionResumeClassification::Degraded,
                SubscriptionSupportOperationalVerdict::DegradedResumePreserved,
            ) => Ok(Self::Degraded {
                basis,
                resume_classification,
                operational_verdict,
            }),
            (
                SubscriptionResumeClassification::RebuildRequired,
                SubscriptionSupportOperationalVerdict::RebuildRequired,
            ) => Ok(Self::RebuildDerived {
                basis,
                resume_classification,
                operational_verdict,
            }),
            (SubscriptionResumeClassification::NotResumable, _)
            | (_, SubscriptionSupportOperationalVerdict::NotResumable)
            | (_, SubscriptionSupportOperationalVerdict::RejectedByPolicy) => Ok(Self::Rejected {
                basis,
                resume_classification,
                operational_verdict,
            }),
            _ => Ok(Self::Rejected {
                basis,
                resume_classification,
                operational_verdict,
            }),
        }
    }
}
