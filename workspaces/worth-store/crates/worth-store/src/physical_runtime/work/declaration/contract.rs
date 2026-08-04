use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::super::profile::PhysicalWorkSignalFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkOperationFamily {
    ArtifactMetadataRead,
    ArtifactRangeRead,
    ArtifactRangeWrite,
    ArtifactPublication,
    CheckpointCapture,
    WalAppend,
    DurabilityBarrier,
    WalReclamation,
    RootPublication,
}

impl PhysicalWorkOperationFamily {
    pub const fn required_signal_family(self) -> PhysicalWorkSignalFamily {
        match self {
            Self::ArtifactMetadataRead | Self::ArtifactRangeRead => {
                PhysicalWorkSignalFamily::ReadFault
            }
            Self::ArtifactRangeWrite => PhysicalWorkSignalFamily::ExactWriteback,
            Self::ArtifactPublication => PhysicalWorkSignalFamily::Publication,
            Self::CheckpointCapture => PhysicalWorkSignalFamily::CheckpointCapture,
            Self::WalAppend => PhysicalWorkSignalFamily::WalAppend,
            Self::DurabilityBarrier => PhysicalWorkSignalFamily::DurabilityBarrier,
            Self::WalReclamation => PhysicalWorkSignalFamily::WalReclamation,
            Self::RootPublication => PhysicalWorkSignalFamily::RootPublication,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkEffectClass {
    ReadOnly,
    ReversibleBeforePublication,
    IdempotentExactWrite,
    PublicationBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkRecoveryDisposition {
    NoEffect,
    RetryExact,
    ContinueSettlement,
    InspectionRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkDurabilityRequirement {
    ReadOnly,
    ArtifactRangeWrite(ArtifactRangeWriteDurabilityRequirement),
    /// Complete WAL bytes are requested, without claiming a durability barrier.
    WalAppend,
    /// Exact admitted backend barrier for one already-appended WAL member.
    WalDurabilityBarrier,
    /// Exact Store-owned checkpoint action admitted only through background pacing.
    CheckpointCapture,
    /// Proof-gated removal of one obsolete WAL artifact.
    WalReclamation,
    /// One exact candidate-sync, catalog-replacement, or namespace-sync root action.
    RootPublication,
}
