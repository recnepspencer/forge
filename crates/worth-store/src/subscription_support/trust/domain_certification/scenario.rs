use crate::subscription_support::trust::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use crate::subscription_support::trust::taxonomy::{SupportTrustClass, SupportTrustStrength};
use crate::subscription_support::{SubscriptionSupportFamilyKind, SubscriptionSupportRole};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum SupportDomainCertificationScenario {
    GeometryCadSessionContinuation,
    WebDataRestartReplication,
    AiBranchWorkspaceDegradedContinuation,
    ChipSimulationLongHistoryRebuild,
    OfflineCollaborativeCapsuleOmission,
}

impl SupportDomainCertificationScenario {
    pub fn first_ship_required() -> &'static [Self] {
        &FIRST_SHIP_DOMAIN_SCENARIOS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportDomainCertificationRowStatus {
    CertifiedSemanticSupport,
    ExplicitAdvancedFamilyDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportRoadmapPhysicalReadinessPosture {
    SemanticSupportTrustCertified,
    PhysicalDatabaseReadinessDeferredToRoadmap2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportDomainCertificationDebtOwner {
    Roadmap2PhysicalDatabaseFoundation,
    Milestone15ExtensionSupportRegistration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportDomainCertificationDebtReason {
    RebuildEquivalenceLaneDeferred,
    OmittedSupportImportLaneDeferred,
}

pub(super) fn required_scenario_family(
    scenario: SupportDomainCertificationScenario,
) -> (
    SubscriptionSupportFamilyKind,
    SubscriptionSupportRole,
    SupportTrustStrength,
    SupportTrustClass,
) {
    match scenario {
        SupportDomainCertificationScenario::GeometryCadSessionContinuation => (
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Exact,
            SupportTrustClass::ExactSupportTrusted,
        ),
        SupportDomainCertificationScenario::WebDataRestartReplication => (
            SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
            SubscriptionSupportRole::NarrowingMaterialization,
            SupportTrustStrength::Exact,
            SupportTrustClass::ExactSupportTrusted,
        ),
        SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation => (
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            SubscriptionSupportRole::DegradedContinuation,
            SupportTrustStrength::Degraded,
            SupportTrustClass::DegradedSupportTrusted,
        ),
        SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild => (
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::RebuildOnly,
            SupportTrustClass::RebuildDerivedSupport,
        ),
        SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission => (
            SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
            SubscriptionSupportRole::ExactContinuation,
            SupportTrustStrength::Rejected,
            SupportTrustClass::StaleSupportRejected,
        ),
    }
}

pub(super) fn required_scenario_row_status(
    scenario: SupportDomainCertificationScenario,
) -> SupportDomainCertificationRowStatus {
    match scenario {
        SupportDomainCertificationScenario::GeometryCadSessionContinuation
        | SupportDomainCertificationScenario::WebDataRestartReplication
        | SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation => {
            SupportDomainCertificationRowStatus::CertifiedSemanticSupport
        }
        SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild
        | SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission => {
            SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt
        }
    }
}

pub(super) fn required_scenario_debt(
    scenario: SupportDomainCertificationScenario,
) -> Result<
    (
        SupportDomainCertificationDebtReason,
        SupportDomainCertificationDebtOwner,
    ),
    SupportTrustFailure,
> {
    match scenario {
        SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild => Ok((
            SupportDomainCertificationDebtReason::RebuildEquivalenceLaneDeferred,
            SupportDomainCertificationDebtOwner::Roadmap2PhysicalDatabaseFoundation,
        )),
        SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission => Ok((
            SupportDomainCertificationDebtReason::OmittedSupportImportLaneDeferred,
            SupportDomainCertificationDebtOwner::Milestone15ExtensionSupportRegistration,
        )),
        _ => Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::RerunCertification,
            "only advanced-family domain scenarios may carry explicit debt metadata",
        )),
    }
}

const FIRST_SHIP_DOMAIN_SCENARIOS: [SupportDomainCertificationScenario; 5] = [
    SupportDomainCertificationScenario::GeometryCadSessionContinuation,
    SupportDomainCertificationScenario::WebDataRestartReplication,
    SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation,
    SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild,
    SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission,
];
