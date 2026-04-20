use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GroupedLaneIdentity {
    grouping_aspect: String,
    lane_key: String,
}

impl GroupedLaneIdentity {
    pub fn grouping_aspect(&self) -> &str {
        &self.grouping_aspect
    }

    pub fn lane_key(&self) -> &str {
        &self.lane_key
    }

    pub(crate) fn new(grouping_aspect: impl Into<String>, lane_key: impl Into<String>) -> Self {
        Self {
            grouping_aspect: grouping_aspect.into(),
            lane_key: lane_key.into(),
        }
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
    grouping_aspect: String,
    lane_identities: Vec<GroupedLaneIdentity>,
    member_states: Vec<GroupedMemberState>,
    row_count: usize,
}

impl GroupedViewResultArtifact {
    pub fn grouping_aspect(&self) -> &str {
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

    pub fn grouping_aspect(&self) -> &str {
        self.result.grouping_aspect()
    }
}

pub(crate) fn desired_state_from_members(
    grouping_aspect: String,
    mut members: Vec<(String, String)>,
) -> GroupedDesiredStateArtifact {
    members.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));
    let member_states = members
        .iter()
        .map(|(member_key, lane_key)| GroupedMemberState {
            member_key: member_key.clone(),
            lane: GroupedLaneIdentity::new(grouping_aspect.clone(), lane_key.clone()),
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
        format!("grouping:{grouping_aspect}"),
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
