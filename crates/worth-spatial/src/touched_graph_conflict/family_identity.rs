use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SpatialConflictFamilyIdentity {
    EvidenceSelection,
    ReplayBoundarySelection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpatialConflictFamilyIdentityAuthority {
    identity: SpatialConflictFamilyIdentity,
}

impl SpatialConflictFamilyIdentityAuthority {
    pub fn evidence_selection() -> Self {
        Self {
            identity: SpatialConflictFamilyIdentity::EvidenceSelection,
        }
    }

    pub fn replay_boundary_selection() -> Self {
        Self {
            identity: SpatialConflictFamilyIdentity::ReplayBoundarySelection,
        }
    }

    const fn identity(self) -> SpatialConflictFamilyIdentity {
        self.identity
    }
}

impl SpatialConflictFamilyIdentity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceSelection => "spatial-conflict.evidence-selection.v1",
            Self::ReplayBoundarySelection => "spatial-conflict.replay-boundary-selection.v1",
        }
    }

    pub fn digest(self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:conflict-family-identity:v1".to_string(),
                self.as_str().to_string(),
            ],
        )
    }
}

pub fn admit_spatial_conflict_family_identity(
    authority: SpatialConflictFamilyIdentityAuthority,
) -> SpatialConflictFamilyIdentity {
    authority.identity()
}
