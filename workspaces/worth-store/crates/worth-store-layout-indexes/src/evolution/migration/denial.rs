use crate::PhysicalArtifactFamilyDeclaration;

use super::{LayoutInterruptionFingerprint, LayoutVersion};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutEvolutionDenial {
    CompatibilityAdmissionMismatch,
    CompatibilityBindingVersionMismatch {
        binding: LayoutVersion,
        compatibility: worth_store_compatibility::ArtifactFormatVersion,
    },
    BindingSourceVersionNotDeclared {
        bound: LayoutVersion,
        declared_source: LayoutVersion,
    },
    StoreAuthorityMismatch {
        family: worth_store_authority::StoreCurrentAuthorityIdentity,
        binding: worth_store_authority::StoreCurrentAuthorityIdentity,
    },
    PhysicalSourceStoreAuthorityMismatch {
        binding: worth_store_authority::StoreCurrentAuthorityIdentity,
        physical_source: worth_store_authority::StoreCurrentAuthorityIdentity,
    },
    FamilyMismatch {
        declared: &'static PhysicalArtifactFamilyDeclaration,
        binding: &'static PhysicalArtifactFamilyDeclaration,
    },
    IncompatibleSourceVersion {
        source: LayoutVersion,
        minimum_readable: LayoutVersion,
        maximum_readable: LayoutVersion,
    },
    UndeclaredCompatibleLayoutVersion {
        source: LayoutVersion,
    },
    UnsupportedMigrationTarget {
        source: LayoutVersion,
        target: LayoutVersion,
    },
    UnsupportedRollbackTarget {
        source: LayoutVersion,
        target: LayoutVersion,
    },
    InterruptStateDoesNotMatchPlan {
        expected: Box<LayoutInterruptionFingerprint>,
        actual: Box<LayoutInterruptionFingerprint>,
    },
    RollbackInterruptStateDoesNotMatchExecution {
        expected: Box<super::LayoutRollbackExecutionFingerprint>,
        actual: Box<super::LayoutRollbackExecutionFingerprint>,
    },
    InterruptionBindingVersionNotDeclared {
        observed: LayoutVersion,
        source: LayoutVersion,
        target: LayoutVersion,
    },
    PhysicalPublicationStoreAuthorityMismatch {
        binding: worth_store_authority::StoreCurrentAuthorityIdentity,
        publication: worth_store_authority::StoreCurrentAuthorityIdentity,
    },
    PhysicalPublicationSourceMismatch {
        expected: super::LayoutBindingSourceIdentity,
        actual: worth_store_physical_format::PhysicalReference,
    },
    PhysicalPublication(Box<worth_store_physical_isolation::PhysicalPublicationDenial>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutEvolutionDenialKind {
    CompatibilityAdmissionMismatch,
    CompatibilityBindingVersionMismatch,
    BindingSourceVersionNotDeclared,
    StoreAuthorityMismatch,
    PhysicalSourceStoreAuthorityMismatch,
    FamilyMismatch,
    IncompatibleSourceVersion,
    UndeclaredCompatibleLayoutVersion,
    UnsupportedMigrationTarget,
    UnsupportedRollbackTarget,
    InterruptStateDoesNotMatchPlan,
    RollbackInterruptStateDoesNotMatchExecution,
    InterruptionBindingVersionNotDeclared,
    PhysicalPublicationStoreAuthorityMismatch,
    PhysicalPublicationSourceMismatch,
    PhysicalPublication,
}

impl LayoutEvolutionDenial {
    pub const fn kind(&self) -> LayoutEvolutionDenialKind {
        match self {
            Self::CompatibilityAdmissionMismatch => {
                LayoutEvolutionDenialKind::CompatibilityAdmissionMismatch
            }
            Self::CompatibilityBindingVersionMismatch { .. } => {
                LayoutEvolutionDenialKind::CompatibilityBindingVersionMismatch
            }
            Self::BindingSourceVersionNotDeclared { .. } => {
                LayoutEvolutionDenialKind::BindingSourceVersionNotDeclared
            }
            Self::StoreAuthorityMismatch { .. } => {
                LayoutEvolutionDenialKind::StoreAuthorityMismatch
            }
            Self::PhysicalSourceStoreAuthorityMismatch { .. } => {
                LayoutEvolutionDenialKind::PhysicalSourceStoreAuthorityMismatch
            }
            Self::FamilyMismatch { .. } => LayoutEvolutionDenialKind::FamilyMismatch,
            Self::IncompatibleSourceVersion { .. } => {
                LayoutEvolutionDenialKind::IncompatibleSourceVersion
            }
            Self::UndeclaredCompatibleLayoutVersion { .. } => {
                LayoutEvolutionDenialKind::UndeclaredCompatibleLayoutVersion
            }
            Self::UnsupportedMigrationTarget { .. } => {
                LayoutEvolutionDenialKind::UnsupportedMigrationTarget
            }
            Self::UnsupportedRollbackTarget { .. } => {
                LayoutEvolutionDenialKind::UnsupportedRollbackTarget
            }
            Self::InterruptStateDoesNotMatchPlan { .. } => {
                LayoutEvolutionDenialKind::InterruptStateDoesNotMatchPlan
            }
            Self::RollbackInterruptStateDoesNotMatchExecution { .. } => {
                LayoutEvolutionDenialKind::RollbackInterruptStateDoesNotMatchExecution
            }
            Self::InterruptionBindingVersionNotDeclared { .. } => {
                LayoutEvolutionDenialKind::InterruptionBindingVersionNotDeclared
            }
            Self::PhysicalPublicationStoreAuthorityMismatch { .. } => {
                LayoutEvolutionDenialKind::PhysicalPublicationStoreAuthorityMismatch
            }
            Self::PhysicalPublicationSourceMismatch { .. } => {
                LayoutEvolutionDenialKind::PhysicalPublicationSourceMismatch
            }
            Self::PhysicalPublication(_) => LayoutEvolutionDenialKind::PhysicalPublication,
        }
    }
}
