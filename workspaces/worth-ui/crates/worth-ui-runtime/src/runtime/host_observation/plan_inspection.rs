use crate::evidence::{
    UiAllocationPlanningInspectionReceipt, UiEvidenceExpansion, UiEvidenceRef, UiEvidenceSliceRef,
};
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    UiAllocationCandidate, WorthUiExecutionPlan, WorthUiExecutionPlanDigest,
    WorthUiExecutionPlanEquivalence, WorthUiRuntimeInspectionAiHarness,
};
use worth_ui_inspection::{UiEvidenceRichness, UiInspectionQuery};

use super::inspection_assembly::{
    assemble_allocation_planning_inspection_receipt, discard_retained_evidence_slice,
    expand_retained_evidence_ref, register_allocation_planning_inspection,
};

impl WorthUiRuntime {
    pub fn digest_execution_plan(&self, plan: &WorthUiExecutionPlan) -> WorthUiExecutionPlanDigest {
        crate::runtime::planning::plan_equivalence::WorthUiExecutionPlanDigestor::digest(plan).0
    }

    pub fn compare_execution_plans(
        &self,
        previous: &WorthUiExecutionPlan,
        next: &WorthUiExecutionPlan,
    ) -> WorthUiExecutionPlanEquivalence {
        crate::runtime::planning::plan_equivalence::WorthUiExecutionPlanDigestor::compare(
            previous, next,
        )
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn inspect_execution_plan(
        &self,
        plan: &WorthUiExecutionPlan,
        authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    ) -> Result<
        crate::runtime::WorthUiExecutionPlanInspection,
        crate::runtime::WorthUiPlanInspectionDenial,
    > {
        crate::runtime::planning::plan_inspection::WorthUiExecutionPlanInspector::inspect(
            plan, authority,
        )
    }

    pub fn inspect_allocation_planning(
        &self,
        allocation_candidate: &UiAllocationCandidate,
    ) -> UiAllocationPlanningInspectionReceipt {
        register_allocation_planning_inspection(self, allocation_candidate.planning())
    }

    pub(crate) fn inspect_allocation_planning_query(
        &self,
        allocation_candidate: &UiAllocationCandidate,
        query: UiInspectionQuery,
    ) -> crate::facade::inspection_bridge::UiInspectionReceipt {
        assemble_allocation_planning_inspection_receipt(
            self,
            allocation_candidate.planning(),
            query,
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
        expand_retained_evidence_ref(self, evidence_ref, requested_richness)
    }

    pub fn discard_evidence_slice(&self, slice_ref: UiEvidenceSliceRef) -> bool {
        discard_retained_evidence_slice(self, slice_ref)
    }
}
