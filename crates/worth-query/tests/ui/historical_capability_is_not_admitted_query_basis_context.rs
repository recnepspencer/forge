use worth_query::facade::foundation::{HistoricalCapabilityDescriptor, HistoricalPathReuseDescriptor};
use worth_query::facade::policy::execute_query_basis_context;

fn main() {
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        "history:snapshot-1",
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let _ = execute_query_basis_context(&capability);
}
