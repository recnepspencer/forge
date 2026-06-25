#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiGraphInvalidationCounters {
    authoritative_fact_count: usize,
    derived_fact_count: usize,
    traversed_edge_count: usize,
    registry_count: usize,
}

impl WorthUiGraphInvalidationCounters {
    pub(super) fn new(
        authoritative_fact_count: usize,
        derived_fact_count: usize,
        traversed_edge_count: usize,
        registry_count: usize,
    ) -> Self {
        Self {
            authoritative_fact_count,
            derived_fact_count,
            traversed_edge_count,
            registry_count,
        }
    }

    pub fn authoritative_fact_count(self) -> usize {
        self.authoritative_fact_count
    }

    pub fn derived_fact_count(self) -> usize {
        self.derived_fact_count
    }

    pub fn traversed_edge_count(self) -> usize {
        self.traversed_edge_count
    }

    pub fn registry_count(self) -> usize {
        self.registry_count
    }
}
