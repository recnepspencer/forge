#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobExportEvidenceCounts {
    exported_chunks: u64,
    exported_bytes: u64,
    manifest_rows: u64,
    skipped_chunks: u64,
}

impl BlobExportEvidenceCounts {
    pub const DIGEST_EVIDENCE_ITEMS: u64 = 5;

    pub const fn new(
        exported_chunks: u64,
        exported_bytes: u64,
        manifest_rows: u64,
        skipped_chunks: u64,
    ) -> Self {
        Self {
            exported_chunks,
            exported_bytes,
            manifest_rows,
            skipped_chunks,
        }
    }

    pub const fn exported_chunks(&self) -> u64 {
        self.exported_chunks
    }

    pub const fn exported_bytes(&self) -> u64 {
        self.exported_bytes
    }

    pub const fn manifest_rows(&self) -> u64 {
        self.manifest_rows
    }

    pub const fn skipped_chunks(&self) -> u64 {
        self.skipped_chunks
    }

    pub const fn digest_evidence_items(&self) -> u64 {
        Self::DIGEST_EVIDENCE_ITEMS
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlobExportBundleCounters {
    exported_chunks: u64,
    exported_bytes: u64,
    manifest_rows: u64,
    skipped_chunks: u64,
    bundle_publications: u64,
    stale_reachability_denials: u64,
    missing_chunk_denials: u64,
    placement_only_denials: u64,
    copied_row_denials: u64,
    terminal_projection_denials: u64,
}

impl BlobExportBundleCounters {
    pub const fn start() -> Self {
        Self {
            exported_chunks: 0,
            exported_bytes: 0,
            manifest_rows: 0,
            skipped_chunks: 0,
            bundle_publications: 0,
            stale_reachability_denials: 0,
            missing_chunk_denials: 0,
            placement_only_denials: 0,
            copied_row_denials: 0,
            terminal_projection_denials: 0,
        }
    }

    pub const fn with_evidence(self, evidence: BlobExportEvidenceCounts) -> Self {
        Self {
            exported_chunks: evidence.exported_chunks(),
            exported_bytes: evidence.exported_bytes(),
            manifest_rows: evidence.manifest_rows(),
            skipped_chunks: evidence.skipped_chunks(),
            bundle_publications: self.bundle_publications + 1,
            ..self
        }
    }

    pub const fn record_stale_reachability_denial(self) -> Self {
        Self {
            stale_reachability_denials: self.stale_reachability_denials + 1,
            ..self
        }
    }

    pub const fn record_missing_chunk_denial(self) -> Self {
        Self {
            missing_chunk_denials: self.missing_chunk_denials + 1,
            ..self
        }
    }

    pub const fn record_placement_only_denial(self) -> Self {
        Self {
            placement_only_denials: self.placement_only_denials + 1,
            ..self
        }
    }

    pub const fn record_copied_row_denial(self) -> Self {
        Self {
            copied_row_denials: self.copied_row_denials + 1,
            ..self
        }
    }

    pub const fn record_terminal_projection_denial(self) -> Self {
        Self {
            terminal_projection_denials: self.terminal_projection_denials + 1,
            ..self
        }
    }

    pub const fn exported_chunks(&self) -> u64 {
        self.exported_chunks
    }

    pub const fn exported_bytes(&self) -> u64 {
        self.exported_bytes
    }

    pub const fn manifest_rows(&self) -> u64 {
        self.manifest_rows
    }

    pub const fn skipped_chunks(&self) -> u64 {
        self.skipped_chunks
    }

    pub const fn bundle_publications(&self) -> u64 {
        self.bundle_publications
    }

    pub const fn stale_reachability_denials(&self) -> u64 {
        self.stale_reachability_denials
    }

    pub const fn missing_chunk_denials(&self) -> u64 {
        self.missing_chunk_denials
    }

    pub const fn placement_only_denials(&self) -> u64 {
        self.placement_only_denials
    }

    pub const fn copied_row_denials(&self) -> u64 {
        self.copied_row_denials
    }

    pub const fn terminal_projection_denials(&self) -> u64 {
        self.terminal_projection_denials
    }
}
