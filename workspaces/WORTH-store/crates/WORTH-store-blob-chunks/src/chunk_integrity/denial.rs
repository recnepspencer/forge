use worth_store_physical_format::PhysicalChunkChecksumDenial;

use crate::BlobChunkIntegrityCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkIntegrityDenial {
    EmptyChunkingRule,
    EmptyByteWindow,
    WholeObjectWindow,
    WindowExceedsChunkRule,
    NonCanonicalInteriorChunk {
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    WindowChecksumLengthMismatch {
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    UnexpectedWindowOffset {
        expected: u64,
        actual: u64,
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    MissingTailChunk {
        expected_total_bytes: u64,
        actual_total_bytes: u64,
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    DuplicateOrReorderedChunk {
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    ChecksumOnlyEvidenceRejected {
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    DigestOnlyEvidenceRejected {
        counters: BlobChunkIntegrityCounterSnapshot,
    },
    PhysicalChecksumDenied(PhysicalChunkChecksumDenial),
}

pub fn reject_checksum_only_evidence_as_blob_chunk_integrity(
    _checksum: worth_store_physical_format::PhysicalChunkChecksumWitness,
) -> BlobChunkIntegrityDenial {
    BlobChunkIntegrityDenial::ChecksumOnlyEvidenceRejected {
        counters: BlobChunkIntegrityCounterSnapshot::start().record_checksum_only_denial(),
    }
}

pub fn reject_digest_only_evidence_as_blob_chunk_integrity(
    _digest: worth_store_contracts::StableDigest,
) -> BlobChunkIntegrityDenial {
    BlobChunkIntegrityDenial::DigestOnlyEvidenceRejected {
        counters: BlobChunkIntegrityCounterSnapshot::start().record_digest_only_denial(),
    }
}
