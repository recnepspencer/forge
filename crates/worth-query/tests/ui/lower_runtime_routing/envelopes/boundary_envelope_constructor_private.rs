use worth_query::facade::runtime::{worth_query_lower_runtime_crossing_inventory, WorthQueryLowerRuntimeBoundaryEnvelope};

fn main() {
    let row = worth_query_lower_runtime_crossing_inventory().rows()[0];
    let _ = WorthQueryLowerRuntimeBoundaryEnvelope::new(
        row,
        "request",
        "eligibility",
        "route",
        "boundary",
        todo!(),
    );
}
