use crate::runtime::{
    WorthUiHighFrequencyFramePolicy, WorthUiHudNode, WorthUiRealtimeCertification,
    WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayHook, WorthUiRendererSurfaceAdmission,
    WorthUiRuntimeHandleAllocationReceipt,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHudPlan {
    handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    support_digest: u64,
    hud_plan_digest: u64,
    frame_policy: WorthUiHighFrequencyFramePolicy,
    rows: Vec<WorthUiHudNode>,
    renderer_surface_admissions: Vec<WorthUiRendererSurfaceAdmission>,
    command_plan_indexes: Vec<u32>,
    accessibility_plan_indexes: Vec<u32>,
    diagnostics_plan_indexes: Vec<u32>,
    overlay_hooks: Vec<WorthUiRealtimeOverlayHook>,
    counters: WorthUiRealtimeLaneCounters,
}

pub(crate) struct WorthUiHudPlanInput {
    pub handle_receipt: WorthUiRuntimeHandleAllocationReceipt,
    pub support_digest: u64,
    pub hud_plan_digest: u64,
    pub frame_policy: WorthUiHighFrequencyFramePolicy,
    pub rows: Vec<WorthUiHudNode>,
    pub renderer_surface_admissions: Vec<WorthUiRendererSurfaceAdmission>,
    pub command_plan_indexes: Vec<u32>,
    pub accessibility_plan_indexes: Vec<u32>,
    pub diagnostics_plan_indexes: Vec<u32>,
    pub overlay_hooks: Vec<WorthUiRealtimeOverlayHook>,
    pub counters: WorthUiRealtimeLaneCounters,
}

impl WorthUiHudPlan {
    pub(crate) fn new(input: WorthUiHudPlanInput) -> Self {
        let WorthUiHudPlanInput {
            handle_receipt,
            support_digest,
            hud_plan_digest,
            frame_policy,
            rows,
            renderer_surface_admissions,
            command_plan_indexes,
            accessibility_plan_indexes,
            diagnostics_plan_indexes,
            overlay_hooks,
            counters,
        } = input;
        Self {
            handle_receipt,
            support_digest,
            hud_plan_digest,
            frame_policy,
            rows,
            renderer_surface_admissions,
            command_plan_indexes,
            accessibility_plan_indexes,
            diagnostics_plan_indexes,
            overlay_hooks,
            counters,
        }
    }

    pub fn handle_receipt(&self) -> WorthUiRuntimeHandleAllocationReceipt {
        self.handle_receipt
    }

    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }

    pub fn hud_plan_digest(&self) -> u64 {
        self.hud_plan_digest
    }

    pub fn frame_policy(&self) -> WorthUiHighFrequencyFramePolicy {
        self.frame_policy
    }

    pub fn rows(&self) -> &[WorthUiHudNode] {
        &self.rows
    }

    pub fn renderer_surfaces(&self) -> &[WorthUiRendererSurfaceAdmission] {
        &self.renderer_surface_admissions
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

    pub fn overlay_hooks(&self) -> &[WorthUiRealtimeOverlayHook] {
        &self.overlay_hooks
    }

    pub fn counters(&self) -> WorthUiRealtimeLaneCounters {
        self.counters
    }

    pub(crate) fn row_for_plan_index(&self, plan_index: u32) -> Option<&WorthUiHudNode> {
        self.rows
            .binary_search_by_key(&plan_index, WorthUiHudNode::plan_index)
            .ok()
            .map(|index| &self.rows[index])
    }

    pub(crate) fn certification(&self) -> WorthUiRealtimeCertification {
        WorthUiRealtimeCertification::new(
            self.hud_plan_digest,
            self.support_digest,
            self.frame_policy.canonical_digest(),
            self.handle_receipt,
        )
    }
}
