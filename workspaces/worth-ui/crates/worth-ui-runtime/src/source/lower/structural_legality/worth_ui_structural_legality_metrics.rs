use crate::capability::SnapshotLookupCounters;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiStructuralLegalityMetrics {
    direct_lookup_count: usize,
    total_family_width: usize,
    families_scanned: usize,
    renderer_dependent_checks: usize,
}

impl WorthUiStructuralLegalityMetrics {
    pub(crate) fn record_lookup(&mut self, counters: SnapshotLookupCounters) {
        self.direct_lookup_count += 1;
        self.total_family_width += counters.family_width();
        self.families_scanned += counters.families_scanned();
    }

    #[cfg(test)]
    pub(crate) fn families_scanned(&self) -> usize {
        self.families_scanned
    }

    #[cfg(test)]
    pub(crate) fn renderer_dependent_checks(&self) -> usize {
        self.renderer_dependent_checks
    }
}
