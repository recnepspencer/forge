#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlobChunkScopeCounterSnapshot {
    readiness_inputs: u64,
    admitted_scope_consumed: u64,
    denials: u64,
}

impl BlobChunkScopeCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            readiness_inputs: 1,
            admitted_scope_consumed: 0,
            denials: 0,
        }
    }

    pub(crate) const fn admitted(self) -> Self {
        Self {
            admitted_scope_consumed: self.admitted_scope_consumed + 1,
            ..self
        }
    }

    pub(crate) const fn denied(self) -> Self {
        Self {
            denials: self.denials + 1,
            ..self
        }
    }

    pub const fn readiness_inputs(self) -> u64 {
        self.readiness_inputs
    }

    pub const fn admitted_scope_consumed(self) -> u64 {
        self.admitted_scope_consumed
    }

    pub const fn denials(self) -> u64 {
        self.denials
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobChunkDedupeCounterSnapshot {
    digest_comparisons: u64,
    foundational_equivalence_comparisons: u64,
    same_scope_admissions: u64,
    cross_scope_denials: u64,
    digest_only_denials: u64,
}

impl BlobChunkDedupeCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            digest_comparisons: 1,
            foundational_equivalence_comparisons: 0,
            same_scope_admissions: 0,
            cross_scope_denials: 0,
            digest_only_denials: 0,
        }
    }

    pub(crate) const fn record_equivalence_comparison(self) -> Self {
        Self {
            foundational_equivalence_comparisons: self.foundational_equivalence_comparisons + 1,
            ..self
        }
    }

    pub(crate) const fn record_same_scope_admission(self) -> Self {
        Self {
            same_scope_admissions: self.same_scope_admissions + 1,
            ..self
        }
    }

    pub(crate) const fn record_cross_scope_denial(self) -> Self {
        Self {
            cross_scope_denials: self.cross_scope_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_digest_only_denial(self) -> Self {
        Self {
            digest_only_denials: self.digest_only_denials + 1,
            ..self
        }
    }

    pub const fn digest_comparisons(self) -> u64 {
        self.digest_comparisons
    }

    pub const fn foundational_equivalence_comparisons(self) -> u64 {
        self.foundational_equivalence_comparisons
    }

    pub const fn same_scope_admissions(self) -> u64 {
        self.same_scope_admissions
    }

    pub const fn cross_scope_denials(self) -> u64 {
        self.cross_scope_denials
    }

    pub const fn digest_only_denials(self) -> u64 {
        self.digest_only_denials
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
