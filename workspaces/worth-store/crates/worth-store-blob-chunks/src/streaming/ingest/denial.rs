use worth_store_budgets::CounterEvidenceStrength;
use worth_store_buffer_pool::{PhysicalOperationAllocationScope, PhysicalResidencyDenial};
use worth_store_io_scheduler::{
    foreground_reservation::ForegroundIoLaneKind, BackgroundIoPressureClass,
    BackgroundPacingStaleRebindKind,
};

use super::super::allocation::BlobStreamingAllocationDenial;
use crate::BlobChunkIntegrityDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobStreamingIngestDenial {
    EmptyStreamingWindow,
    EmptySourceFrame,
    EmptyDeclaredObject,
    WindowExceedsResidentEnvelope {
        window_bytes: u64,
        envelope_bytes: u64,
    },
    WholeObjectMaterializationRejected {
        bytes: u64,
    },
    SourceWindowExceedsResidentEnvelope {
        window_bytes: u64,
        envelope_bytes: u64,
    },
    MissingExactCounters {
        actual: CounterEvidenceStrength,
    },
    AllocationDenied(PhysicalResidencyDenial),
    AllocationScopeMismatch {
        actual: PhysicalOperationAllocationScope,
    },
    AllocationWindowExceeded {
        window_bytes: u64,
        allocation_bytes: u64,
    },
    AllocationCountersUnavailable,
    BackgroundPressureDeferred,
    BackgroundPressureDenied,
    BackgroundPressureStale {
        kind: BackgroundPacingStaleRebindKind,
    },
    BackgroundPressureViolation,
    BackgroundPressureClassMismatch {
        actual: BackgroundIoPressureClass,
    },
    ForegroundReservationNotAdmitted {
        lane: ForegroundIoLaneKind,
    },
    ForegroundReservationLaneMismatch {
        lane: ForegroundIoLaneKind,
    },
    ScalarBackendCertificationRejected,
    BackendWriteOrdinalMismatch {
        expected: u64,
        actual: u64,
    },
    BackendWriteBytesMismatch {
        expected: u64,
        actual: u64,
    },
    BackendWritePayloadMismatch {
        ordinal: u64,
    },
    ResumeSessionRequestMismatch,
    ChunkIntegrity(BlobChunkIntegrityDenial),
}

impl From<BlobChunkIntegrityDenial> for BlobStreamingIngestDenial {
    fn from(denial: BlobChunkIntegrityDenial) -> Self {
        Self::ChunkIntegrity(denial)
    }
}

impl From<PhysicalResidencyDenial> for BlobStreamingIngestDenial {
    fn from(denial: PhysicalResidencyDenial) -> Self {
        Self::AllocationDenied(denial)
    }
}

impl From<BlobStreamingAllocationDenial> for BlobStreamingIngestDenial {
    fn from(denial: BlobStreamingAllocationDenial) -> Self {
        match denial {
            BlobStreamingAllocationDenial::WrongScope { actual } => {
                Self::AllocationScopeMismatch { actual }
            }
            BlobStreamingAllocationDenial::WindowExceedsAllocation {
                window_bytes,
                allocation_bytes,
            } => Self::AllocationWindowExceeded {
                window_bytes,
                allocation_bytes,
            },
            BlobStreamingAllocationDenial::CountersUnavailable => {
                Self::AllocationCountersUnavailable
            }
        }
    }
}

pub fn reject_full_blob_vec_as_streaming_ingest(bytes: Vec<u8>) -> BlobStreamingIngestDenial {
    BlobStreamingIngestDenial::WholeObjectMaterializationRejected {
        bytes: bytes.len() as u64,
    }
}
