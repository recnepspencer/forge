#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TopologyUndoFamilyIdentity {
    TraversalViewsRollback,
    MaterializedGraphRollback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyUndoFamilyIdentityAuthority {
    identity: TopologyUndoFamilyIdentity,
}

impl TopologyUndoFamilyIdentityAuthority {
    pub fn traversal_views() -> Self {
        Self {
            identity: TopologyUndoFamilyIdentity::TraversalViewsRollback,
        }
    }

    pub fn materialized_graph() -> Self {
        Self {
            identity: TopologyUndoFamilyIdentity::MaterializedGraphRollback,
        }
    }

    pub const fn identity(&self) -> TopologyUndoFamilyIdentity {
        self.identity
    }
}

impl TopologyUndoFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TraversalViewsRollback => "traversal-views-rollback",
            Self::MaterializedGraphRollback => "materialized-graph-rollback",
        }
    }
}

pub fn admit_topology_undo_family_identity(
    authority: TopologyUndoFamilyIdentityAuthority,
) -> TopologyUndoFamilyIdentity {
    authority.identity()
}
