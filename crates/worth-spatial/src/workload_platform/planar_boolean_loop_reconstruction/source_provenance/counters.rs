#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopSourceProvenanceCounters {
    split_chains_consumed: usize,
    source_carriers_recovered: usize,
    fragment_memberships_recovered: usize,
    overlap_chain_lineages_recovered: usize,
    dangling_reference_denials: usize,
    foreign_lineage_denials: usize,
}

impl PlanarBooleanLoopSourceProvenanceCounters {
    pub(crate) fn consumed_split_chain(&mut self) {
        self.split_chains_consumed += 1;
    }

    pub(crate) fn recovered_source_carrier(&mut self) {
        self.source_carriers_recovered += 1;
    }

    pub(crate) fn recovered_fragment_membership(&mut self) {
        self.fragment_memberships_recovered += 1;
    }

    pub(crate) fn recovered_overlap_chain_lineage(&mut self) {
        self.overlap_chain_lineages_recovered += 1;
    }

    pub(crate) fn rejected_dangling_reference(&mut self) {
        self.dangling_reference_denials += 1;
    }

    pub(crate) fn rejected_foreign_lineage(&mut self) {
        self.foreign_lineage_denials += 1;
    }

    pub fn split_chains_consumed(self) -> usize {
        self.split_chains_consumed
    }

    pub fn source_carriers_recovered(self) -> usize {
        self.source_carriers_recovered
    }

    pub fn fragment_memberships_recovered(self) -> usize {
        self.fragment_memberships_recovered
    }

    pub fn overlap_chain_lineages_recovered(self) -> usize {
        self.overlap_chain_lineages_recovered
    }

    pub fn dangling_reference_denials(self) -> usize {
        self.dangling_reference_denials
    }

    pub fn foreign_lineage_denials(self) -> usize {
        self.foreign_lineage_denials
    }
}
