use crate::identity::hash_parts;
use worth_foundational::facade::AspectKey;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GroupedLaneIdentity {
    grouping_aspect: AspectKey,
    lane_key: String,
}

impl GroupedLaneIdentity {
    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn lane_key(&self) -> &str {
        &self.lane_key
    }

    pub(crate) fn new(grouping_aspect: AspectKey, lane_key: impl Into<String>) -> Self {
        Self {
            grouping_aspect,
            lane_key: lane_key.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGroupedBaselineMember {
    member_key: String,
    lane_key: String,
}

impl WorthQueryGroupedBaselineMember {
    pub fn from_authoritative_member_lane_keys(
        member_key: impl Into<String>,
        lane_key: impl Into<String>,
    ) -> Self {
        Self {
            member_key: member_key.into(),
            lane_key: lane_key.into(),
        }
    }

    pub fn member_key(&self) -> &str {
        &self.member_key
    }

    pub fn lane_key(&self) -> &str {
        &self.lane_key
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedMemberState {
    member_key: String,
    lane: GroupedLaneIdentity,
}

impl GroupedMemberState {
    pub fn member_key(&self) -> &str {
        &self.member_key
    }

    pub fn lane(&self) -> &GroupedLaneIdentity {
        &self.lane
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedViewResultArtifact {
    grouping_aspect: AspectKey,
    lane_identities: Vec<GroupedLaneIdentity>,
    member_states: Vec<GroupedMemberState>,
    row_count: usize,
}

impl GroupedViewResultArtifact {
    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        &self.grouping_aspect
    }

    pub fn lane_identities(&self) -> &[GroupedLaneIdentity] {
        &self.lane_identities
    }

    pub fn member_states(&self) -> &[GroupedMemberState] {
        &self.member_states
    }

    pub fn lane_count(&self) -> usize {
        self.lane_identities.len()
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroupedDesiredStateArtifact {
    digest: String,
    result: GroupedViewResultArtifact,
}

impl GroupedDesiredStateArtifact {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn result(&self) -> &GroupedViewResultArtifact {
        &self.result
    }

    pub fn native_grouping_aspect_key(&self) -> &AspectKey {
        self.result.native_grouping_aspect_key()
    }
}

pub(crate) fn desired_state_from_members(
    grouping_aspect: AspectKey,
    mut members: Vec<WorthQueryGroupedBaselineMember>,
) -> GroupedDesiredStateArtifact {
    members.sort_by(|left, right| {
        left.member_key
            .cmp(&right.member_key)
            .then_with(|| left.lane_key.cmp(&right.lane_key))
    });
    let member_states = members
        .iter()
        .map(|member| GroupedMemberState {
            member_key: member.member_key.clone(),
            lane: GroupedLaneIdentity::new(grouping_aspect.clone(), member.lane_key.clone()),
        })
        .collect::<Vec<_>>();
    let mut lane_identities = member_states
        .iter()
        .map(|member| member.lane.clone())
        .collect::<Vec<_>>();
    lane_identities.sort();
    lane_identities.dedup();
    let row_count = member_states.len();
    let result = GroupedViewResultArtifact {
        grouping_aspect: grouping_aspect.clone(),
        lane_identities,
        member_states,
        row_count,
    };
    let digest = hash_parts(&[
        format!("grouping:{}", grouping_aspect.as_str()),
        format!("row_count:{row_count}"),
        format!(
            "members:{}",
            result
                .member_states()
                .iter()
                .map(|member| format!("{}@{}", member.member_key(), member.lane().lane_key()))
                .collect::<Vec<_>>()
                .join(",")
        ),
    ]);
    GroupedDesiredStateArtifact { digest, result }
}
