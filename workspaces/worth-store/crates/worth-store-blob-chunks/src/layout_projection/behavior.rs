use worth_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobLayoutScopeSafeAbsenceBehavior {
    ExactIndex,
    ScopedMaintenanceScan,
    ScopedVerifierScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobLayoutCorruptionBehavior {
    RebuildToParity,
    QuarantineRequired,
}

pub(crate) const fn declared_rebuild_posture(
    family_id: DurableArtifactFamilyId,
) -> DurableArtifactRebuildPosture {
    match family_id {
        DurableArtifactFamilyId::DedupeIndex | DurableArtifactFamilyId::ReachabilityEdge => {
            DurableArtifactRebuildPosture::RebuildFromAuthoritativeState
        }
        DurableArtifactFamilyId::RetentionHold | DurableArtifactFamilyId::QuarantineRecord => {
            DurableArtifactRebuildPosture::QuarantineOnly
        }
        DurableArtifactFamilyId::ReclaimReceipt => DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactFamilyId::MaintenanceCompaction => {
            DurableArtifactRebuildPosture::PartialRebuildOnly
        }
        _ => panic!("blob layout behavior requested for a non-blob projection family"),
    }
}

pub(crate) const fn corruption_behavior_for(
    rebuild_posture: DurableArtifactRebuildPosture,
) -> BlobLayoutCorruptionBehavior {
    match rebuild_posture {
        DurableArtifactRebuildPosture::NoRebuild
        | DurableArtifactRebuildPosture::QuarantineOnly => {
            BlobLayoutCorruptionBehavior::QuarantineRequired
        }
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState
        | DurableArtifactRebuildPosture::ReplayRebuildable
        | DurableArtifactRebuildPosture::PartialRebuildOnly => {
            BlobLayoutCorruptionBehavior::RebuildToParity
        }
    }
}
