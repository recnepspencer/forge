use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::super::profile::PhysicalWorkSignalFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkOperationFamily {
    ArtifactMetadataRead,
    ArtifactRangeRead,
    ArtifactRangeWrite,
    ArtifactPublication,
    WalAppend,
    DurabilityBarrier,
}

impl PhysicalWorkOperationFamily {
    pub const fn required_signal_family(self) -> PhysicalWorkSignalFamily {
        match self {
            Self::ArtifactMetadataRead | Self::ArtifactRangeRead => {
                PhysicalWorkSignalFamily::ReadFault
            }
            Self::ArtifactRangeWrite => PhysicalWorkSignalFamily::ExactWriteback,
            Self::ArtifactPublication => PhysicalWorkSignalFamily::Publication,
            Self::WalAppend => PhysicalWorkSignalFamily::WalAppend,
            Self::DurabilityBarrier => PhysicalWorkSignalFamily::DurabilityBarrier,
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
}
