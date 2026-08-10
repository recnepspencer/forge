use super::digest::stable_digest;
use super::domain_bundle::SupportDomainCertificationBundle;
use super::domain_counter::SupportDomainCertificationCounterSnapshot;
use super::generic_certification::SupportGenericCertificationReport;
use super::scenario::SupportRoadmapPhysicalReadinessPosture;
use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationHandoffReport {
    generic_certification_digest: String,
    domain_certification_digest: String,
    semantic_support_trust_closed: bool,
    roadmap_physical_readiness_posture: SupportRoadmapPhysicalReadinessPosture,
    handoff_counter_snapshot: SupportDomainCertificationCounterSnapshot,
    handoff_digest: String,
}

impl SupportCertificationHandoffReport {
    pub fn from_generic_and_domain_certification(
        generic_report: &SupportGenericCertificationReport,
        domain_bundle: &SupportDomainCertificationBundle,
    ) -> Result<Self, SupportTrustFailure> {
        if domain_bundle
            .counter_snapshot()
            .physical_readiness_debt_count()
            == 0
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::WaitForMilestone14OrRoadmap2Evidence,
                "support trust handoff must keep Roadmap 2 physical readiness debt explicit",
            ));
        }
        let mut report = Self {
            generic_certification_digest: generic_report.generic_certification_digest().to_string(),
            domain_certification_digest: domain_bundle.domain_certification_digest().to_string(),
            semantic_support_trust_closed: true,
            roadmap_physical_readiness_posture:
                SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2,
            handoff_counter_snapshot: domain_bundle.counter_snapshot(),
            handoff_digest: String::new(),
        };
        report.handoff_digest = stable_digest(&SupportCertificationHandoffDigestBasis {
            generic_certification_digest: &report.generic_certification_digest,
            domain_certification_digest: &report.domain_certification_digest,
            semantic_support_trust_closed: report.semantic_support_trust_closed,
            roadmap_physical_readiness_posture: report.roadmap_physical_readiness_posture,
            handoff_counter_snapshot: report.handoff_counter_snapshot,
        })?;
        Ok(report)
    }

    pub fn semantic_support_trust_closed(&self) -> bool {
        self.semantic_support_trust_closed
    }

    pub fn generic_certification_digest(&self) -> &str {
        &self.generic_certification_digest
    }

    pub fn domain_certification_digest(&self) -> &str {
        &self.domain_certification_digest
    }

    pub fn roadmap_physical_readiness_posture(&self) -> SupportRoadmapPhysicalReadinessPosture {
        self.roadmap_physical_readiness_posture
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

#[derive(Serialize)]
struct SupportCertificationHandoffDigestBasis<'a> {
    generic_certification_digest: &'a str,
    domain_certification_digest: &'a str,
    semantic_support_trust_closed: bool,
    roadmap_physical_readiness_posture: SupportRoadmapPhysicalReadinessPosture,
    handoff_counter_snapshot: SupportDomainCertificationCounterSnapshot,
}
