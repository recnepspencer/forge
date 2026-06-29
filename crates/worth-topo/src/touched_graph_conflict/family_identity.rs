use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TopologyConflictFamilyIdentity {
    AspectSelection,
    ValidatorSelection,
    ReplayBoundarySelection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyConflictFamilyIdentityAuthority {
    identity: TopologyConflictFamilyIdentity,
}

impl TopologyConflictFamilyIdentityAuthority {
    pub fn aspect_selection() -> Self {
        Self {
            identity: TopologyConflictFamilyIdentity::AspectSelection,
        }
    }

    pub fn validator_selection() -> Self {
        Self {
            identity: TopologyConflictFamilyIdentity::ValidatorSelection,
        }
    }

    pub fn replay_boundary_selection() -> Self {
        Self {
            identity: TopologyConflictFamilyIdentity::ReplayBoundarySelection,
        }
    }

    const fn identity(self) -> TopologyConflictFamilyIdentity {
        self.identity
    }
}

impl TopologyConflictFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AspectSelection => "topology-conflict.aspect-selection.v1",
            Self::ValidatorSelection => "topology-conflict.validator-selection.v1",
            Self::ReplayBoundarySelection => "topology-conflict.replay-boundary-selection.v1",
        }
    }

    pub fn digest(self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:conflict-family-identity:v1".to_string(),
                self.as_str().to_string(),
            ],
        )
    }
}

pub fn admit_topology_conflict_family_identity(
    authority: TopologyConflictFamilyIdentityAuthority,
) -> TopologyConflictFamilyIdentity {
    authority.identity()
}
