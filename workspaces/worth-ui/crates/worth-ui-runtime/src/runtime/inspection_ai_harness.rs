use crate::evidence::{
    UiAllocationPlanningInspectionReceipt, UiEvidenceExpansion, UiEvidenceRef, UiEvidenceSliceRef,
};
use crate::facade::UiInspectionReceipt;
use crate::runtime::{WorthUiAllocationPlanning, WorthUiRuntimeHost};
use worth_ui_inspection::{UiEvidenceRichness, UiInspectionQuery};

pub struct WorthUiRuntimeInspectionAiHarness<'a> {
    runtime: &'a WorthUiRuntimeHost,
}

impl<'a> WorthUiRuntimeInspectionAiHarness<'a> {
    pub const fn new(runtime: &'a WorthUiRuntimeHost) -> Self {
        Self { runtime }
    }

    pub fn inspect_allocation_planning(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
    ) -> UiAllocationPlanningInspectionReceipt {
        self.runtime.inspect_allocation_planning(allocation_planning)
    }

    pub fn inspect_allocation_planning_query(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
        query: UiInspectionQuery,
    ) -> UiInspectionReceipt {
        self.runtime
            .inspect_allocation_planning_query(allocation_planning, query)
    }

    pub fn expand_evidence_ref(
        &self,
        evidence_ref: UiEvidenceRef,
        richness: UiEvidenceRichness,
    ) -> UiEvidenceExpansion {
        self.runtime.expand_evidence_ref(evidence_ref, richness)
    }

    pub fn discard_evidence_slice(&self, slice_ref: UiEvidenceSliceRef) -> bool {
        self.runtime.discard_evidence_slice(slice_ref)
    }
}
