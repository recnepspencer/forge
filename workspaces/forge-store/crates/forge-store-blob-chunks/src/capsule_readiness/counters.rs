#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobCapsuleReadinessCounters {
    planned_chunks: u64,
    materialized_chunks: u64,
    skipped_chunks: u64,
    denied_chunks: u64,
    declared_bytes: u64,
    readiness_publications: u64,
}

impl BlobCapsuleReadinessCounters {
    pub const fn start() -> Self {
        Self {
            planned_chunks: 0,
            materialized_chunks: 0,
            skipped_chunks: 0,
            denied_chunks: 0,
            declared_bytes: 0,
            readiness_publications: 0,
        }
    }

    pub const fn with_planned_chunks(self, planned_chunks: u64) -> Self {
        Self {
            planned_chunks,
            ..self
        }
    }

    pub const fn with_skipped_chunks(self, skipped_chunks: u64) -> Self {
        Self {
            skipped_chunks,
            ..self
        }
    }

    pub const fn with_declared_bytes(self, declared_bytes: u64) -> Self {
        Self {
            declared_bytes,
            ..self
        }
    }

    pub const fn record_denied_chunk(self) -> Self {
        Self {
            denied_chunks: self.denied_chunks + 1,
            ..self
        }
    }

    pub const fn record_materialized_chunks(self, materialized_chunks: u64) -> Self {
        Self {
            materialized_chunks,
            ..self
        }
    }

    pub const fn record_readiness_publication(self) -> Self {
        Self {
            readiness_publications: self.readiness_publications + 1,
            ..self
        }
    }

    pub const fn planned_chunks(self) -> u64 {
        self.planned_chunks
    }
    pub const fn materialized_chunks(self) -> u64 {
        self.materialized_chunks
    }
    pub const fn skipped_chunks(self) -> u64 {
        self.skipped_chunks
    }
    pub const fn denied_chunks(self) -> u64 {
        self.denied_chunks
    }
    pub const fn declared_bytes(self) -> u64 {
        self.declared_bytes
    }
    pub const fn readiness_publications(self) -> u64 {
        self.readiness_publications
    }
}
