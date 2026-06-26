#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthGraphReadAccessPhaseSixCounters {
    declaration_candidate_count: usize,
    capability_gap_count: usize,
    deletion_item_count: usize,
    excluded_certification_only_count: usize,
    excluded_out_of_scope_count: usize,
}

impl WorthGraphReadAccessPhaseSixCounters {
    pub(crate) const fn new(
        declaration_candidate_count: usize,
        capability_gap_count: usize,
        deletion_item_count: usize,
        excluded_certification_only_count: usize,
        excluded_out_of_scope_count: usize,
    ) -> Self {
        Self {
            declaration_candidate_count,
            capability_gap_count,
            deletion_item_count,
            excluded_certification_only_count,
            excluded_out_of_scope_count,
        }
    }

    pub const fn declaration_candidate_count(&self) -> usize {
        self.declaration_candidate_count
    }

    pub const fn capability_gap_count(&self) -> usize {
        self.capability_gap_count
    }

    pub const fn deletion_item_count(&self) -> usize {
        self.deletion_item_count
    }

    pub const fn excluded_certification_only_count(&self) -> usize {
        self.excluded_certification_only_count
    }

    pub const fn excluded_out_of_scope_count(&self) -> usize {
        self.excluded_out_of_scope_count
    }
}
