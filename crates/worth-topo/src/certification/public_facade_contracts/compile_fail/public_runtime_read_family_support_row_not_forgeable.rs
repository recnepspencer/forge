use topology::facade::{
    TopologyDomainQueryRequestFamily, TopologyQueryReadFamilySupportStatus,
    TopologyRuntimeReadFamilySupportRow,
};

fn main() {
    let _ = TopologyRuntimeReadFamilySupportRow {
        family: TopologyDomainQueryRequestFamily::LoopCycleNeighborhood,
        status: TopologyQueryReadFamilySupportStatus::Admitted,
        reason: String::new(),
        row_digest: String::new(),
    };
}
