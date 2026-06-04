use std::collections::BTreeMap;

use crate::identity::hash_parts;
use crate::view_shape::{GroupedDeltaAdmissionPolicy, KanbanGroupedLiveContract};

use super::grouped_state::{GroupedDesiredStateArtifact, GroupedLaneIdentity};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupedRefreshReason {
    PlanContractRequiresRefresh,
    CoreRefreshRequested,
    PolicyExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupedMembershipTransitionKind {
    EnteredLane,
    LeftLane,
    MovedLane,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedMembershipTransition {
    member_key: String,
    kind: GroupedMembershipTransitionKind,
    from_lane: Option<GroupedLaneIdentity>,
    to_lane: Option<GroupedLaneIdentity>,
}

impl GroupedMembershipTransition {
    pub fn member_key(&self) -> &str {
        &self.member_key
    }

    pub fn kind(&self) -> &GroupedMembershipTransitionKind {
        &self.kind
    }

    pub fn from_lane(&self) -> Option<&GroupedLaneIdentity> {
        self.from_lane.as_ref()
    }

    pub fn to_lane(&self) -> Option<&GroupedLaneIdentity> {
        self.to_lane.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedDeltaArtifact {
    digest: String,
    prior: GroupedDesiredStateArtifact,
    next: GroupedDesiredStateArtifact,
    transitions: Vec<GroupedMembershipTransition>,
    contract: KanbanGroupedLiveContract,
}

impl GroupedDeltaArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn prior(&self) -> &GroupedDesiredStateArtifact {
        &self.prior
    }

    pub fn next(&self) -> &GroupedDesiredStateArtifact {
        &self.next
    }

    pub fn transitions(&self) -> &[GroupedMembershipTransition] {
        &self.transitions
    }

    pub fn contract(&self) -> &KanbanGroupedLiveContract {
        &self.contract
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GroupedDeltaComputation {
    DeltaBound {
        delta: GroupedDeltaArtifact,
        next_state: GroupedDesiredStateArtifact,
    },
    RefreshDeferredDebt {
        reason: GroupedRefreshReason,
        prior_state: GroupedDesiredStateArtifact,
    },
}

pub(crate) fn build_grouped_delta(
    prior_state: &GroupedDesiredStateArtifact,
    next_state: &GroupedDesiredStateArtifact,
    policy: &GroupedDeltaAdmissionPolicy,
) -> GroupedDeltaComputation {
    if policy.contract() == &KanbanGroupedLiveContract::RefreshDeferredDebt {
        return GroupedDeltaComputation::RefreshDeferredDebt {
            reason: GroupedRefreshReason::PlanContractRequiresRefresh,
            prior_state: prior_state.clone(),
        };
    }
    if prior_state.native_grouping_aspect_key() != next_state.native_grouping_aspect_key() {
        return GroupedDeltaComputation::RefreshDeferredDebt {
            reason: GroupedRefreshReason::PolicyExceeded,
            prior_state: prior_state.clone(),
        };
    }

    let prior_members = prior_state
        .result()
        .member_states()
        .iter()
        .map(|member| (member.member_key(), member.lane().clone()))
        .collect::<BTreeMap<_, _>>();
    let next_members = next_state
        .result()
        .member_states()
        .iter()
        .map(|member| (member.member_key(), member.lane().clone()))
        .collect::<BTreeMap<_, _>>();

    let mut all_members = prior_members
        .keys()
        .chain(next_members.keys())
        .copied()
        .collect::<Vec<_>>();
    all_members.sort_unstable();
    all_members.dedup();

    let mut transitions = Vec::new();
    for member_key in all_members {
        let prior_lane = prior_members.get(member_key).cloned();
        let next_lane = next_members.get(member_key).cloned();
        let kind = match (&prior_lane, &next_lane) {
            (None, Some(_)) => Some(GroupedMembershipTransitionKind::EnteredLane),
            (Some(_), None) => Some(GroupedMembershipTransitionKind::LeftLane),
            (Some(from), Some(to)) if from != to => {
                Some(GroupedMembershipTransitionKind::MovedLane)
            }
            _ => None,
        };
        if let Some(kind) = kind {
            transitions.push(GroupedMembershipTransition {
                member_key: member_key.to_string(),
                kind,
                from_lane: prior_lane,
                to_lane: next_lane,
            });
        }
    }

    let lane_reassignments = transitions
        .iter()
        .filter(|transition| {
            matches!(
                transition.kind(),
                GroupedMembershipTransitionKind::MovedLane
            )
        })
        .count();
    let member_transitions = transitions.len();
    if member_transitions > policy.max_member_transitions()
        || lane_reassignments > policy.max_lane_reassignments()
    {
        return GroupedDeltaComputation::RefreshDeferredDebt {
            reason: GroupedRefreshReason::PolicyExceeded,
            prior_state: prior_state.clone(),
        };
    }

    let delta = GroupedDeltaArtifact {
        digest: hash_parts(&[
            format!("prior:{}", prior_state.digest()),
            format!("next:{}", next_state.digest()),
            format!("transition_count:{}", transitions.len()),
            format!(
                "contract:{}",
                KanbanGroupedLiveContract::DeltaBound.as_str()
            ),
        ]),
        prior: prior_state.clone(),
        next: next_state.clone(),
        transitions,
        contract: KanbanGroupedLiveContract::DeltaBound,
    };

    GroupedDeltaComputation::DeltaBound {
        delta,
        next_state: next_state.clone(),
    }
}
