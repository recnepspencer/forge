use crate::planning::FallbackDisposition;

use super::grouped_planning::GroupedViewPlanningArtifact;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum KanbanGroupedLiveContract {
    DeltaBound,
    RefreshDeferredDebt,
}

impl KanbanGroupedLiveContract {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeltaBound => "kanban_grouped_delta_bound",
            Self::RefreshDeferredDebt => "kanban_grouped_refresh_deferred_debt",
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
        let grouped_delta_bound_admitted = grouped_planning.grouped_binding_width() <= 3
            && grouped_planning.grouped_projection_width() <= 3
            && grouped_planning.traversal_count() == 0
            && grouped_planning.ordering_count() <= 1
            && grouped_planning.predicate_count() <= 1
            && !matches!(
                grouped_planning.fallback(),
                FallbackDisposition::AdmittedAndSelected
            );

        if grouped_delta_bound_admitted {
            Self {
                contract: KanbanGroupedLiveContract::DeltaBound,
                max_member_transitions: 1,
                max_lane_reassignments: 1,
            }
        } else {
            Self {
                contract: KanbanGroupedLiveContract::RefreshDeferredDebt,
                max_member_transitions: 0,
                max_lane_reassignments: 0,
            }
        }
    }

    pub(crate) fn refresh_deferred_debt() -> Self {
        Self {
            contract: KanbanGroupedLiveContract::RefreshDeferredDebt,
            max_member_transitions: 0,
            max_lane_reassignments: 0,
        }
    }
}
