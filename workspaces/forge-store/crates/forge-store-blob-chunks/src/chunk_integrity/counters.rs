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
