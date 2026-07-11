use forge_store_contracts::{DurableArtifactFamilyId, DurableArtifactRebuildPosture};
use forge_store_layout_indexes::layout_families::layout_declarations;

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

pub(crate) fn declared_rebuild_posture(
    family_id: DurableArtifactFamilyId,
) -> DurableArtifactRebuildPosture {
    layout_declarations()
        .declaration(family_id)
        .expect("phase family must stay declared")
        .rebuild_posture()
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
