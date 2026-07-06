use forge_store_budgets::{AllocationEnvelopeSet, AllocationScope, CounterEvidenceStrength};
use forge_store_buffer_pool::{AllocationReceipt, AllocationRequestKind};
use forge_store_contracts::StableDigest;

use super::performance::{
    counter_backed_streaming_read_performance_receipt,
    BlobStreamingReadCounterBackedPerformanceReceipt,
};
use crate::{
    BlobChunkOrdinal, BlobChunkProofLeaf, BlobChunkQuarantine, BlobCorruptedChunkLocalization,
    BlobCorruptionGuard, BlobCorruptionPlacementClass, BlobGeneration, BlobObjectId,
    BlobQuarantineAuthority, BlobStreamingReadAdmission, BlobStreamingReadCounterSnapshot,
    BlobStreamingReadDenial, BlobStreamingReadObservation, BlobStreamingReadRequest,
    BlobStreamingReadWindow, ChunkTreeRoot, LogicalContentDigest,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobStreamingVerifiedRead {
    object_id: BlobObjectId,
    generation: BlobGeneration,
    chunk_tree_root: ChunkTreeRoot,
    logical_content_digest: LogicalContentDigest,
    counters: BlobStreamingReadCounterSnapshot,
    performance: BlobStreamingReadCounterBackedPerformanceReceipt,
}

impl BlobStreamingVerifiedRead {
    #[cfg(test)]
    pub(crate) fn for_movement_certification_test(
        object_id: BlobObjectId,
        generation: BlobGeneration,
        chunk_tree_root: ChunkTreeRoot,
        logical_content_digest: LogicalContentDigest,
        bytes_read: u64,
    ) -> Self {
        let counters = BlobStreamingReadCounterSnapshot::start(CounterEvidenceStrength::Exact)
            .observe_read_window(bytes_read)
            .record_verified_chunk();
        let performance = counter_backed_streaming_read_performance_receipt(counters);
        Self {
            object_id,
            generation,
            chunk_tree_root,
            logical_content_digest,
            counters,
            performance,
        }
    }

    pub fn verify_bounded(
        request: BlobStreamingReadRequest,
        window: BlobStreamingReadWindow,
        allocation: AllocationReceipt,
        envelopes: AllocationEnvelopeSet,
        admission: BlobStreamingReadAdmission,
        quarantine_authority: BlobQuarantineAuthority,
        observations: impl IntoIterator<Item = BlobStreamingReadObservation>,
        counter_strength: CounterEvidenceStrength,
    ) -> Result<Self, BlobStreamingReadDenial> {
        if !counter_strength.satisfies(CounterEvidenceStrength::Exact) {
            return Err(BlobStreamingReadDenial::MissingExactCounters {
                actual: counter_strength,
            });
        }
        require_streaming_allocation(allocation, envelopes)?;
        let mut counters = admission
            .seed_counters(BlobStreamingReadCounterSnapshot::start(counter_strength))
            .record_allocation();
        require_stable_read_bytes(
            admission,
            request.frontier().proof_frontier().total_bytes(),
            counters,
        )?;
        let mut verifier = StreamingReadVerifier::new(request, window, quarantine_authority);
        for observation in observations {
            verifier.observe(observation, &mut counters)?;
        }
        verifier.finish(counters)
    }

    pub const fn object_id(&self) -> &BlobObjectId {
        &self.object_id
    }

    pub const fn generation(&self) -> BlobGeneration {
        self.generation
    }

    pub const fn chunk_tree_root(&self) -> &ChunkTreeRoot {
        &self.chunk_tree_root
    }

    pub const fn logical_content_digest(&self) -> &LogicalContentDigest {
        &self.logical_content_digest
    }

    pub const fn counters(&self) -> BlobStreamingReadCounterSnapshot {
        self.counters
    }

    pub const fn counter_backed_performance_receipt(
        &self,
    ) -> &BlobStreamingReadCounterBackedPerformanceReceipt {
        &self.performance
    }
}

fn require_streaming_allocation(
    allocation: AllocationReceipt,
    envelopes: AllocationEnvelopeSet,
) -> Result<(), BlobStreamingReadDenial> {
    if allocation.scope() != AllocationScope::Streaming {
        return Err(BlobStreamingReadDenial::AllocationScopeMismatch);
    }
    if allocation.kind() != AllocationRequestKind::StreamingWindow {
        return Err(BlobStreamingReadDenial::AllocationKindMismatch);
    }
    let envelope_bytes = envelopes.budget(AllocationScope::Streaming).as_bytes();
    if allocation.bytes() > envelope_bytes {
        return Err(BlobStreamingReadDenial::ResidentEnvelopeExceeded {
            peak_resident_bytes: allocation.bytes(),
            envelope_bytes,
        });
    }
    let streaming_counters = allocation.counters().scope(AllocationScope::Streaming);
    if streaming_counters.allocated_bytes() == 0 {
        return Err(BlobStreamingReadDenial::AllocationCountersHidden);
    }
    Ok(())
}

fn require_stable_read_bytes(
    admission: BlobStreamingReadAdmission,
    expected: u64,
    counters: BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    let actual = admission.stable_read().counters().guarded_bytes();
    if actual < expected {
        return Err(BlobStreamingReadDenial::StableReadBytesInsufficient {
            expected,
            actual,
            counters: counters.record_stale_read_denial(),
        });
    }
    Ok(())
}

struct StreamingReadVerifier {
    request: BlobStreamingReadRequest,
    window: BlobStreamingReadWindow,
    quarantine_authority: Option<BlobQuarantineAuthority>,
    next_index: usize,
    logical_content_basis: u64,
}

impl StreamingReadVerifier {
    fn new(
        request: BlobStreamingReadRequest,
        window: BlobStreamingReadWindow,
        quarantine_authority: BlobQuarantineAuthority,
    ) -> Self {
        Self {
            request,
            window,
            quarantine_authority: Some(quarantine_authority),
            next_index: 0,
            logical_content_basis: accumulator_seed("logical-content"),
        }
    }

    fn observe(
        &mut self,
        observation: BlobStreamingReadObservation,
        counters: &mut BlobStreamingReadCounterSnapshot,
    ) -> Result<(), BlobStreamingReadDenial> {
        let expected = match self.expected_leaf() {
            Some(expected) => expected.clone(),
            None => {
                return Err(match observation_ordinal(&observation) {
                    Some(ordinal) => BlobStreamingReadDenial::ExtraChunk {
                        ordinal,
                        counters: counters.record_order_denial(),
                    },
                    None => BlobStreamingReadDenial::MissingChunk {
                        ordinal: BlobChunkOrdinal::first(),
                        counters: counters.record_missing_chunk_denial(),
                    },
                });
            }
        };
        match observation {
            BlobStreamingReadObservation::Chunk(chunk) => {
                reject_reordered_chunk(&expected, chunk.ordinal(), counters)?;
                reject_range_mismatch(&expected, chunk.byte_range(), counters)?;
                if chunk.payload().bytes_checked() > self.window.max_resident_bytes() {
                    return Err(BlobStreamingReadDenial::ReadWindowExceedsResidentEnvelope {
                        window_bytes: chunk.payload().bytes_checked(),
                        envelope_bytes: self.window.max_resident_bytes(),
                    });
                }
                *counters = counters.observe_read_window(chunk.payload().bytes_checked());
                reject_corrupted_chunk(
                    &self.request,
                    &mut self.quarantine_authority,
                    &expected,
                    &chunk,
                    counters,
                )?;
                self.logical_content_basis =
                    accumulate_bytes(self.logical_content_basis, chunk.payload().payload_bytes());
                *counters = counters.record_verified_chunk();
                self.next_index += 1;
                Ok(())
            }
            BlobStreamingReadObservation::ColdUnavailable { ordinal, .. } => {
                reject_reordered_chunk(&expected, ordinal, counters)?;
                *counters = counters.record_cold_unavailable_denial();
                Err(BlobStreamingReadDenial::ColdChunkUnavailable {
                    ordinal,
                    counters: *counters,
                })
            }
        }
    }

    fn finish(
        self,
        counters: BlobStreamingReadCounterSnapshot,
    ) -> Result<BlobStreamingVerifiedRead, BlobStreamingReadDenial> {
        let frontier = self.request.frontier().proof_frontier();
        if self.next_index < frontier.ordered_leaves().len() {
            let counters = counters.record_missing_chunk_denial();
            return Err(BlobStreamingReadDenial::MissingChunk {
                ordinal: frontier.ordered_leaves()[self.next_index].ordinal(),
                counters,
            });
        }
        let digest = LogicalContentDigest::from_declared_digest(accumulated_digest(
            "logical-content",
            self.logical_content_basis,
            frontier.total_bytes(),
            frontier.chunk_count(),
        ));
        if &digest != self.request.logical_content_digest() {
            return Err(BlobStreamingReadDenial::LogicalContentDigestMismatch);
        }
        let performance = counter_backed_streaming_read_performance_receipt(counters);
        Ok(BlobStreamingVerifiedRead {
            object_id: self.request.object_id().clone(),
            generation: self.request.generation(),
            chunk_tree_root: self.request.chunk_tree_root().clone(),
            logical_content_digest: digest,
            counters,
            performance,
        })
    }

    fn expected_leaf(&self) -> Option<&BlobChunkProofLeaf> {
        self.request
            .frontier()
            .proof_frontier()
            .ordered_leaves()
            .get(self.next_index)
    }
}

fn reject_reordered_chunk(
    expected: &BlobChunkProofLeaf,
    actual: BlobChunkOrdinal,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    if expected.ordinal() == actual {
        Ok(())
    } else {
        *counters = counters.record_order_denial();
        Err(BlobStreamingReadDenial::ReorderedChunk {
            expected: expected.ordinal(),
            actual,
            counters: *counters,
        })
    }
}

fn reject_range_mismatch(
    expected: &BlobChunkProofLeaf,
    actual: crate::BlobChunkByteRange,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    if expected.byte_range() == actual {
        Ok(())
    } else {
        *counters = counters.record_order_denial();
        Err(BlobStreamingReadDenial::ChunkRangeMismatch {
            ordinal: expected.ordinal(),
            expected: expected.byte_range(),
            actual,
            counters: *counters,
        })
    }
}

fn reject_corrupted_chunk(
    request: &BlobStreamingReadRequest,
    quarantine_authority: &mut Option<BlobQuarantineAuthority>,
    expected: &BlobChunkProofLeaf,
    actual: &crate::BlobStreamingReadObservedChunk,
    counters: &mut BlobStreamingReadCounterSnapshot,
) -> Result<(), BlobStreamingReadDenial> {
    if expected.checksum_digest() == actual.payload().checksum().checksum().digest() {
        Ok(())
    } else {
        *counters = counters.record_corrupt_chunk_denial();
        let localization = BlobCorruptedChunkLocalization::from_streaming_read_request(
            request,
            expected.ordinal(),
            BlobCorruptionPlacementClass::LocalPhysical,
        )
        .map_err(BlobStreamingReadDenial::CorruptionReferenceEdgeMismatch)?;
        let quarantine = BlobChunkQuarantine::seal(
            localization,
            quarantine_authority
                .take()
                .expect("corruption verifier has a single quarantine authority"),
        );
        let guard = BlobCorruptionGuard::from_quarantine(quarantine);
        let _denial = guard.deny_verified_read_publication();
        Err(BlobStreamingReadDenial::CorruptedChunk {
            ordinal: expected.ordinal(),
            quarantine: guard.quarantine().clone(),
            counters: *counters,
        })
    }
}

fn observation_ordinal(observation: &BlobStreamingReadObservation) -> Option<BlobChunkOrdinal> {
    match observation {
        BlobStreamingReadObservation::Chunk(chunk) => Some(chunk.ordinal()),
        BlobStreamingReadObservation::ColdUnavailable { ordinal, .. } => Some(*ordinal),
    }
}

fn accumulated_digest(
    lane: &str,
    accumulator: u64,
    total_bytes: u64,
    chunk_count: u64,
) -> StableDigest {
    let evidence = format!("{accumulator:016x}:{total_bytes}:{chunk_count}");
    stable_digest_for_read(
        lane,
        "s7.sequence.v1",
        BlobChunkOrdinal::first(),
        crate::BlobChunkByteRange::new(chunk_count, evidence.len() as u64)
            .expect("finalized read sequence has nonempty evidence"),
        &evidence,
    )
}

fn stable_digest_for_read(
    domain: &str,
    rule: &str,
    ordinal: BlobChunkOrdinal,
    range: crate::BlobChunkByteRange,
    bytes: &str,
) -> StableDigest {
    let mut hash = accumulator_seed(domain);
    hash = accumulate_bytes(hash, rule.as_bytes());
    hash = accumulate_u64(hash, ordinal.get());
    hash = accumulate_u64(hash, range.start());
    hash = accumulate_u64(hash, range.len());
    hash = accumulate_bytes(hash, bytes.as_bytes());
    StableDigest::new(format!("s7:{domain}:{hash:016x}",))
        .expect("blob read stable digest should be nonempty")
}

fn accumulator_seed(lane: &str) -> u64 {
    accumulate_bytes(0xcbf2_9ce4_8422_2325, lane.as_bytes())
}

fn accumulate_bytes(mut hash: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn accumulate_u64(hash: u64, value: u64) -> u64 {
    accumulate_bytes(hash, &value.to_le_bytes())
}
