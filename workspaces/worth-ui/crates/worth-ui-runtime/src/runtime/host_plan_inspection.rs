use crate::evidence::{
    preflight_evidence_expansion, project_allocation_planning_inspection_receipt,
    UiAllocationPlanningInspectionReceipt, UiEvidenceExpansion, UiEvidenceRef, UiEvidenceSliceAssembly,
    UiEvidenceSliceAssemblyInput, UiEvidenceSliceRef, UiInspectionCostMetrics,
};
use crate::facade::UiInspectionReceipt;
use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiExecutionPlan, WorthUiExecutionPlanDigest,
    WorthUiExecutionPlanEquivalence, WorthUiExecutionPlanInspection, WorthUiPlanInspectionDenial,
    WorthUiRuntimeInspectionAiHarness,
};
use worth_ui_inspection::{
    RUNTIME_INSPECTION_SCOPE_INVENTORY, UiEvidenceRichness, UiInspectionQuery,
    UiInspectionRelevanceOutcome, UiInspectionScope,
};

impl WorthUiRuntimeHost {
    pub fn digest_execution_plan(&self, plan: &WorthUiExecutionPlan) -> WorthUiExecutionPlanDigest {
        crate::runtime::plan_equivalence::WorthUiExecutionPlanDigestor::digest(plan).0
    }

    pub fn compare_execution_plans(
        &self,
        previous: &WorthUiExecutionPlan,
        next: &WorthUiExecutionPlan,
    ) -> WorthUiExecutionPlanEquivalence {
        crate::runtime::plan_equivalence::WorthUiExecutionPlanDigestor::compare(previous, next)
    }

    pub fn inspect_execution_plan(
        &self,
        plan: &WorthUiExecutionPlan,
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> Result<WorthUiExecutionPlanInspection, WorthUiPlanInspectionDenial> {
        crate::runtime::plan_inspection::WorthUiExecutionPlanInspector::inspect(
            plan,
            allocation_planning,
        )
    }

    pub fn inspect_allocation_planning(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> UiAllocationPlanningInspectionReceipt {
        let receipt = project_allocation_planning_inspection_receipt(allocation_planning);
        self.retained_allocation_planning_evidence
            .register(&receipt);
        receipt
    }

    pub(crate) fn inspect_allocation_planning_query(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
        query: UiInspectionQuery,
    ) -> UiInspectionReceipt {
        let projected = self.inspect_allocation_planning(allocation_planning);
        let authority_generation = projected.evidence_slice().authority_generation();
        let support_report = RUNTIME_INSPECTION_SCOPE_INVENTORY.support_report(query.scope());
        let relevance_admission = query
            .admit_relevance()
            .refined_for_support_report(support_report);

        if query.scope() != UiInspectionScope::Planning
            || !matches!(
                relevance_admission.outcome(),
                UiInspectionRelevanceOutcome::Matched
            )
        {
            return UiInspectionReceipt::from_support(
                query,
                relevance_admission,
                support_report,
                Some(authority_generation),
            );
        }

        let assembly = UiEvidenceSliceAssembly::assemble(
            &query,
            UiEvidenceSliceAssemblyInput::new(
                authority_generation,
                projected.evidence_slice().refs().to_vec().into_boxed_slice(),
            )
            .with_materialized_detail(projected.evidence_slice().materialized_detail().cloned())
            .with_detail_available(true)
            .with_cost_metrics(UiInspectionCostMetrics::new(
                1,
                projected.evidence_slice().refs().len(),
                0,
                false,
            )),
        );

        UiInspectionReceipt::from_support_and_assembled_slice(
            query,
            relevance_admission,
            support_report,
            authority_generation,
            assembly,
        )
    }

    pub fn inspection_ai_harness(&self) -> WorthUiRuntimeInspectionAiHarness<'_> {
        WorthUiRuntimeInspectionAiHarness::new(self)
    }

    pub fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        requested_richness: UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        let current_generation = self
            .retained_allocation_planning_evidence
            .current_generation_for(evidence_ref.handle().handle_digest())
            .unwrap_or_else(|| evidence_ref.authority_generation());
        let evidence_ref = self
            .retained_allocation_planning_evidence
            .discarded_ref(evidence_ref)
            .unwrap_or(evidence_ref);
        if let Some(preflight) =
            preflight_evidence_expansion(current_generation, evidence_ref, requested_richness)
        {
            return preflight;
        }

        match self
            .retained_allocation_planning_evidence
            .retained_receipt(evidence_ref.handle().handle_digest())
        {
            Some(receipt) => receipt.expand_evidence_ref(evidence_ref, requested_richness),
            None => UiEvidenceExpansion::new(
                evidence_ref,
                requested_richness,
                worth_ui_inspection::UiEvidenceExpansionOutcome::Unsupported,
                None,
                Box::new([]),
                None,
            ),
        }
    }

    pub fn discard_evidence_slice(&self, slice_ref: UiEvidenceSliceRef) -> bool {
        self.retained_allocation_planning_evidence
            .discard_slice(slice_ref)
    }
}
