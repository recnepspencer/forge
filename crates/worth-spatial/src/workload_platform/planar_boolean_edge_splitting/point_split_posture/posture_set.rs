use super::counters::PlanarBooleanPointSplitPostureCounters;
use super::posture::PosturedPointSplitCandidate;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointSplitPostureSet {
    posture_set_identity: String,
    point_candidate_set_identity: String,
    participation_index_identity: String,
    postured_candidates: Vec<PosturedPointSplitCandidate>,
    counters: PlanarBooleanPointSplitPostureCounters,
}

impl PlanarBooleanPointSplitPostureSet {
    pub(crate) fn new(
        posture_set_identity: String,
        point_candidate_set_identity: String,
        participation_index_identity: String,
        postured_candidates: Vec<PosturedPointSplitCandidate>,
        counters: PlanarBooleanPointSplitPostureCounters,
    ) -> Self {
        Self {
            posture_set_identity,
            point_candidate_set_identity,
            participation_index_identity,
            postured_candidates,
            counters,
        }
    }

    pub fn posture_set_identity(&self) -> &str {
        &self.posture_set_identity
    }

    pub fn point_candidate_set_identity(&self) -> &str {
        &self.point_candidate_set_identity
    }

    pub fn participation_index_identity(&self) -> &str {
        &self.participation_index_identity
    }

    pub fn postured_candidates(&self) -> &[PosturedPointSplitCandidate] {
        &self.postured_candidates
    }

    pub fn counters(&self) -> PlanarBooleanPointSplitPostureCounters {
        self.counters
    }
}
