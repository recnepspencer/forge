use std::rc::Rc;

use crate::runtime::{
    WorthUiFrameWorkScope, WorthUiHandleResolutionEvidence, WorthUiQueryBindingIdentity,
    WorthUiRuntimeHandle, WorthUiVirtualizedDataCertification, WorthUiVirtualizedDataCounters,
    WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataLane, WorthUiVisibleRange,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataFrameReceipt {
    target: WorthUiVirtualizedDataFrameTarget,
    lane: WorthUiVirtualizedDataLane,
    visible_range: WorthUiVisibleRange,
    touched_plan_index: u32,
    touched_runtime_handle: WorthUiRuntimeHandle,
    binding_identity: Rc<WorthUiQueryBindingIdentity>,
    evidence: worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceReference,
    counters: WorthUiVirtualizedDataCounters,
    certification: WorthUiVirtualizedDataCertification,
    resolution_evidence: WorthUiHandleResolutionEvidence,
    work_scope: WorthUiFrameWorkScope,
}

pub(crate) struct WorthUiVirtualizedDataFrameReceiptInput {
    pub target: WorthUiVirtualizedDataFrameTarget,
    pub lane: WorthUiVirtualizedDataLane,
    pub visible_range: WorthUiVisibleRange,
    pub touched_plan_index: u32,
    pub touched_runtime_handle: WorthUiRuntimeHandle,
    pub binding_identity: Rc<WorthUiQueryBindingIdentity>,
    pub evidence: worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceReference,
    pub counters: WorthUiVirtualizedDataCounters,
    pub certification: WorthUiVirtualizedDataCertification,
    pub resolution_evidence: WorthUiHandleResolutionEvidence,
    pub work_scope: WorthUiFrameWorkScope,
}

impl WorthUiVirtualizedDataFrameReceipt {
    pub(crate) fn new(input: WorthUiVirtualizedDataFrameReceiptInput) -> Self {
        Self {
            target: input.target,
            lane: input.lane,
            visible_range: input.visible_range,
            touched_plan_index: input.touched_plan_index,
            touched_runtime_handle: input.touched_runtime_handle,
            binding_identity: input.binding_identity,
            evidence: input.evidence,
            counters: input.counters,
            certification: input.certification,
            resolution_evidence: input.resolution_evidence,
            work_scope: input.work_scope,
        }
    }

    pub fn target(&self) -> WorthUiVirtualizedDataFrameTarget {
        self.target
    }

    pub fn lane(&self) -> WorthUiVirtualizedDataLane {
        self.lane
    }

    pub fn visible_range(&self) -> WorthUiVisibleRange {
        self.visible_range
    }

    pub fn touched_plan_index(&self) -> u32 {
        self.touched_plan_index
    }

    pub fn touched_runtime_handle(&self) -> WorthUiRuntimeHandle {
        self.touched_runtime_handle
    }

    pub fn binding_identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.binding_identity
    }

    pub fn evidence(&self) -> &worth_ui_query_binding::WorthUiQueryViewExecutionEvidenceReference {
        &self.evidence
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        self.counters
    }

    pub fn certification(&self) -> WorthUiVirtualizedDataCertification {
        self.certification
    }

    pub fn resolution_evidence(&self) -> WorthUiHandleResolutionEvidence {
        self.resolution_evidence
    }

    pub fn work_scope(&self) -> WorthUiFrameWorkScope {
        self.work_scope
    }
}
