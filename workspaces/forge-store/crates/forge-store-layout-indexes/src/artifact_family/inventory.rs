use super::{inventory_rows::ROWS, PhysicalArtifactFamilyDeclaration};
use crate::ArtifactFamilyDenial;
use forge_store_contracts::{
    CompatibilityFamilyKind, DerivedFamilyRetentionPolicy, DurableArtifactFamilyId,
    LayoutFamilyCompactionUnit, MaintenanceArtifactFamily, PlacementArtifactFamily,
    PublicationFamily, SupportArtifactFamily, WalRecordFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactFamilyInventoryRow {
    declaration: PhysicalArtifactFamilyDeclaration,
}

impl ArtifactFamilyInventoryRow {
    pub(super) const fn new(declaration: PhysicalArtifactFamilyDeclaration) -> Self {
        Self { declaration }
    }

    pub const fn declaration(&self) -> &PhysicalArtifactFamilyDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8ArtifactFamilyInventory;

pub trait ExistingArtifactFamilySurface: private::Sealed {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId;
}

mod private {
    pub trait Sealed {}
}

impl S8ArtifactFamilyInventory {
    pub const fn current() -> Self {
        Self
    }

    pub const fn rows(&self) -> &'static [ArtifactFamilyInventoryRow] {
        ROWS
    }

    pub fn declaration(
        &self,
        family_id: DurableArtifactFamilyId,
    ) -> Result<&'static PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
        declaration_in_rows(ROWS, family_id)
    }

    pub fn admit_existing_family(
        &self,
        family: &impl ExistingArtifactFamilySurface,
    ) -> Result<&'static PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
        self.declaration(family.canonical_family_id())
    }
}

pub(crate) fn declaration_in_rows<'a>(
    rows: &'a [ArtifactFamilyInventoryRow],
    family_id: DurableArtifactFamilyId,
) -> Result<&'a PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
    let mut index = 0;
    while index < rows.len() {
        let row = &rows[index];
        if row.declaration().family_id() == family_id {
            return Ok(row.declaration());
        }
        index += 1;
    }
    Err(ArtifactFamilyDenial::MissingFamilyDeclaration)
}

impl private::Sealed for WalRecordFamily {}
impl ExistingArtifactFamilySurface for WalRecordFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::DurableMutationIntent => DurableArtifactFamilyId::WalDurableMutationIntent,
            Self::HostedRuntimeCommitResult => {
                DurableArtifactFamilyId::WalHostedRuntimeCommitResult
            }
            Self::BulkCheckpointPublicationIntent => {
                DurableArtifactFamilyId::WalBulkCheckpointPublicationIntent
            }
            Self::DurablePublicationProgress => {
                DurableArtifactFamilyId::WalDurablePublicationProgress
            }
            Self::RecoveryDecision => DurableArtifactFamilyId::WalRecoveryDecision,
        }
    }
}

impl private::Sealed for CompatibilityFamilyKind {}
impl ExistingArtifactFamilySurface for CompatibilityFamilyKind {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::CommitEnvelope => DurableArtifactFamilyId::CompatibilityCommitEnvelope,
            Self::BranchVersionDagRecord => {
                DurableArtifactFamilyId::CompatibilityBranchVersionDagRecord
            }

            Self::WalRestartRecord => DurableArtifactFamilyId::CompatibilityWalRestartRecord,
            Self::SchemaLineageCursorCheckpointSupport => {
                DurableArtifactFamilyId::CompatibilitySchemaLineageCursorCheckpointSupport
            }
            Self::EmbeddedCheckpointAuthority => {
                DurableArtifactFamilyId::CompatibilityEmbeddedCheckpointAuthority
            }
            Self::SnapshotRecord => DurableArtifactFamilyId::CompatibilitySnapshotRecord,
            Self::DeltaRecord => DurableArtifactFamilyId::CompatibilityDeltaRecord,
            Self::Milestone6LayoutBlockChunkRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone6LayoutBlockChunkRecord
            }
            Self::Milestone8BasisContinuationDescriptor => {
                DurableArtifactFamilyId::CompatibilityMilestone8BasisContinuationDescriptor
            }
            Self::Milestone9BulkRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone9BulkRecord
            }
            Self::Milestone10RetentionRebuildRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone10RetentionRebuildRecord
            }
            Self::Milestone11MaintenanceRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone11MaintenanceRecord
            }
            Self::Milestone13TieringRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord
            }
        }
    }
}

impl private::Sealed for MaintenanceArtifactFamily {}
impl ExistingArtifactFamilySurface for MaintenanceArtifactFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::Snapshot => DurableArtifactFamilyId::MaintenanceSnapshot,
            Self::Compaction => DurableArtifactFamilyId::MaintenanceCompaction,
            Self::Reclaim => DurableArtifactFamilyId::MaintenanceReclaim,
            Self::Capsule => DurableArtifactFamilyId::MaintenanceCapsule,
        }
    }
}

impl private::Sealed for SupportArtifactFamily {}
impl ExistingArtifactFamilySurface for SupportArtifactFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::SchemaSupport => DurableArtifactFamilyId::SupportSchema,
            Self::LineageSupport => DurableArtifactFamilyId::SupportLineage,
            Self::CursorSupport => DurableArtifactFamilyId::SupportCursor,
            Self::EmbeddedCheckpoint => DurableArtifactFamilyId::SupportEmbeddedCheckpoint,
        }
    }
}

impl private::Sealed for PlacementArtifactFamily {}
impl ExistingArtifactFamilySurface for PlacementArtifactFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::AuthoritativeBranchHead => {
                DurableArtifactFamilyId::PlacementAuthoritativeBranchHead
            }
            Self::RetainedAuthority => DurableArtifactFamilyId::PlacementRetainedAuthority,
            Self::StableBasis => DurableArtifactFamilyId::PlacementStableBasis,
            Self::SnapshotFamily => DurableArtifactFamilyId::PlacementSnapshotFamily,
            Self::BranchDeltaFamily => DurableArtifactFamilyId::PlacementBranchDeltaFamily,
            Self::Milestone6LayoutFamily => {
                DurableArtifactFamilyId::PlacementMilestone6LayoutFamily
            }
        }
    }
}

impl private::Sealed for PublicationFamily {}
impl ExistingArtifactFamilySurface for PublicationFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::WalIntent => DurableArtifactFamilyId::PublicationWalIntent,
            Self::WalCanonicalResult => DurableArtifactFamilyId::PublicationWalCanonicalResult,
            Self::WalPublicationProgress => {
                DurableArtifactFamilyId::PublicationWalPublicationProgress
            }
            Self::AuthoritativeCommitAppendUnit => {
                DurableArtifactFamilyId::PublicationAuthoritativeCommitAppendUnit
            }
            Self::BranchHeadPublication => {
                DurableArtifactFamilyId::PublicationBranchHeadPublication
            }
            Self::AcknowledgmentEligibility => {
                DurableArtifactFamilyId::PublicationAcknowledgmentEligibility
            }
            Self::SnapshotBasis => DurableArtifactFamilyId::PublicationSnapshotBasis,
            Self::SnapshotImage => DurableArtifactFamilyId::PublicationSnapshotImage,
        }
    }
}

impl private::Sealed for DerivedFamilyRetentionPolicy {}
impl ExistingArtifactFamilySurface for DerivedFamilyRetentionPolicy {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::Milestone6LayoutMaterialization => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization
            }
            Self::Milestone6ScopeSliceMembership => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6ScopeSliceMembership
            }
            Self::Milestone6StructuralBlock => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6StructuralBlock
            }
            Self::Milestone6ChunkMembership => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership
            }
        }
    }
}

impl private::Sealed for LayoutFamilyCompactionUnit {}
impl ExistingArtifactFamilySurface for LayoutFamilyCompactionUnit {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self.family_kind() {
            forge_store_contracts::LayoutCompactionFamilyKind::LayoutCompactionUnit => {
                DurableArtifactFamilyId::LayoutCompactionUnit
            }
        }
    }
}
