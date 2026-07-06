use forge_store_budgets::CounterEvidenceStrength;
use forge_store_buffer_pool::AllocationDenial;
use forge_store_io_scheduler::{
    foreground_reservation::{
        ForegroundIoLaneKind, ForegroundReservationAdmissionDenial, ForegroundReservationState,
    },
    BackgroundIoPressureClass, BackgroundPacingDenial, BackgroundPacingStaleRebindKind,
};
use forge_store_physical_isolation::PhysicalReadExecutionDenial;

use crate::{
    BlobChunkByteRange, BlobChunkOrdinal, BlobCorruptionDenial, BlobDamageCase,
    BlobQuarantineDiagnostics, BlobStreamingReadCounterSnapshot,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobStreamingReadDenial {
    EmptyReadWindow,
    EmptyObservedReadChunk,
    ReadWindowExceedsResidentEnvelope {
        window_bytes: u64,
        envelope_bytes: u64,
    },
    WholeObjectExpectedBufferRejected {
        bytes: u64,
    },
    MissingExactCounters {
        actual: CounterEvidenceStrength,
    },
    AllocationDenied(AllocationDenial),
    AllocationScopeMismatch,
    AllocationKindMismatch,
    AllocationCountersHidden,
    ResidentEnvelopeExceeded {
        peak_resident_bytes: u64,
        envelope_bytes: u64,
    },
    ForegroundReservationNotAdmitted {
        lane: ForegroundIoLaneKind,
        state: ForegroundReservationState,
    },
    ForegroundReservationAdmissionDenied(ForegroundReservationAdmissionDenial),
    ForegroundReservationLaneMismatch {
        lane: ForegroundIoLaneKind,
    },
    StablePhysicalReadDenied(PhysicalReadExecutionDenial),
    VerificationPressureClassMismatch {
        actual: BackgroundIoPressureClass,
    },
    VerificationPressureYielded {
        counters: BlobStreamingReadCounterSnapshot,
    },
    VerificationPressureDeferred {
        counters: BlobStreamingReadCounterSnapshot,
    },
    VerificationPressureThrottledWithoutAdmittedCapacity {
        counters: BlobStreamingReadCounterSnapshot,
    },
    VerificationPressureDenied {
        denial: BackgroundPacingDenial,
        counters: BlobStreamingReadCounterSnapshot,
    },
    VerificationPressureStaleRebindRequired {
        kind: BackgroundPacingStaleRebindKind,
        counters: BlobStreamingReadCounterSnapshot,
    },
    VerificationPressureViolation {
        counters: BlobStreamingReadCounterSnapshot,
    },
    StableReadBytesInsufficient {
        expected: u64,
        actual: u64,
        counters: BlobStreamingReadCounterSnapshot,
    },
    MissingChunk {
        ordinal: BlobChunkOrdinal,
        counters: BlobStreamingReadCounterSnapshot,
    },
    ReorderedChunk {
        expected: BlobChunkOrdinal,
        actual: BlobChunkOrdinal,
        counters: BlobStreamingReadCounterSnapshot,
    },
    ChunkRangeMismatch {
        ordinal: BlobChunkOrdinal,
        expected: BlobChunkByteRange,
        actual: BlobChunkByteRange,
        counters: BlobStreamingReadCounterSnapshot,
    },
    CorruptedChunk {
        ordinal: BlobChunkOrdinal,
        damage_case: BlobDamageCase,
        diagnostics: BlobQuarantineDiagnostics,
        counters: BlobStreamingReadCounterSnapshot,
    },
    ColdChunkUnavailable {
        ordinal: BlobChunkOrdinal,
        counters: BlobStreamingReadCounterSnapshot,
    },
    ExtraChunk {
        ordinal: BlobChunkOrdinal,
        counters: BlobStreamingReadCounterSnapshot,
    },
    CorruptionReferenceEdgeMismatch(BlobCorruptionDenial),
    LogicalContentDigestMismatch,
    ChunkTreeRootMismatch,
}

pub fn reject_full_blob_vec_as_streaming_read(bytes: Vec<u8>) -> BlobStreamingReadDenial {
    BlobStreamingReadDenial::WholeObjectExpectedBufferRejected {
        bytes: bytes.len() as u64,
    }
}
