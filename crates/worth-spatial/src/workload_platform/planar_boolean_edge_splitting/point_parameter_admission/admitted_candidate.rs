use super::counters::PlanarBooleanSplitPointAdmissionCounters;
use super::endpoint_posture::PlanarBooleanSplitPointEndpointPosture;
use crate::workload_platform::planar_boolean_edge_splitting::point_split_candidates::PlanarBooleanPointSplitCandidate;

#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedPointSplitCandidate {
    candidate: PlanarBooleanPointSplitCandidate,
    endpoint_posture: PlanarBooleanSplitPointEndpointPosture,
    exact_endpoint_source_identity: Option<String>,
    exact_projected_endpoint_fact_identity: Option<String>,
}

impl AdmittedPointSplitCandidate {
    pub(crate) fn new(
        candidate: PlanarBooleanPointSplitCandidate,
        endpoint_posture: PlanarBooleanSplitPointEndpointPosture,
        exact_endpoint_source_identity: Option<String>,
        exact_projected_endpoint_fact_identity: Option<String>,
    ) -> Self {
        Self {
            candidate,
            endpoint_posture,
            exact_endpoint_source_identity,
            exact_projected_endpoint_fact_identity,
        }
    }

    pub fn candidate(&self) -> &PlanarBooleanPointSplitCandidate {
        &self.candidate
    }

    pub fn endpoint_posture(&self) -> PlanarBooleanSplitPointEndpointPosture {
        self.endpoint_posture
    }

    pub fn exact_endpoint_source_identity(&self) -> Option<&str> {
        self.exact_endpoint_source_identity.as_deref()
    }

    pub fn exact_projected_endpoint_fact_identity(&self) -> Option<&str> {
        self.exact_projected_endpoint_fact_identity.as_deref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanAdmittedPointSplitCandidateSet {
    point_candidate_set_identity: String,
    participation_index_identity: String,
    admitted_candidates: Vec<AdmittedPointSplitCandidate>,
    counters: PlanarBooleanSplitPointAdmissionCounters,
}

impl PlanarBooleanAdmittedPointSplitCandidateSet {
    pub(crate) fn new(
        point_candidate_set_identity: String,
        participation_index_identity: String,
        admitted_candidates: Vec<AdmittedPointSplitCandidate>,
        counters: PlanarBooleanSplitPointAdmissionCounters,
    ) -> Self {
        Self {
            point_candidate_set_identity,
            participation_index_identity,
            admitted_candidates,
            counters,
        }
    }

    pub fn point_candidate_set_identity(&self) -> &str {
        &self.point_candidate_set_identity
    }

    pub fn participation_index_identity(&self) -> &str {
        &self.participation_index_identity
    }

    pub fn admitted_candidates(&self) -> &[AdmittedPointSplitCandidate] {
        &self.admitted_candidates
    }

    pub fn counters(&self) -> PlanarBooleanSplitPointAdmissionCounters {
        self.counters
    }
}
