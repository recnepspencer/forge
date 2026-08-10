use std::collections::BTreeSet;

use serde::Serialize;

use super::lane_kinds::{
    Milestone12CertificationLaneId, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneRejection, Milestone12CertificationLaneStatus,
};
use super::outcomes::Milestone12CertificationLaneOutcome;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CompatibilityMatrixEntry {
    lane_id: Milestone12CertificationLaneId,
    lane_kind: Milestone12CertificationLaneKind,
    status: Milestone12CertificationLaneStatus,
}

impl Milestone12CompatibilityMatrixEntry {
    fn from_outcome(outcome: &Milestone12CertificationLaneOutcome) -> Self {
        Self {
            lane_id: outcome.lane_id().clone(),
            lane_kind: outcome.lane_kind(),
            status: outcome.status(),
        }
    }

    pub fn lane_id(&self) -> &Milestone12CertificationLaneId {
        &self.lane_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CompatibilityMatrixStatus {
    Complete,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CompatibilityMatrix {
    entries: Vec<Milestone12CompatibilityMatrixEntry>,
    status: Milestone12CompatibilityMatrixStatus,
}

impl Milestone12CompatibilityMatrix {
    pub fn from_lane_outcomes(
        outcomes: &[Milestone12CertificationLaneOutcome],
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        let mut seen = BTreeSet::new();
        for outcome in outcomes {
            if !seen.insert(outcome.lane_id().clone()) {
                return Err(Milestone12CertificationLaneRejection::DuplicateLane);
            }
        }
        for kind in Milestone12CertificationLaneKind::mandatory_phase_5a() {
            if !seen.contains(&kind.lane_id()) {
                return Err(Milestone12CertificationLaneRejection::MissingMandatoryLane);
            }
        }
        let mut entries = outcomes
            .iter()
            .map(Milestone12CompatibilityMatrixEntry::from_outcome)
            .collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.lane_id().clone());
        Ok(Self {
            entries,
            status: Milestone12CompatibilityMatrixStatus::Complete,
        })
    }

    pub fn entries(&self) -> &[Milestone12CompatibilityMatrixEntry] {
        &self.entries
    }

    pub fn status(&self) -> Milestone12CompatibilityMatrixStatus {
        self.status
    }
}
