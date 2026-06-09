use topology::runtime_support::{
    TopologyRuntimeCloseoutFamily, TopologyRuntimeCloseoutRow,
    TopologyRuntimeCloseoutStatus,
};

fn main() {
    let _ = TopologyRuntimeCloseoutRow {
        family: TopologyRuntimeCloseoutFamily::QueryNativeTopologyReadFamilies,
        status: TopologyRuntimeCloseoutStatus::Satisfied,
        reason: String::new(),
        row_digest: String::new(),
    };
}
