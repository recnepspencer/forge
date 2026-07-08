use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WalRecordFamily {
    DurableMutationIntent,
    HostedRuntimeCommitResult,
    BulkCheckpointPublicationIntent,
    DurablePublicationProgress,
    RecoveryDecision,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceArtifactFamily {
    Snapshot,
    Compaction,
    Reclaim,
    Capsule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SupportArtifactFamily {
    SchemaSupport,
    LineageSupport,
    CursorSupport,
    EmbeddedCheckpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementArtifactFamily {
    AuthoritativeBranchHead,
    RetainedAuthority,
    StableBasis,
    SnapshotFamily,
    BranchDeltaFamily,
    Milestone6LayoutFamily,
}

impl PlacementArtifactFamily {
    pub const fn label(self) -> &'static str {
        match self {
            Self::AuthoritativeBranchHead => "authoritative_branch_head",
            Self::RetainedAuthority => "retained_authority",
            Self::StableBasis => "stable_basis",
            Self::SnapshotFamily => "snapshot_family",
            Self::BranchDeltaFamily => "branch_delta_family",
            Self::Milestone6LayoutFamily => "milestone6_layout_family",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "authoritative_branch_head" => Some(Self::AuthoritativeBranchHead),
            "retained_authority" => Some(Self::RetainedAuthority),
            "stable_basis" => Some(Self::StableBasis),
            "snapshot_family" => Some(Self::SnapshotFamily),
            "branch_delta_family" => Some(Self::BranchDeltaFamily),
            "milestone6_layout_family" => Some(Self::Milestone6LayoutFamily),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicationFamily {
    WalIntent,
    WalCanonicalResult,
    WalPublicationProgress,
    AuthoritativeCommitAppendUnit,
    BranchHeadPublication,
    AcknowledgmentEligibility,
    SnapshotBasis,
    SnapshotImage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DerivedFamilyRetentionPolicy {
    Milestone6LayoutMaterialization,
    Milestone6ScopeSliceMembership,
    Milestone6StructuralBlock,
    Milestone6ChunkMembership,
}

impl DerivedFamilyRetentionPolicy {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Milestone6LayoutMaterialization => "milestone_6_layout_materialization",
            Self::Milestone6ScopeSliceMembership => "milestone_6_scope_slice_membership",
            Self::Milestone6StructuralBlock => "milestone_6_structural_block",
            Self::Milestone6ChunkMembership => "milestone_6_chunk_membership",
        }
    }

    pub const fn requires_retained_basis_survival(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutCompactionFamilyKind {
    LayoutCompactionUnit,
}

impl LayoutCompactionFamilyKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LayoutCompactionUnit => "layout_compaction_unit",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutFamilyCompactionUnit {
    retained_basis_label: String,
    family_kind: LayoutCompactionFamilyKind,
    artifact_id: String,
}

impl LayoutFamilyCompactionUnit {
    pub fn new(
        retained_basis_label: impl Into<String>,
        family_kind: LayoutCompactionFamilyKind,
        artifact_id: impl Into<String>,
    ) -> Self {
        Self {
            retained_basis_label: retained_basis_label.into(),
            family_kind,
            artifact_id: artifact_id.into(),
        }
    }

    pub fn retained_basis_label(&self) -> &str {
        &self.retained_basis_label
    }

    pub fn family_label(&self) -> &str {
        self.family_kind.label()
    }

    pub fn family_kind(&self) -> LayoutCompactionFamilyKind {
        self.family_kind
    }

    pub fn artifact_id(&self) -> &str {
        &self.artifact_id
    }
}
