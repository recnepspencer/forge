use super::super::{ArtifactFamilyInventoryRow, PhysicalArtifactFamilyDeclaration};
use forge_store_contracts::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
    DurableArtifactFamilyId, DurableArtifactMigrationPosture, DurableArtifactOwningBoundary,
    DurableArtifactProjectionClass, DurableArtifactRebuildPosture,
};

pub(super) const NONE: &[DurableArtifactProjectionClass] = &[];
pub(super) const OFFLINE_ONLY: &[DurableArtifactProjectionClass] =
    &[DurableArtifactProjectionClass::OfflineObservation];
pub(super) const TERMINAL_ONLY: &[DurableArtifactProjectionClass] =
    &[DurableArtifactProjectionClass::TerminalReport];
pub(super) const EVIDENCE_ONLY: &[DurableArtifactProjectionClass] = &[
    DurableArtifactProjectionClass::OfflineObservation,
    DurableArtifactProjectionClass::CertificationEvidence,
];

#[allow(clippy::too_many_arguments)]
pub(super) const fn row(
    family_id: DurableArtifactFamilyId,
    authority: ArtifactFamilyAuthorityClass,
    lifecycle: ArtifactFamilyLifecycleClass,
    access_lane: ArtifactFamilyAccessLane,
    owning_boundary: DurableArtifactOwningBoundary,
    rebuild_posture: DurableArtifactRebuildPosture,
    migration_posture: DurableArtifactMigrationPosture,
    projection_classes: &'static [DurableArtifactProjectionClass],
) -> ArtifactFamilyInventoryRow {
    ArtifactFamilyInventoryRow::new(PhysicalArtifactFamilyDeclaration::declare(
        family_id,
        authority,
        lifecycle,
        access_lane,
        owning_boundary,
        rebuild_posture,
        migration_posture,
        projection_classes,
    ))
}
