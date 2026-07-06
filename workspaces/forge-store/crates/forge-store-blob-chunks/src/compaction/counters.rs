use forge_store_physical_isolation::CompactionReadInterlockCounters;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlobCompactionCounterSnapshot {
    chunks_scanned: u64,
    chunks_rewritten: u64,
    dedupe_edges_preserved: u64,
    references_transferred: u64,
    bytes_moved: u64,
    foreground_yields: u64,
    residue_localized: u64,
    denied_compactions: u64,
    physical: CompactionReadInterlockCounters,
}

impl BlobCompactionCounterSnapshot {
    pub(crate) fn start(chunks: u64, references: u64, bytes: u64) -> Self {
        Self {
            chunks_scanned: chunks,
            references_transferred: references,
            bytes_moved: bytes,
            chunks_rewritten: 0,
            dedupe_edges_preserved: 0,
            foreground_yields: 0,
            residue_localized: 0,
            denied_compactions: 0,
            physical: CompactionReadInterlockCounters::default(),
        }
    }

    pub(crate) const fn with_physical(mut self, physical: CompactionReadInterlockCounters) -> Self {
        self.physical = physical;
        self.chunks_rewritten = physical.copied_pages();
        self
    }

    pub(crate) const fn preserve_dedupe_edges(mut self, edges: u64) -> Self {
        self.dedupe_edges_preserved = edges;
        self
    }

    pub(crate) const fn record_foreground_yields(mut self, yields: u64) -> Self {
        self.foreground_yields = yields;
        self
    }

    pub(crate) const fn record_denial(mut self) -> Self {
        self.denied_compactions += 1;
        self
    }

    pub(crate) const fn record_residue_localized(mut self) -> Self {
        self.residue_localized += 1;
        self
    }

    pub const fn chunks_scanned(self) -> u64 {
        self.chunks_scanned
    }

    pub const fn chunks_rewritten(self) -> u64 {
        self.chunks_rewritten
    }

    pub const fn dedupe_edges_preserved(self) -> u64 {
        self.dedupe_edges_preserved
    }

    pub const fn references_transferred(self) -> u64 {
        self.references_transferred
    }

    pub const fn bytes_moved(self) -> u64 {
        self.bytes_moved
    }

    pub const fn foreground_yields(self) -> u64 {
        self.foreground_yields
    }

    pub const fn residue_localized(self) -> u64 {
        self.residue_localized
    }

    pub const fn denied_compactions(self) -> u64 {
        self.denied_compactions
    }

    pub const fn physical(self) -> CompactionReadInterlockCounters {
        self.physical
    }
}
