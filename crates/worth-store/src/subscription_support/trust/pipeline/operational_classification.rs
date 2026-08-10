use super::super::classification::{
    SupportTrustClassificationCostSurface, SupportTrustClassificationCounterSnapshot,
};
use super::super::drift::SupportTrustDriftReport;
use super::super::epochs::SupportTrustFreshnessWitness;
use super::super::equivalence::SupportTrustTransformedEquivalenceWitness;
use super::super::failure::{
    SupportTrustFailure, SupportTrustFailureKind, SupportTrustRecoveryPosture,
};
use super::super::reports::OperationalSupportTrustReport;
use super::super::taxonomy::SupportTrustStrength;
use super::super::translation::SupportTrustTranslationPlan;
use super::super::witnesses::{
    DegradedSupportTrustWitness, ExactSupportTrustWitness, RebuildDerivedSupportTrustWitness,
    RejectedSupportTrustWitness,
};
use super::equivalence_checked::SupportTrustEquivalenceChecked;
use super::request::RawSupportTrustRequest;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct OperationalSupportTrustClassified {
    report: OperationalSupportTrustReport,
    cost_surface: SupportTrustClassificationCostSurface,
    counter_snapshot: SupportTrustClassificationCounterSnapshot,
}

impl OperationalSupportTrustClassified {
    pub fn report(&self) -> &OperationalSupportTrustReport {
        &self.report
    }

    pub fn cost_surface(&self) -> SupportTrustClassificationCostSurface {
        self.cost_surface
    }

    pub fn counter_snapshot(&self) -> SupportTrustClassificationCounterSnapshot {
        self.counter_snapshot
    }

    pub(super) fn into_certification_report(self) -> OperationalSupportTrustReport {
        self.report
    }
}

pub fn classify_operational_support_trust(
    equivalence_checked: SupportTrustEquivalenceChecked,
) -> Result<OperationalSupportTrustClassified, SupportTrustFailure> {
    let (drift_checked, transformed_equivalence, equivalence_checks_performed) =
        equivalence_checked.into_operational_inputs();
    let (translated, drift_report) = drift_checked.into_operational_inputs();
    let (admitted, translation_plan, receipt_count) = translated.into_operational_inputs();
    let request = admitted.request();
    let cost_surface = classification_cost_surface(
        request,
        receipt_count,
        &drift_report,
        equivalence_checks_performed,
    );
    let freshness = SupportTrustFreshnessWitness::new(request.epoch());
    let report = report_for_translation_plan(
        translation_plan,
        transformed_equivalence,
        request,
        freshness,
        cost_surface,
    )?;
    if request.requested_strength() == SupportTrustStrength::Exact
        && report.trust_strength() != SupportTrustStrength::Exact
    {
        return Err(SupportTrustFailure::new(
            SupportTrustFailureKind::SupportTrustForbiddenExactOverclaim,
            SupportTrustRecoveryPosture::RetryWithFresherReceipts,
            "weaker support posture cannot satisfy an exact trust request",
        ));
    }
    let counter_snapshot = counters_for(&report, cost_surface);
    Ok(OperationalSupportTrustClassified {
        report,
        cost_surface,
        counter_snapshot,
    })
}

fn classification_cost_surface(
    request: &RawSupportTrustRequest,
    receipt_count: u64,
    drift_report: &SupportTrustDriftReport,
    equivalence_checks_performed: u64,
) -> SupportTrustClassificationCostSurface {
    SupportTrustClassificationCostSurface::new(
        request.batch_cardinality().artifact_count(),
        receipt_count,
        drift_report.checks_performed(),
        equivalence_checks_performed,
        request.performance_plan().expected_index_probes() + drift_report.index_probes(),
        request.performance_plan().expected_allocation_count(),
        request.performance_plan().expected_clone_count(),
        drift_report.stale_rejection_count(),
        drift_report.coverage_drift_count(),
        drift_report.placement_advisory_count(),
        drift_report.global_scan_debt_count(),
    )
}

fn report_for_translation_plan(
    translation_plan: SupportTrustTranslationPlan,
    transformed_equivalence: Option<SupportTrustTransformedEquivalenceWitness>,
    request: &RawSupportTrustRequest,
    freshness: SupportTrustFreshnessWitness,
    cost_surface: SupportTrustClassificationCostSurface,
) -> Result<OperationalSupportTrustReport, SupportTrustFailure> {
    let report = match translation_plan {
        SupportTrustTranslationPlan::Exact(translation) => {
            let witness = match transformed_equivalence {
                Some(equivalence) => ExactSupportTrustWitness::from_equivalent_operational_basis(
                    translation,
                    request.provenance(),
                    freshness,
                    equivalence.into_operational_witness(),
                )?,
                None => ExactSupportTrustWitness::from_exact_translation(
                    translation,
                    request.provenance(),
                    freshness,
                )?,
            };
            OperationalSupportTrustReport::from_exact_witness_with_cost(witness, cost_surface)
        }
        SupportTrustTranslationPlan::Degraded { basis, .. } => {
            let witness = DegradedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_degraded_witness(
                witness,
                request.provenance(),
                cost_surface,
            )
        }
        SupportTrustTranslationPlan::RebuildDerived { basis, .. } => {
            let witness = RebuildDerivedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_rebuild_witness(
                witness,
                request.provenance(),
                cost_surface,
            )
        }
        SupportTrustTranslationPlan::Rejected { basis, .. } => {
            let witness = RejectedSupportTrustWitness::new(basis, freshness);
            OperationalSupportTrustReport::from_rejected_witness(
                witness,
                request.provenance(),
                cost_surface,
            )
        }
    };
    Ok(report)
}

fn counters_for(
    report: &OperationalSupportTrustReport,
    cost_surface: SupportTrustClassificationCostSurface,
) -> SupportTrustClassificationCounterSnapshot {
    SupportTrustClassificationCounterSnapshot::new(
        1,
        u64::from(report.trust_strength() == SupportTrustStrength::Exact),
        u64::from(report.trust_strength() == SupportTrustStrength::Degraded),
        u64::from(report.trust_strength() == SupportTrustStrength::RebuildOnly),
        u64::from(report.trust_strength() == SupportTrustStrength::Rejected),
        cost_surface.receipts_consumed(),
        cost_surface.drift_checks_performed(),
        cost_surface.equivalence_checks_performed(),
        0,
        cost_surface.stale_rejection_count(),
        cost_surface.coverage_drift_count(),
        cost_surface.placement_advisory_count(),
        cost_surface.global_scan_debt_count(),
    )
}
