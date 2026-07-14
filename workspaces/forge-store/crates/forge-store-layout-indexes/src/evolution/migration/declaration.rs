use crate::ArtifactFamilyAuthorityWitness;

use super::{
    LayoutCompatibilityWindow, LayoutInterruptionPolicy, LayoutReadCompatibilityPosture,
    LayoutVersion, LayoutWriteCompatibilityPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutEvolutionDeclaration {
    family: ArtifactFamilyAuthorityWitness,
    layout_version: LayoutVersion,
    compatibility_window: LayoutCompatibilityWindow,
    migration_source: LayoutVersion,
    migration_target: LayoutVersion,
    rollback_source: LayoutVersion,
    rollback_target: LayoutVersion,
    interruption_policy: LayoutInterruptionPolicy,
}

impl LayoutEvolutionDeclaration {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        family: ArtifactFamilyAuthorityWitness,
        layout_version: LayoutVersion,
        compatibility_window: LayoutCompatibilityWindow,
        migration_source: LayoutVersion,
        migration_target: LayoutVersion,
        rollback_source: LayoutVersion,
        rollback_target: LayoutVersion,
        interruption_policy: LayoutInterruptionPolicy,
    ) -> Self {
        Self {
            family,
            layout_version,
            compatibility_window,
            migration_source,
            migration_target,
            rollback_source,
            rollback_target,
            interruption_policy,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub const fn from_admitted_family(
        family: crate::AdmittedPhysicalArtifactFamily,
        layout_version: LayoutVersion,
        compatibility_window: LayoutCompatibilityWindow,
        migration_source: LayoutVersion,
        migration_target: LayoutVersion,
        rollback_source: LayoutVersion,
        rollback_target: LayoutVersion,
        interruption_policy: LayoutInterruptionPolicy,
    ) -> Self {
        Self::new(
            family.lifecycle().authority(),
            layout_version,
            compatibility_window,
            migration_source,
            migration_target,
            rollback_source,
            rollback_target,
            interruption_policy,
        )
    }

    pub(crate) const fn family(self) -> ArtifactFamilyAuthorityWitness {
        self.family
    }

    pub const fn family_declaration(
        self,
    ) -> &'static crate::declarations::PhysicalArtifactFamilyDeclaration {
        self.family.declaration()
    }

    pub const fn layout_version(self) -> LayoutVersion {
        self.layout_version
    }

    pub const fn compatibility_window(self) -> LayoutCompatibilityWindow {
        self.compatibility_window
    }

    pub const fn migration_source(self) -> LayoutVersion {
        self.migration_source
    }

    pub const fn migration_target(self) -> LayoutVersion {
        self.migration_target
    }

    pub const fn rollback_source(self) -> LayoutVersion {
        self.rollback_source
    }

    pub const fn rollback_target(self) -> LayoutVersion {
        self.rollback_target
    }

    pub const fn interruption_policy(self) -> LayoutInterruptionPolicy {
        self.interruption_policy
    }

    pub const fn read_posture(self) -> LayoutReadCompatibilityPosture {
        self.compatibility_window.read_posture()
    }

    pub const fn write_posture(self) -> LayoutWriteCompatibilityPosture {
        self.compatibility_window.write_posture()
    }

    pub fn declares_readable_version(self, version: LayoutVersion) -> bool {
        version == self.layout_version
            || version == self.migration_source
            || version == self.migration_target
            || version == self.rollback_source
            || version == self.rollback_target
    }
}
