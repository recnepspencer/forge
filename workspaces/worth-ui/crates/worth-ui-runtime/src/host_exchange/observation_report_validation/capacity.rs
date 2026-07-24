#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationCapacity {
    local_reports: usize,
    local_bytes: usize,
    global_reports: usize,
    global_bytes: usize,
    quarantined_batches: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationCapacityInput {
    pub local_reports: usize,
    pub local_bytes: usize,
    pub global_reports: usize,
    pub global_bytes: usize,
    pub quarantined_batches: usize,
}

impl Default for UiHostObservationCapacity {
    fn default() -> Self {
        Self {
            local_reports: 64,
            local_bytes: 16 * 1024,
            global_reports: 512,
            global_bytes: 128 * 1024,
            quarantined_batches: 32,
        }
    }
}

impl UiHostObservationCapacity {
    pub const fn new(input: UiHostObservationCapacityInput) -> Self {
        Self {
            local_reports: input.local_reports,
            local_bytes: input.local_bytes,
            global_reports: input.global_reports,
            global_bytes: input.global_bytes,
            quarantined_batches: input.quarantined_batches,
        }
    }

    pub(crate) const fn local_reports(self) -> usize {
        self.local_reports
    }

    pub(crate) const fn local_bytes(self) -> usize {
        self.local_bytes
    }

    pub(crate) const fn global_reports(self) -> usize {
        self.global_reports
    }

    pub(crate) const fn global_bytes(self) -> usize {
        self.global_bytes
    }

    pub(crate) const fn quarantined_batches(self) -> usize {
        self.quarantined_batches
    }
}
