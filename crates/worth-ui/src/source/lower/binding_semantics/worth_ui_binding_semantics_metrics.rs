use crate::capability::SnapshotLookupCounters;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiBindingSemanticsMetrics {
    direct_lookup_count: usize,
    total_family_width: usize,
    families_scanned: usize,
    query_owned_semantic_checks: usize,
}

impl WorthUiBindingSemanticsMetrics {
    pub(crate) fn record_lookup(&mut self, counters: SnapshotLookupCounters) {
        self.direct_lookup_count += 1;
        self.total_family_width += counters.family_width();
        self.families_scanned += counters.families_scanned();
    }

    pub(crate) fn record_query_owned_semantic_check(&mut self) {
        self.query_owned_semantic_checks += 1;
    }

    pub(crate) fn direct_lookup_count(&self) -> usize {
        self.direct_lookup_count
    }

    pub(crate) fn families_scanned(&self) -> usize {
        self.families_scanned
    }

    pub(crate) fn query_owned_semantic_checks(&self) -> usize {
        self.query_owned_semantic_checks
    }
}
