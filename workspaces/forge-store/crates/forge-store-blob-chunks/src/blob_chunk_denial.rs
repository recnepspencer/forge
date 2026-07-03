use forge_foundational::CanonicalEquivalenceBasis;
use forge_store_readiness::S51SecurityScopeReadinessFamily;
use forge_store_security::{
    StoreAuthenticityRequirement, StoreCustodyPosture, StoreKeyScope, StoreTenantScope,
};

use crate::{BlobChunkDedupeCounterSnapshot, BlobChunkScopeCounterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkSecurityScopeDenial {
    WrongReadinessFamily {
        actual: S51SecurityScopeReadinessFamily,
        counters: BlobChunkScopeCounterSnapshot,
    },
    WrongKeyScope {
        actual: StoreKeyScope,
        counters: BlobChunkScopeCounterSnapshot,
    },
    WrongTenantScope {
        actual: StoreTenantScope,
        counters: BlobChunkScopeCounterSnapshot,
    },
    WrongAuthenticityRequirement {
        actual: StoreAuthenticityRequirement,
        counters: BlobChunkScopeCounterSnapshot,
    },
    UnsupportedCustodyPosture {
        actual: StoreCustodyPosture,
        counters: BlobChunkScopeCounterSnapshot,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkStreamingDenial {
    EmptyStreamingWindow,
    WindowDigestMismatch,
    WholeObjectResidencyRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkDedupeAdmissionDenial {
    ContentDigestMismatch {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    MissingFoundationalCanonicalEquivalence {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    DigestOnlyEquivalenceRejected,
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
    UnboundFoundationalEquivalence {
        counters: BlobChunkDedupeCounterSnapshot,
    },
    UnsupportedFoundationalEquivalenceBasis {
        basis: CanonicalEquivalenceBasis,
    },
}
