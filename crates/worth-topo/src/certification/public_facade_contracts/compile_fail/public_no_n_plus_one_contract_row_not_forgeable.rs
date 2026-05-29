use topology::facade::{
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow,
    TopologyNoNPlusOneContractStatus,
};

fn main() {
    let _ = TopologyNoNPlusOneContractRow {
        contract: TopologyNoNPlusOneContract::LoweringBreadth,
        status: TopologyNoNPlusOneContractStatus::Satisfied,
        reason: String::new(),
        row_digest: String::new(),
    };
}
