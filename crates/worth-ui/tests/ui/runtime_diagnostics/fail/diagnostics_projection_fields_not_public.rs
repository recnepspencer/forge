use worth_ui::facade::{
    WorthUiDiagnosticsProjection, WorthUiDiagnosticsProjectionCounters,
    WorthUiFrameCostSurface, WorthUiPlanInspectionSurface, WorthUiQueryStatusSurface,
    WorthUiReloadStatusSurface,
};

fn main() {
    let _projection = WorthUiDiagnosticsProjection {
        active_artifact_digest: 1,
        active_plan_digest: 2,
        projection_digest: 3,
        rows: Vec::new(),
        reload_status: WorthUiReloadStatusSurface {
            active_artifact_digest: 1,
            active_plan_digest: 2,
            failures: Vec::new(),
        },
        plan_inspection: WorthUiPlanInspectionSurface {
            plan_digest: 2,
            nodes: Vec::new(),
            lanes: Vec::new(),
        },
        frame_costs: WorthUiFrameCostSurface {
            source_digest: 4,
            rows: Vec::new(),
        },
        query_status: WorthUiQueryStatusSurface { rows: Vec::new() },
        counters: WorthUiDiagnosticsProjectionCounters::default(),
    };
}
