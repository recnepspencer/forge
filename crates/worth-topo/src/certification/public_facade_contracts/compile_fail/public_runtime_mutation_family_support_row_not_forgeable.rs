use topology::facade::{
    TopologyMutationFamily, TopologyQueryMutationFamilySupportStatus,
    TopologyQueryMutationLane, TopologyRuntimeMutationFamilySupportRow,
};

fn main() {
    let _ = TopologyRuntimeMutationFamilySupportRow {
        family: TopologyMutationFamily::CreateTopologyEntity,
        status: TopologyQueryMutationFamilySupportStatus::Admitted,
        admitted_lanes: vec![TopologyQueryMutationLane::CreateTopologyEntity],
        reason: String::new(),
        row_digest: String::new(),
    };
}
