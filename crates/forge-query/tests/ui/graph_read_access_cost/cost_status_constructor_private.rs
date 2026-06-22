use forge_query::facade::runtime::{
    ForgeQueryGraphReadCostEstimateStatus, ForgeQueryGraphReadCostEstimateStatusKind,
};

fn main() {
    let _ = ForgeQueryGraphReadCostEstimateStatus {
        kind: ForgeQueryGraphReadCostEstimateStatusKind::Measured,
    };
}
