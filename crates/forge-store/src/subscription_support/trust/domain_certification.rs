use super::certification::{
    SupportCertificationBatchScope, SupportCertificationBatchScopeKind,
    SupportCertificationCoverageWitness, SupportCertificationSummary,
};
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::performance::{
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
};
use super::reports::CertifiedSupportTrustReport;
use super::taxonomy::{SupportTrustClass, SupportTrustStrength};
use crate::subscription_support::{SubscriptionSupportFamilyKind, SubscriptionSupportRole};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportGenericCertificationCounterSnapshot {
    certified_support_report_count: u64,
    generic_row_count: u64,
    index_probe_count: u64,
    receipt_reuse_count: u64,
    allocation_count: u64,
    physical_readiness_debt_count: u64,
}

impl SupportGenericCertificationCounterSnapshot {
    pub fn new(
        certified_support_report_count: u64,
        generic_row_count: u64,
        index_probe_count: u64,
        receipt_reuse_count: u64,
        allocation_count: u64,
        physical_readiness_debt_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if certified_support_report_count == 0 || generic_row_count == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "generic support trust certification requires at least one certified support report and one generic row",
            ));
        }
        Ok(Self {
            certified_support_report_count,
            generic_row_count,
            index_probe_count,
            receipt_reuse_count,
            allocation_count,
            physical_readiness_debt_count,
        })
    }

    pub fn certified_support_report_count(&self) -> u64 {
        self.certified_support_report_count
    }

    pub fn physical_readiness_debt_count(&self) -> u64 {
        self.physical_readiness_debt_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportGenericCertificationReport {
    generic_row_id: String,
    certified_report: CertifiedSupportTrustReport,
    coverage_summary: SupportCertificationSummary,
    counter_snapshot: SupportGenericCertificationCounterSnapshot,
    generic_certification_digest: String,
}

impl SupportGenericCertificationReport {
    pub fn from_certified_support_trust(
        generic_row_id: impl Into<String>,
        certified_report: CertifiedSupportTrustReport,
        coverage_witness: &SupportCertificationCoverageWitness,
        counter_snapshot: SupportGenericCertificationCounterSnapshot,
    ) -> Result<Self, SupportTrustFailure> {
        let generic_row_id = require_non_empty("generic row id", generic_row_id)?;
        if certified_report.certification_stamp().trust_strength() == SupportTrustStrength::Rejected
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "generic support certification cannot advertise rejected support as certified semantic support",
            ));
        }
        let coverage_summary = coverage_witness.summary().clone();
        let mut report = Self {
            generic_row_id,
            certified_report,
            coverage_summary,
            counter_snapshot,
            generic_certification_digest: String::new(),
        };
        report.generic_certification_digest =
            stable_digest(&SupportGenericCertificationDigestBasis {
                generic_row_id: &report.generic_row_id,
                certified_report: &report.certified_report,
                coverage_summary: &report.coverage_summary,
                counter_snapshot: report.counter_snapshot,
            })?;
        Ok(report)
    }

    pub fn generic_row_id(&self) -> &str {
        &self.generic_row_id
    }

    pub fn certified_report(&self) -> &CertifiedSupportTrustReport {
        &self.certified_report
    }

    pub fn coverage_summary(&self) -> &SupportCertificationSummary {
        &self.coverage_summary
    }

    pub fn counter_snapshot(&self) -> SupportGenericCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn generic_certification_digest(&self) -> &str {
        &self.generic_certification_digest
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationCounterSnapshot {
    scenario_row_count: u64,
    certified_semantic_row_count: u64,
    explicit_debt_row_count: u64,
    index_probe_count: u64,
    receipt_reuse_count: u64,
    allocation_count: u64,
    physical_readiness_debt_count: u64,
}

impl SupportDomainCertificationCounterSnapshot {
    pub fn new(
        scenario_row_count: u64,
        certified_semantic_row_count: u64,
        explicit_debt_row_count: u64,
        index_probe_count: u64,
        receipt_reuse_count: u64,
        allocation_count: u64,
        physical_readiness_debt_count: u64,
    ) -> Self {
        Self {
            scenario_row_count,
            certified_semantic_row_count,
            explicit_debt_row_count,
            index_probe_count,
            receipt_reuse_count,
            allocation_count,
            physical_readiness_debt_count,
        }
    }

    pub fn scenario_row_count(&self) -> u64 {
        self.scenario_row_count
    }

    pub fn physical_readiness_debt_count(&self) -> u64 {
        self.physical_readiness_debt_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationBatchPlan {
    scenario_width: u64,
    family_role_row_width: u64,
    batch_scope: SupportCertificationBatchScope,
    max_scenario_rows: u64,
}

impl SupportDomainCertificationBatchPlan {
    pub fn new(
        scenario_width: u64,
        family_role_row_width: u64,
        batch_scope: SupportCertificationBatchScope,
        max_scenario_rows: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if scenario_width == 0 || family_role_row_width == 0 || max_scenario_rows == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain certification plans require non-zero scenario, family-role, and budget widths",
            ));
        }
        if batch_scope.scope_kind() != SupportCertificationBatchScopeKind::DomainScenarioLocal {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "domain certification plans require domain-scenario-local batch scope",
            ));
        }
        if batch_scope.density_class() != SupportTrustDensityClass::DomainScenarioLocal
            || batch_scope.path_class() != SupportTrustPathClass::DomainCertificationPath
            || batch_scope.allocation_scope() != SupportTrustAllocationScope::DomainCertification
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "domain certification plans require domain density, path, and allocation scope",
            ));
        }
        if scenario_width > max_scenario_rows {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustPayloadBudgetExceeded,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain certification scenario width exceeds the declared scenario budget",
            ));
        }
        Ok(Self {
            scenario_width,
            family_role_row_width,
            batch_scope,
            max_scenario_rows,
        })
    }

    pub fn scenario_width(&self) -> u64 {
        self.scenario_width
    }

    pub fn family_role_row_width(&self) -> u64 {
        self.family_role_row_width
    }

    pub fn batch_scope(&self) -> SupportCertificationBatchScope {
        self.batch_scope
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportDomainCertificationBundle {
    rows: Vec<SupportDomainCertificationRow>,
    batch_plan: SupportDomainCertificationBatchPlan,
    counter_snapshot: SupportDomainCertificationCounterSnapshot,
    domain_certification_digest: String,
}

impl SupportDomainCertificationBundle {
    pub fn new(
        mut rows: Vec<SupportDomainCertificationRow>,
        batch_plan: SupportDomainCertificationBatchPlan,
        counter_snapshot: SupportDomainCertificationCounterSnapshot,
    ) -> Result<Self, SupportTrustFailure> {
        rows.sort_by_key(SupportDomainCertificationRow::scenario);
        validate_required_domain_rows(&rows)?;
        validate_domain_counters(&rows, &batch_plan, counter_snapshot)?;
        let mut bundle = Self {
            rows,
            batch_plan,
            counter_snapshot,
            domain_certification_digest: String::new(),
        };
        bundle.domain_certification_digest = stable_digest(&SupportDomainBundleDigestBasis {
            rows: &bundle.rows,
            batch_plan: &bundle.batch_plan,
            counter_snapshot: bundle.counter_snapshot,
        })?;
        Ok(bundle)
    }

    pub fn rows(&self) -> &[SupportDomainCertificationRow] {
        &self.rows
    }

    pub fn counter_snapshot(&self) -> SupportDomainCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub fn domain_certification_digest(&self) -> &str {
        &self.domain_certification_digest
    }
}

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

    pub fn roadmap_physical_readiness_posture(&self) -> SupportRoadmapPhysicalReadinessPosture {
        self.roadmap_physical_readiness_posture
    }

    pub fn handoff_digest(&self) -> &str {
        &self.handoff_digest
    }
}

#[derive(Serialize)]
struct SupportGenericCertificationDigestBasis<'a> {
    generic_row_id: &'a str,
    certified_report: &'a CertifiedSupportTrustReport,
    coverage_summary: &'a SupportCertificationSummary,
    counter_snapshot: SupportGenericCertificationCounterSnapshot,
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

#[derive(Serialize)]
struct SupportDomainBundleDigestBasis<'a> {
    rows: &'a [SupportDomainCertificationRow],
    batch_plan: &'a SupportDomainCertificationBatchPlan,
    counter_snapshot: SupportDomainCertificationCounterSnapshot,
}

#[derive(Serialize)]
struct SupportCertificationHandoffDigestBasis<'a> {
    generic_certification_digest: &'a str,
    domain_certification_digest: &'a str,
    semantic_support_trust_closed: bool,
    roadmap_physical_readiness_posture: SupportRoadmapPhysicalReadinessPosture,
    handoff_counter_snapshot: SupportDomainCertificationCounterSnapshot,
}

const FIRST_SHIP_DOMAIN_SCENARIOS: [SupportDomainCertificationScenario; 5] = [
    SupportDomainCertificationScenario::GeometryCadSessionContinuation,
    SupportDomainCertificationScenario::WebDataRestartReplication,
    SupportDomainCertificationScenario::AiBranchWorkspaceDegradedContinuation,
    SupportDomainCertificationScenario::ChipSimulationLongHistoryRebuild,
    SupportDomainCertificationScenario::OfflineCollaborativeCapsuleOmission,
];

fn required_scenario_family(
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

fn required_scenario_row_status(
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

fn required_scenario_debt(
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

fn validate_required_domain_rows(
    rows: &[SupportDomainCertificationRow],
) -> Result<(), SupportTrustFailure> {
    let mut seen = BTreeSet::new();
    for row in rows {
        if !seen.insert(row.scenario()) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification cannot contain duplicate scenario rows",
            ));
        }
        if row.row_status() != required_scenario_row_status(row.scenario()) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification row status must match the scenario's required certification posture",
            ));
        }
        let expected = required_scenario_family(row.scenario());
        if row.family_kind() != expected.0
            || row.support_role() != expected.1
            || row.required_trust_strength() != expected.2
            || row.required_trust_class() != expected.3
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustRoleMismatch,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification row must preserve the scenario family, role, and required trust posture",
            ));
        }
        if row.row_status() == SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt
            && (row.debt_reason().is_none() || row.required_future_milestone().is_none())
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "explicit domain support debt rows must name the debt reason and future owner",
            ));
        }
        if row.row_status() == SupportDomainCertificationRowStatus::CertifiedSemanticSupport
            && (row.debt_reason().is_some() || row.required_future_milestone().is_some())
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "certified semantic support rows cannot carry future-debt metadata",
            ));
        }
    }
    for scenario in SupportDomainCertificationScenario::first_ship_required() {
        if !seen.contains(scenario) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "domain support certification is missing a required first-ship scenario row",
            ));
        }
    }
    Ok(())
}

fn validate_domain_counters(
    rows: &[SupportDomainCertificationRow],
    batch_plan: &SupportDomainCertificationBatchPlan,
    counter_snapshot: SupportDomainCertificationCounterSnapshot,
) -> Result<(), SupportTrustFailure> {
    let row_count = rows.len() as u64;
    let certified_count = rows
        .iter()
        .filter(|row| {
            row.row_status() == SupportDomainCertificationRowStatus::CertifiedSemanticSupport
        })
        .count() as u64;
    let debt_count = rows
        .iter()
        .filter(|row| {
            row.row_status() == SupportDomainCertificationRowStatus::ExplicitAdvancedFamilyDebt
        })
        .count() as u64;
    let scope = batch_plan.batch_scope();
    if batch_plan.scenario_width() != row_count
        || batch_plan.family_role_row_width() != row_count
        || scope.row_count() != row_count
        || counter_snapshot.scenario_row_count != row_count
        || counter_snapshot.certified_semantic_row_count != certified_count
        || counter_snapshot.explicit_debt_row_count != debt_count
        || counter_snapshot.index_probe_count != scope.expected_index_probes()
        || counter_snapshot.receipt_reuse_count != scope.expected_receipt_reuse_count()
        || counter_snapshot.allocation_count != scope.expected_allocation_count()
        || counter_snapshot.physical_readiness_debt_count != debt_count
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification counters must match declared scenario width and explicit debt rows",
        ));
    }
    Ok(())
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> Result<String, SupportTrustFailure> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "domain support certification evidence must serialize deterministically",
        )
    })?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    Ok(format!("{:x}", hasher.finalize()))
}

fn require_non_empty(
    label: &'static str,
    value: impl Into<String>,
) -> Result<String, SupportTrustFailure> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            format!("support trust domain certification {label} must be non-empty"),
        ));
    }
    Ok(value)
}
