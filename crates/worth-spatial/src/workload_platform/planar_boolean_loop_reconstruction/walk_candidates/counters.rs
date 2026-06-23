#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanClosedWalkCandidateCounters {
    continuation_rows_consumed: usize,
    walk_candidates_assembled: usize,
    fragment_consumption_rows_emitted: usize,
}

impl PlanarBooleanClosedWalkCandidateCounters {
    pub(crate) fn consumed_continuation_row(&mut self) {
        self.continuation_rows_consumed += 1;
    }

    pub(crate) fn assembled_walk_candidate(&mut self) {
        self.walk_candidates_assembled += 1;
    }

    pub(crate) fn emitted_fragment_consumption_row(&mut self) {
        self.fragment_consumption_rows_emitted += 1;
    }

    pub fn continuation_rows_consumed(&self) -> usize {
        self.continuation_rows_consumed
    }

    pub fn walk_candidates_assembled(&self) -> usize {
        self.walk_candidates_assembled
    }

    pub fn fragment_consumption_rows_emitted(&self) -> usize {
        self.fragment_consumption_rows_emitted
    }
}
