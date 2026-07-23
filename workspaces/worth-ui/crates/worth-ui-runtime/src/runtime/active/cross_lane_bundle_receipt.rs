use crate::runtime::{
    WorthUiCanvasSpatialPlanAvailability, WorthUiExecutionPlanDigest,
    WorthUiOrdinaryPlanAvailability, WorthUiPlanConstructionCounters,
    WorthUiRealtimePlanAvailability, WorthUiVirtualizedPlanAvailability,
};

/// Compact, read-only proof that one active bundle owns every lane posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiCrossLaneBundleReceipt {
    plan_digest: WorthUiExecutionPlanDigest,
    handle_allocation_basis_digest: u64,
    lane_support_digest: u64,
    lane_plan_input_basis_digest: u64,
    construction_counters: WorthUiPlanConstructionCounters,
    ordinary: WorthUiOrdinaryPlanAvailability,
    virtualized: WorthUiVirtualizedPlanAvailability,
    canvas_spatial: WorthUiCanvasSpatialPlanAvailability,
    realtime_overlay: WorthUiRealtimePlanAvailability,
}

pub(crate) struct WorthUiCrossLaneBundleReceiptInput {
    pub plan_digest: WorthUiExecutionPlanDigest,
    pub handle_allocation_basis_digest: u64,
    pub lane_support_digest: u64,
    pub lane_plan_input_basis_digest: u64,
    pub construction_counters: WorthUiPlanConstructionCounters,
    pub ordinary: WorthUiOrdinaryPlanAvailability,
    pub virtualized: WorthUiVirtualizedPlanAvailability,
    pub canvas_spatial: WorthUiCanvasSpatialPlanAvailability,
    pub realtime_overlay: WorthUiRealtimePlanAvailability,
}

impl WorthUiCrossLaneBundleReceipt {
    pub(crate) fn new(input: WorthUiCrossLaneBundleReceiptInput) -> Self {
        Self {
            plan_digest: input.plan_digest,
            handle_allocation_basis_digest: input.handle_allocation_basis_digest,
            lane_support_digest: input.lane_support_digest,
            lane_plan_input_basis_digest: input.lane_plan_input_basis_digest,
            construction_counters: input.construction_counters,
            ordinary: input.ordinary,
            virtualized: input.virtualized,
            canvas_spatial: input.canvas_spatial,
            realtime_overlay: input.realtime_overlay,
        }
    }

    pub fn plan_digest(self) -> WorthUiExecutionPlanDigest {
        self.plan_digest
    }

    pub fn handle_allocation_basis_digest(self) -> u64 {
        self.handle_allocation_basis_digest
    }

    pub fn lane_support_digest(self) -> u64 {
        self.lane_support_digest
    }

    pub fn lane_plan_input_basis_digest(self) -> u64 {
        self.lane_plan_input_basis_digest
    }

    pub fn construction_counters(self) -> WorthUiPlanConstructionCounters {
        self.construction_counters
    }

    pub fn ordinary(self) -> WorthUiOrdinaryPlanAvailability {
        self.ordinary
    }

    pub fn virtualized(self) -> WorthUiVirtualizedPlanAvailability {
        self.virtualized
    }

    pub fn canvas_spatial(self) -> WorthUiCanvasSpatialPlanAvailability {
        self.canvas_spatial
    }

    pub fn realtime_overlay(self) -> WorthUiRealtimePlanAvailability {
        self.realtime_overlay
    }
}
