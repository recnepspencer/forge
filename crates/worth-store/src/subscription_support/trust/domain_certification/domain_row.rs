use super::digest::stable_digest;
use super::generic_certification::SupportGenericCertificationReport;
use super::scenario::{
    required_scenario_debt, required_scenario_family, required_scenario_row_status,
    SupportDomainCertificationDebtOwner, SupportDomainCertificationDebtReason,
    SupportDomainCertificationRowStatus, SupportDomainCertificationScenario,
    SupportRoadmapPhysicalReadinessPosture,
};
use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use crate::subscription_support::trust::taxonomy::{SupportTrustClass, SupportTrustStrength};
use crate::subscription_support::{SubscriptionSupportFamilyKind, SubscriptionSupportRole};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationRow {
    scenario: SupportDomainCertificationScenario,
    row_status: SupportDomainCertificationRowStatus,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    required_trust_strength: SupportTrustStrength,
    required_trust_class: SupportTrustClass,
    source_generic_row_id: String,
    semantic_support_digest: String,
    physical_readiness_posture: SupportRoadmapPhysicalReadinessPosture,
    debt_reason: Option<SupportDomainCertificationDebtReason>,
    required_future_milestone: Option<SupportDomainCertificationDebtOwner>,
    row_digest: String,
}

impl SupportDomainCertificationRow {
    pub fn certified_from_generic_report(
        scenario: SupportDomainCertificationScenario,
        generic_report: &SupportGenericCertificationReport,
    ) -> Result<Self, SupportTrustFailure> {
        if required_scenario_row_status(scenario)
            != SupportDomainCertificationRowStatus::CertifiedSemanticSupport
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::WaitForMilestone14OrRoadmap2Evidence,
                "domain support certification cannot mark an advanced-family debt scenario as certified semantic support",
            ));
        }
        let certified = generic_report.certified_report();
        let operational = certified.witness().operational();
        let basis = operational.basis();
        let expected = required_scenario_family(scenario);
        if basis.family_kind() != expected.0 || basis.support_role() != expected.1 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustRoleMismatch,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification row must be bound to the scenario family kind and support role",
            ));
        }
        if operational.trust_strength() != expected.2 || certified.trust_class() != expected.3 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification row must preserve the scenario trust strength and class",
            ));
        }
        Self::new(
            scenario,
            SupportDomainCertificationRowStatus::CertifiedSemanticSupport,
            basis.family_kind(),
            basis.support_role(),
            expected.2,
            expected.3,
            generic_report,
            SupportRoadmapPhysicalReadinessPosture::SemanticSupportTrustCertified,
            None,
            None,
        )
    }

    pub fn explicit_advanced_family_debt(
        scenario: SupportDomainCertificationScenario,
        generic_report: &SupportGenericCertificationReport,
    ) -> Result<Self, SupportTrustFailure> {
        if required_scenario_row_status(scenario)
            != SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "first-ship domain scenarios require certified semantic support rows, not explicit advanced-family debt",
            ));
        }
        let (family_kind, support_role, trust_strength, trust_class) =
            required_scenario_family(scenario);
        let (debt_reason, required_future_milestone) = required_scenario_debt(scenario)?;
        Self::new(
            scenario,
            SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt,
            family_kind,
            support_role,
            trust_strength,
            trust_class,
            generic_report,
            SupportRoadmapPhysicalReadinessPosture::PhysicalDatabaseReadinessDeferredToRoadmap2,
            Some(debt_reason),
            Some(required_future_milestone),
        )
    }

    pub fn scenario(&self) -> SupportDomainCertificationScenario {
        self.scenario
    }

    pub fn row_status(&self) -> SupportDomainCertificationRowStatus {
        self.row_status
    }

    pub fn physical_readiness_posture(&self) -> SupportRoadmapPhysicalReadinessPosture {
        self.physical_readiness_posture
    }

    pub fn family_kind(&self) -> SubscriptionSupportFamilyKind {
        self.family_kind
    }

    pub fn support_role(&self) -> SubscriptionSupportRole {
        self.support_role
    }

    pub fn required_trust_strength(&self) -> SupportTrustStrength {
        self.required_trust_strength
    }

    pub fn required_trust_class(&self) -> SupportTrustClass {
        self.required_trust_class
    }

    pub fn debt_reason(&self) -> Option<SupportDomainCertificationDebtReason> {
        self.debt_reason
    }

    pub fn required_future_milestone(&self) -> Option<SupportDomainCertificationDebtOwner> {
        self.required_future_milestone
    }

    pub(crate) fn row_digest(&self) -> &str {
        &self.row_digest
    }

    fn new(
        scenario: SupportDomainCertificationScenario,
        row_status: SupportDomainCertificationRowStatus,
        family_kind: SubscriptionSupportFamilyKind,
        support_role: SubscriptionSupportRole,
        required_trust_strength: SupportTrustStrength,
        required_trust_class: SupportTrustClass,
        generic_report: &SupportGenericCertificationReport,
        physical_readiness_posture: SupportRoadmapPhysicalReadinessPosture,
        debt_reason: Option<SupportDomainCertificationDebtReason>,
        required_future_milestone: Option<SupportDomainCertificationDebtOwner>,
    ) -> Result<Self, SupportTrustFailure> {
        let mut row = Self {
            scenario,
            row_status,
            family_kind,
            support_role,
            required_trust_strength,
            required_trust_class,
            source_generic_row_id: generic_report.generic_row_id().to_string(),
            semantic_support_digest: generic_report.generic_certification_digest().to_string(),
            physical_readiness_posture,
            debt_reason,
            required_future_milestone,
            row_digest: String::new(),
        };
        row.row_digest = stable_digest(&SupportDomainCertificationRowDigestBasis {
            scenario: row.scenario,
            row_status: row.row_status,
            family_kind: row.family_kind,
            support_role: row.support_role,
            required_trust_strength: row.required_trust_strength,
            required_trust_class: row.required_trust_class,
            source_generic_row_id: &row.source_generic_row_id,
            semantic_support_digest: &row.semantic_support_digest,
            physical_readiness_posture: row.physical_readiness_posture,
            debt_reason: row.debt_reason,
            required_future_milestone: row.required_future_milestone,
        })?;
        Ok(row)
    }
}

#[derive(Serialize)]
struct SupportDomainCertificationRowDigestBasis<'a> {
    scenario: SupportDomainCertificationScenario,
    row_status: SupportDomainCertificationRowStatus,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    required_trust_strength: SupportTrustStrength,
    required_trust_class: SupportTrustClass,
    source_generic_row_id: &'a str,
    semantic_support_digest: &'a str,
    physical_readiness_posture: SupportRoadmapPhysicalReadinessPosture,
    debt_reason: Option<SupportDomainCertificationDebtReason>,
    required_future_milestone: Option<SupportDomainCertificationDebtOwner>,
}
