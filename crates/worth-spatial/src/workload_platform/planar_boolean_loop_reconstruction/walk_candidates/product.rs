use super::construction::assemble_closed_walk_candidates;
use super::counters::PlanarBooleanClosedWalkCandidateCounters;
use super::input::PlanarBooleanClosedWalkCandidateSetInput;
use super::proof::PlanarBooleanFragmentConsumptionProof;
use super::row::PlanarBooleanClosedWalkCandidate;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanClosedWalkCandidateSet {
    closed_walk_candidate_set_identity: String,
    request_identity: String,
    continuation_index_identity: String,
    rows: Vec<PlanarBooleanClosedWalkCandidate>,
    counters: PlanarBooleanClosedWalkCandidateCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanClosedWalkCandidateAssembly {
    closed_walk_candidates: PlanarBooleanClosedWalkCandidateSet,
    fragment_consumption_proof: PlanarBooleanFragmentConsumptionProof,
}

impl PlanarBooleanClosedWalkCandidateAssembly {
    pub fn assemble(input: PlanarBooleanClosedWalkCandidateSetInput<'_>) -> Self {
        assemble_closed_walk_candidates(input)
    }

    pub(crate) fn new(
        closed_walk_candidates: PlanarBooleanClosedWalkCandidateSet,
        fragment_consumption_proof: PlanarBooleanFragmentConsumptionProof,
    ) -> Self {
        Self {
            closed_walk_candidates,
            fragment_consumption_proof,
        }
    }

    pub fn closed_walk_candidates(&self) -> &PlanarBooleanClosedWalkCandidateSet {
        &self.closed_walk_candidates
    }

    pub fn fragment_consumption_proof(&self) -> &PlanarBooleanFragmentConsumptionProof {
        &self.fragment_consumption_proof
    }
}

impl PlanarBooleanClosedWalkCandidateSet {
    pub(crate) fn new(
        closed_walk_candidate_set_identity: String,
        request_identity: String,
        continuation_index_identity: String,
        rows: Vec<PlanarBooleanClosedWalkCandidate>,
        counters: PlanarBooleanClosedWalkCandidateCounters,
    ) -> Self {
        Self {
            closed_walk_candidate_set_identity,
            request_identity,
            continuation_index_identity,
            rows,
            counters,
        }
    }

    pub fn closed_walk_candidate_set_identity(&self) -> &str {
        &self.closed_walk_candidate_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn continuation_index_identity(&self) -> &str {
        &self.continuation_index_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanClosedWalkCandidate] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanClosedWalkCandidateCounters {
        self.counters
    }
}
