use worth_query::facade::{
    execute_query_basis_context, HistoricalCapabilityDescriptor, HistoricalPathReuseDescriptor,
};

fn main() {
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let _ = execute_query_basis_context(&capability);
}
