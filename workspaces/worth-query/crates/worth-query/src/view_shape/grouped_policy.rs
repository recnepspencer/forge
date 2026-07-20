use crate::planning::FallbackDisposition;

use super::grouped_planning::GroupedViewPlanningArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KanbanGroupedLiveContract {
    DeltaBound,
}

impl KanbanGroupedLiveContract {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeltaBound => "kanban_grouped_delta_bound",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedReplayDeliveryPosture {
    grouped_delivery: bool,
    replay_commits_grouping_contract: bool,
}

impl GroupedReplayDeliveryPosture {
    pub fn grouped_delivery(&self) -> bool {
        self.grouped_delivery
    }

    pub fn replay_commits_grouping_contract(&self) -> bool {
        self.replay_commits_grouping_contract
    }

    pub(crate) fn grouped_committed() -> Self {
        Self {
            grouped_delivery: true,
            replay_commits_grouping_contract: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedDeltaAdmissionPolicy {
    contract: KanbanGroupedLiveContract,
    max_member_transitions: usize,
    max_lane_reassignments: usize,
}

impl GroupedDeltaAdmissionPolicy {
    pub(crate) fn admitted_grouped_delta() -> Self {
        Self {
            contract: KanbanGroupedLiveContract::DeltaBound,
            max_member_transitions: usize::MAX,
            max_lane_reassignments: usize::MAX,
        }
    }

    pub fn contract(&self) -> &KanbanGroupedLiveContract {
        &self.contract
    }

    pub fn max_member_transitions(&self) -> usize {
        self.max_member_transitions
    }

    pub fn max_lane_reassignments(&self) -> usize {
        self.max_lane_reassignments
    }

    pub(crate) fn derive_from_grouped_planning(
        grouped_planning: &GroupedViewPlanningArtifact,
    ) -> Self {
        let _ = grouped_planning;
        let _ = FallbackDisposition::AdmittedAndSelected;
        Self::admitted_grouped_delta()
    }
}
