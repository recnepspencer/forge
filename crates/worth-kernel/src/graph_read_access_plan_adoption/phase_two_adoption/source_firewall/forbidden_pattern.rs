pub(crate) const FORBIDDEN_EXECUTION_PATTERNS: &[&str] = &[
    "local_adjacency_map",
    "caller_owned_graph_work",
    "fabricated_receipt",
    "manual_access_plan",
    "strategy_hint",
    "compatibility_wrapper",
    "ForgeQueryReadReceipt",
    "ForgeQueryGraphReadAccessPlanConsumption",
];
