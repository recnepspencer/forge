use crate::runtime::{
    WorthUiCanvasSpatialCertification, WorthUiCanvasSpatialCounters,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiRuntimeHandle,
    WorthUiStateSlotHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCanvasSpatialFrameReceipt {
    target: WorthUiCanvasSpatialFrameTarget,
    lane: WorthUiCanvasSpatialLane,
    touched_plan_indexes: Vec<u32>,
    touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
    command_plan_indexes: Vec<u32>,
    diagnostics_plan_indexes: Vec<u32>,
    selection_state_slot_handles: Vec<WorthUiStateSlotHandle>,
    counters: WorthUiCanvasSpatialCounters,
    certification: WorthUiCanvasSpatialCertification,
}

impl WorthUiCanvasSpatialFrameReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        target: WorthUiCanvasSpatialFrameTarget,
        lane: WorthUiCanvasSpatialLane,
        touched_plan_indexes: Vec<u32>,
        touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
        command_plan_indexes: Vec<u32>,
        diagnostics_plan_indexes: Vec<u32>,
        selection_state_slot_handles: Vec<WorthUiStateSlotHandle>,
        counters: WorthUiCanvasSpatialCounters,
        certification: WorthUiCanvasSpatialCertification,
    ) -> Self {
        Self {
            target,
            lane,
            touched_plan_indexes,
            touched_runtime_handles,
            command_plan_indexes,
            diagnostics_plan_indexes,
            selection_state_slot_handles,
            counters,
            certification,
        }
    }

    pub fn target(&self) -> WorthUiCanvasSpatialFrameTarget {
        self.target
    }

    pub fn lane(&self) -> WorthUiCanvasSpatialLane {
        self.lane
    }

    pub fn touched_plan_indexes(&self) -> &[u32] {
        &self.touched_plan_indexes
    }

    pub fn touched_runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.touched_runtime_handles
    }

    pub fn command_plan_indexes(&self) -> &[u32] {
        &self.command_plan_indexes
    }

    pub fn diagnostics_plan_indexes(&self) -> &[u32] {
        &self.diagnostics_plan_indexes
    }

    pub fn selection_state_slot_handles(&self) -> &[WorthUiStateSlotHandle] {
        &self.selection_state_slot_handles
    }

    pub fn counters(&self) -> WorthUiCanvasSpatialCounters {
        self.counters
    }

    pub fn certification(&self) -> WorthUiCanvasSpatialCertification {
        self.certification
    }
}
