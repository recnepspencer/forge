#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BridgeCorrespondenceRebuildReport {
    authoritative_query_dependencies: usize,
    authoritative_allocation_records: usize,
    rebuilt_allocation_keys: usize,
    exact_query_dependency_index_parity: bool,
    exact_mapping_index_parity: bool,
    exact_index_parity: bool,
}

impl BridgeCorrespondenceRebuildReport {
    pub(crate) const fn new(
        authoritative_query_dependencies: usize,
        authoritative_allocation_records: usize,
        rebuilt_allocation_keys: usize,
        exact_query_dependency_index_parity: bool,
        exact_mapping_index_parity: bool,
        exact_index_parity: bool,
    ) -> Self {
        Self {
            authoritative_query_dependencies,
            authoritative_allocation_records,
            rebuilt_allocation_keys,
            exact_query_dependency_index_parity,
            exact_mapping_index_parity,
            exact_index_parity,
        }
    }

    pub const fn authoritative_allocation_records(self) -> usize {
        self.authoritative_allocation_records
    }

    pub const fn authoritative_query_dependencies(self) -> usize {
        self.authoritative_query_dependencies
    }

    pub const fn rebuilt_allocation_keys(self) -> usize {
        self.rebuilt_allocation_keys
    }

    pub const fn exact_query_dependency_index_parity(self) -> bool {
        self.exact_query_dependency_index_parity
    }

    pub const fn exact_mapping_index_parity(self) -> bool {
        self.exact_mapping_index_parity
    }

    pub const fn exact_index_parity(self) -> bool {
        self.exact_index_parity
    }
}
