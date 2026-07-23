#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCollectionCapabilityCounters {
    pub collection_contract_checks: usize,
    pub current_generation_checks: usize,
    pub native_layout_checks: usize,
    pub identity_relationship_checks: usize,
    pub identity_rows_indexed: usize,
    pub maintenance_rows_indexed: usize,
    pub ordering_terms_retained: usize,
    pub unrelated_rows_scanned: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryCollectionWindowCounters {
    pub authority_checks: usize,
    pub cursor_checks: usize,
    pub breadth_checks: usize,
    pub ordered_index_probes: usize,
    pub rows_visited: usize,
    pub window_rows_materialized: usize,
    pub unrelated_rows_scanned: usize,
}
