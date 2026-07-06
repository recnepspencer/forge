use forge_foundational::CanonicalEquivalenceBasis;
use forge_store_security::{StoreKeyScope, StoreTenantScope};

use crate::dedupe::verification::BlobChunkCollisionVerificationReceipt;
use crate::{
    BlobChunkDedupeCollisionPosture, BlobChunkDedupeCounterSnapshot, BlobChunkDedupePolicy,
    BlobChunkQuarantine,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobChunkDedupeAdmissionDenial {
    ContentDigestMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    MissingFoundationalCanonicalEquivalence {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DigestOnlyEquivalenceRejected,
    CanonicalRootComparisonRequired {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnboundRootCanonicalComparison {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    ChunkByteComparisonRequired {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnboundByteComparison {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    ByteComparisonPayloadMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DigestCollisionDenied {
        receipt: BlobChunkCollisionVerificationReceipt,
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    CrossTenantScopeRequiresExplicitEquivalence {
        left: StoreTenantScope,
        right: StoreTenantScope,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    CrossKeyScopeRequiresExplicitEquivalence {
        left: StoreKeyScope,
        right: StoreKeyScope,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    CrossScopeSecurityWitnessMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    QuarantinedChunkDenied {
        quarantine: BlobChunkQuarantine,
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DedupeIndexPartitioned {
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    ChunkRewrittenUnderNewDigestBasis {
        posture: BlobChunkDedupeCollisionPosture,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DedupePolicyDenied {
        policy: BlobChunkDedupePolicy,
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DedupeReferenceEdgeMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnboundFoundationalEquivalence {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnsupportedFoundationalEquivalenceBasis {
        basis: CanonicalEquivalenceBasis,
    },
}