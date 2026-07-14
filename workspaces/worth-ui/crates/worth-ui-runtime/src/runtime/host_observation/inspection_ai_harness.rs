use crate::evidence::{
    UiAllocationPlanningInspectionReceipt, UiEvidenceExpansion, UiEvidenceRef, UiEvidenceSliceRef,
};
use crate::facade::inspection_bridge::UiInspectionReceipt;
use crate::runtime::{UiAllocationCandidate, WorthUiRuntime};
use worth_ui_inspection::{UiEvidenceRichness, UiInspectionQuery};

pub struct WorthUiRuntimeInspectionAiHarness<'a> {
    runtime: &'a WorthUiRuntime,
}

impl<'a> WorthUiRuntimeInspectionAiHarness<'a> {
    pub const fn new(runtime: &'a WorthUiRuntime) -> Self {
        Self { runtime }
    }

    pub fn inspect_allocation_planning(
        &self,
        allocation_candidate: &UiAllocationCandidate,
    ) -> UiAllocationPlanningInspectionReceipt {
        self.runtime
            .inspect_allocation_planning(allocation_candidate)
    }

    pub fn inspect_allocation_planning_query(
        &self,
        allocation_candidate: &UiAllocationCandidate,
        query: UiInspectionQuery,
    ) -> UiInspectionReceipt {
        self.runtime
            .inspect_allocation_planning_query(allocation_candidate, query)
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
