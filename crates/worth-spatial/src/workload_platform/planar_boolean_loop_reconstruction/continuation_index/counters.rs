#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanFragmentContinuationCounters {
    split_vertices_consumed: usize,
    overlap_chains_consumed: usize,
    fragment_continuations_indexed: usize,
    dangling_reference_denials: usize,
    duplicate_slot_denials: usize,
    foreign_lineage_denials: usize,
}

impl PlanarBooleanFragmentContinuationCounters {
    pub(crate) fn consumed_split_vertex(&mut self) {
        self.split_vertices_consumed += 1;
    }

    pub(crate) fn consumed_overlap_chain(&mut self) {
        self.overlap_chains_consumed += 1;
    }

    pub(crate) fn indexed_fragment_continuation(&mut self) {
        self.fragment_continuations_indexed += 1;
    }

    pub(crate) fn rejected_dangling_reference(&mut self) {
        self.dangling_reference_denials += 1;
    }

    pub(crate) fn rejected_duplicate_slot(&mut self) {
        self.duplicate_slot_denials += 1;
    }

    pub(crate) fn rejected_foreign_lineage(&mut self) {
        self.foreign_lineage_denials += 1;
    }

    pub fn split_vertices_consumed(self) -> usize {
        self.split_vertices_consumed
    }

    pub fn overlap_chains_consumed(self) -> usize {
        self.overlap_chains_consumed
    }

    pub fn fragment_continuations_indexed(self) -> usize {
        self.fragment_continuations_indexed
    }

    pub fn dangling_reference_denials(self) -> usize {
        self.dangling_reference_denials
    }

    pub fn duplicate_slot_denials(self) -> usize {
        self.duplicate_slot_denials
    }

    pub fn foreign_lineage_denials(self) -> usize {
        self.foreign_lineage_denials
    }
}
