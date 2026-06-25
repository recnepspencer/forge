#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OfflineVerifierCounterSnapshot {
    root_candidates_inspected: u32,
    manifest_rows_decoded: u32,
    header_decodes: u32,
    slot_directory_entries: u32,
    extent_membership_checks: u32,
    free_space_entries_checked: u32,
    backend_residue_rejections: u32,
    parity_ready_references: u32,
    semantic_decode_attempts: u32,
}

impl OfflineVerifierCounterSnapshot {
    pub const fn empty() -> Self {
        Self {
            root_candidates_inspected: 0,
            manifest_rows_decoded: 0,
            header_decodes: 0,
            slot_directory_entries: 0,
            extent_membership_checks: 0,
            free_space_entries_checked: 0,
            backend_residue_rejections: 0,
            parity_ready_references: 0,
            semantic_decode_attempts: 0,
        }
    }

    pub const fn with_root_candidates_inspected(mut self, count: u32) -> Self {
        self.root_candidates_inspected += count;
        self
    }

    pub const fn with_manifest_rows_decoded(mut self, count: u32) -> Self {
        self.manifest_rows_decoded += count;
        self
    }

    pub const fn with_header_decode(mut self) -> Self {
        self.header_decodes += 1;
        self
    }

    pub const fn with_slot_directory_entries(mut self, count: u32) -> Self {
        self.slot_directory_entries += count;
        self
    }

    pub const fn with_extent_membership_check(mut self) -> Self {
        self.extent_membership_checks += 1;
        self
    }

    pub const fn with_free_space_entry_checked(mut self) -> Self {
        self.free_space_entries_checked += 1;
        self
    }

    pub const fn with_backend_residue_rejection(mut self) -> Self {
        self.backend_residue_rejections += 1;
        self
    }

    pub const fn with_parity_ready_references(mut self, count: u32) -> Self {
        self.parity_ready_references += count;
        self
    }

    pub const fn root_candidates_inspected(self) -> u32 {
        self.root_candidates_inspected
    }

    pub const fn manifest_rows_decoded(self) -> u32 {
        self.manifest_rows_decoded
    }

    pub const fn header_decodes(self) -> u32 {
        self.header_decodes
    }

    pub const fn slot_directory_entries(self) -> u32 {
        self.slot_directory_entries
    }

    pub const fn extent_membership_checks(self) -> u32 {
        self.extent_membership_checks
    }

    pub const fn free_space_entries_checked(self) -> u32 {
        self.free_space_entries_checked
    }

    pub const fn backend_residue_rejections(self) -> u32 {
        self.backend_residue_rejections
    }

    pub const fn parity_ready_references(self) -> u32 {
        self.parity_ready_references
    }

    pub const fn semantic_decode_attempts(self) -> u32 {
        self.semantic_decode_attempts
    }
}
