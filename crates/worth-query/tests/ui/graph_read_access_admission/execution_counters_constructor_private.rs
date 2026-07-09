use worth_query::facade::runtime::WorthQueryGraphReadAccessExecutionCounters;

fn main() {
    let _ = WorthQueryGraphReadAccessExecutionCounters {
        executor_entry_count: 0,
        strategy_recompute_count: 0,
        edge_scan_count: 0,
        materialized_row_count: 0,
    };
}
