use super::super::certification::SupportCertificationEvidenceBundle;
use super::super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportGenericCertificationReport,
};
use super::super::failure::SupportTrustFailure;
use super::access_closeout::SubscriptionSupportAccuracyAccessCloseout;
use super::certification_run::SubscriptionSupportAccuracyCertificationRun;
use super::lane_evidence_set::SubscriptionSupportAccuracyLaneEvidenceSet;
use super::performance_closeout::SubscriptionSupportAccuracyPerformanceCloseout;
use super::persistence_posture::SubscriptionSupportAccuracyPersistencePosture;
use super::suite::SubscriptionSupportAccuracyCertificationSuite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct SubscriptionSupportAccuracyCertificationRunner {
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
}

impl Default for SubscriptionSupportAccuracyCertificationRunner {
    fn default() -> Self {
        Self::production()
    }
}

impl SubscriptionSupportAccuracyCertificationRunner {
    pub fn production() -> Self {
        Self {
            persistence_posture:
                SubscriptionSupportAccuracyPersistencePosture::InMemoryCertificationOnly,
        }
    }

    pub fn certify(
        &self,
        evidence_bundle: &SupportCertificationEvidenceBundle,
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        lane_evidence: &SubscriptionSupportAccuracyLaneEvidenceSet,
    ) -> Result<SubscriptionSupportAccuracyCertificationRun, SupportTrustFailure> {
        let suite =
            SubscriptionSupportAccuracyCertificationSuite::from_phase_artifacts_and_lane_evidence(
                evidence_bundle,
                generic_report,
                domain_bundle,
                handoff_report,
                lane_evidence,
            )?;
        let performance_closeout =
            SubscriptionSupportAccuracyPerformanceCloseout::from_phase_artifacts(
                evidence_bundle,
                generic_report,
                domain_bundle,
            )?;
        let access_closeout = SubscriptionSupportAccuracyAccessCloseout::from_phase_artifacts(
            domain_bundle,
            handoff_report,
            self.persistence_posture,
        )?;
        SubscriptionSupportAccuracyCertificationRun::from_closeouts(
            suite,
            performance_closeout,
            access_closeout,
            self.persistence_posture,
        )
    }
}
