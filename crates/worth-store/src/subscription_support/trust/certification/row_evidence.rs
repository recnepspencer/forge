use super::super::drift::{SupportTrustDriftCause, SupportTrustSuppressedCause};
use super::super::epochs::{SupportCertificationEpoch, SupportOperationalLedgerEpoch};
use super::super::failure::SupportTrustFailure;
use super::super::reports::OperationalSupportTrustReport;
use super::super::taxonomy::{SupportTrustClass, SupportTrustProvenance, SupportTrustStrength};
use super::certification_validation::{require_non_empty, stable_digest};
use super::lane_digest_set::SupportCertificationLaneDigestSet;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationRowEvidence {
    pub(super) row_id: String,
    pub(super) family_id: SubscriptionSupportFamilyId,
    pub(super) family_kind: SubscriptionSupportFamilyKind,
    pub(super) artifact_id: SubscriptionSupportArtifactId,
    pub(super) support_role: SubscriptionSupportRole,
    pub(super) trust_class: SupportTrustClass,
    pub(super) trust_strength: SupportTrustStrength,
    pub(super) provenance: SupportTrustProvenance,
    pub(super) operational_verdict: SubscriptionSupportOperationalVerdict,
    pub(super) resume_classification: SubscriptionResumeClassification,
    pub(super) basis_digest: String,
    pub(super) cursor_checkpoint_digest: String,
    pub(super) compatibility_epoch: String,
    pub(super) operational_ledger_epoch: SupportOperationalLedgerEpoch,
    pub(super) certification_epoch: SupportCertificationEpoch,
    pub(super) lane_digests: SupportCertificationLaneDigestSet,
    pub(super) artifact_digest: String,
    pub(super) subscription_support_digest: String,
    pub(super) diagnostics_digest: String,
    pub(super) counter_digest: String,
    pub(super) primary_drift_cause: Option<SupportTrustDriftCause>,
    pub(super) suppressed_drift_causes: Vec<SupportTrustSuppressedCause>,
    pub(super) forbidden_exact_overclaim_count: u64,
    pub(super) global_scan_debt_count: u64,
    pub(super) declared_row_digest: String,
}

impl SupportCertificationRowEvidence {
    #[allow(clippy::too_many_arguments)]
    pub fn from_operational_report(
        row_id: impl Into<String>,
        report: &OperationalSupportTrustReport,
        resume_classification: SubscriptionResumeClassification,
        operational_verdict: SubscriptionSupportOperationalVerdict,
        operational_ledger_epoch: SupportOperationalLedgerEpoch,
        certification_epoch: SupportCertificationEpoch,
        lane_digests: SupportCertificationLaneDigestSet,
        artifact_digest: impl Into<String>,
        subscription_support_digest: impl Into<String>,
        diagnostics_digest: impl Into<String>,
        primary_drift_cause: Option<SupportTrustDriftCause>,
        suppressed_drift_causes: Vec<SupportTrustSuppressedCause>,
    ) -> Result<Self, SupportTrustFailure> {
        let counter_digest = stable_digest(&report.cost_surface())?;
        let mut evidence = Self {
            row_id: require_non_empty("row id", row_id)?,
            family_id: report.basis().family_id().clone(),
            family_kind: report.basis().family_kind(),
            artifact_id: report.basis().artifact_id().clone(),
            support_role: report.basis().support_role(),
            trust_class: report.trust_class(),
            trust_strength: report.trust_strength(),
            provenance: report.provenance(),
            operational_verdict,
            resume_classification,
            basis_digest: report.basis().basis_digest().to_string(),
            cursor_checkpoint_digest: format!(
                "{}:{}",
                report.basis().cursor_digest(),
                report.basis().checkpoint_digest()
            ),
            compatibility_epoch: report.basis().compatibility_digest().to_string(),
            operational_ledger_epoch,
            certification_epoch,
            lane_digests,
            artifact_digest: require_non_empty("artifact digest", artifact_digest)?,
            subscription_support_digest: require_non_empty(
                "subscription-support digest",
                subscription_support_digest,
            )?,
            diagnostics_digest: require_non_empty("diagnostics digest", diagnostics_digest)?,
            counter_digest,
            primary_drift_cause,
            suppressed_drift_causes,
            forbidden_exact_overclaim_count: 0,
            global_scan_debt_count: report.cost_surface().global_scan_debt_count(),
            declared_row_digest: String::new(),
        };
        evidence.declared_row_digest = evidence.recomputed_row_digest()?;
        Ok(evidence)
    }

    pub fn with_declared_row_digest(
        mut self,
        declared_row_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        self.declared_row_digest = require_non_empty("declared row digest", declared_row_digest)?;
        Ok(self)
    }

    pub fn row_id(&self) -> &str {
        &self.row_id
    }

    pub fn declared_row_digest(&self) -> &str {
        &self.declared_row_digest
    }

    pub fn recomputed_row_digest(&self) -> Result<String, SupportTrustFailure> {
        stable_digest(&SupportCertificationRowDigestBasis {
            row_id: &self.row_id,
            family_id: &self.family_id,
            family_kind: self.family_kind,
            artifact_id: &self.artifact_id,
            support_role: self.support_role,
            trust_class: self.trust_class,
            trust_strength: self.trust_strength,
            provenance: self.provenance,
            operational_verdict: self.operational_verdict,
            resume_classification: self.resume_classification,
            basis_digest: &self.basis_digest,
            cursor_checkpoint_digest: &self.cursor_checkpoint_digest,
            compatibility_epoch: &self.compatibility_epoch,
            operational_ledger_epoch: self.operational_ledger_epoch,
            certification_epoch: self.certification_epoch,
            lane_digests: &self.lane_digests,
            artifact_digest: &self.artifact_digest,
            subscription_support_digest: &self.subscription_support_digest,
            diagnostics_digest: &self.diagnostics_digest,
            counter_digest: &self.counter_digest,
            primary_drift_cause: self.primary_drift_cause,
            suppressed_drift_causes: &self.suppressed_drift_causes,
            forbidden_exact_overclaim_count: self.forbidden_exact_overclaim_count,
            global_scan_debt_count: self.global_scan_debt_count,
        })
    }
}

#[derive(Serialize)]
struct SupportCertificationRowDigestBasis<'a> {
    row_id: &'a str,
    family_id: &'a SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    artifact_id: &'a SubscriptionSupportArtifactId,
    support_role: SubscriptionSupportRole,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    resume_classification: SubscriptionResumeClassification,
    basis_digest: &'a str,
    cursor_checkpoint_digest: &'a str,
    compatibility_epoch: &'a str,
    operational_ledger_epoch: SupportOperationalLedgerEpoch,
    certification_epoch: SupportCertificationEpoch,
    lane_digests: &'a SupportCertificationLaneDigestSet,
    artifact_digest: &'a str,
    subscription_support_digest: &'a str,
    diagnostics_digest: &'a str,
    counter_digest: &'a str,
    primary_drift_cause: Option<SupportTrustDriftCause>,
    suppressed_drift_causes: &'a [SupportTrustSuppressedCause],
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}
