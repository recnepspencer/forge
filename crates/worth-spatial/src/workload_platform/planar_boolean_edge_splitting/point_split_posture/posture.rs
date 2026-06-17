use crate::workload_platform::planar_boolean_edge_splitting::point_parameter_admission::AdmittedPointSplitCandidate;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPointSplitPosture {
    InteriorSplit,
    TJunctionPromotion,
    SharedEndpoint,
    EndpointNoOp,
}

impl PlanarBooleanPointSplitPosture {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::InteriorSplit => "interior-split",
            Self::TJunctionPromotion => "t-junction-promotion",
            Self::SharedEndpoint => "shared-endpoint",
            Self::EndpointNoOp => "endpoint-no-op",
        }
    }

    pub fn produces_split_vertex(self) -> bool {
        matches!(self, Self::InteriorSplit | Self::TJunctionPromotion)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PosturedPointSplitCandidate {
    postured_candidate_identity: String,
    admitted_candidate: AdmittedPointSplitCandidate,
    posture: PlanarBooleanPointSplitPosture,
}

impl PosturedPointSplitCandidate {
    pub(crate) fn new(
        postured_candidate_identity: String,
        admitted_candidate: AdmittedPointSplitCandidate,
        posture: PlanarBooleanPointSplitPosture,
    ) -> Self {
        Self {
            postured_candidate_identity,
            admitted_candidate,
            posture,
        }
    }

    pub fn postured_candidate_identity(&self) -> &str {
        &self.postured_candidate_identity
    }

    pub fn admitted_candidate(&self) -> &AdmittedPointSplitCandidate {
        &self.admitted_candidate
    }

    pub fn posture(&self) -> PlanarBooleanPointSplitPosture {
        self.posture
    }
}
