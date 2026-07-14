use worth_query::facade::runtime::{GroupedDesiredStateArtifact, GroupedExecutionLaneValue, GroupedLaneIdentity, GroupedViewResultArtifact};

fn main() {
    let lane = lane_fixture();
    let _ = lane.grouping_aspect();

    let result = result_fixture();
    let _ = result.grouping_aspect();

    let desired = desired_fixture();
    let _ = desired.grouping_aspect();

    let execution_lane = execution_lane_fixture();
    let _ = execution_lane.grouping_aspect();
}

fn lane_fixture() -> GroupedLaneIdentity {
    panic!("fixture only")
}

fn result_fixture() -> GroupedViewResultArtifact {
    panic!("fixture only")
}

fn desired_fixture() -> GroupedDesiredStateArtifact {
    panic!("fixture only")
}

fn execution_lane_fixture() -> GroupedExecutionLaneValue {
    panic!("fixture only")
}
