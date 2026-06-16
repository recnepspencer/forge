#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitChainValidationCounters {
    source_edges_checked: usize,
    fragment_schedules_checked: usize,
    fragments_checked: usize,
    overlap_chains_checked: usize,
    overlap_members_checked: usize,
    gaps_rejected: usize,
    overlaps_rejected: usize,
    dangling_references_rejected: usize,
    mismatched_interval_basis_rejected: usize,
    foreign_chain_sets_rejected: usize,
    out_of_interval_references_rejected: usize,
    denied_chains: usize,
}

impl PlanarBooleanSplitChainValidationCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        source_edges_checked: usize,
        fragment_schedules_checked: usize,
        fragments_checked: usize,
        overlap_chains_checked: usize,
        overlap_members_checked: usize,
        gaps_rejected: usize,
        overlaps_rejected: usize,
        dangling_references_rejected: usize,
        mismatched_interval_basis_rejected: usize,
        foreign_chain_sets_rejected: usize,
        out_of_interval_references_rejected: usize,
        denied_chains: usize,
    ) -> Self {
        Self {
            source_edges_checked,
            fragment_schedules_checked,
            fragments_checked,
            overlap_chains_checked,
            overlap_members_checked,
            gaps_rejected,
            overlaps_rejected,
            dangling_references_rejected,
            mismatched_interval_basis_rejected,
            foreign_chain_sets_rejected,
            out_of_interval_references_rejected,
            denied_chains,
        }
    }

    pub fn source_edges_checked(self) -> usize {
        self.source_edges_checked
    }
    pub fn fragment_schedules_checked(self) -> usize {
        self.fragment_schedules_checked
    }
    pub fn fragments_checked(self) -> usize {
        self.fragments_checked
    }
    pub fn overlap_chains_checked(self) -> usize {
        self.overlap_chains_checked
    }
    pub fn overlap_members_checked(self) -> usize {
        self.overlap_members_checked
    }
    pub fn gaps_rejected(self) -> usize {
        self.gaps_rejected
    }
    pub fn overlaps_rejected(self) -> usize {
        self.overlaps_rejected
    }
    pub fn dangling_references_rejected(self) -> usize {
        self.dangling_references_rejected
    }
    pub fn mismatched_interval_basis_rejected(self) -> usize {
        self.mismatched_interval_basis_rejected
    }
    pub fn foreign_chain_sets_rejected(self) -> usize {
        self.foreign_chain_sets_rejected
    }
    pub fn out_of_interval_references_rejected(self) -> usize {
        self.out_of_interval_references_rejected
    }
    pub fn denied_chains(self) -> usize {
        self.denied_chains
    }
}
