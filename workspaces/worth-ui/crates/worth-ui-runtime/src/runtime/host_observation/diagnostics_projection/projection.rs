use crate::runtime::{
    WorthUiBindingObservationSurface, WorthUiFrameCostSurface, WorthUiPlanInspectionSurface,
    WorthUiReloadStatusSurface, WorthUiRuntimeDiagnostic,
};

use super::counters::WorthUiDiagnosticsProjectionCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDiagnosticsProjection {
    active_artifact_digest: u64,
    active_plan_digest: u64,
    projection_digest: u64,
    rows: Vec<WorthUiRuntimeDiagnostic>,
    reload_status: WorthUiReloadStatusSurface,
    plan_inspection: WorthUiPlanInspectionSurface,
    frame_costs: WorthUiFrameCostSurface,
    binding_observations: WorthUiBindingObservationSurface,
    counters: WorthUiDiagnosticsProjectionCounters,
}

pub(crate) struct WorthUiDiagnosticsProjectionInput {
    pub active_artifact_digest: u64,
    pub active_plan_digest: u64,
    pub projection_digest: u64,
    pub rows: Vec<WorthUiRuntimeDiagnostic>,
    pub reload_status: WorthUiReloadStatusSurface,
    pub plan_inspection: WorthUiPlanInspectionSurface,
    pub frame_costs: WorthUiFrameCostSurface,
    pub binding_observations: WorthUiBindingObservationSurface,
    pub counters: WorthUiDiagnosticsProjectionCounters,
}

impl WorthUiDiagnosticsProjection {
    pub(crate) fn new(input: WorthUiDiagnosticsProjectionInput) -> Self {
        let WorthUiDiagnosticsProjectionInput {
            active_artifact_digest,
            active_plan_digest,
            projection_digest,
            rows,
            reload_status,
            plan_inspection,
            frame_costs,
            binding_observations,
            counters,
        } = input;
        Self {
            active_artifact_digest,
            active_plan_digest,
            projection_digest,
            rows,
            reload_status,
            plan_inspection,
            frame_costs,
            binding_observations,
            counters,
        }
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn active_plan_digest(&self) -> u64 {
        self.active_plan_digest
    }

    pub fn projection_digest(&self) -> u64 {
        self.projection_digest
    }

    pub fn rows(&self) -> &[WorthUiRuntimeDiagnostic] {
        &self.rows
    }

    pub fn reload_status(&self) -> &WorthUiReloadStatusSurface {
        &self.reload_status
    }

    pub fn plan_inspection(&self) -> &WorthUiPlanInspectionSurface {
        &self.plan_inspection
    }

    pub fn frame_costs(&self) -> &WorthUiFrameCostSurface {
        &self.frame_costs
    }

    pub fn binding_observations(&self) -> &WorthUiBindingObservationSurface {
        &self.binding_observations
    }

    pub fn counters(&self) -> WorthUiDiagnosticsProjectionCounters {
        self.counters
    }
}
