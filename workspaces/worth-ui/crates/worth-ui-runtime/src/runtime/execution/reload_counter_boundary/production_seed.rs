use crate::facade::prepared_application_authority::WorthUiPreparedApplicationGenerationIdentity;
use crate::runtime::{
    WorthUiPlanConstructionCounters, WorthUiPlanEquivalenceSummary,
    WorthUiReloadCounterBoundaryDenial, WorthUiReloadLoweringCounterReceipt,
    WorthUiReloadLoweringCounterReceiptBuilder, WorthUiReplacementLoweringReady,
};

/// Counter state captured from the real replacement phases before activation
/// consumes their typed transition values.
#[derive(Clone, Debug)]
pub(crate) struct WorthUiReloadCostSeed {
    builder: WorthUiReloadLoweringCounterReceiptBuilder,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    affected_scope_count: usize,
}

impl WorthUiReloadCostSeed {
    pub(crate) fn from_lowering(lowering: &WorthUiReplacementLoweringReady) -> Self {
        let report = lowering.admitted.report();
        let builder = WorthUiReloadLoweringCounterReceiptBuilder::new(
            super::WorthUiReloadCounterStopStage::PlanEquivalence,
        )
        .record_admission_counters(report.counters())
        .record_artifact_comparison_counters(lowering.artifact_comparison_counters)
        .record_impact_narrowing_counters(lowering.narrowing.counters())
        .record_identity_match_counters(lowering.identity_match_counters)
        .record_reconciliation_counters(lowering.reconciliation_plan.counters())
        .record_query_rebind_counters(lowering.query_rebind_plan.counters());
        Self {
            builder,
            active_artifact_digest: lowering.narrowing.active_artifact_digest(),
            candidate_artifact_digest: lowering.narrowing.candidate_artifact_digest(),
            affected_scope_count: lowering.narrowing.affected_handle_count(),
        }
    }

    pub(crate) fn finish(
        self,
        active_generation: WorthUiPreparedApplicationGenerationIdentity,
        candidate_generation: WorthUiPreparedApplicationGenerationIdentity,
        active_plan_digest: u64,
        construction: WorthUiPlanConstructionCounters,
        equivalence: WorthUiPlanEquivalenceSummary,
    ) -> Result<WorthUiReloadLoweringCounterReceipt, WorthUiReloadCounterBoundaryDenial> {
        let context = super::WorthUiReloadCostContext::new(
            active_generation,
            candidate_generation,
            self.active_artifact_digest,
            self.candidate_artifact_digest,
            active_plan_digest,
            equivalence.candidate_fingerprint(),
            self.affected_scope_count,
        );
        self.builder
            .record_plan_lowering_counters(construction.lowering())
            .record_plan_assembly_counters(
                construction.handle_allocation(),
                construction.topology(),
                crate::runtime::WorthUiExecutionPlanEquivalenceCounters::for_reload_receipt(
                    equivalence,
                ),
            )
            .with_cost_context(context)
            .seal()
    }
}
