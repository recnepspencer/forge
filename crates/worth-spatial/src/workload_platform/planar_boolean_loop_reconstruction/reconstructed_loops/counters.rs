#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanReconstructedLoopBoundaryCounters {
    loop_candidates_consumed: usize,
    fragment_memberships_consumed: usize,
    overlap_chain_lineages_consumed: usize,
    admitted_reconstructed_loops_emitted: usize,
    born_loops_emitted: usize,
    denied_candidates: usize,
}

impl PlanarBooleanReconstructedLoopBoundaryCounters {
    pub(crate) fn consumed_loop_candidate(&mut self) {
        self.loop_candidates_consumed += 1;
    }

    pub(crate) fn consumed_fragment_membership(&mut self) {
        self.fragment_memberships_consumed += 1;
    }

    pub(crate) fn consumed_overlap_chain_lineage(&mut self) {
        self.overlap_chain_lineages_consumed += 1;
    }

    pub(crate) fn emitted_admitted_reconstructed_loop(&mut self) {
        self.admitted_reconstructed_loops_emitted += 1;
    }

    pub(crate) fn emitted_born_loop(&mut self) {
        self.born_loops_emitted += 1;
    }

    pub(crate) fn denied_candidate(&mut self) {
        self.denied_candidates += 1;
    }

    pub fn loop_candidates_consumed(self) -> usize {
        self.loop_candidates_consumed
    }

    pub fn fragment_memberships_consumed(self) -> usize {
        self.fragment_memberships_consumed
    }

    pub fn overlap_chain_lineages_consumed(self) -> usize {
        self.overlap_chain_lineages_consumed
    }

    pub fn admitted_reconstructed_loops_emitted(self) -> usize {
        self.admitted_reconstructed_loops_emitted
    }

    pub fn born_loops_emitted(self) -> usize {
        self.born_loops_emitted
    }

    pub fn denied_candidates(self) -> usize {
        self.denied_candidates
    }
}
