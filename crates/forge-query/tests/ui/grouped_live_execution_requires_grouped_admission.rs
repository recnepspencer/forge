use forge_query::facade::{
    execute_grouped_live_view_shape_change, BridgeChangeSummary, GroupedExecutionSurfaceArtifact,
    LiveViewShapeArtifact,
};

fn main() {
    let _fn_ptr: fn(
        &LiveViewShapeArtifact,
        &BridgeChangeSummary,
        &GroupedExecutionSurfaceArtifact,
    ) -> _ = execute_grouped_live_view_shape_change;
}
