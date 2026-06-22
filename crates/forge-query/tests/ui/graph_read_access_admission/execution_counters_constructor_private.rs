use forge_query::facade::runtime::ForgeQueryGraphReadAccessExecutionCounters;

fn main() {
    let _ = ForgeQueryGraphReadAccessExecutionCounters {
        executor_entry_count: 0,
        strategy_recompute_count: 0,
        edge_scan_count: 0,
        materialized_row_count: 0,
    };
}
