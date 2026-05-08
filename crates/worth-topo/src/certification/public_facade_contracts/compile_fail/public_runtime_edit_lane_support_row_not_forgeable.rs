use topology::facade::{
    TopologyQueryEditLane, TopologyQueryEditLaneExecutionShape,
    TopologyQueryEditLaneSupportStatus, TopologyRuntimeEditLaneSupportRow,
};

fn main() {
    let _ = TopologyRuntimeEditLaneSupportRow {
        lane: TopologyQueryEditLane::CreateInnerLoopOnExistingFace,
        status: TopologyQueryEditLaneSupportStatus::Admitted,
        execution_shape: TopologyQueryEditLaneExecutionShape::GraphComposition,
        reason: String::new(),
        row_digest: String::new(),
    };
}
