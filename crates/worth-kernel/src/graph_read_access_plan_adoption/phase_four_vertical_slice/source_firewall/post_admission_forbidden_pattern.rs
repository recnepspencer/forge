pub(crate) const POST_ADMISSION_FORBIDDEN_GRAPH_READ_PATTERNS: &[&str] = &[
    "local_adjacency_map",
    "local_graph_traversal",
    "broad_graph_scan",
    "fabricated_receipt",
    "manual_read_plan",
    "operator_strategy_hint",
    "compatibility_wrapper",
    "local_cache",
    "call_query_after_local_traversal",
];
