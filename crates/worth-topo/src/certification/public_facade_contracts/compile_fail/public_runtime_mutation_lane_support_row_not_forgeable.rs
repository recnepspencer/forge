use topology::runtime_support::{
    TopologyQueryMutationLane, TopologyQueryMutationLaneExecutionShape,
    TopologyQueryMutationLaneSupportStatus, TopologyRuntimeMutationLaneSupportRow,
};

fn main() {
    let _ = TopologyRuntimeMutationLaneSupportRow {
        lane: TopologyQueryMutationLane::CreateInnerLoopOnExistingFace,
        status: TopologyQueryMutationLaneSupportStatus::Admitted,
        execution_shape: TopologyQueryMutationLaneExecutionShape::GraphComposition,
        reason: String::new(),
        row_digest: String::new(),
    };
}
