use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadMaterializationCounters {
    digest: String,
    touched_edges: usize,
    frontier_pages: usize,
    allocated_bytes: usize,
    emitted_rows: usize,
    checkpoint_count: usize,
    cancellation_poll_count: usize,
}

impl ForgeQueryGraphReadMaterializationCounters {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn touched_edges(&self) -> usize {
        self.touched_edges
    }

    pub fn frontier_pages(&self) -> usize {
        self.frontier_pages
    }

    pub fn allocated_bytes(&self) -> usize {
        self.allocated_bytes
    }

    pub fn emitted_rows(&self) -> usize {
        self.emitted_rows
    }

    pub fn checkpoint_count(&self) -> usize {
        self.checkpoint_count
    }

    pub fn cancellation_poll_count(&self) -> usize {
        self.cancellation_poll_count
    }

    pub(crate) fn new(
        touched_edges: usize,
        frontier_pages: usize,
        allocated_bytes: usize,
        emitted_rows: usize,
        checkpoint_count: usize,
        cancellation_poll_count: usize,
    ) -> Self {
        let digest = hash_parts(&[
            "forge_query_graph_read_materialization_counters_v1".to_string(),
            format!("touched_edges:{touched_edges}"),
            format!("frontier_pages:{frontier_pages}"),
            format!("allocated_bytes:{allocated_bytes}"),
            format!("emitted_rows:{emitted_rows}"),
            format!("checkpoint_count:{checkpoint_count}"),
            format!("cancellation_poll_count:{cancellation_poll_count}"),
        ]);
        Self {
            digest,
            touched_edges,
            frontier_pages,
            allocated_bytes,
            emitted_rows,
            checkpoint_count,
            cancellation_poll_count,
        }
    }
}
