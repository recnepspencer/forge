use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanWalkOutcomeSet;

pub struct PlanarBooleanLoopCandidateBoundaryInput<'a> {
    walk_outcomes: &'a PlanarBooleanWalkOutcomeSet,
}

impl<'a> PlanarBooleanLoopCandidateBoundaryInput<'a> {
    pub fn from_walk_outcomes(walk_outcomes: &'a PlanarBooleanWalkOutcomeSet) -> Self {
        Self { walk_outcomes }
    }

    pub(crate) fn walk_outcomes(&self) -> &'a PlanarBooleanWalkOutcomeSet {
        self.walk_outcomes
    }
}
