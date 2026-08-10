use super::super::drift::{
    SupportTrustDriftCause, SupportTrustDriftLocality, SupportTrustDriftReport,
    SupportTrustDriftScanPlan,
};
use super::super::failure::{SupportTrustFailure, SupportTrustRecoveryPosture};
use super::admission::SupportTrustRequestAdmitted;
use super::translation::SupportTrustTranslatedInputs;
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportOperationalVerdict,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportTrustDriftChecked {
    translated: SupportTrustTranslatedInputs,
    drift_report: SupportTrustDriftReport,
}

impl SupportTrustDriftChecked {
    pub fn drift_report(&self) -> &SupportTrustDriftReport {
        &self.drift_report
    }

    pub(super) fn translated(&self) -> &SupportTrustTranslatedInputs {
        &self.translated
    }

    pub(super) fn into_operational_inputs(
        self,
    ) -> (SupportTrustTranslatedInputs, SupportTrustDriftReport) {
        (self.translated, self.drift_report)
    }
}

pub fn check_support_trust_drift(
    translated: SupportTrustTranslatedInputs,
    scan_plan: SupportTrustDriftScanPlan,
) -> Result<SupportTrustDriftChecked, SupportTrustFailure> {
    let admitted = translated.admitted();
    let mut causes = Vec::new();
    collect_request_identity_drift(admitted, &mut causes);
    collect_receipt_basis_drift(admitted, &mut causes);
    collect_operational_verdict_drift(admitted, &mut causes);
    collect_scan_plan_drift(&scan_plan, &mut causes);
    let drift_report = if causes.is_empty() {
        SupportTrustDriftReport::fresh(&scan_plan)
    } else {
        SupportTrustDriftReport::from_causes(&scan_plan, causes)
    };
    if let Some(cause) = drift_report.blocking_cause() {
        return Err(SupportTrustFailure::new_with_drift_report(
            cause.failure_kind(),
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "support trust drift report rejected stale support evidence",
            drift_report,
        ));
    }
    Ok(SupportTrustDriftChecked {
        translated,
        drift_report,
    })
}

fn collect_request_identity_drift(
    admitted: &SupportTrustRequestAdmitted,
    causes: &mut Vec<(SupportTrustDriftCause, SupportTrustDriftLocality)>,
) {
    let basis = admitted.receipt_bundle().operational().basis();
    let request = admitted.request();
    if request.family_id() != basis.family_id() {
        causes.push((
            SupportTrustDriftCause::Family,
            SupportTrustDriftLocality::FamilyRole,
        ));
    }
    if request.support_role() != basis.support_role() {
        causes.push((
            SupportTrustDriftCause::Role,
            SupportTrustDriftLocality::FamilyRole,
        ));
    }
    if request.artifact_id() != basis.artifact_id() {
        causes.push((
            SupportTrustDriftCause::SupportDigest,
            SupportTrustDriftLocality::SupportIdentity,
        ));
    }
}

fn collect_receipt_basis_drift(
    admitted: &SupportTrustRequestAdmitted,
    causes: &mut Vec<(SupportTrustDriftCause, SupportTrustDriftLocality)>,
) {
    let basis = admitted.receipt_bundle().operational().basis();
    if admitted.receipt_bundle().basis().basis_digest() != basis.basis_digest() {
        causes.push((
            SupportTrustDriftCause::Basis,
            SupportTrustDriftLocality::BasisLocal,
        ));
    }
    if admitted
        .receipt_bundle()
        .cursor_checkpoint()
        .cursor_checkpoint_digest()
        != format!("{}:{}", basis.cursor_digest(), basis.checkpoint_digest())
    {
        causes.push((
            SupportTrustDriftCause::CursorCheckpoint,
            SupportTrustDriftLocality::CursorCheckpointLocal,
        ));
    }
    if admitted
        .receipt_bundle()
        .compatibility()
        .compatibility_digest()
        != basis.compatibility_digest()
    {
        causes.push((
            SupportTrustDriftCause::Compatibility,
            SupportTrustDriftLocality::CompatibilityEpoch,
        ));
    }
    if admitted.receipt_bundle().portability().portability_digest() != basis.portability_digest() {
        causes.push((
            SupportTrustDriftCause::Portability,
            SupportTrustDriftLocality::SupportIdentity,
        ));
    }
}

fn collect_operational_verdict_drift(
    admitted: &SupportTrustRequestAdmitted,
    causes: &mut Vec<(SupportTrustDriftCause, SupportTrustDriftLocality)>,
) {
    if !operational_verdict_matches_resume_classification(
        admitted.receipt_bundle().resume().classification(),
        admitted.receipt_bundle().operational().verdict(),
    ) {
        causes.push((
            SupportTrustDriftCause::OperationalVerdict,
            SupportTrustDriftLocality::SupportIdentity,
        ));
    }
}

fn collect_scan_plan_drift(
    scan_plan: &SupportTrustDriftScanPlan,
    causes: &mut Vec<(SupportTrustDriftCause, SupportTrustDriftLocality)>,
) {
    if scan_plan.certification_coverage_is_missing() {
        causes.push((
            SupportTrustDriftCause::CertificationCoverage,
            SupportTrustDriftLocality::CertificationScope,
        ));
    }
    if scan_plan.locality() == SupportTrustDriftLocality::PlacementCostAdvisory {
        causes.push((
            SupportTrustDriftCause::PlacementCost,
            SupportTrustDriftLocality::PlacementCostAdvisory,
        ));
    }
}

fn operational_verdict_matches_resume_classification(
    resume_classification: SubscriptionResumeClassification,
    operational_verdict: SubscriptionSupportOperationalVerdict,
) -> bool {
    match resume_classification {
        SubscriptionResumeClassification::Exact => {
            operational_verdict == SubscriptionSupportOperationalVerdict::ExactResumePreserved
        }
        SubscriptionResumeClassification::Degraded => {
            operational_verdict == SubscriptionSupportOperationalVerdict::DegradedResumePreserved
        }
        SubscriptionResumeClassification::RebuildRequired => {
            operational_verdict == SubscriptionSupportOperationalVerdict::RebuildRequired
        }
        SubscriptionResumeClassification::NotResumable => matches!(
            operational_verdict,
            SubscriptionSupportOperationalVerdict::NotResumable
                | SubscriptionSupportOperationalVerdict::RejectedByPolicy
        ),
    }
}
