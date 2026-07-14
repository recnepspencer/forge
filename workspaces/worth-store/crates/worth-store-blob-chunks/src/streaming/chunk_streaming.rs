use worth_store_contracts::StableDigest;

use crate::{
    BlobChunkIdentity, BlobChunkSecurityScope, BlobChunkStreamingCounterSnapshot,
    BlobChunkStreamingDenial,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobChunkStreamingOperationKind {
    Ingest,
    Verification,
    ExportReadPreparation,
    TierMovement,
    ReclaimPreparation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkStreamingOperation {
    kind: BlobChunkStreamingOperationKind,
    scope: BlobChunkSecurityScope,
    counters: BlobChunkStreamingCounterSnapshot,
}

impl BlobChunkStreamingOperation {
    pub fn ingest(scope: BlobChunkSecurityScope) -> Self {
        Self::new(BlobChunkStreamingOperationKind::Ingest, scope)
    }

    pub fn verification(scope: BlobChunkSecurityScope) -> Self {
        Self::new(BlobChunkStreamingOperationKind::Verification, scope)
    }

    pub fn export_read_preparation(scope: BlobChunkSecurityScope) -> Self {
        Self::new(
            BlobChunkStreamingOperationKind::ExportReadPreparation,
            scope,
        )
    }

    pub fn tier_movement(scope: BlobChunkSecurityScope) -> Self {
        Self::new(BlobChunkStreamingOperationKind::TierMovement, scope)
    }

    pub fn reclaim_preparation(scope: BlobChunkSecurityScope) -> Self {
        Self::new(BlobChunkStreamingOperationKind::ReclaimPreparation, scope)
    }

    pub fn observe_window(
        self,
        window: BlobChunkStreamingWindow,
    ) -> Result<BlobChunkStreamingObservation, BlobChunkStreamingDenial> {
        if window.identity.chunk_digest() != window.content_digest() {
            return Err(BlobChunkStreamingDenial::WindowDigestMismatch);
        }

        let bytes_observed = window.bytes_observed();
        Ok(BlobChunkStreamingObservation {
            kind: self.kind,
            scope: self.scope,
            window,
            counters: self.counters.observe_window(bytes_observed),
        })
    }

    fn new(kind: BlobChunkStreamingOperationKind, scope: BlobChunkSecurityScope) -> Self {
        Self {
            kind,
            scope,
            counters: BlobChunkStreamingCounterSnapshot::for_operation(kind),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobChunkStreamingWindow {
    identity: BlobChunkIdentity,
    content_digest: StableDigest,
    bytes_observed: u64,
    residency: BlobChunkStreamingResidencyProof,
}

impl BlobChunkStreamingWindow {
    pub fn new(
        identity: BlobChunkIdentity,
        content_digest: StableDigest,
        residency: BlobChunkStreamingResidencyProof,
    ) -> Result<Self, BlobChunkStreamingDenial> {
        let bytes_observed = residency.window_bytes();
        if bytes_observed == 0 {
            return Err(BlobChunkStreamingDenial::EmptyStreamingWindow);
        }

        Ok(Self {
            identity,
            content_digest,
            bytes_observed,
            residency,
        })
    }

    pub const fn identity(&self) -> &BlobChunkIdentity {
        &self.identity
    }

    pub const fn content_digest(&self) -> &StableDigest {
        &self.content_digest
    }

    pub const fn bytes_observed(&self) -> u64 {
        self.bytes_observed
    }

    pub const fn residency(&self) -> BlobChunkStreamingResidencyProof {
        self.residency
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkStreamingResidencyProof {
    object_bytes: u64,
    window_bytes: u64,
}

impl BlobChunkStreamingResidencyProof {
    pub const fn bounded_window(
        object_bytes: u64,
        window_bytes: u64,
    ) -> Result<Self, BlobChunkStreamingDenial> {
        if window_bytes == 0 {
            return Err(BlobChunkStreamingDenial::EmptyStreamingWindow);
        }
        if window_bytes >= object_bytes {
            return Err(BlobChunkStreamingDenial::WholeObjectResidencyRequired);
        }

        Ok(Self {
            object_bytes,
            window_bytes,
        })
    }

    pub const fn object_bytes(self) -> u64 {
        self.object_bytes
    }

    pub const fn window_bytes(self) -> u64 {
        self.window_bytes
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlobChunkStreamingObservation {
    kind: BlobChunkStreamingOperationKind,
    scope: BlobChunkSecurityScope,
    window: BlobChunkStreamingWindow,
    counters: BlobChunkStreamingCounterSnapshot,
}

impl BlobChunkStreamingObservation {
    pub fn complete_without_whole_object_residency(self) -> Result<Self, BlobChunkStreamingDenial> {
        if self.counters.max_resident_windows() > 1 {
            return Err(BlobChunkStreamingDenial::WholeObjectResidencyRequired);
        }

        Ok(self)
    }

    pub const fn kind(&self) -> BlobChunkStreamingOperationKind {
        self.kind
    }

    pub const fn scope(&self) -> &BlobChunkSecurityScope {
        &self.scope
    }

    pub const fn window(&self) -> &BlobChunkStreamingWindow {
        &self.window
    }

    pub const fn counters(&self) -> BlobChunkStreamingCounterSnapshot {
        self.counters
    }
}
