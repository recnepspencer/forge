use forge_store_budgets::AllocationEnvelopeSet;
use forge_store_budgets::CounterEvidenceStrength;
use forge_store_buffer_pool::{AllocationDenial, AllocationReceipt};
use forge_store_io_scheduler::{
    admit_background_pacing,
    foreground_reservation::{
        ForegroundIoLaneKind, ForegroundReservationReceipt, ForegroundReservationState,
    },
    BackgroundCapacityAdmission, BackgroundIdleCapacityLeaseRequest, BackgroundIoPressureClass,
    BackgroundPacingAdmissionBasis, BackgroundPacingOutcome,
};
use forge_store_physical_backend::{
    BlobBackendChunkWriteObservation, BlobBackendChunkWriteObservationKind,
};

use crate::{
    blob_streaming_performance::{
        counter_backed_streaming_performance_receipt, BlobStreamingCounterBackedPerformanceReceipt,
    },
    AdmittedBlobChunkSequence, BlobChunkOrdinal, BlobChunkSequenceAdmission,
    BlobStreamingChunkWriter, BlobStreamingContentFrontier, BlobStreamingIngestCounterSnapshot,
    BlobStreamingIngestDenial, BlobStreamingIngestRequest, BlobStreamingResidencyProof,
    BlobStreamingResumeAdmission, BlobStreamingResumePosture, BlobStreamingSourceFrame,
    BlobStreamingWindow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobStreamingPressureAdmission {
    basis: BackgroundPacingAdmissionBasis,
    outcome: BackgroundPacingOutcome,
}

impl BlobStreamingPressureAdmission {
    pub fn from_s6_background_capacity(
        capacity: BackgroundCapacityAdmission,
        foreground_pressure_events: u64,
        late_yield: bool,
    ) -> Result<Self, BlobStreamingIngestDenial> {
        let basis = capacity.basis();
        if basis.class() != BackgroundIoPressureClass::BlobIngestPressure {
            return Err(BlobStreamingIngestDenial::BackgroundPressureClassMismatch {
                actual: basis.class(),
            });
        }
        match basis.foreground_lane() {
            ForegroundIoLaneKind::CommitCriticalWalWrite
            | ForegroundIoLaneKind::OrdinaryPageWrite => {
                let mut request = BackgroundIdleCapacityLeaseRequest::new(capacity)
                    .with_foreground_pressure_events(foreground_pressure_events);
                if late_yield {
                    request = request.with_late_yield();
                }
                Ok(Self {
                    basis,
                    outcome: admit_background_pacing(request),
                })
            }
            lane => Err(BlobStreamingIngestDenial::ForegroundReservationLaneMismatch { lane }),
        }
    }

    pub fn reject_unbound_foreground_reservation(
        foreground: ForegroundReservationReceipt,
    ) -> BlobStreamingIngestDenial {
        if foreground.state() != ForegroundReservationState::ReservationAdmitted {
            BlobStreamingIngestDenial::ForegroundReservationNotAdmitted {
                lane: foreground.lane(),
            }
        } else {
            BlobStreamingIngestDenial::ForegroundReservationLaneMismatch {
                lane: foreground.lane(),
            }
        }
    }

    pub const fn basis(self) -> BackgroundPacingAdmissionBasis {
        self.basis
    }

    pub const fn outcome(self) -> BackgroundPacingOutcome {
        self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingIngest {
    sequence: AdmittedBlobChunkSequence,
    frontier: BlobStreamingContentFrontier,
    resumability: BlobStreamingResumePosture,
    residency: BlobStreamingResidencyProof,
    counters: BlobStreamingIngestCounterSnapshot,
    performance: BlobStreamingCounterBackedPerformanceReceipt,
}

impl BlobStreamingIngest {
    pub(crate) fn run_bounded<W>(
        request: BlobStreamingIngestRequest,
        window: BlobStreamingWindow,
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        pressure: BlobStreamingPressureAdmission,
        source_frames: impl IntoIterator<Item = BlobStreamingSourceFrame>,
        writer: &mut W,
        counter_strength: CounterEvidenceStrength,
    ) -> Result<Self, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        if !counter_strength.satisfies(CounterEvidenceStrength::Exact) {
            return Err(BlobStreamingIngestDenial::MissingExactCounters {
                actual: counter_strength,
            });
        }
        let mut counters = pressure_counters(pressure)?.record_allocation();
        let (security_scope, rule, declared_total_bytes) = request.into_parts();
        let chunk_size = rule.chunk_size().bytes() as usize;
        let mut admission =
            BlobChunkSequenceAdmission::start(security_scope, rule, declared_total_bytes)?;
        let mut chunking = BlobStreamingChunkingSession::new(chunk_size);

        for frame in source_frames {
            let frame_bytes = frame.into_bytes();
            let frame_len = frame_bytes.len() as u64;
            if frame_len >= declared_total_bytes {
                return Err(
                    BlobStreamingIngestDenial::WholeObjectMaterializationRejected {
                        bytes: frame_len,
                    },
                );
            }
            counters = counters.observe_source_window(frame_len, 0);
            let pushed = chunking.push_frame(
                &frame_bytes,
                window,
                declared_total_bytes,
                admission,
                writer,
                counters,
            )?;
            admission = pushed.admission;
            counters = pushed.counters;
        }

        let finished = chunking.finish(admission, writer, counters)?;
        admission = finished.admission;
        counters = finished.counters;
        let sequence = admission.finish()?;
        let frontier = BlobStreamingContentFrontier::from_sequence(&sequence);
        let resumability = BlobStreamingResumePosture::from_frontier(&frontier);
        let residency = BlobStreamingResidencyProof::from_executed_streaming_session(
            allocation,
            envelopes,
            counters.peak_resident_bytes(),
            window,
            counter_strength,
        )?;
        let performance = counter_backed_streaming_performance_receipt(counters);
        Ok(Self {
            sequence,
            frontier,
            resumability,
            residency,
            counters,
            performance,
        })
    }

    pub const fn sequence(&self) -> &AdmittedBlobChunkSequence {
        &self.sequence
    }

    pub const fn frontier(&self) -> &BlobStreamingContentFrontier {
        &self.frontier
    }

    pub const fn resumability(&self) -> &BlobStreamingResumePosture {
        &self.resumability
    }

    pub const fn residency(&self) -> BlobStreamingResidencyProof {
        self.residency
    }

    pub const fn counters(&self) -> BlobStreamingIngestCounterSnapshot {
        self.counters
    }

    pub const fn counter_backed_performance_receipt(
        &self,
    ) -> &BlobStreamingCounterBackedPerformanceReceipt {
        &self.performance
    }

    pub(crate) fn bind_resume_admission(mut self, admission: BlobStreamingResumeAdmission) -> Self {
        self.resumability = self
            .resumability
            .with_resume_session(admission.session_digest());
        self
    }
}

pub fn reject_scalar_backend_api_as_streaming_ingest(
    observation: BlobBackendChunkWriteObservation,
) -> BlobStreamingIngestDenial {
    if observation.kind() == BlobBackendChunkWriteObservationKind::ScalarFramedRecordApi {
        BlobStreamingIngestDenial::ScalarBackendCertificationRejected
    } else {
        BlobStreamingIngestDenial::BackendWriteOrdinalMismatch {
            expected: 0,
            actual: observation.ordinal(),
        }
    }
}

pub fn reject_allocation_denial_as_streaming_ingest(
    denial: AllocationDenial,
) -> BlobStreamingIngestDenial {
    BlobStreamingIngestDenial::AllocationDenied(denial)
}

fn pressure_counters(
    pressure: BlobStreamingPressureAdmission,
) -> Result<BlobStreamingIngestCounterSnapshot, BlobStreamingIngestDenial> {
    let counters = BlobStreamingIngestCounterSnapshot::start();
    match pressure.outcome() {
        BackgroundPacingOutcome::Yield(yielded) => Ok(counters
            .record_yield()
            .record_scheduler_waits(yielded.counters().foreground_pressure_events())),
        BackgroundPacingOutcome::Throttled(throttled) => Ok(counters
            .record_throttle()
            .record_scheduler_waits(throttled.counters().foreground_pressure_events())),
        BackgroundPacingOutcome::AdmittedWithDebt(admitted) => Ok(counters
            .record_admission()
            .record_scheduler_waits(admitted.counters().foreground_pressure_events())),
        BackgroundPacingOutcome::Deferred(_) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureDeferred)
        }
        BackgroundPacingOutcome::Denied(_) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureDenied)
        }
        BackgroundPacingOutcome::StaleRebindRequired(stale) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureStale { kind: stale.kind() })
        }
        BackgroundPacingOutcome::Violation(_) => {
            Err(BlobStreamingIngestDenial::BackgroundPressureViolation)
        }
    }
}

struct BlobStreamingChunkingSession {
    pending: Vec<u8>,
    start_offset: u64,
    ordinal: BlobChunkOrdinal,
    chunk_size: usize,
}

struct BlobStreamingChunkingStep {
    admission: BlobChunkSequenceAdmission,
    counters: BlobStreamingIngestCounterSnapshot,
}

impl BlobStreamingChunkingSession {
    fn new(chunk_size: usize) -> Self {
        Self {
            pending: Vec::with_capacity(chunk_size),
            start_offset: 0,
            ordinal: BlobChunkOrdinal::first(),
            chunk_size,
        }
    }

    fn push_frame<W>(
        &mut self,
        frame: &[u8],
        window: BlobStreamingWindow,
        declared_total_bytes: u64,
        mut admission: BlobChunkSequenceAdmission,
        writer: &mut W,
        mut counters: BlobStreamingIngestCounterSnapshot,
    ) -> Result<BlobStreamingChunkingStep, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        let mut remaining = frame;
        while !remaining.is_empty() {
            let take = self
                .chunk_size
                .saturating_sub(self.pending.len())
                .min(remaining.len());
            self.pending.extend_from_slice(&remaining[..take]);
            remaining = &remaining[take..];
            counters = counters.observe_residency(self.pending.len() as u64);
            if self.pending.len() as u64 > window.max_resident_bytes() {
                return Err(
                    BlobStreamingIngestDenial::SourceWindowExceedsResidentEnvelope {
                        window_bytes: self.pending.len() as u64,
                        envelope_bytes: window.max_resident_bytes(),
                    },
                );
            }
            if self.pending.len() == self.chunk_size {
                let written = writer.write_streaming_chunk(self.ordinal, &self.pending)?;
                self.require_written_payload_matches_pending_source(&written)?;
                admission = self.push_written_chunk(admission, written, &mut counters)?;
            }
            if self.start_offset > declared_total_bytes {
                return Err(
                    BlobStreamingIngestDenial::WholeObjectMaterializationRejected {
                        bytes: self.start_offset,
                    },
                );
            }
        }
        Ok(BlobStreamingChunkingStep {
            admission,
            counters,
        })
    }

    fn finish<W>(
        mut self,
        mut admission: BlobChunkSequenceAdmission,
        writer: &mut W,
        mut counters: BlobStreamingIngestCounterSnapshot,
    ) -> Result<BlobStreamingChunkingStep, BlobStreamingIngestDenial>
    where
        W: BlobStreamingChunkWriter,
    {
        if !self.pending.is_empty() {
            let written = writer.write_streaming_chunk(self.ordinal, &self.pending)?;
            self.require_written_payload_matches_pending_source(&written)?;
            admission = self.push_written_chunk(admission, written, &mut counters)?;
        }
        Ok(BlobStreamingChunkingStep {
            admission,
            counters,
        })
    }

    fn push_written_chunk(
        &mut self,
        admission: BlobChunkSequenceAdmission,
        written: crate::BlobStreamingWrittenChunk,
        counters: &mut BlobStreamingIngestCounterSnapshot,
    ) -> Result<BlobChunkSequenceAdmission, BlobStreamingIngestDenial> {
        let (payload, backend_write) = written.into_parts();
        if backend_write.ordinal() != self.ordinal.get() {
            return Err(BlobStreamingIngestDenial::BackendWriteOrdinalMismatch {
                expected: self.ordinal.get(),
                actual: backend_write.ordinal(),
            });
        }
        let bytes = payload.bytes_checked();
        if backend_write.bytes_written() != bytes {
            return Err(BlobStreamingIngestDenial::BackendWriteBytesMismatch {
                expected: bytes,
                actual: backend_write.bytes_written(),
            });
        }
        let admission = admission.push_payload(self.start_offset, payload)?;
        self.start_offset += bytes;
        self.ordinal = self.ordinal.next();
        self.pending.clear();
        *counters = (*counters).observe_chunk_read().observe_chunk_write();
        Ok(admission)
    }

    fn require_written_payload_matches_pending_source(
        &self,
        written: &crate::BlobStreamingWrittenChunk,
    ) -> Result<(), BlobStreamingIngestDenial> {
        if written.payload_bytes() == self.pending.as_slice() {
            Ok(())
        } else {
            Err(BlobStreamingIngestDenial::BackendWritePayloadMismatch {
                ordinal: self.ordinal.get(),
            })
        }
    }
}
