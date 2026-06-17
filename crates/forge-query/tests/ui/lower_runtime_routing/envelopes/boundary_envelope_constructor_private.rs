use forge_query::facade::{
    forge_query_lower_runtime_crossing_inventory, ForgeQueryLowerRuntimeBoundaryEnvelope,
};

fn main() {
    let row = forge_query_lower_runtime_crossing_inventory().rows()[0];
    let _ = ForgeQueryLowerRuntimeBoundaryEnvelope::new(
        row,
        "request",
        "eligibility",
        "route",
        "boundary",
        todo!(),
    );
}
