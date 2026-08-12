//! Bank-owned numeric projection-work description.

use worth_query_host::facade::primary_graph::WorthQueryInvariantProjectionWork;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BankMutationProjectionWork {
    equality_lookups: usize,
    index_candidates_examined: usize,
    adjacency_lists_read: usize,
    adjacency_edges_inspected: usize,
    endpoint_records_read: usize,
    field_reads: usize,
    aggregate_lookups: usize,
    aggregate_cache_hits: usize,
    aggregate_rebuild_input_rows: usize,
    reconstructive_scans: usize,
}

impl BankMutationProjectionWork {
    pub(crate) const fn from_query(work: WorthQueryInvariantProjectionWork) -> Self {
        Self {
            equality_lookups: work.equality_lookups(),
            index_candidates_examined: work.index_candidates_examined(),
            adjacency_lists_read: work.adjacency_lists_read(),
            adjacency_edges_inspected: work.adjacency_edges_inspected(),
            endpoint_records_read: work.endpoint_records_read(),
            field_reads: work.field_reads(),
            aggregate_lookups: work.aggregate_lookups(),
            aggregate_cache_hits: work.aggregate_cache_hits(),
            aggregate_rebuild_input_rows: work.aggregate_rebuild_input_rows(),
            reconstructive_scans: work.reconstructive_scans(),
        }
    }

    pub const fn equality_lookups(self) -> usize {
        self.equality_lookups
    }
    pub const fn index_candidates_examined(self) -> usize {
        self.index_candidates_examined
    }
    pub const fn adjacency_lists_read(self) -> usize {
        self.adjacency_lists_read
    }
    pub const fn adjacency_edges_inspected(self) -> usize {
        self.adjacency_edges_inspected
    }
    pub const fn endpoint_records_read(self) -> usize {
        self.endpoint_records_read
    }
    pub const fn field_reads(self) -> usize {
        self.field_reads
    }
    pub const fn aggregate_lookups(self) -> usize {
        self.aggregate_lookups
    }
    pub const fn aggregate_cache_hits(self) -> usize {
        self.aggregate_cache_hits
    }
    pub const fn aggregate_rebuild_input_rows(self) -> usize {
        self.aggregate_rebuild_input_rows
    }
    pub const fn reconstructive_scans(self) -> usize {
        self.reconstructive_scans
    }

    pub const fn provider_work_units(self) -> usize {
        self.equality_lookups
            + self.index_candidates_examined
            + self.adjacency_lists_read
            + self.adjacency_edges_inspected
            + self.endpoint_records_read
            + self.field_reads
            + self.aggregate_lookups
            + self.reconstructive_scans
    }
}
