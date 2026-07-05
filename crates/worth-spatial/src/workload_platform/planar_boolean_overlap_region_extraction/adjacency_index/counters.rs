#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapAdjacencyIndexCounters {
    neighborhoods_indexed: usize,
    adjacency_rows_emitted: usize,
    chain_lineage_rows_consumed: usize,
    denied_neighborhoods: usize,
}

impl PlanarBooleanOverlapAdjacencyIndexCounters {
    pub(crate) fn indexed_neighborhood(&mut self) {
        self.neighborhoods_indexed += 1;
    }

    pub(crate) fn emitted_row(&mut self) {
        self.adjacency_rows_emitted += 1;
    }

    pub(crate) fn consumed_chain_lineage_row(&mut self) {
        self.chain_lineage_rows_consumed += 1;
    }

    pub(crate) fn denied_neighborhood(&mut self) {
        self.denied_neighborhoods += 1;
    }

    pub fn neighborhoods_indexed(self) -> usize {
        self.neighborhoods_indexed
    }

    pub fn adjacency_rows_emitted(self) -> usize {
        self.adjacency_rows_emitted
    }

    pub fn chain_lineage_rows_consumed(self) -> usize {
        self.chain_lineage_rows_consumed
    }

    pub fn denied_neighborhoods(self) -> usize {
        self.denied_neighborhoods
    }
}
