use forge_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityKind, RelationalBridgeRecordIdentityParts,
    RelationalBridgeSnapshotIdentityParts, TruthCommitIdentity, TruthSnapshotIdentity,
};

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryCommitIdentity {
    Absent,
    RelationalBridge {
        bridge_identity: TruthCommitIdentity,
    },
    Preview {
        evidence_identity: ForgeQueryEvidenceIdentity,
    },
}

impl ForgeQueryCommitIdentity {
    pub fn from_relational_commit_id(commit_id: u64) -> Self {
        Self::RelationalBridge {
            bridge_identity: TruthCommitIdentity::from_relational_commit_id(commit_id),
        }
    }

    pub fn preview(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self::Preview { evidence_identity }
    }

    pub fn from_external_authority_label(label: impl AsRef<str>) -> Self {
        Self::Preview {
            evidence_identity: ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
            )
            .field_identity(ForgeQueryEvidenceTag::new("external_commit_label"), label)
            .seal(),
        }
    }

    pub fn absent() -> Self {
        Self::Absent
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Absent)
    }

    pub fn bridge_identity(&self) -> Option<&TruthCommitIdentity> {
        match self {
            Self::Absent => None,
            Self::RelationalBridge { bridge_identity } => Some(bridge_identity),
            Self::Preview { .. } => None,
        }
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::Absent => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
            )
            .field_shape(ForgeQueryEvidenceTag::new("commit_state"), "absent")
            .seal(),
            Self::RelationalBridge { bridge_identity } => {
                let Some(commit_id) = bridge_identity.relational_commit_id() else {
                    panic!("forge-query commit identity must retain relational commit payload");
                };
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::WriteReceiptCommitIdentity,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("relational_commit_id"),
                    commit_id as usize,
                )
                .seal()
            }
            Self::Preview { evidence_identity } => evidence_identity.clone(),
        }
    }
}

impl std::fmt::Display for ForgeQueryCommitIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.evidence_identity().as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQuerySnapshotIdentity {
    EmptyRelationalState,
    RelationalBridge {
        bridge_identity: TruthSnapshotIdentity,
        parts: RelationalBridgeSnapshotIdentityParts,
    },
    Preview {
        evidence_identity: ForgeQueryEvidenceIdentity,
    },
}

impl ForgeQuerySnapshotIdentity {
    pub fn empty_relational_state() -> Self {
        Self::EmptyRelationalState
    }

    pub fn from_relational_snapshot(parts: RelationalBridgeSnapshotIdentityParts) -> Self {
        Self::RelationalBridge {
            bridge_identity: TruthSnapshotIdentity::from_relational_snapshot(parts),
            parts,
        }
    }

    pub fn preview(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self::Preview { evidence_identity }
    }

    pub fn from_external_authority_label(label: impl AsRef<str>) -> Self {
        Self::Preview {
            evidence_identity: ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
            )
            .field_identity(ForgeQueryEvidenceTag::new("external_snapshot_label"), label)
            .seal(),
        }
    }

    pub fn bridge_identity(&self) -> Option<&TruthSnapshotIdentity> {
        match self {
            Self::EmptyRelationalState => None,
            Self::RelationalBridge {
                bridge_identity, ..
            } => Some(bridge_identity),
            Self::Preview { .. } => None,
        }
    }

    pub fn relational_parts(&self) -> Option<RelationalBridgeSnapshotIdentityParts> {
        match self {
            Self::EmptyRelationalState => None,
            Self::RelationalBridge { parts, .. } => Some(*parts),
            Self::Preview { .. } => None,
        }
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::EmptyRelationalState => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("snapshot_state"),
                "empty-relational",
            )
            .seal(),
            Self::RelationalBridge { parts, .. } => ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::WriteReceiptSnapshotIdentity,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("relational_snapshot_id"),
                parts.snapshot_id() as usize,
            )
            .field_usize(
                ForgeQueryEvidenceTag::new("relational_version_id"),
                parts.version_id() as usize,
            )
            .seal(),
            Self::Preview { evidence_identity } => evidence_identity.clone(),
        }
    }
}

impl std::fmt::Display for ForgeQuerySnapshotIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.evidence_identity().as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForgeQueryEntityIdentity {
    RelationalRecord {
        relational_record: RelationalBridgeRecordIdentityParts,
    },
    Preview {
        evidence_identity: ForgeQueryEvidenceIdentity,
    },
}

impl ForgeQueryEntityIdentity {
    pub fn from_relational_record(relational_record: RelationalBridgeRecordIdentityParts) -> Self {
        Self::RelationalRecord { relational_record }
    }

    pub fn preview(evidence_identity: ForgeQueryEvidenceIdentity) -> Self {
        Self::Preview { evidence_identity }
    }

    pub fn authored_command(identity: impl AsRef<str>) -> Self {
        Self::Preview {
            evidence_identity: ForgeQueryEvidenceIdentity::compose(
                ForgeQueryEvidenceScope::AuthoredCommandEntityIdentity,
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("authored_entity_identity"),
                identity,
            )
            .seal(),
        }
    }

    pub fn relational_record_parts(&self) -> Option<RelationalBridgeRecordIdentityParts> {
        match self {
            Self::RelationalRecord { relational_record } => Some(*relational_record),
            Self::Preview { .. } => None,
        }
    }

    pub fn evidence_identity(&self) -> ForgeQueryEvidenceIdentity {
        match self {
            Self::RelationalRecord { relational_record } => {
                let kind = match relational_record.kind() {
                    RelationalBridgeRecordIdentityKind::Entity => "entity",
                    RelationalBridgeRecordIdentityKind::Relation => "relation",
                };
                ForgeQueryEvidenceIdentity::compose(
                    ForgeQueryEvidenceScope::WriteReceiptEntityIdentity,
                )
                .field_shape(ForgeQueryEvidenceTag::new("record_kind"), kind)
                .field_usize(
                    ForgeQueryEvidenceTag::new("partition_id"),
                    relational_record.partition_id() as usize,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("local_slot"),
                    relational_record.local_slot() as usize,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("generation"),
                    relational_record.generation() as usize,
                )
                .seal()
            }
            Self::Preview { evidence_identity } => evidence_identity.clone(),
        }
    }
}

impl std::fmt::Display for ForgeQueryEntityIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.evidence_identity().as_str())
    }
}

impl PartialOrd for ForgeQueryEntityIdentity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ForgeQueryEntityIdentity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evidence_identity()
            .as_str()
            .cmp(other.evidence_identity().as_str())
    }
}

impl Hash for ForgeQueryEntityIdentity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.evidence_identity().as_str().hash(state);
    }
}
