use worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement;

use super::super::profile::PhysicalWorkSignalFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkOperationFamily {
    ArtifactMetadataRead,
    ArtifactRangeRead,
    ArtifactRangeWrite,
    ArtifactPublication,
}

impl PhysicalWorkOperationFamily {
    pub const fn required_signal_family(self) -> PhysicalWorkSignalFamily {
        match self {
            Self::ArtifactMetadataRead | Self::ArtifactRangeRead => {
                PhysicalWorkSignalFamily::ReadFault
            }
            Self::ArtifactRangeWrite => PhysicalWorkSignalFamily::ExactWriteback,
            Self::ArtifactPublication => PhysicalWorkSignalFamily::Publication,
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
}
