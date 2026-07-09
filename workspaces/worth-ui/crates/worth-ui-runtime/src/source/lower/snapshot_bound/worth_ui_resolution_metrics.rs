use crate::capability::SnapshotLookupCounters;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiResolutionMetrics {
    direct_lookup_count: usize,
    total_family_width: usize,
    families_scanned: usize,
}

impl WorthUiResolutionMetrics {
    pub(crate) fn record_lookup(&mut self, counters: SnapshotLookupCounters) {
        self.direct_lookup_count += 1;
        self.total_family_width += counters.family_width();
        self.families_scanned += counters.families_scanned();
    }

    #[cfg(test)]
    pub(crate) fn direct_lookup_count(&self) -> usize {
        self.direct_lookup_count
    }

    #[cfg(test)]
    pub(crate) fn total_family_width(&self) -> usize {
        self.total_family_width
    }

    #[cfg(test)]
    pub(crate) fn families_scanned(&self) -> usize {
        self.families_scanned
    }
}
