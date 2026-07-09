use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ArtifactFamilyId(String);

impl ArtifactFamilyId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CompatibilityAuthorityClassification {
    Authoritative,
    Derived,
}

impl CompatibilityAuthorityClassification {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Derived => "derived",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CompatibilityFamilyKind {
    CommitEnvelope,
    BranchVersionDagRecord,
    WalRestartRecord,
    SchemaLineageCursorCheckpointSupport,
    EmbeddedCheckpointAuthority,
    SnapshotRecord,
    DeltaRecord,
    Milestone6LayoutBlockChunkRecord,
    Milestone8BasisContinuationDescriptor,
    Milestone9BulkRecord,
    Milestone10RetentionRebuildRecord,
    Milestone11MaintenanceRecord,
    Milestone13TieringRecord,
}

impl CompatibilityFamilyKind {
    pub const fn label(self) -> &'static str {
        match self {
            Self::CommitEnvelope => "commit_envelope",
            Self::BranchVersionDagRecord => "branch_version_dag_record",
            Self::WalRestartRecord => "wal_restart_record",
            Self::SchemaLineageCursorCheckpointSupport => {
                "schema_lineage_cursor_checkpoint_support"
            }
            Self::EmbeddedCheckpointAuthority => "embedded_checkpoint_authority",
            Self::SnapshotRecord => "snapshot_record",
            Self::DeltaRecord => "delta_record",
            Self::Milestone6LayoutBlockChunkRecord => "milestone_6_layout_block_chunk_record",
            Self::Milestone8BasisContinuationDescriptor => {
                "milestone_8_basis_continuation_descriptor"
            }
            Self::Milestone9BulkRecord => "milestone_9_bulk_record",
            Self::Milestone10RetentionRebuildRecord => "milestone_10_retention_rebuild_record",
            Self::Milestone11MaintenanceRecord => "milestone_11_maintenance_record",
            Self::Milestone13TieringRecord => "milestone_13_tiering_record",
        }
    }

    pub fn family_id(self) -> ArtifactFamilyId {
        ArtifactFamilyId::new(self.label())
    }

    pub const fn authority_classification(self) -> CompatibilityAuthorityClassification {
        match self {
            Self::CommitEnvelope
            | Self::BranchVersionDagRecord
            | Self::WalRestartRecord
            | Self::SchemaLineageCursorCheckpointSupport
            | Self::EmbeddedCheckpointAuthority => {
                CompatibilityAuthorityClassification::Authoritative
            }
            Self::SnapshotRecord
            | Self::DeltaRecord
            | Self::Milestone6LayoutBlockChunkRecord
            | Self::Milestone8BasisContinuationDescriptor
            | Self::Milestone9BulkRecord
            | Self::Milestone10RetentionRebuildRecord
            | Self::Milestone11MaintenanceRecord
            | Self::Milestone13TieringRecord => CompatibilityAuthorityClassification::Derived,
        }
    }
}

pub const FIRST_SHIP_COMPATIBILITY_FAMILIES: [CompatibilityFamilyKind; 13] = [
    CompatibilityFamilyKind::CommitEnvelope,
    CompatibilityFamilyKind::BranchVersionDagRecord,
    CompatibilityFamilyKind::WalRestartRecord,
    CompatibilityFamilyKind::SchemaLineageCursorCheckpointSupport,
    CompatibilityFamilyKind::EmbeddedCheckpointAuthority,
    CompatibilityFamilyKind::SnapshotRecord,
    CompatibilityFamilyKind::DeltaRecord,
    CompatibilityFamilyKind::Milestone6LayoutBlockChunkRecord,
    CompatibilityFamilyKind::Milestone8BasisContinuationDescriptor,
    CompatibilityFamilyKind::Milestone9BulkRecord,
    CompatibilityFamilyKind::Milestone10RetentionRebuildRecord,
    CompatibilityFamilyKind::Milestone11MaintenanceRecord,
    CompatibilityFamilyKind::Milestone13TieringRecord,
];

pub const FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT: usize = FIRST_SHIP_COMPATIBILITY_FAMILIES.len();
