#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobImportReadmissionCounters {
    imported_declarations: u64,
    readmitted_chunks: u64,
    stale_scope_denials: u64,
    missing_chunk_denials: u64,
    copied_row_denials: u64,
    terminal_projection_denials: u64,
    placement_only_denials: u64,
    witness_constructions: u64,
}

impl BlobImportReadmissionCounters {
    pub const fn start() -> Self {
        Self {
            imported_declarations: 0,
            readmitted_chunks: 0,
            stale_scope_denials: 0,
            missing_chunk_denials: 0,
            copied_row_denials: 0,
            terminal_projection_denials: 0,
            placement_only_denials: 0,
            witness_constructions: 0,
        }
    }

    pub const fn record_imported_declaration(self) -> Self {
        Self {
            imported_declarations: self.imported_declarations + 1,
            ..self
        }
    }

    pub const fn with_readmitted_chunks(self, readmitted_chunks: u64) -> Self {
        Self {
            readmitted_chunks,
            ..self
        }
    }

    pub const fn record_stale_scope_denial(self) -> Self {
        Self {
            stale_scope_denials: self.stale_scope_denials + 1,
            ..self
        }
    }

    pub const fn record_missing_chunk_denial(self) -> Self {
        Self {
            missing_chunk_denials: self.missing_chunk_denials + 1,
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

    pub const fn record_placement_only_denial(self) -> Self {
        Self {
            placement_only_denials: self.placement_only_denials + 1,
            ..self
        }
    }

    pub const fn record_witness_construction(self) -> Self {
        Self {
            witness_constructions: self.witness_constructions + 1,
            ..self
        }
    }

    pub const fn imported_declarations(self) -> u64 {
        self.imported_declarations
    }

    pub const fn readmitted_chunks(self) -> u64 {
        self.readmitted_chunks
    }

    pub const fn stale_scope_denials(self) -> u64 {
        self.stale_scope_denials
    }

    pub const fn missing_chunk_denials(self) -> u64 {
        self.missing_chunk_denials
    }

    pub const fn copied_row_denials(self) -> u64 {
        self.copied_row_denials
    }

    pub const fn terminal_projection_denials(self) -> u64 {
        self.terminal_projection_denials
    }

    pub const fn placement_only_denials(self) -> u64 {
        self.placement_only_denials
    }

    pub const fn witness_constructions(self) -> u64 {
        self.witness_constructions
    }
}
