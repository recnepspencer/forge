#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapEdgeChainCounters {
    schedules_inspected: usize,
    interval_subdivisions_inspected: usize,
    fragment_rows_inspected: usize,
    chains_emitted: usize,
    chain_members_emitted: usize,
    partial_overlap_chains: usize,
    identical_parallel_chains: usize,
    identical_antiparallel_chains: usize,
    different_parameterization_chains: usize,
    opposite_sense_chains: usize,
    missing_fragment_references_rejected: usize,
    missing_subdivision_references_rejected: usize,
    mismatched_fragment_authority_rejected: usize,
    foreign_fragment_sets_rejected: usize,
    topology_products_emitted: usize,
}

impl PlanarBooleanOverlapEdgeChainCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        schedules_inspected: usize,
        interval_subdivisions_inspected: usize,
        fragment_rows_inspected: usize,
        chains_emitted: usize,
        chain_members_emitted: usize,
        partial_overlap_chains: usize,
        identical_parallel_chains: usize,
        identical_antiparallel_chains: usize,
        different_parameterization_chains: usize,
        opposite_sense_chains: usize,
        missing_fragment_references_rejected: usize,
        missing_subdivision_references_rejected: usize,
        mismatched_fragment_authority_rejected: usize,
        foreign_fragment_sets_rejected: usize,
        topology_products_emitted: usize,
    ) -> Self {
        Self {
            schedules_inspected,
            interval_subdivisions_inspected,
            fragment_rows_inspected,
            chains_emitted,
            chain_members_emitted,
            partial_overlap_chains,
            identical_parallel_chains,
            identical_antiparallel_chains,
            different_parameterization_chains,
            opposite_sense_chains,
            missing_fragment_references_rejected,
            missing_subdivision_references_rejected,
            mismatched_fragment_authority_rejected,
            foreign_fragment_sets_rejected,
            topology_products_emitted,
        }
    }

    pub fn schedules_inspected(self) -> usize {
        self.schedules_inspected
    }
    pub fn interval_subdivisions_inspected(self) -> usize {
        self.interval_subdivisions_inspected
    }
    pub fn fragment_rows_inspected(self) -> usize {
        self.fragment_rows_inspected
    }
    pub fn chains_emitted(self) -> usize {
        self.chains_emitted
    }
    pub fn chain_members_emitted(self) -> usize {
        self.chain_members_emitted
    }
    pub fn partial_overlap_chains(self) -> usize {
        self.partial_overlap_chains
    }
    pub fn identical_parallel_chains(self) -> usize {
        self.identical_parallel_chains
    }
    pub fn identical_antiparallel_chains(self) -> usize {
        self.identical_antiparallel_chains
    }
    pub fn different_parameterization_chains(self) -> usize {
        self.different_parameterization_chains
    }
    pub fn opposite_sense_chains(self) -> usize {
        self.opposite_sense_chains
    }
    pub fn missing_fragment_references_rejected(self) -> usize {
        self.missing_fragment_references_rejected
    }
    pub fn missing_subdivision_references_rejected(self) -> usize {
        self.missing_subdivision_references_rejected
    }
    pub fn mismatched_fragment_authority_rejected(self) -> usize {
        self.mismatched_fragment_authority_rejected
    }
    pub fn foreign_fragment_sets_rejected(self) -> usize {
        self.foreign_fragment_sets_rejected
    }
    pub fn topology_products_emitted(self) -> usize {
        self.topology_products_emitted
    }
}
