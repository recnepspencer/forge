use forge_store_contracts::StableDigest;
use forge_store_physical_format::PhysicalChunkChecksumWitness;

use crate::BlobChunkRootCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkRootPublicationDenial {
    CanonicalBasisConstructionDenied {
        counters: BlobChunkRootCounterSnapshot,
    },
    CanonicalDigestDerivationDenied {
        counters: BlobChunkRootCounterSnapshot,
    },
    CanonicalComparisonPreparationDenied {
        counters: BlobChunkRootCounterSnapshot,
    },
    CanonicalComparisonMismatched {
        counters: BlobChunkRootCounterSnapshot,
    },
    CanonicalComparisonUnsupported {
        counters: BlobChunkRootCounterSnapshot,
    },
    ChecksumOnlyEvidenceRejected {
        counters: BlobChunkRootCounterSnapshot,
    },
    DigestOnlyEvidenceRejected {
        counters: BlobChunkRootCounterSnapshot,
    },
}

pub fn reject_checksum_only_evidence_as_chunk_root_publication(
    _checksum: PhysicalChunkChecksumWitness,
) -> BlobChunkRootPublicationDenial {
    BlobChunkRootPublicationDenial::ChecksumOnlyEvidenceRejected {
        counters: BlobChunkRootCounterSnapshot::start().record_denial(),
    }
}

pub fn reject_digest_only_evidence_as_chunk_root_publication(
    _digest: StableDigest,
) -> BlobChunkRootPublicationDenial {
    BlobChunkRootPublicationDenial::DigestOnlyEvidenceRejected {
        counters: BlobChunkRootCounterSnapshot::start().record_denial(),
    }
}
