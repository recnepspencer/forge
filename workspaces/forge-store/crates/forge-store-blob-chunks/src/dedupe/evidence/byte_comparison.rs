use forge_store_physical_format::PhysicalChunkPayloadIntegrityWitness;

use super::candidate::BlobChunkDedupeCandidate;
use crate::{
    BlobChunkDedupeAdmissionDenial, BlobChunkDedupeCounterSnapshot, BlobChunkIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkDedupeByteComparison {
    existing_identity: BlobChunkIdentity,
    candidate_identity: BlobChunkIdentity,
    bytes_compared: u64,
    equivalent: bool,
    counters: BlobChunkDedupeCounterSnapshot,
}

impl BlobChunkDedupeByteComparison {
    pub fn compare_chunk_payloads(
        existing: &BlobChunkDedupeCandidate,
        candidate: &BlobChunkDedupeCandidate,
        existing_payload: &PhysicalChunkPayloadIntegrityWitness,
        candidate_payload: &PhysicalChunkPayloadIntegrityWitness,
    ) -> Result<Self, BlobChunkDedupeAdmissionDenial> {
        if existing.proof().checksum() != existing_payload.checksum()
            || candidate.proof().checksum() != candidate_payload.checksum()
            || existing.proof().byte_range().len() != existing_payload.bytes_checked()
            || candidate.proof().byte_range().len() != candidate_payload.bytes_checked()
        {
            return Err(
                BlobChunkDedupeAdmissionDenial::ByteComparisonPayloadMismatch {
                    counters: BlobChunkDedupeCounterSnapshot::start().record_dedupe_miss(),
                },
            );
        }

        let bytes_compared = existing_payload
            .bytes_checked()
            .max(candidate_payload.bytes_checked());
        Ok(Self {
            existing_identity: existing.identity().clone(),
            candidate_identity: candidate.identity().clone(),
            bytes_compared,
            equivalent: existing_payload.payload_bytes() == candidate_payload.payload_bytes(),
            counters: BlobChunkDedupeCounterSnapshot::start()
                .record_collision_probe()
                .record_byte_verify_probe(),
        })
    }

    pub(crate) fn matches_candidate_identities(
        &self,
        existing: &BlobChunkIdentity,
        candidate: &BlobChunkIdentity,
    ) -> bool {
        &self.existing_identity == existing && &self.candidate_identity == candidate
    }

    pub const fn is_equivalent(&self) -> bool {
        self.equivalent
    }

    pub const fn bytes_compared(&self) -> u64 {
        self.bytes_compared
    }

    pub const fn counters(&self) -> BlobChunkDedupeCounterSnapshot {
        self.counters
    }

    pub(crate) fn counters_for_collision_denial(self) -> BlobChunkDedupeCounterSnapshot {
        self.counters.record_collision_denial()
    }
}