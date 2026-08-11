use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::reports::OperationalSupportTrustReport;
use super::certification_row::SupportCertificationRow;
use super::coverage_plan::SubscriptionSupportCertificationCoveragePlan;
use super::row_evidence::SupportCertificationRowEvidence;
use super::row_requirement::SupportCertificationRowRequirement;
use super::summary::{summarize_rows, SupportCertificationGapReport, SupportCertificationSummary};
use crate::subscription_support::{SubscriptionSupportFamilyKind, SubscriptionSupportRole};
use serde::Serialize;

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

    pub(super) fn into_witness(self) -> SupportCertificationCoverageWitness {
        self.witness
    }
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

pub(super) fn validate_first_ship_family_coverage(
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
