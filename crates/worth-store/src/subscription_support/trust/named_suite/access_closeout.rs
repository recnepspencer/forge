use super::super::domain_certification::{
    SupportCertificationHandoffReport, SupportDomainCertificationBundle,
    SupportDomainCertificationDebtOwner,
};
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::handoff_validation::validate_handoff;
use super::persistence_posture::SubscriptionSupportAccuracyPersistencePosture;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyAccessCloseout {
    certified_semantic_domain_row_count: u64,
    explicit_advanced_family_debt_count: u64,
    roadmap2_physical_debt_explicit: bool,
    milestone15_extension_debt_explicit: bool,
    handoff_semantic_trust_closed: bool,
    persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
}

impl SubscriptionSupportAccuracyAccessCloseout {
    pub(super) fn from_phase_artifacts(
        domain_bundle: &SupportDomainCertificationBundle,
        handoff_report: &SupportCertificationHandoffReport,
        persistence_posture: SubscriptionSupportAccuracyPersistencePosture,
    ) -> Result<Self, SupportTrustFailure> {
        validate_handoff(handoff_report)?;
        let counters = domain_bundle.counter_snapshot();
        let certified_semantic_domain_row_count = counters.certified_semantic_row_count();
        let explicit_advanced_family_debt_count = counters.explicit_debt_row_count();
        let roadmap2_physical_debt_explicit = domain_bundle.rows().iter().any(|row| {
            row.required_future_milestone()
                == Some(SupportDomainCertificationDebtOwner::Roadmap2PhysicalDatabaseFoundation)
        });
        let milestone15_extension_debt_explicit = domain_bundle.rows().iter().any(|row| {
            row.required_future_milestone()
                == Some(
                    SupportDomainCertificationDebtOwner::Milestone15ExtensionSupportRegistration,
                )
        });
        if certified_semantic_domain_row_count == 0
            || explicit_advanced_family_debt_count != 2
            || !roadmap2_physical_debt_explicit
            || !milestone15_extension_debt_explicit
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "subscription-support accuracy closeout requires certified first-ship domain rows and explicit future-owned advanced-family debt",
            ));
        }
        Ok(Self {
            certified_semantic_domain_row_count,
            explicit_advanced_family_debt_count,
            roadmap2_physical_debt_explicit,
            milestone15_extension_debt_explicit,
            handoff_semantic_trust_closed: handoff_report.semantic_support_trust_closed(),
            persistence_posture,
        })
    }

    pub fn certified_semantic_domain_row_count(&self) -> u64 {
        self.certified_semantic_domain_row_count
    }

    pub fn explicit_advanced_family_debt_count(&self) -> u64 {
        self.explicit_advanced_family_debt_count
    }

    pub fn roadmap2_physical_debt_explicit(&self) -> bool {
        self.roadmap2_physical_debt_explicit
    }

    pub fn milestone15_extension_debt_explicit(&self) -> bool {
        self.milestone15_extension_debt_explicit
    }

    pub fn handoff_semantic_trust_closed(&self) -> bool {
        self.handoff_semantic_trust_closed
    }

    pub fn persistence_posture(&self) -> SubscriptionSupportAccuracyPersistencePosture {
        self.persistence_posture
    }
}
