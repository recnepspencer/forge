use worth_query::facade::runtime::{WorthQueryGraphReadCostEstimateStatus, WorthQueryGraphReadCostEstimateStatusKind};

fn main() {
    let _ = WorthQueryGraphReadCostEstimateStatus {
        kind: WorthQueryGraphReadCostEstimateStatusKind::Measured,
    };
}
