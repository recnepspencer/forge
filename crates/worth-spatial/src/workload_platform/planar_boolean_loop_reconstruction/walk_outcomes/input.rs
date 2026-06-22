use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanClosedWalkCandidateSet, PlanarBooleanFragmentConsumptionProof,
};

pub struct PlanarBooleanWalkOutcomeSetInput<'a> {
    closed_walk_candidates: &'a PlanarBooleanClosedWalkCandidateSet,
    fragment_consumption_proof: &'a PlanarBooleanFragmentConsumptionProof,
}

impl<'a> PlanarBooleanWalkOutcomeSetInput<'a> {
    pub fn from_closed_walk_candidates(
        closed_walk_candidates: &'a PlanarBooleanClosedWalkCandidateSet,
        fragment_consumption_proof: &'a PlanarBooleanFragmentConsumptionProof,
    ) -> Self {
        Self {
            closed_walk_candidates,
            fragment_consumption_proof,
        }
    }

    pub(crate) fn closed_walk_candidates(&self) -> &'a PlanarBooleanClosedWalkCandidateSet {
        self.closed_walk_candidates
    }

    pub(crate) fn fragment_consumption_proof(&self) -> &'a PlanarBooleanFragmentConsumptionProof {
        self.fragment_consumption_proof
    }
}
