use super::lane_set::OpenClassParityLaneSet;
use super::open_class::OpenTopologyClass;
use crate::workload_platform::projection_fact_parity::ProjectionFactParityLane;
use crate::workload_platform::projection_workload::ProjectedPlanarWorkload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassLaneAuthorityEvidence {
    topology_class: OpenTopologyClass,
    lane: ProjectionFactParityLane,
    topology_identity: String,
    evidence_identity: String,
}

impl OpenClassLaneAuthorityEvidence {
    pub fn retained_checkpoint_from_lane_set(lane_set: &OpenClassParityLaneSet) -> Option<Self> {
        Some(Self::from_lane_set(
            lane_set,
            ProjectionFactParityLane::Retained,
            lane_set.retained_lane_identity()?,
        ))
    }

    pub fn projection_consumed_from_lane_set(lane_set: &OpenClassParityLaneSet) -> Option<Self> {
        Some(Self::from_lane_set(
            lane_set,
            ProjectionFactParityLane::ProjectionConsumed,
            lane_set.projection_consumed_lane_identity()?,
        ))
    }

    fn from_lane_set(
        lane_set: &OpenClassParityLaneSet,
        lane: ProjectionFactParityLane,
        evidence_identity: &str,
    ) -> Self {
        Self {
            topology_class: lane_set.topology_class(),
            lane,
            topology_identity: lane_set.topology_identity().to_string(),
            evidence_identity: evidence_identity.to_string(),
        }
    }

    pub fn topology_class(&self) -> OpenTopologyClass {
        self.topology_class
    }

    pub fn lane(&self) -> ProjectionFactParityLane {
        self.lane
    }

    pub fn topology_identity(&self) -> &str {
        &self.topology_identity
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenClassStormExtractionEvidence {
    projection_stage_identity: String,
}

impl OpenClassStormExtractionEvidence {
    pub(crate) fn from_digest(projection_stage_identity: &str) -> Self {
        Self {
            projection_stage_identity: projection_stage_identity.to_string(),
        }
    }

    pub fn from_projected_workload(projected: &ProjectedPlanarWorkload) -> Self {
        Self {
            projection_stage_identity: projected.receipts().stage_identity().receipt_identity(),
        }
    }

    pub fn projection_stage_identity(&self) -> &str {
        &self.projection_stage_identity
    }
}
