#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiReloadFailureCounters {
    preservation_receipt_count: usize,
    active_state_mutation_count: usize,
    durable_state_mutation_count: usize,
    query_binding_mutation_count: usize,
    fallback_runtime_creation_count: usize,
    source_reparse_count: usize,
    registry_rebuild_count: usize,
    semantic_replanning_count: usize,
    query_replanning_count: usize,
}

impl WorthUiReloadFailureCounters {
    pub(crate) fn preserved_without_runtime_mutation() -> Self {
        Self {
            preservation_receipt_count: 1,
            active_state_mutation_count: 0,
            durable_state_mutation_count: 0,
            query_binding_mutation_count: 0,
            fallback_runtime_creation_count: 0,
            source_reparse_count: 0,
            registry_rebuild_count: 0,
            semantic_replanning_count: 0,
            query_replanning_count: 0,
        }
    }

    pub fn preservation_receipt_count(self) -> usize {
        self.preservation_receipt_count
    }

    pub fn active_state_mutation_count(self) -> usize {
        self.active_state_mutation_count
    }

    pub fn durable_state_mutation_count(self) -> usize {
        self.durable_state_mutation_count
    }

    pub fn query_binding_mutation_count(self) -> usize {
        self.query_binding_mutation_count
    }

    pub fn fallback_runtime_creation_count(self) -> usize {
        self.fallback_runtime_creation_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn registry_rebuild_count(self) -> usize {
        self.registry_rebuild_count
    }

    pub fn semantic_replanning_count(self) -> usize {
        self.semantic_replanning_count
    }

    pub fn query_replanning_count(self) -> usize {
        self.query_replanning_count
    }
}
