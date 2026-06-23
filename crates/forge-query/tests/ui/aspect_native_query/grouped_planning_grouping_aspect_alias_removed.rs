use forge_query::facade::{
    GroupedBaselineMaterializationContract, GroupedViewPlanningArtifact,
};

fn main() {
    let baseline = baseline_fixture();
    let _ = baseline.grouping_aspect();

    let planning = planning_fixture();
    let _ = planning.grouping_aspect();
}

fn baseline_fixture() -> GroupedBaselineMaterializationContract {
    panic!("fixture only")
}

fn planning_fixture() -> GroupedViewPlanningArtifact {
    panic!("fixture only")
}
