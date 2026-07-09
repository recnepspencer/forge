use crate::{LayoutCorruptionClassification, PhysicalArtifactFamily, S8LayoutCoverageWitness};
use worth_store_recovery_physics::RecoveryLayoutReadmissionIdentity;

use super::classification::{S8LayoutCorruptionClass, S8LayoutReadmissionSource};
use super::quarantine::S8LayoutQuarantineWitness;
use super::readmission::S8LayoutReadmissionWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8LayoutCorruptionOutcome {
    Clean {
        coverage: S8LayoutCoverageWitness,
    },
    NotFound {
        family: PhysicalArtifactFamily,
    },
    Unsupported {
        family: PhysicalArtifactFamily,
        state: crate::S8MaterializationStateClass,
    },
    StaleBinding {
        coverage: S8LayoutCoverageWitness,
    },
    DerivedProjectionRebuildRequired {
        classification: LayoutCorruptionClassification,
    },
    AuthoritativeArtifactQuarantineRequired(S8LayoutQuarantineWitness),
    QuarantineReadmissionRequired {
        quarantine: S8LayoutQuarantineWitness,
        identity: RecoveryLayoutReadmissionIdentity,
    },
    OfflineEvidenceReadmissionRequired {
        family: PhysicalArtifactFamily,
        identity: RecoveryLayoutReadmissionIdentity,
    },
    TerminalImportReadmissionRequired {
        family: PhysicalArtifactFamily,
        identity: RecoveryLayoutReadmissionIdentity,
    },
    MigrationRequired {
        family: PhysicalArtifactFamily,
    },
}

impl S8LayoutCorruptionOutcome {
    pub const fn class(&self) -> S8LayoutCorruptionClass {
        match self {
            Self::Clean { .. } => S8LayoutCorruptionClass::Clean,
            Self::NotFound { .. } => S8LayoutCorruptionClass::NotFound,
            Self::Unsupported { .. } => S8LayoutCorruptionClass::Unsupported,
            Self::StaleBinding { .. } => S8LayoutCorruptionClass::StaleBinding,
            Self::DerivedProjectionRebuildRequired { .. } => {
                S8LayoutCorruptionClass::DerivedProjectionCorruption
            }
            Self::AuthoritativeArtifactQuarantineRequired(_) => {
                S8LayoutCorruptionClass::AuthoritativeArtifactCorruption
            }
            Self::QuarantineReadmissionRequired { .. }
            | Self::OfflineEvidenceReadmissionRequired { .. }
            | Self::TerminalImportReadmissionRequired { .. } => {
                S8LayoutCorruptionClass::ReadmissionRequired
            }
            Self::MigrationRequired { .. } => S8LayoutCorruptionClass::MigrationRequired,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum S8LayoutReadmissionOutcome {
    Readmitted(S8LayoutReadmissionWitness),
    Denied(super::denial::S8CorruptionDenial),
}

impl S8LayoutReadmissionOutcome {
    pub const fn source(&self) -> Option<S8LayoutReadmissionSource> {
        match self {
            Self::Readmitted(witness) => Some(witness.source()),
            Self::Denied(_) => None,
        }
    }
}
