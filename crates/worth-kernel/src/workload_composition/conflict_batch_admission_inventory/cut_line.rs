use super::counters::ConflictBatchAdmissionInventoryCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConflictBatchAdmissionCutLine {
    counters: ConflictBatchAdmissionInventoryCounters,
    required_surface_count: usize,
    missing_required_surface_count: usize,
    duplicate_surface_count: usize,
    discovered_surface_count: usize,
    unclassified_surface_count: usize,
}

impl ConflictBatchAdmissionCutLine {
    pub(crate) const fn from_counts(
        counters: ConflictBatchAdmissionInventoryCounters,
        required_surface_count: usize,
        missing_required_surface_count: usize,
        duplicate_surface_count: usize,
        discovered_surface_count: usize,
        unclassified_surface_count: usize,
    ) -> Self {
        Self {
            counters,
            required_surface_count,
            missing_required_surface_count,
            duplicate_surface_count,
            discovered_surface_count,
            unclassified_surface_count,
        }
    }

    pub const fn ready_for_aspect_routed_replacement(&self) -> bool {
        self.missing_required_surface_count == 0
            && self.duplicate_surface_count == 0
            && self.unclassified_surface_count == 0
            && self.discovered_surface_count > 0
            && self.counters.query_support_rows() > 0
            && self.counters.seeded_surface_rows() >= 3
            && self.counters.operational_overlap_rows() > 0
    }

    pub const fn counters(&self) -> &ConflictBatchAdmissionInventoryCounters {
        &self.counters
    }

    pub const fn required_surface_count(&self) -> usize {
        self.required_surface_count
    }

    pub const fn missing_required_surface_count(&self) -> usize {
        self.missing_required_surface_count
    }

    pub const fn duplicate_surface_count(&self) -> usize {
        self.duplicate_surface_count
    }

    pub const fn discovered_surface_count(&self) -> usize {
        self.discovered_surface_count
    }

    pub const fn unclassified_surface_count(&self) -> usize {
        self.unclassified_surface_count
    }
}
