use worth_query::facade::foundation::BridgeChangeSummary;
use worth_query::facade::runtime::{execute_grouped_live_view_shape_change, GroupedExecutionSurfaceArtifact, LiveViewShapeArtifact};

fn main() {
    let _fn_ptr: fn(
        &LiveViewShapeArtifact,
        &BridgeChangeSummary,
        &GroupedExecutionSurfaceArtifact,
    ) -> _ = execute_grouped_live_view_shape_change;
}
