use serde::Serialize;

use super::lane_kinds::Milestone12CertificationLaneStatus;
use super::outcomes::Milestone12CertificationLaneOutcome;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationRunSummary {
    accepted_lane_count: u64,
    rejected_lane_count: u64,
    invalidated_lane_count: u64,
    rebuild_required_lane_count: u64,
    evidence_only_lane_count: u64,
}

impl Milestone12CertificationRunSummary {
    pub fn from_outcomes(outcomes: &[Milestone12CertificationLaneOutcome]) -> Self {
        let mut summary = Self {
            accepted_lane_count: 0,
            rejected_lane_count: 0,
            invalidated_lane_count: 0,
            rebuild_required_lane_count: 0,
            evidence_only_lane_count: 0,
        };
        for outcome in outcomes {
            match outcome.status() {
                Milestone12CertificationLaneStatus::Accepted => {
                    summary.accepted_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::Rejected => {
                    summary.rejected_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::Invalidated => {
                    summary.invalidated_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::RebuildRequired => {
                    summary.rebuild_required_lane_count += 1;
                }
                Milestone12CertificationLaneStatus::EvidenceOnly => {
                    summary.evidence_only_lane_count += 1;
                }
            }
        }
        summary
    }

    pub fn accepted_lane_count(&self) -> u64 {
        self.accepted_lane_count
    }

    pub fn rejected_lane_count(&self) -> u64 {
        self.rejected_lane_count
    }
}
