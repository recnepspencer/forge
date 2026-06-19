use super::construction::promote_loop_candidates;
use super::counters::PlanarBooleanLoopCandidateCounters;
use super::input::PlanarBooleanLoopCandidateBoundaryInput;
use super::row::{PlanarBooleanDeniedLoopCandidate, PlanarBooleanLoopCandidate};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopCandidateSet {
    loop_candidate_set_identity: String,
    request_identity: String,
    walk_outcome_set_identity: String,
    rows: Vec<PlanarBooleanLoopCandidate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDeniedLoopCandidateSet {
    denied_loop_candidate_set_identity: String,
    request_identity: String,
    walk_outcome_set_identity: String,
    rows: Vec<PlanarBooleanDeniedLoopCandidate>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopCandidateBoundary {
    loop_candidates: PlanarBooleanLoopCandidateSet,
    denied_loop_candidates: PlanarBooleanDeniedLoopCandidateSet,
    counters: PlanarBooleanLoopCandidateCounters,
}

impl PlanarBooleanLoopCandidateBoundary {
    pub fn promote(input: PlanarBooleanLoopCandidateBoundaryInput<'_>) -> Self {
        promote_loop_candidates(input)
    }

    pub(crate) fn new(
        loop_candidates: PlanarBooleanLoopCandidateSet,
        denied_loop_candidates: PlanarBooleanDeniedLoopCandidateSet,
        counters: PlanarBooleanLoopCandidateCounters,
    ) -> Self {
        Self {
            loop_candidates,
            denied_loop_candidates,
            counters,
        }
    }

    pub fn loop_candidates(&self) -> &PlanarBooleanLoopCandidateSet {
        &self.loop_candidates
    }

    pub fn denied_loop_candidates(&self) -> &PlanarBooleanDeniedLoopCandidateSet {
        &self.denied_loop_candidates
    }

    pub fn counters(&self) -> PlanarBooleanLoopCandidateCounters {
        self.counters
    }
}

impl PlanarBooleanLoopCandidateSet {
    pub(crate) fn new(
        loop_candidate_set_identity: String,
        request_identity: String,
        walk_outcome_set_identity: String,
        rows: Vec<PlanarBooleanLoopCandidate>,
    ) -> Self {
        Self {
            loop_candidate_set_identity,
            request_identity,
            walk_outcome_set_identity,
            rows,
        }
    }

    pub fn loop_candidate_set_identity(&self) -> &str {
        &self.loop_candidate_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn walk_outcome_set_identity(&self) -> &str {
        &self.walk_outcome_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopCandidate] {
        &self.rows
    }
}

impl PlanarBooleanDeniedLoopCandidateSet {
    pub(crate) fn new(
        denied_loop_candidate_set_identity: String,
        request_identity: String,
        walk_outcome_set_identity: String,
        rows: Vec<PlanarBooleanDeniedLoopCandidate>,
    ) -> Self {
        Self {
            denied_loop_candidate_set_identity,
            request_identity,
            walk_outcome_set_identity,
            rows,
        }
    }

    pub fn denied_loop_candidate_set_identity(&self) -> &str {
        &self.denied_loop_candidate_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn walk_outcome_set_identity(&self) -> &str {
        &self.walk_outcome_set_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanDeniedLoopCandidate] {
        &self.rows
    }
}
