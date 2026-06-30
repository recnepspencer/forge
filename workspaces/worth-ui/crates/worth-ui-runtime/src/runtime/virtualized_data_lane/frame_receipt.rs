use crate::runtime::{
    WorthUiQueryPatchPosture, WorthUiRuntimeHandle, WorthUiVirtualizedDataCertification,
    WorthUiVirtualizedDataCounters, WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataLane,
    WorthUiVisibleRange,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiVirtualizedDataFrameReceipt {
    target: WorthUiVirtualizedDataFrameTarget,
    lane: WorthUiVirtualizedDataLane,
    visible_range: WorthUiVisibleRange,
    touched_plan_indexes: Vec<u32>,
    touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
    query_patch_posture: WorthUiQueryPatchPosture,
    counters: WorthUiVirtualizedDataCounters,
    certification: WorthUiVirtualizedDataCertification,
}

impl WorthUiVirtualizedDataFrameReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        target: WorthUiVirtualizedDataFrameTarget,
        lane: WorthUiVirtualizedDataLane,
        visible_range: WorthUiVisibleRange,
        touched_plan_indexes: Vec<u32>,
        touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
        query_patch_posture: WorthUiQueryPatchPosture,
        counters: WorthUiVirtualizedDataCounters,
        certification: WorthUiVirtualizedDataCertification,
    ) -> Self {
        Self {
            target,
            lane,
            visible_range,
            touched_plan_indexes,
            touched_runtime_handles,
            query_patch_posture,
            counters,
            certification,
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

    pub fn touched_plan_indexes(&self) -> &[u32] {
        &self.touched_plan_indexes
    }

    pub fn touched_runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.touched_runtime_handles
    }

    pub fn query_patch_posture(&self) -> &WorthUiQueryPatchPosture {
        &self.query_patch_posture
    }

    pub fn counters(&self) -> WorthUiVirtualizedDataCounters {
        self.counters
    }

    pub fn certification(&self) -> WorthUiVirtualizedDataCertification {
        self.certification
    }
}
