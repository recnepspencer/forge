#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkStreamingCounterSnapshot {
    windows_observed: u64,
    bytes_observed: u64,
    max_resident_windows: u64,
    ingest_operations: u64,
    verification_operations: u64,
    export_read_preparations: u64,
    tier_movements: u64,
    reclaim_preparations: u64,
}

impl BlobChunkStreamingCounterSnapshot {
    pub(crate) const fn for_operation(kind: crate::BlobChunkStreamingOperationKind) -> Self {
        Self {
            windows_observed: 0,
            bytes_observed: 0,
            max_resident_windows: 0,
            ingest_operations: matches_u64(kind, crate::BlobChunkStreamingOperationKind::Ingest),
            verification_operations: matches_u64(
                kind,
                crate::BlobChunkStreamingOperationKind::Verification,
            ),
            export_read_preparations: matches_u64(
                kind,
                crate::BlobChunkStreamingOperationKind::ExportReadPreparation,
            ),
            tier_movements: matches_u64(kind, crate::BlobChunkStreamingOperationKind::TierMovement),
            reclaim_preparations: matches_u64(
                kind,
                crate::BlobChunkStreamingOperationKind::ReclaimPreparation,
            ),
        }
    }

    pub(crate) const fn observe_window(self, bytes: u64) -> Self {
        Self {
            windows_observed: self.windows_observed + 1,
            bytes_observed: self.bytes_observed + bytes,
            max_resident_windows: 1,
            ..self
        }
    }

    pub const fn windows_observed(self) -> u64 {
        self.windows_observed
    }

    pub const fn bytes_observed(self) -> u64 {
        self.bytes_observed
    }

    pub const fn max_resident_windows(self) -> u64 {
        self.max_resident_windows
    }

    pub const fn ingest_operations(self) -> u64 {
        self.ingest_operations
    }

    pub const fn verification_operations(self) -> u64 {
        self.verification_operations
    }

    pub const fn export_read_preparations(self) -> u64 {
        self.export_read_preparations
    }

    pub const fn tier_movements(self) -> u64 {
        self.tier_movements
    }

    pub const fn reclaim_preparations(self) -> u64 {
        self.reclaim_preparations
    }
}

const fn matches_u64(
    left: crate::BlobChunkStreamingOperationKind,
    right: crate::BlobChunkStreamingOperationKind,
) -> u64 {
    if left as u8 == right as u8 {
        1
    } else {
        0
    }
}