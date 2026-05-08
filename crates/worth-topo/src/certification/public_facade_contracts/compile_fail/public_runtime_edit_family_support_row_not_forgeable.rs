use topology::facade::{
    TopologyEditFamily, TopologyQueryEditFamilySupportStatus,
    TopologyQueryEditLane, TopologyRuntimeEditFamilySupportRow,
};

fn main() {
    let _ = TopologyRuntimeEditFamilySupportRow {
        family: TopologyEditFamily::CreateTopologyEntity,
        status: TopologyQueryEditFamilySupportStatus::Admitted,
        admitted_lanes: vec![TopologyQueryEditLane::CreateTopologyEntity],
        reason: String::new(),
        row_digest: String::new(),
    };
}
