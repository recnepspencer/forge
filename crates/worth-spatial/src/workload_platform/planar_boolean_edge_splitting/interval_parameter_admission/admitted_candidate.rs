use crate::workload_platform::planar_boolean_edge_splitting::interval_split_candidates::PlanarBooleanIntervalSplitCandidate;

use super::counters::PlanarBooleanSplitIntervalAdmissionCounters;

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedIntervalSplitCandidate {
    candidate: PlanarBooleanIntervalSplitCandidate,
    admitted_parameter_range: [f64; 2],
}

impl AdmittedIntervalSplitCandidate {
    pub(crate) fn new(
        candidate: PlanarBooleanIntervalSplitCandidate,
        admitted_parameter_range: [f64; 2],
    ) -> Self {
        Self {
            candidate,
            admitted_parameter_range,
        }
    }

    pub fn candidate(&self) -> &PlanarBooleanIntervalSplitCandidate {
        &self.candidate
    }

    pub fn admitted_parameter_range(&self) -> [f64; 2] {
        self.admitted_parameter_range
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanAdmittedIntervalSplitCandidateSet {
    interval_candidate_set_identity: String,
    participation_index_identity: String,
    admitted_candidates: Vec<AdmittedIntervalSplitCandidate>,
    counters: PlanarBooleanSplitIntervalAdmissionCounters,
}

impl PlanarBooleanAdmittedIntervalSplitCandidateSet {
    pub(crate) fn new(
        interval_candidate_set_identity: String,
        participation_index_identity: String,
        admitted_candidates: Vec<AdmittedIntervalSplitCandidate>,
        counters: PlanarBooleanSplitIntervalAdmissionCounters,
    ) -> Self {
        Self {
            interval_candidate_set_identity,
            participation_index_identity,
            admitted_candidates,
            counters,
        }
    }

    pub fn interval_candidate_set_identity(&self) -> &str {
        &self.interval_candidate_set_identity
    }

    pub fn participation_index_identity(&self) -> &str {
        &self.participation_index_identity
    }

    pub fn admitted_candidates(&self) -> &[AdmittedIntervalSplitCandidate] {
        &self.admitted_candidates
    }

    pub fn counters(&self) -> PlanarBooleanSplitIntervalAdmissionCounters {
        self.counters
    }
}
