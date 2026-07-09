use worth_query::facade::QuerySubscriptionBridgeParityFailure;

fn bridge_parity_projection_golden_path(failure: &QuerySubscriptionBridgeParityFailure) {
    let _ = failure.source_projection().label();
}

fn main() {}
