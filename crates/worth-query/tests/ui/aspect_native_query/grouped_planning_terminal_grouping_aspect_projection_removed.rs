use worth_query::facade::{
    GroupedBaselineMaterializationContract, GroupedViewPlanningArtifact,
};

fn assert_no_terminal_grouping_projection(
    baseline: &GroupedBaselineMaterializationContract,
    planning: &GroupedViewPlanningArtifact,
) {
    let _ = baseline.terminal_grouping_aspect_projection();
    let _ = planning.terminal_grouping_aspect_projection();
}

fn main() {}
