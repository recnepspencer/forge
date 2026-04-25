use super::drift::{SupportTrustDriftCause, SupportTrustSuppressedCause};
use super::epochs::{SupportCertificationEpoch, SupportOperationalLedgerEpoch};
use super::failure::{SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture};
use super::performance::{
    SupportTrustAllocationScope, SupportTrustDensityClass, SupportTrustPathClass,
};
use super::reports::OperationalSupportTrustReport;
use super::taxonomy::{SupportTrustClass, SupportTrustProvenance, SupportTrustStrength};
use crate::subscription_support::{
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportFamilyId,
    SubscriptionSupportFamilyKind, SubscriptionSupportOperationalVerdict, SubscriptionSupportRole,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationLaneDigestSet {
    control_lane_digest: String,
    hostile_lane_digest: String,
    rebuild_or_replay_lane_digest: String,
}

impl SupportCertificationLaneDigestSet {
    pub fn new(
        control_lane_digest: impl Into<String>,
        hostile_lane_digest: impl Into<String>,
        rebuild_or_replay_lane_digest: impl Into<String>,
    ) -> Result<Self, SupportTrustFailure> {
        let control_lane_digest = require_non_empty("control lane digest", control_lane_digest)?;
        let hostile_lane_digest = require_non_empty("hostile lane digest", hostile_lane_digest)?;
        let rebuild_or_replay_lane_digest = require_non_empty(
            "rebuild or replay lane digest",
            rebuild_or_replay_lane_digest,
        )?;
        if control_lane_digest == hostile_lane_digest
            || control_lane_digest == rebuild_or_replay_lane_digest
            || hostile_lane_digest == rebuild_or_replay_lane_digest
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification rows cannot compare a lane to itself",
            ));
        }
        Ok(Self {
            control_lane_digest,
            hostile_lane_digest,
            rebuild_or_replay_lane_digest,
        })
    }

    pub fn control_lane_digest(&self) -> &str {
        &self.control_lane_digest
    }

    pub fn hostile_lane_digest(&self) -> &str {
        &self.hostile_lane_digest
    }

    pub fn rebuild_or_replay_lane_digest(&self) -> &str {
        &self.rebuild_or_replay_lane_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationRowRequirement {
    row_id: String,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    support_role: SubscriptionSupportRole,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    resume_classification: SubscriptionResumeClassification,
    primary_drift_cause: Option<SupportTrustDriftCause>,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportCertificationCoveragePlan {
    operational_ledger_epoch: SupportOperationalLedgerEpoch,
    certification_epoch: SupportCertificationEpoch,
    required_rows: Vec<SupportCertificationRowRequirement>,
}

impl SubscriptionSupportCertificationCoveragePlan {
    pub fn new(
        operational_ledger_epoch: SupportOperationalLedgerEpoch,
        certification_epoch: SupportCertificationEpoch,
        mut required_rows: Vec<SupportCertificationRowRequirement>,
    ) -> Result<Self, SupportTrustFailure> {
        if required_rows.is_empty() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification coverage plans require at least one row",
            ));
        }
        required_rows.sort_by(|left, right| left.row_id.cmp(&right.row_id));
        if required_rows
            .windows(2)
            .any(|pair| pair[0].row_id == pair[1].row_id)
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification coverage plans cannot require duplicate row ids",
            ));
        }
        Ok(Self {
            operational_ledger_epoch,
            certification_epoch,
            required_rows,
        })
    }

    pub fn required_rows(&self) -> &[SupportCertificationRowRequirement] {
        &self.required_rows
    }

    pub fn certification_epoch(&self) -> SupportCertificationEpoch {
        self.certification_epoch
    }

    pub fn operational_ledger_epoch(&self) -> SupportOperationalLedgerEpoch {
        self.operational_ledger_epoch
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationRowEvidence {
    row_id: String,
    family_id: SubscriptionSupportFamilyId,
    family_kind: SubscriptionSupportFamilyKind,
    artifact_id: SubscriptionSupportArtifactId,
    support_role: SubscriptionSupportRole,
    trust_class: SupportTrustClass,
    trust_strength: SupportTrustStrength,
    provenance: SupportTrustProvenance,
    operational_verdict: SubscriptionSupportOperationalVerdict,
    resume_classification: SubscriptionResumeClassification,
    basis_digest: String,
    cursor_checkpoint_digest: String,
    compatibility_epoch: String,
    operational_ledger_epoch: SupportOperationalLedgerEpoch,
    certification_epoch: SupportCertificationEpoch,
    lane_digests: SupportCertificationLaneDigestSet,
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
    primary_drift_cause: Option<SupportTrustDriftCause>,
    suppressed_drift_causes: Vec<SupportTrustSuppressedCause>,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
    declared_row_digest: String,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationRow {
    evidence: SupportCertificationRowEvidence,
}

impl SupportCertificationRow {
    pub fn new(evidence: SupportCertificationRowEvidence) -> Result<Self, SupportTrustFailure> {
        if evidence.declared_row_digest() != evidence.recomputed_row_digest()? {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification row digest does not match structured evidence",
            ));
        }
        if evidence.forbidden_exact_overclaim_count != 0 || evidence.global_scan_debt_count != 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification rows require zero exact-overclaim and global-scan debt counters",
            ));
        }
        Ok(Self { evidence })
    }

    pub fn evidence(&self) -> &SupportCertificationRowEvidence {
        &self.evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationGapReport {
    missing_row_ids: Vec<String>,
}

impl SupportCertificationGapReport {
    pub fn from_plan_and_rows(
        plan: &SubscriptionSupportCertificationCoveragePlan,
        rows: &[SupportCertificationRow],
    ) -> Self {
        let missing_row_ids = plan
            .required_rows()
            .iter()
            .filter(|required| {
                !rows
                    .iter()
                    .any(|row| row.evidence().row_id() == required.row_id())
            })
            .map(|required| required.row_id().to_string())
            .collect();
        Self { missing_row_ids }
    }

    pub fn is_empty(&self) -> bool {
        self.missing_row_ids.is_empty()
    }

    pub fn missing_row_ids(&self) -> &[String] {
        &self.missing_row_ids
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationSummary {
    row_count: u64,
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_digest: String,
    certification_summary_digest: String,
}

impl SupportCertificationSummary {
    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    pub fn certification_summary_digest(&self) -> &str {
        &self.certification_summary_digest
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn subscription_support_digest(&self) -> &str {
        &self.subscription_support_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationCoverageWitness {
    summary: SupportCertificationSummary,
}

impl SupportCertificationCoverageWitness {
    pub(crate) fn new(summary: SupportCertificationSummary) -> Self {
        Self { summary }
    }

    pub fn summary(&self) -> &SupportCertificationSummary {
        &self.summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationCoverageMatrix {
    rows: Vec<SupportCertificationRow>,
    gap_report: SupportCertificationGapReport,
    summary: SupportCertificationSummary,
    witness: SupportCertificationCoverageWitness,
}

impl SupportCertificationCoverageMatrix {
    pub fn from_rows(
        plan: &SubscriptionSupportCertificationCoveragePlan,
        mut rows: Vec<SupportCertificationRow>,
    ) -> Result<Self, SupportTrustFailure> {
        rows.sort_by(|left, right| left.evidence().row_id().cmp(right.evidence().row_id()));
        if rows
            .windows(2)
            .any(|pair| pair[0].evidence().row_id() == pair[1].evidence().row_id())
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification coverage cannot contain duplicate row ids",
            ));
        }
        for required in plan.required_rows() {
            let row = rows
                .iter()
                .find(|row| row.evidence().row_id() == required.row_id())
                .ok_or_else(|| {
                    SupportTrustFailure::new(
                        SupportTrustFailureKind::SupportTrustCoverageMissing,
                        SupportTrustRecoveryPosture::RerunCertification,
                        "support trust certification coverage is missing a required row",
                    )
                })?;
            validate_row_matches_requirement(plan, row.evidence(), required)?;
        }
        let gap_report = SupportCertificationGapReport::from_plan_and_rows(plan, &rows);
        if !gap_report.is_empty() {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification gap report blocks coverage completion",
            ));
        }
        let summary = summarize_rows(&rows)?;
        let witness = SupportCertificationCoverageWitness::new(summary.clone());
        Ok(Self {
            rows,
            gap_report,
            summary,
            witness,
        })
    }

    pub fn rows(&self) -> &[SupportCertificationRow] {
        &self.rows
    }

    pub fn gap_report(&self) -> &SupportCertificationGapReport {
        &self.gap_report
    }

    pub fn summary(&self) -> &SupportCertificationSummary {
        &self.summary
    }

    pub(crate) fn covered_row_id_for_operational_report(
        &self,
        report: &OperationalSupportTrustReport,
    ) -> Option<&str> {
        let cursor_checkpoint_digest = format!(
            "{}:{}",
            report.basis().cursor_digest(),
            report.basis().checkpoint_digest()
        );
        self.rows.iter().find_map(|row| {
            let evidence = row.evidence();
            let matches_report = evidence.family_id == *report.basis().family_id()
                && evidence.family_kind == report.basis().family_kind()
                && evidence.artifact_id == *report.basis().artifact_id()
                && evidence.support_role == report.basis().support_role()
                && evidence.trust_class == report.trust_class()
                && evidence.trust_strength == report.trust_strength()
                && evidence.provenance == report.provenance()
                && evidence.basis_digest == report.basis().basis_digest()
                && evidence.cursor_checkpoint_digest == cursor_checkpoint_digest
                && evidence.compatibility_epoch == report.basis().compatibility_digest();
            matches_report.then_some(evidence.row_id())
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[allow(dead_code)]
pub enum SupportCertificationBatchScopeKind {
    FamilyLocal,
    BasisLocal,
    CertificationScopeLocal,
    DomainScenarioLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportCertificationBatchScope {
    scope_kind: SupportCertificationBatchScopeKind,
    density_class: SupportTrustDensityClass,
    path_class: SupportTrustPathClass,
    allocation_scope: SupportTrustAllocationScope,
    row_count: u64,
    expected_index_probes: u64,
    expected_receipt_reuse_count: u64,
    expected_allocation_count: u64,
}

impl SupportCertificationBatchScope {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        scope_kind: SupportCertificationBatchScopeKind,
        density_class: SupportTrustDensityClass,
        path_class: SupportTrustPathClass,
        allocation_scope: SupportTrustAllocationScope,
        row_count: u64,
        expected_index_probes: u64,
        expected_receipt_reuse_count: u64,
        expected_allocation_count: u64,
    ) -> Result<Self, SupportTrustFailure> {
        if row_count == 0 {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification batch scopes require at least one row",
            ));
        }
        let density_matches_scope = matches!(
            (scope_kind, density_class),
            (
                SupportCertificationBatchScopeKind::FamilyLocal,
                SupportTrustDensityClass::FamilyLocal
            ) | (
                SupportCertificationBatchScopeKind::BasisLocal,
                SupportTrustDensityClass::BasisLocal
            ) | (
                SupportCertificationBatchScopeKind::CertificationScopeLocal,
                SupportTrustDensityClass::CertificationScopeLocal
            ) | (
                SupportCertificationBatchScopeKind::DomainScenarioLocal,
                SupportTrustDensityClass::DomainScenarioLocal
            )
        );
        if !density_matches_scope
            || path_class == SupportTrustPathClass::ForegroundResumeTrustPath
            || allocation_scope == SupportTrustAllocationScope::ForegroundScratch
        {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustAccessStructureDebt,
                SupportTrustRecoveryPosture::RebuildTrustCache,
                "support trust certification batch scopes must use matching bounded batch density, path, and allocation",
            ));
        }
        Ok(Self {
            scope_kind,
            density_class,
            path_class,
            allocation_scope,
            row_count,
            expected_index_probes,
            expected_receipt_reuse_count,
            expected_allocation_count,
        })
    }

    pub fn row_count(&self) -> u64 {
        self.row_count
    }

    pub fn scope_kind(&self) -> SupportCertificationBatchScopeKind {
        self.scope_kind
    }

    pub fn density_class(&self) -> SupportTrustDensityClass {
        self.density_class
    }

    pub fn path_class(&self) -> SupportTrustPathClass {
        self.path_class
    }

    pub fn allocation_scope(&self) -> SupportTrustAllocationScope {
        self.allocation_scope
    }

    pub fn expected_receipt_reuse_count(&self) -> u64 {
        self.expected_receipt_reuse_count
    }

    pub fn expected_index_probes(&self) -> u64 {
        self.expected_index_probes
    }

    pub fn expected_allocation_count(&self) -> u64 {
        self.expected_allocation_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportCertificationCounterSnapshot {
    coverage_row_count: u64,
    first_ship_family_count: u64,
    receipt_reuse_count: u64,
    index_probe_count: u64,
    allocation_count: u64,
    forbidden_exact_overclaim_count: u64,
    global_scan_debt_count: u64,
}

impl SupportCertificationCounterSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        coverage_row_count: u64,
        first_ship_family_count: u64,
        receipt_reuse_count: u64,
        index_probe_count: u64,
        allocation_count: u64,
        forbidden_exact_overclaim_count: u64,
        global_scan_debt_count: u64,
    ) -> Self {
        Self {
            coverage_row_count,
            first_ship_family_count,
            receipt_reuse_count,
            index_probe_count,
            allocation_count,
            forbidden_exact_overclaim_count,
            global_scan_debt_count,
        }
    }

    pub fn coverage_row_count(&self) -> u64 {
        self.coverage_row_count
    }

    pub fn first_ship_family_count(&self) -> u64 {
        self.first_ship_family_count
    }

    pub fn receipt_reuse_count(&self) -> u64 {
        self.receipt_reuse_count
    }

    pub fn forbidden_exact_overclaim_count(&self) -> u64 {
        self.forbidden_exact_overclaim_count
    }

    pub fn global_scan_debt_count(&self) -> u64 {
        self.global_scan_debt_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportCertificationEvidenceBundle {
    run_id: String,
    coverage_matrix: SupportCertificationCoverageMatrix,
    batch_scope: SupportCertificationBatchScope,
    counter_snapshot: SupportCertificationCounterSnapshot,
    artifact_digest: String,
    subscription_support_digest: String,
    diagnostics_digest: String,
    counter_snapshot_digest: String,
    certification_summary_digest: String,
    evidence_bundle_digest: String,
}

impl SupportCertificationEvidenceBundle {
    pub fn new(
        run_id: impl Into<String>,
        coverage_matrix: SupportCertificationCoverageMatrix,
        batch_scope: SupportCertificationBatchScope,
        counter_snapshot: SupportCertificationCounterSnapshot,
    ) -> Result<Self, SupportTrustFailure> {
        validate_first_ship_family_coverage(&coverage_matrix)?;
        validate_certification_counters(&coverage_matrix, batch_scope, counter_snapshot)?;
        let summary = coverage_matrix.summary();
        let counter_snapshot_digest = stable_digest(&counter_snapshot)?;
        let mut bundle = Self {
            run_id: require_non_empty("run id", run_id)?,
            artifact_digest: summary.artifact_digest().to_string(),
            subscription_support_digest: summary.subscription_support_digest().to_string(),
            diagnostics_digest: summary.diagnostics_digest().to_string(),
            counter_snapshot_digest,
            certification_summary_digest: summary.certification_summary_digest().to_string(),
            coverage_matrix,
            batch_scope,
            counter_snapshot,
            evidence_bundle_digest: String::new(),
        };
        bundle.evidence_bundle_digest =
            stable_digest(&SupportCertificationEvidenceBundleDigestBasis {
                run_id: &bundle.run_id,
                artifact_digest: &bundle.artifact_digest,
                subscription_support_digest: &bundle.subscription_support_digest,
                diagnostics_digest: &bundle.diagnostics_digest,
                counter_snapshot_digest: &bundle.counter_snapshot_digest,
                certification_summary_digest: &bundle.certification_summary_digest,
                batch_scope: &bundle.batch_scope,
                counter_snapshot: &bundle.counter_snapshot,
            })?;
        Ok(bundle)
    }

    pub fn evidence_bundle_digest(&self) -> &str {
        &self.evidence_bundle_digest
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn subscription_support_digest(&self) -> &str {
        &self.subscription_support_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn certification_summary_digest(&self) -> &str {
        &self.certification_summary_digest
    }

    pub fn counter_snapshot(&self) -> SupportCertificationCounterSnapshot {
        self.counter_snapshot
    }

    pub(crate) fn coverage_rows(&self) -> &[SupportCertificationRow] {
        self.coverage_matrix.rows()
    }

    pub(crate) fn covered_row_id_for_operational_report(
        &self,
        report: &OperationalSupportTrustReport,
    ) -> Option<&str> {
        self.coverage_matrix
            .covered_row_id_for_operational_report(report)
    }

    pub(crate) fn into_witness(self) -> SupportCertificationCoverageWitness {
        self.coverage_matrix.witness
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

#[derive(Serialize)]
struct SupportCertificationEvidenceBundleDigestBasis<'a> {
    run_id: &'a str,
    artifact_digest: &'a str,
    subscription_support_digest: &'a str,
    diagnostics_digest: &'a str,
    counter_snapshot_digest: &'a str,
    certification_summary_digest: &'a str,
    batch_scope: &'a SupportCertificationBatchScope,
    counter_snapshot: &'a SupportCertificationCounterSnapshot,
}

fn validate_row_matches_requirement(
    plan: &SubscriptionSupportCertificationCoveragePlan,
    evidence: &SupportCertificationRowEvidence,
    required: &SupportCertificationRowRequirement,
) -> Result<(), SupportTrustFailure> {
    let matches_requirement = evidence.family_id == required.family_id
        && evidence.family_kind == required.family_kind
        && evidence.support_role == required.support_role
        && evidence.trust_class == required.trust_class
        && evidence.trust_strength == required.trust_strength
        && evidence.provenance == required.provenance
        && evidence.operational_verdict == required.operational_verdict
        && evidence.resume_classification == required.resume_classification
        && evidence.primary_drift_cause == required.primary_drift_cause
        && evidence.certification_epoch == plan.certification_epoch()
        && evidence.operational_ledger_epoch == plan.operational_ledger_epoch();
    if matches_requirement {
        Ok(())
    } else {
        Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "support trust certification row label does not match structured evidence",
        ))
    }
}

fn validate_first_ship_family_coverage(
    matrix: &SupportCertificationCoverageMatrix,
) -> Result<(), SupportTrustFailure> {
    for (row_id, family_id, family_kind, support_role) in [
        (
            "row:basis-bound-exact",
            "basis-bound-continuation-support",
            SubscriptionSupportFamilyKind::BasisBoundContinuationSupport,
            SubscriptionSupportRole::ExactContinuation,
        ),
        (
            "row:materialized-narrowing-exact",
            "materialized-narrowing-support",
            SubscriptionSupportFamilyKind::MaterializedNarrowingSupport,
            SubscriptionSupportRole::NarrowingMaterialization,
        ),
        (
            "row:degraded-continuation",
            "degraded-continuation-support",
            SubscriptionSupportFamilyKind::DegradedContinuationSupport,
            SubscriptionSupportRole::DegradedContinuation,
        ),
        (
            "row:extension-defined-rejected",
            "extension-defined-support",
            SubscriptionSupportFamilyKind::ExtensionDefinedSupport,
            SubscriptionSupportRole::ExactContinuation,
        ),
    ] {
        if !matrix.rows().iter().any(|row| {
            let evidence = row.evidence();
            evidence.row_id() == row_id
                && evidence.family_id.as_str() == family_id
                && evidence.family_kind == family_kind
                && evidence.support_role == support_role
        }) {
            return Err(SupportTrustFailure::new(
                SupportTrustFailureKind::SupportTrustCoverageMissing,
                SupportTrustRecoveryPosture::RerunCertification,
                "support trust certification bundle is missing required canonical first-ship family coverage",
            ));
        }
    }
    Ok(())
}

fn validate_certification_counters(
    matrix: &SupportCertificationCoverageMatrix,
    batch_scope: SupportCertificationBatchScope,
    counter_snapshot: SupportCertificationCounterSnapshot,
) -> Result<(), SupportTrustFailure> {
    let row_count = matrix.rows().len() as u64;
    let first_ship_family_count = matrix
        .rows()
        .iter()
        .map(|row| row.evidence().family_kind)
        .collect::<BTreeSet<_>>()
        .len() as u64;
    if batch_scope.row_count() != row_count
        || counter_snapshot.coverage_row_count != row_count
        || counter_snapshot.first_ship_family_count != first_ship_family_count
        || counter_snapshot.receipt_reuse_count != batch_scope.expected_receipt_reuse_count()
        || counter_snapshot.index_probe_count != batch_scope.expected_index_probes()
        || counter_snapshot.allocation_count != batch_scope.expected_allocation_count()
        || counter_snapshot.forbidden_exact_overclaim_count != 0
        || counter_snapshot.global_scan_debt_count != 0
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "support trust certification bundle counters must match declared scope and zero-overclaim invariants",
        ));
    }
    Ok(())
}

fn summarize_rows(
    rows: &[SupportCertificationRow],
) -> Result<SupportCertificationSummary, SupportTrustFailure> {
    let artifact_digests = rows
        .iter()
        .map(|row| row.evidence().artifact_digest.as_str())
        .collect::<Vec<_>>();
    let subscription_support_digests = rows
        .iter()
        .map(|row| row.evidence().subscription_support_digest.as_str())
        .collect::<Vec<_>>();
    let diagnostics_digests = rows
        .iter()
        .map(|row| row.evidence().diagnostics_digest.as_str())
        .collect::<Vec<_>>();
    let counter_digests = rows
        .iter()
        .map(|row| row.evidence().counter_digest.as_str())
        .collect::<Vec<_>>();
    let row_digests = rows
        .iter()
        .map(|row| row.evidence().declared_row_digest())
        .collect::<Vec<_>>();
    let mut summary = SupportCertificationSummary {
        row_count: rows.len() as u64,
        artifact_digest: stable_digest(&artifact_digests)?,
        subscription_support_digest: stable_digest(&subscription_support_digests)?,
        diagnostics_digest: stable_digest(&diagnostics_digests)?,
        counter_digest: stable_digest(&counter_digests)?,
        certification_summary_digest: String::new(),
    };
    summary.certification_summary_digest = stable_digest(&(
        summary.row_count,
        &summary.artifact_digest,
        &summary.subscription_support_digest,
        &summary.diagnostics_digest,
        &summary.counter_digest,
        row_digests,
    ))?;
    Ok(summary)
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> Result<String, SupportTrustFailure> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustCoverageMissing,
            SupportTrustRecoveryPosture::RerunCertification,
            "support trust certification evidence must serialize deterministically",
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
            format!("support trust certification {label} must be non-empty"),
        ));
    }
    Ok(value)
}
