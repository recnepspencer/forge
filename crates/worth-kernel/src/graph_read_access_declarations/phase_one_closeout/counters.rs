use super::super::seed_contract::WorthGraphReadAccessDeclarationAdmittedSeed;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseOneCounters {
    declaration_candidate_count: usize,
    capability_gap_count: usize,
    deletion_item_count: usize,
    excluded_certification_only_count: usize,
    excluded_out_of_scope_count: usize,
}

impl WorthGraphReadAccessDeclarationPhaseOneCounters {
    pub(crate) fn from_admitted_seed(seed: &WorthGraphReadAccessDeclarationAdmittedSeed) -> Self {
        Self {
            declaration_candidate_count: seed.declaration_candidates().len(),
            capability_gap_count: seed.capability_gaps().len(),
            deletion_item_count: seed.deletion_items().len(),
            excluded_certification_only_count: seed
                .milestone_seven_seed()
                .counters()
                .excluded_certification_only_count(),
            excluded_out_of_scope_count: seed
                .milestone_seven_seed()
                .counters()
                .excluded_out_of_scope_count(),
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
