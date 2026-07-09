use super::PhysicalArtifactFamilyDeclaration;
use worth_store_contracts::DurableArtifactFamilyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyAuthorityDisposition {
    Authoritative,
    Derived,
    Diagnostic,
    Terminal,
    Certification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyLifecycleDisposition {
    StrategyHotPath,
    StrategyMaintenancePath,
    VerifierOnly,
    ReadmissionRequired,
    TransferBoundaryOnly,
    OfflineImportOnly,
    TerminalOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactFamilyClassification {
    declaration: &'static PhysicalArtifactFamilyDeclaration,
    authority: ArtifactFamilyAuthorityDisposition,
    lifecycle: ArtifactFamilyLifecycleDisposition,
}

impl ArtifactFamilyClassification {
    pub(crate) const fn new(
        declaration: &'static PhysicalArtifactFamilyDeclaration,
        authority: ArtifactFamilyAuthorityDisposition,
        lifecycle: ArtifactFamilyLifecycleDisposition,
    ) -> Self {
        Self {
            declaration,
            authority,
            lifecycle,
        }
    }

    pub const fn family_id(self) -> DurableArtifactFamilyId {
        self.declaration.family_id()
    }

    pub const fn declaration(self) -> &'static PhysicalArtifactFamilyDeclaration {
        self.declaration
    }

    pub const fn authority(self) -> ArtifactFamilyAuthorityDisposition {
        self.authority
    }

    pub const fn lifecycle(self) -> ArtifactFamilyLifecycleDisposition {
        self.lifecycle
    }
}
