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
    LegacyLayoutBlockChunkRecord,
    LegacyBasisContinuationDescriptor,
    LegacyBulkRecord,
    LegacyRetentionRebuildRecord,
    LegacyMaintenanceRecord,
    LegacyTieringRecord,
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
            Self::LegacyLayoutBlockChunkRecord => "milestone_6_layout_block_chunk_record",
            Self::LegacyBasisContinuationDescriptor => "milestone_8_basis_continuation_descriptor",
            Self::LegacyBulkRecord => "milestone_9_bulk_record",
            Self::LegacyRetentionRebuildRecord => "milestone_10_retention_rebuild_record",
            Self::LegacyMaintenanceRecord => "milestone_11_maintenance_record",
            Self::LegacyTieringRecord => "milestone_13_tiering_record",
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
            | Self::LegacyLayoutBlockChunkRecord
            | Self::LegacyBasisContinuationDescriptor
            | Self::LegacyBulkRecord
            | Self::LegacyRetentionRebuildRecord
            | Self::LegacyMaintenanceRecord
            | Self::LegacyTieringRecord => CompatibilityAuthorityClassification::Derived,
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
    CompatibilityFamilyKind::LegacyLayoutBlockChunkRecord,
    CompatibilityFamilyKind::LegacyBasisContinuationDescriptor,
    CompatibilityFamilyKind::LegacyBulkRecord,
    CompatibilityFamilyKind::LegacyRetentionRebuildRecord,
    CompatibilityFamilyKind::LegacyMaintenanceRecord,
    CompatibilityFamilyKind::LegacyTieringRecord,
];

pub const FIRST_SHIP_COMPATIBILITY_FAMILY_COUNT: usize = FIRST_SHIP_COMPATIBILITY_FAMILIES.len();
