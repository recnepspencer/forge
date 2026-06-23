#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanDegenerateLoopOutcomeBoundaryCounters {
    loops_consumed: usize,
    reconstructed_loops_consumed: usize,
    born_loops_consumed: usize,
    admitted_for_identity_minting: usize,
    tiny_cardinality_outcomes_emitted: usize,
    self_touching_outcomes_emitted: usize,
    zero_area_outcomes_emitted: usize,
    geometry_policy_required_outcomes_emitted: usize,
    policy_required_outcomes_emitted: usize,
}

impl PlanarBooleanDegenerateLoopOutcomeBoundaryCounters {
    pub(crate) fn consumed_reconstructed_loop(&mut self) {
        self.loops_consumed += 1;
        self.reconstructed_loops_consumed += 1;
    }

    pub(crate) fn consumed_born_loop(&mut self) {
        self.loops_consumed += 1;
        self.born_loops_consumed += 1;
    }

    pub(crate) fn emitted_admitted(&mut self) {
        self.admitted_for_identity_minting += 1;
    }

    pub(crate) fn emitted_tiny_cardinality(&mut self) {
        self.tiny_cardinality_outcomes_emitted += 1;
    }

    pub(crate) fn emitted_self_touching(&mut self) {
        self.self_touching_outcomes_emitted += 1;
    }

    pub(crate) fn emitted_zero_area(&mut self) {
        self.zero_area_outcomes_emitted += 1;
    }

    pub(crate) fn emitted_geometry_policy_required(&mut self) {
        self.geometry_policy_required_outcomes_emitted += 1;
    }

    pub(crate) fn emitted_policy_required(&mut self) {
        self.policy_required_outcomes_emitted += 1;
    }

    pub fn loops_consumed(&self) -> usize {
        self.loops_consumed
    }

    pub fn reconstructed_loops_consumed(&self) -> usize {
        self.reconstructed_loops_consumed
    }

    pub fn born_loops_consumed(&self) -> usize {
        self.born_loops_consumed
    }

    pub fn admitted_for_identity_minting(&self) -> usize {
        self.admitted_for_identity_minting
    }

    pub fn tiny_cardinality_outcomes_emitted(&self) -> usize {
        self.tiny_cardinality_outcomes_emitted
    }

    pub fn self_touching_outcomes_emitted(&self) -> usize {
        self.self_touching_outcomes_emitted
    }

    pub fn zero_area_outcomes_emitted(&self) -> usize {
        self.zero_area_outcomes_emitted
    }

    pub fn geometry_policy_required_outcomes_emitted(&self) -> usize {
        self.geometry_policy_required_outcomes_emitted
    }

    pub fn policy_required_outcomes_emitted(&self) -> usize {
        self.policy_required_outcomes_emitted
    }
}
