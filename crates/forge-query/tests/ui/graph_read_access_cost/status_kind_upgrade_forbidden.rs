use forge_query::facade::runtime::{
    ForgeQueryGraphReadCostEstimateStatus, ForgeQueryGraphReadCostEstimateStatusKind,
};

fn main() {
    let _: ForgeQueryGraphReadCostEstimateStatus =
        ForgeQueryGraphReadCostEstimateStatusKind::Measured.into();
}
