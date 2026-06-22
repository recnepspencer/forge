#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanLoopCandidateCounters {
    closed_walks_considered: usize,
    loop_candidates_promoted: usize,
    denied_loop_candidates_emitted: usize,
}

impl PlanarBooleanLoopCandidateCounters {
    pub(crate) fn considered_closed_walk(&mut self) {
        self.closed_walks_considered += 1;
    }

    pub(crate) fn promoted_loop_candidate(&mut self) {
        self.loop_candidates_promoted += 1;
    }

    pub(crate) fn emitted_denied_loop_candidate(&mut self) {
        self.denied_loop_candidates_emitted += 1;
    }

    pub fn closed_walks_considered(&self) -> usize {
        self.closed_walks_considered
    }

    pub fn loop_candidates_promoted(&self) -> usize {
        self.loop_candidates_promoted
    }

    pub fn denied_loop_candidates_emitted(&self) -> usize {
        self.denied_loop_candidates_emitted
    }
}
