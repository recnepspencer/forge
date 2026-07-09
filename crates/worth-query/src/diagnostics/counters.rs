#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct CanonicalizationCounters {
    pub raw_clause_count: usize,
    pub normalized_clause_count: usize,
    pub projection_entry_count: usize,
    pub traversal_clause_count: usize,
    pub result_shape_field_count: usize,
    pub binding_descriptor_count: usize,
    pub query_deduplication_count: usize,
    pub result_shape_deduplication_count: usize,
    pub canonicalization_warning_count: usize,
    pub canonicalization_fallback_count: usize,
}
