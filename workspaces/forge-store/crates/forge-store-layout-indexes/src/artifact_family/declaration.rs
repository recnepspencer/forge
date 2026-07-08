use forge_store_contracts::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
    DurableArtifactFamilyId, DurableArtifactMigrationPosture, DurableArtifactOwningBoundary,
    DurableArtifactProjectionClass, DurableArtifactRebuildPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalArtifactFamily(DurableArtifactFamilyId);

impl PhysicalArtifactFamily {
    pub const fn id(self) -> DurableArtifactFamilyId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalArtifactFamilyDeclaration {
    family: PhysicalArtifactFamily,
    authority: ArtifactFamilyAuthorityClass,
    lifecycle: ArtifactFamilyLifecycleClass,
    access_lane: ArtifactFamilyAccessLane,
    owning_boundary: DurableArtifactOwningBoundary,
    rebuild_posture: DurableArtifactRebuildPosture,
    migration_posture: DurableArtifactMigrationPosture,
    non_authority_projection_classes: &'static [DurableArtifactProjectionClass],
}

impl PhysicalArtifactFamilyDeclaration {
    pub(crate) const fn declare(
        family_id: DurableArtifactFamilyId,
        authority: ArtifactFamilyAuthorityClass,
        lifecycle: ArtifactFamilyLifecycleClass,
        access_lane: ArtifactFamilyAccessLane,
        owning_boundary: DurableArtifactOwningBoundary,
        rebuild_posture: DurableArtifactRebuildPosture,
        migration_posture: DurableArtifactMigrationPosture,
        non_authority_projection_classes: &'static [DurableArtifactProjectionClass],
    ) -> Self {
        Self {
            family: PhysicalArtifactFamily(family_id),
            authority,
            lifecycle,
            access_lane,
            owning_boundary,
            rebuild_posture,
            migration_posture,
            non_authority_projection_classes,
        }
    }

    pub const fn family(&self) -> PhysicalArtifactFamily {
        self.family
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family.id()
    }

    pub(crate) const fn authority(&self) -> ArtifactFamilyAuthorityClass {
        self.authority
    }

    pub(crate) const fn lifecycle(&self) -> ArtifactFamilyLifecycleClass {
        self.lifecycle
    }

    pub(crate) const fn access_lane(&self) -> ArtifactFamilyAccessLane {
        self.access_lane
    }

    pub const fn owning_boundary(&self) -> DurableArtifactOwningBoundary {
        self.owning_boundary
    }

    pub(crate) const fn rebuild_posture(&self) -> DurableArtifactRebuildPosture {
        self.rebuild_posture
    }

    pub(crate) const fn migration_posture(&self) -> DurableArtifactMigrationPosture {
        self.migration_posture
    }

    pub(crate) const fn non_authority_projection_classes(
        &self,
    ) -> &'static [DurableArtifactProjectionClass] {
        self.non_authority_projection_classes
    }
}
