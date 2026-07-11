use super::{
    ArtifactAuthorityRoleWitness, ArtifactFamilyAuthorityDisposition, ArtifactFamilyDenial,
};
use forge_store_contracts::DurableArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedAccuracyClass {
    Exact,
    Conservative,
    Approximate,
    Heuristic,
    Advisory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactDerivedAccuracyWitness {
    role: ArtifactAuthorityRoleWitness,
    accuracy: DerivedAccuracyClass,
}

impl ArtifactDerivedAccuracyWitness {
    pub(crate) const fn new(
        role: ArtifactAuthorityRoleWitness,
        accuracy: DerivedAccuracyClass,
    ) -> Self {
        Self { role, accuracy }
    }

    pub const fn role(self) -> ArtifactAuthorityRoleWitness {
        self.role
    }

    pub const fn family_id(self) -> DurableArtifactFamilyId {
        self.role.family_id()
    }

    pub const fn accuracy(self) -> DerivedAccuracyClass {
        self.accuracy
    }
}

pub(crate) fn declare_derived_accuracy_class(
    role: ArtifactAuthorityRoleWitness,
) -> ArtifactDerivedAccuracyWitness {
    let accuracy = match role.classification().authority() {
        ArtifactFamilyAuthorityDisposition::Authoritative => DerivedAccuracyClass::Exact,
        ArtifactFamilyAuthorityDisposition::Diagnostic
        | ArtifactFamilyAuthorityDisposition::Terminal
        | ArtifactFamilyAuthorityDisposition::Certification => DerivedAccuracyClass::Advisory,
        ArtifactFamilyAuthorityDisposition::Derived => match role.family_id() {
            DurableArtifactFamilyId::ChunkTreeRoot
            | DurableArtifactFamilyId::DedupeIndex
            | DurableArtifactFamilyId::ReachabilityEdge
            | DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization
            | DurableArtifactFamilyId::DerivedRetentionMilestone6ScopeSliceMembership
            | DurableArtifactFamilyId::DerivedRetentionMilestone6StructuralBlock
            | DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership => {
                DerivedAccuracyClass::Exact
            }
            DurableArtifactFamilyId::LayoutCompactionUnit
            | DurableArtifactFamilyId::PlacementStableBasis
            | DurableArtifactFamilyId::PlacementSnapshotFamily
            | DurableArtifactFamilyId::PlacementBranchDeltaFamily
            | DurableArtifactFamilyId::PlacementMilestone6LayoutFamily
            | DurableArtifactFamilyId::SchedulerReservationIndex
            | DurableArtifactFamilyId::TierPlacementManifest
            | DurableArtifactFamilyId::ColdRecallQueue
            | DurableArtifactFamilyId::RecallAmplificationIndex
            | DurableArtifactFamilyId::SupportSchema
            | DurableArtifactFamilyId::SupportLineage
            | DurableArtifactFamilyId::SupportCursor
            | DurableArtifactFamilyId::SupportEmbeddedCheckpoint => {
                DerivedAccuracyClass::Conservative
            }
            DurableArtifactFamilyId::CompatibilitySnapshotRecord
            | DurableArtifactFamilyId::CompatibilityDeltaRecord
            | DurableArtifactFamilyId::CompatibilityMilestone6LayoutBlockChunkRecord
            | DurableArtifactFamilyId::CompatibilityMilestone8BasisContinuationDescriptor
            | DurableArtifactFamilyId::CompatibilityMilestone9BulkRecord
            | DurableArtifactFamilyId::CompatibilityMilestone10RetentionRebuildRecord
            | DurableArtifactFamilyId::CompatibilityMilestone11MaintenanceRecord
            | DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord => {
                DerivedAccuracyClass::Approximate
            }
            DurableArtifactFamilyId::MaintenanceSnapshot
            | DurableArtifactFamilyId::MaintenanceCompaction
            | DurableArtifactFamilyId::MaintenanceReclaim
            | DurableArtifactFamilyId::MaintenanceCapsule
            | DurableArtifactFamilyId::MaintenanceQueueDeclaration => {
                DerivedAccuracyClass::Heuristic
            }
            _ => DerivedAccuracyClass::Advisory,
        },
    };

    ArtifactDerivedAccuracyWitness::new(role, accuracy)
}

pub(crate) fn require_exact_accuracy_claim(
    accuracy: ArtifactDerivedAccuracyWitness,
) -> Result<ArtifactDerivedAccuracyWitness, ArtifactFamilyDenial> {
    match accuracy.accuracy() {
        DerivedAccuracyClass::Exact => Ok(accuracy),
        DerivedAccuracyClass::Conservative
        | DerivedAccuracyClass::Approximate
        | DerivedAccuracyClass::Heuristic
        | DerivedAccuracyClass::Advisory => {
            Err(ArtifactFamilyDenial::InexactFamilyCannotSatisfyExactClaim)
        }
    }
}
