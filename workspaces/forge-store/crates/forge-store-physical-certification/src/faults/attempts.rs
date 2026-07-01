use super::denial::FaultDeliveryDenial;
use super::event::PhysicalFaultEvent;
use super::locus::PhysicalArtifactFaultLocus;
use forge_store_physical_backend::ProductionStorageBoundarySeam;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultDeliveryAttempt {
    PrivateMutation,
    ArbitraryByteScribble,
    SameProcessCrash,
    PostDecodeCorruption,
    AmbiguousLocus,
}

impl FaultDeliveryAttempt {
    pub const fn private_mutation() -> Self {
        Self::PrivateMutation
    }

    pub const fn arbitrary_byte_scribble() -> Self {
        Self::ArbitraryByteScribble
    }

    pub const fn same_process_crash() -> Self {
        Self::SameProcessCrash
    }

    pub const fn post_decode_corruption() -> Self {
        Self::PostDecodeCorruption
    }

    pub const fn ambiguous_locus() -> Self {
        Self::AmbiguousLocus
    }

    pub fn admit(self) -> Result<PhysicalFaultEvent, FaultDeliveryDenial> {
        match self {
            Self::PrivateMutation => Err(FaultDeliveryDenial::PrivateMutationDenied),
            Self::ArbitraryByteScribble => Err(FaultDeliveryDenial::ArbitraryByteScribbleDenied),
            Self::SameProcessCrash => Err(FaultDeliveryDenial::SameProcessCrashDenied),
            Self::PostDecodeCorruption => Err(FaultDeliveryDenial::PostDecodeCorruptionDenied),
            Self::AmbiguousLocus => PhysicalFaultEvent::byte_corruption(
                ProductionStorageBoundarySeam::WalAppendBeforeFlush,
                PhysicalArtifactFaultLocus::ambiguous_for_denial(),
            ),
        }
    }
}
