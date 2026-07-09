use worth_query::facade::{
    GroupedDesiredStateArtifact, GroupedExecutionLaneValue, GroupedLaneIdentity,
    GroupedViewResultArtifact,
};

fn assert_no_terminal_grouped_live_projection(
    lane: &GroupedLaneIdentity,
    result: &GroupedViewResultArtifact,
    desired: &GroupedDesiredStateArtifact,
    execution_lane: &GroupedExecutionLaneValue,
) {
    let _ = lane.terminal_grouping_aspect_projection();
    let _ = result.terminal_grouping_aspect_projection();
    let _ = desired.terminal_grouping_aspect_projection();
    let _ = execution_lane.terminal_grouping_aspect_projection();
}

fn main() {}
