#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiCollectionProjectionWorkCounters {
    rows_visited: usize,
    selected_key_accesses: usize,
    indexed_row_lookups: usize,
    native_values_materialized: usize,
    native_bytes_retained: usize,
    continuation_operations: usize,
    key_resolution_declaration_checks: usize,
    key_resolution_indexed_slot_lookups: usize,
    key_resolution_path_matches: usize,
    key_resolution_key_scans: usize,
    key_resolution_path_parses: usize,
    unrelated_width_scans: usize,
}

impl UiCollectionProjectionWorkCounters {
    pub(crate) fn visit_row(&mut self) {
        self.rows_visited += 1;
    }

    pub(crate) fn record_native_access(
        &mut self,
        query: worth_query::facade::installed::collection::WorthQueryCollectionNativeAccessCounters,
        byte_len: usize,
    ) {
        self.selected_key_accesses += 1;
        self.indexed_row_lookups += query.indexed_row_lookups;
        self.native_values_materialized += query.native_facts_materialized;
        self.native_bytes_retained += byte_len;
    }

    pub(crate) fn record_continuation(&mut self) {
        self.continuation_operations += 1;
    }

    pub(crate) fn record_key_resolution(
        &mut self,
        query: worth_query::facade::domain::WorthQueryNativeKeyResolutionCounters,
    ) {
        self.key_resolution_declaration_checks += query.declaration_checks;
        self.key_resolution_indexed_slot_lookups += query.indexed_slot_lookups;
        self.key_resolution_path_matches += query.path_matches;
        self.key_resolution_key_scans += query.key_scans;
        self.key_resolution_path_parses += query.path_parses;
    }

    pub fn rows_visited(self) -> usize {
        self.rows_visited
    }

    pub fn selected_key_accesses(self) -> usize {
        self.selected_key_accesses
    }

    pub fn indexed_row_lookups(self) -> usize {
        self.indexed_row_lookups
    }

    pub fn native_values_materialized(self) -> usize {
        self.native_values_materialized
    }

    pub fn native_bytes_retained(self) -> usize {
        self.native_bytes_retained
    }

    pub fn continuation_operations(self) -> usize {
        self.continuation_operations
    }

    pub fn key_resolution_declaration_checks(self) -> usize {
        self.key_resolution_declaration_checks
    }

    pub fn key_resolution_indexed_slot_lookups(self) -> usize {
        self.key_resolution_indexed_slot_lookups
    }

    pub fn key_resolution_path_matches(self) -> usize {
        self.key_resolution_path_matches
    }

    pub fn key_resolution_key_scans(self) -> usize {
        self.key_resolution_key_scans
    }

    pub fn key_resolution_path_parses(self) -> usize {
        self.key_resolution_path_parses
    }

    pub fn unrelated_width_scans(self) -> usize {
        self.unrelated_width_scans
    }
}
