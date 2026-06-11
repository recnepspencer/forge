use super::{
    case::ProjectionFactParityCase, counters::ProjectionFactParityCounters,
    evidence_basis::ProjectionFactParityLaneEvidence, lane::ProjectionFactParityLane,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionFactParityReceipt {
    case: ProjectionFactParityCase,
    parity_digest: String,
    workload_basis_identity: String,
    declaration: String,
    lane_evidence: Vec<ProjectionFactParityLaneEvidence>,
    counters: ProjectionFactParityCounters,
}

impl ProjectionFactParityReceipt {
    pub(crate) fn new(
        case: ProjectionFactParityCase,
        parity_digest: String,
        workload_basis_identity: String,
        declaration: String,
        lane_evidence: Vec<ProjectionFactParityLaneEvidence>,
        counters: ProjectionFactParityCounters,
    ) -> Self {
        Self {
            case,
            parity_digest,
            workload_basis_identity,
            declaration,
            lane_evidence,
            counters,
        }
    }

    pub fn case(&self) -> ProjectionFactParityCase {
        self.case
    }

    pub fn parity_digest(&self) -> &str {
        &self.parity_digest
    }

    pub fn workload_basis_identity(&self) -> &str {
        &self.workload_basis_identity
    }

    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    pub fn lane_evidence(&self) -> &[ProjectionFactParityLaneEvidence] {
        &self.lane_evidence
    }

    pub fn evidence_for_lane(
        &self,
        lane: ProjectionFactParityLane,
    ) -> Option<&ProjectionFactParityLaneEvidence> {
        self.lane_evidence
            .iter()
            .find(|evidence| evidence.lane() == lane)
    }

    pub fn counters(&self) -> ProjectionFactParityCounters {
        self.counters
    }
}
