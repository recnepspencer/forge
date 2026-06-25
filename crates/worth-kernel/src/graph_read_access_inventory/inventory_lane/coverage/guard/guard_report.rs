#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessCoverageGuardReport {
    discovered_surface_count: usize,
    covered_source_count: usize,
    admitted_surface_count: usize,
    unclassified_surface_count: usize,
    production_shaped_test_support_gap_count: usize,
}

impl WorthGraphReadAccessCoverageGuardReport {
    pub(crate) const fn clean(
        discovered_surface_count: usize,
        covered_source_count: usize,
        admitted_surface_count: usize,
    ) -> Self {
        Self {
            discovered_surface_count,
            covered_source_count,
            admitted_surface_count,
            unclassified_surface_count: 0,
            production_shaped_test_support_gap_count: 0,
        }
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_inventory::inventory_lane) const fn clean_for_tests(
        row_count: usize,
    ) -> Self {
        Self::clean(row_count, row_count, row_count)
    }

    pub const fn discovered_surface_count(&self) -> usize {
        self.discovered_surface_count
    }

    pub const fn covered_source_count(&self) -> usize {
        self.covered_source_count
    }

    pub const fn admitted_surface_count(&self) -> usize {
        self.admitted_surface_count
    }

    pub const fn unclassified_surface_count(&self) -> usize {
        self.unclassified_surface_count
    }

    pub const fn production_shaped_test_support_gap_count(&self) -> usize {
        self.production_shaped_test_support_gap_count
    }
}
