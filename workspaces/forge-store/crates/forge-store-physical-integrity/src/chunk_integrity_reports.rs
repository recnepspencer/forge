use crate::{ChunkIntegrityCounters, PhysicalScopeBasis};
use forge_store_physical_format::PhysicalReferenceScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkIntegrityInputIdentity {
    scope: PhysicalReferenceScope,
    object_bytes: u64,
    window_bytes: u64,
}

impl ChunkIntegrityInputIdentity {
    pub(crate) const fn new(
        scope: PhysicalReferenceScope,
        object_bytes: u64,
        window_bytes: u64,
    ) -> Self {
        Self {
            scope,
            object_bytes,
            window_bytes,
        }
    }

    pub const fn scope(self) -> PhysicalReferenceScope {
        self.scope
    }

    pub const fn object_bytes(self) -> u64 {
        self.object_bytes
    }

    pub const fn window_bytes(self) -> u64 {
        self.window_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkIntegrityLifecycleClaims;

impl ChunkIntegrityLifecycleClaims {
    pub(crate) const fn none() -> Self {
        Self
    }

    pub const fn claims_dedupe_correctness(self) -> bool {
        false
    }

    pub const fn claims_reachability(self) -> bool {
        false
    }

    pub const fn claims_resumability(self) -> bool {
        false
    }

    pub const fn claims_blob_retention(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkIntegrityReport {
    basis: PhysicalScopeBasis,
    input_identity: ChunkIntegrityInputIdentity,
    counters: ChunkIntegrityCounters,
    lifecycle_claims: ChunkIntegrityLifecycleClaims,
}

impl ChunkIntegrityReport {
    pub(crate) fn new(
        basis: PhysicalScopeBasis,
        object_bytes: u64,
        window_bytes: u64,
        counters: ChunkIntegrityCounters,
    ) -> Self {
        let input_identity =
            ChunkIntegrityInputIdentity::new(basis.scope(), object_bytes, window_bytes);
        Self {
            basis,
            input_identity,
            counters,
            lifecycle_claims: ChunkIntegrityLifecycleClaims::none(),
        }
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn input_identity(&self) -> ChunkIntegrityInputIdentity {
        self.input_identity
    }

    pub const fn counters(&self) -> ChunkIntegrityCounters {
        self.counters
    }

    pub const fn lifecycle_claims(&self) -> ChunkIntegrityLifecycleClaims {
        self.lifecycle_claims
    }
}
