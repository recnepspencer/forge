use crate::runtime::{
    WorthUiRealtimeCertification, WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters,
    WorthUiRealtimeOverlayLane, WorthUiRendererSurfaceAdmission, WorthUiRuntimeHandle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRealtimeFrameReceipt {
    target: WorthUiRealtimeFrameTarget,
    lane: WorthUiRealtimeOverlayLane,
    renderer_surface_admission: WorthUiRendererSurfaceAdmission,
    touched_plan_indexes: Vec<u32>,
    touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
    command_plan_indexes: Vec<u32>,
    accessibility_plan_indexes: Vec<u32>,
    diagnostics_plan_indexes: Vec<u32>,
    counters: WorthUiRealtimeLaneCounters,
    certification: WorthUiRealtimeCertification,
}

pub(crate) struct WorthUiRealtimeFrameReceiptInput {
    pub target: WorthUiRealtimeFrameTarget,
    pub lane: WorthUiRealtimeOverlayLane,
    pub renderer_surface_admission: WorthUiRendererSurfaceAdmission,
    pub touched_plan_indexes: Vec<u32>,
    pub touched_runtime_handles: Vec<WorthUiRuntimeHandle>,
    pub command_plan_indexes: Vec<u32>,
    pub accessibility_plan_indexes: Vec<u32>,
    pub diagnostics_plan_indexes: Vec<u32>,
    pub counters: WorthUiRealtimeLaneCounters,
    pub certification: WorthUiRealtimeCertification,
}

impl WorthUiRealtimeFrameReceipt {
    pub(crate) fn new(input: WorthUiRealtimeFrameReceiptInput) -> Self {
        let WorthUiRealtimeFrameReceiptInput {
            target,
            lane,
            renderer_surface_admission,
            touched_plan_indexes,
            touched_runtime_handles,
            command_plan_indexes,
            accessibility_plan_indexes,
            diagnostics_plan_indexes,
            counters,
            certification,
        } = input;
        Self {
            target,
            lane,
            renderer_surface_admission,
            touched_plan_indexes,
            touched_runtime_handles,
            command_plan_indexes,
            accessibility_plan_indexes,
            diagnostics_plan_indexes,
            counters,
            certification,
        }
    }

    pub fn target(&self) -> WorthUiRealtimeFrameTarget {
        self.target
    }

    pub fn lane(&self) -> WorthUiRealtimeOverlayLane {
        self.lane
    }

    pub fn renderer_surface_admission(&self) -> WorthUiRendererSurfaceAdmission {
        self.renderer_surface_admission
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

    pub fn accessibility_plan_indexes(&self) -> &[u32] {
        &self.accessibility_plan_indexes
    }

    pub fn diagnostics_plan_indexes(&self) -> &[u32] {
        &self.diagnostics_plan_indexes
    }

    pub fn counters(&self) -> WorthUiRealtimeLaneCounters {
        self.counters
    }

    pub fn certification(&self) -> WorthUiRealtimeCertification {
        self.certification
    }
}
