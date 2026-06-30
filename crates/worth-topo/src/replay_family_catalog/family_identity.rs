#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TopologyReplayFamilyIdentity {
    TraversalViewsReplay,
    MaterializedGraphReplay,
}

impl TopologyReplayFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TraversalViewsReplay => "traversal-views-replay",
            Self::MaterializedGraphReplay => "materialized-graph-replay",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyReplayFamilyIdentityAuthority {
    identity: TopologyReplayFamilyIdentity,
}

impl TopologyReplayFamilyIdentityAuthority {
    pub fn traversal_views() -> Self {
        Self {
            identity: TopologyReplayFamilyIdentity::TraversalViewsReplay,
        }
    }

    pub fn materialized_graph() -> Self {
        Self {
            identity: TopologyReplayFamilyIdentity::MaterializedGraphReplay,
        }
    }

    pub const fn identity(&self) -> TopologyReplayFamilyIdentity {
        self.identity
    }
}

pub fn admit_topology_replay_family_identity(
    authority: TopologyReplayFamilyIdentityAuthority,
) -> TopologyReplayFamilyIdentity {
    authority.identity()
}
