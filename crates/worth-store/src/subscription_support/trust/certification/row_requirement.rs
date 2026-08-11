use super::super::drift::SupportTrustDriftCause;
use super::super::failure::SupportTrustFailure;
use super::super::taxonomy::{SupportTrustClass, SupportTrustProvenance, SupportTrustStrength};
use super::certification_validation::require_non_empty;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportFamilyId, SubscriptionSupportFamilyKind,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationRowRequirement {
    pub(super) row_id: String,
    pub(super) family_id: SubscriptionSupportFamilyId,
    pub(super) family_kind: SubscriptionSupportFamilyKind,
    pub(super) support_role: SubscriptionSupportRole,
    pub(super) trust_class: SupportTrustClass,
    pub(super) trust_strength: SupportTrustStrength,
    pub(super) provenance: SupportTrustProvenance,
    pub(super) operational_verdict: SubscriptionSupportOperationalVerdict,
    pub(super) resume_classification: SubscriptionResumeClassification,
    pub(super) primary_drift_cause: Option<SupportTrustDriftCause>,
}

impl SupportCertificationRowRequirement {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: impl Into<String>,
        family_id: SubscriptionSupportFamilyId,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        trust_class: SupportTrustClass,
        trust_strength: SupportTrustStrength,
        provenance: SupportTrustProvenance,
        operational_verdict: SubscriptionSupportOperationalVerdict,
        resume_classification: SubscriptionResumeClassification,
        primary_drift_cause: Option<SupportTrustDriftCause>,
    ) -> Result<Self, SupportTrustFailure> {
        Ok(Self {
            row_id: require_non_empty("row id", row_id)?,
            family_id,
            family_kind,
            support_role,
            trust_class,
            trust_strength,
            provenance,
            operational_verdict,
            resume_classification,
            primary_drift_cause,
        })
    }

    pub fn row_id(&self) -> &str {
        &self.row_id
    }
}
