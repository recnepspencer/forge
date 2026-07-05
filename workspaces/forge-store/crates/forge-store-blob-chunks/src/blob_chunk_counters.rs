#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlobChunkScopeCounterSnapshot {
    readiness_inputs: u64,
    admitted_scope_consumed: u64,
    denials: u64,
    key_scope_preservations: u64,
    key_version_preservations: u64,
    tenant_scope_preservations: u64,
    authenticity_preservations: u64,
    custody_preservations: u64,
    metadata_witnesses_issued: u64,
    hostile_metadata_denials: u64,
}

impl BlobChunkScopeCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            readiness_inputs: 1,
            admitted_scope_consumed: 0,
            denials: 0,
            key_scope_preservations: 0,
            key_version_preservations: 0,
            tenant_scope_preservations: 0,
            authenticity_preservations: 0,
            custody_preservations: 0,
            metadata_witnesses_issued: 0,
            hostile_metadata_denials: 0,
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

    pub(crate) const fn denied_hostile_metadata(self) -> Self {
        Self {
            denials: self.denials + 1,
            hostile_metadata_denials: self.hostile_metadata_denials + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_key_scope(self) -> Self {
        Self {
            key_scope_preservations: self.key_scope_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_key_version(self) -> Self {
        Self {
            key_version_preservations: self.key_version_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_tenant_scope(self) -> Self {
        Self {
            tenant_scope_preservations: self.tenant_scope_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_authenticity(self) -> Self {
        Self {
            authenticity_preservations: self.authenticity_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn preserve_custody(self) -> Self {
        Self {
            custody_preservations: self.custody_preservations + 1,
            ..self
        }
    }

    pub(crate) const fn issue_metadata_witness(self) -> Self {
        Self {
            metadata_witnesses_issued: self.metadata_witnesses_issued + 1,
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

    pub const fn key_scope_preservations(self) -> u64 {
        self.key_scope_preservations
    }

    pub const fn key_version_preservations(self) -> u64 {
        self.key_version_preservations
    }

    pub const fn tenant_scope_preservations(self) -> u64 {
        self.tenant_scope_preservations
    }

    pub const fn authenticity_preservations(self) -> u64 {
        self.authenticity_preservations
    }

    pub const fn custody_preservations(self) -> u64 {
        self.custody_preservations
    }

    pub const fn metadata_witnesses_issued(self) -> u64 {
        self.metadata_witnesses_issued
    }

    pub const fn hostile_metadata_denials(self) -> u64 {
        self.hostile_metadata_denials
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
pub struct BlobChunkIntegrityCounterSnapshot {
    bytes_chunked: u64,
    chunks_emitted: u64,
    checksums_computed: u64,
    digest_updates: u64,
    chunk_tree_nodes_materialized: u64,
    order_checks: u64,
    order_denials: u64,
    checksum_only_denials: u64,
    digest_only_denials: u64,
}

impl BlobChunkIntegrityCounterSnapshot {
    pub(crate) const fn start() -> Self {
        Self {
            bytes_chunked: 0,
            chunks_emitted: 0,
            checksums_computed: 0,
            digest_updates: 0,
            chunk_tree_nodes_materialized: 0,
            order_checks: 0,
            order_denials: 0,
            checksum_only_denials: 0,
            digest_only_denials: 0,
        }
    }

    pub(crate) const fn record_chunk_admitted(self, bytes: u64) -> Self {
        Self {
            bytes_chunked: self.bytes_chunked + bytes,
            chunks_emitted: self.chunks_emitted + 1,
            digest_updates: self.digest_updates + 2,
            order_checks: self.order_checks + 1,
            ..self
        }
    }

    pub(crate) const fn record_checksum_computed(self) -> Self {
        Self {
            checksums_computed: self.checksums_computed + 1,
            ..self
        }
    }

    pub(crate) const fn record_sequence_finalized(self, chunk_count: u64) -> Self {
        Self {
            chunk_tree_nodes_materialized: self.chunk_tree_nodes_materialized + chunk_count,
            digest_updates: self.digest_updates + 2,
            ..self
        }
    }

    pub(crate) const fn record_order_denial(self) -> Self {
        Self {
            order_checks: self.order_checks + 1,
            order_denials: self.order_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_checksum_only_denial(self) -> Self {
        Self {
            checksum_only_denials: self.checksum_only_denials + 1,
            ..self
        }
    }

    pub(crate) const fn record_digest_only_denial(self) -> Self {
        Self {
            digest_only_denials: self.digest_only_denials + 1,
            ..self
        }
    }

    pub const fn bytes_chunked(self) -> u64 {
        self.bytes_chunked
    }

    pub const fn chunks_emitted(self) -> u64 {
        self.chunks_emitted
    }

    pub const fn checksums_computed(self) -> u64 {
        self.checksums_computed
    }

    pub const fn digest_updates(self) -> u64 {
        self.digest_updates
    }

    pub const fn chunk_tree_nodes_materialized(self) -> u64 {
        self.chunk_tree_nodes_materialized
    }

    pub const fn order_denials(self) -> u64 {
        self.order_denials
    }

    pub const fn checksum_only_denials(self) -> u64 {
        self.checksum_only_denials
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
