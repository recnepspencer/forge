use crate::runtime::{WorthUiOrdinaryExecutionLane, WorthUiPlanChildRange, WorthUiRuntimeHandle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneNode {
    plan_index: u32,
    runtime_handle: WorthUiRuntimeHandle,
    lane: WorthUiOrdinaryExecutionLane,
    child_range: Option<WorthUiPlanChildRange>,
    egui_contact_count: usize,
}

impl WorthUiOrdinaryLaneNode {
    pub(crate) fn new(
        runtime_handle: WorthUiRuntimeHandle,
        lane: WorthUiOrdinaryExecutionLane,
        child_range: Option<WorthUiPlanChildRange>,
        egui_contact_count: usize,
    ) -> Self {
        Self {
            plan_index: runtime_handle.plan_index(),
            runtime_handle,
            lane,
            child_range,
            egui_contact_count,
        }
    }

    pub fn plan_index(&self) -> u32 {
        self.plan_index
    }

    pub fn runtime_handle(&self) -> WorthUiRuntimeHandle {
        self.runtime_handle
    }

    pub fn lane(&self) -> WorthUiOrdinaryExecutionLane {
        self.lane
    }

    pub fn child_range(&self) -> Option<WorthUiPlanChildRange> {
        self.child_range
    }

    pub fn egui_contact_count(&self) -> usize {
        self.egui_contact_count
    }
}
