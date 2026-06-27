use crate::runtime::{
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneCertification, WorthUiOrdinaryLaneCounters,
    WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiOrdinaryLaneFrameReceipt {
    target: WorthUiOrdinaryFrameTarget,
    touched_plan_indexes: Vec<u32>,
    touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
    counters: WorthUiOrdinaryLaneCounters,
    certification: WorthUiOrdinaryLaneCertification,
}

impl WorthUiOrdinaryLaneFrameReceipt {
    pub(crate) fn new(
        target: WorthUiOrdinaryFrameTarget,
        touched_plan_indexes: Vec<u32>,
        touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
        counters: WorthUiOrdinaryLaneCounters,
        certification: WorthUiOrdinaryLaneCertification,
    ) -> Self {
        Self {
            target,
            touched_plan_indexes,
            touched_runtime_handles,
            counters,
            certification,
        }
    }

    pub fn target(&self) -> WorthUiOrdinaryFrameTarget {
        self.target
    }

    pub fn touched_plan_indexes(&self) -> &[u32] {
        &self.touched_plan_indexes
    }

    pub fn touched_runtime_handles(&self) -> &[WorthUiRuntimeHandle] {
        &self.touched_runtime_handles
    }

    pub fn counters(&self) -> WorthUiOrdinaryLaneCounters {
        self.counters
    }

    pub fn certification(&self) -> WorthUiOrdinaryLaneCertification {
        self.certification
    }
}
