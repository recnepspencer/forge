#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ContainerIntegrityCounters {
    protected_window_reads: u32,
    header_witness_checks: u32,
    body_boundary_checks: u32,
    frame_boundary_checks: u32,
    extent_boundary_checks: u32,
    slot_directory_reads: u32,
    slot_entries_inspected: u32,
    skipped_record_view_constructions: u32,
}

impl ContainerIntegrityCounters {
    pub const fn start() -> Self {
        Self {
            protected_window_reads: 1,
            header_witness_checks: 0,
            body_boundary_checks: 0,
            frame_boundary_checks: 0,
            extent_boundary_checks: 0,
            slot_directory_reads: 0,
            slot_entries_inspected: 0,
            skipped_record_view_constructions: 0,
        }
    }

    pub const fn with_header_witness_check(mut self) -> Self {
        self.header_witness_checks += 1;
        self
    }

    pub const fn with_body_boundary_check(mut self) -> Self {
        self.body_boundary_checks += 1;
        self
    }

    pub const fn with_frame_boundary_check(mut self) -> Self {
        self.frame_boundary_checks += 1;
        self
    }

    pub const fn with_extent_boundary_check(mut self) -> Self {
        self.extent_boundary_checks += 1;
        self
    }

    pub const fn with_slot_directory_read(mut self) -> Self {
        self.slot_directory_reads += 1;
        self
    }

    pub const fn with_slot_entry_inspected(mut self) -> Self {
        self.slot_entries_inspected += 1;
        self
    }

    pub const fn with_skipped_record_view(mut self) -> Self {
        self.skipped_record_view_constructions += 1;
        self
    }

    pub const fn protected_window_reads(self) -> u32 {
        self.protected_window_reads
    }

    pub const fn header_witness_checks(self) -> u32 {
        self.header_witness_checks
    }

    pub const fn body_boundary_checks(self) -> u32 {
        self.body_boundary_checks
    }

    pub const fn frame_boundary_checks(self) -> u32 {
        self.frame_boundary_checks
    }

    pub const fn extent_boundary_checks(self) -> u32 {
        self.extent_boundary_checks
    }

    pub const fn slot_directory_reads(self) -> u32 {
        self.slot_directory_reads
    }

    pub const fn slot_entries_inspected(self) -> u32 {
        self.slot_entries_inspected
    }

    pub const fn skipped_record_view_constructions(self) -> u32 {
        self.skipped_record_view_constructions
    }
}
