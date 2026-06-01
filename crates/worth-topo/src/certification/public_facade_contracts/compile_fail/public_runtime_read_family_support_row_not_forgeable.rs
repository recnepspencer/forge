use topology::facade::{
    TopologyQueryReadFamilySupportStatus, TopologyRuntimeReadFamilySupportRow,
};
use topology::query_domain::TopologyReadRequestFamily;

fn main() {
    let _ = TopologyRuntimeReadFamilySupportRow {
        family: TopologyReadRequestFamily::LoopCycleNeighborhood,
        status: TopologyQueryReadFamilySupportStatus::Admitted,
        reason: String::new(),
        row_digest: String::new(),
    };
}
