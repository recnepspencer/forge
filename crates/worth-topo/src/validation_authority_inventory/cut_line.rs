use super::counters::WorthValidationAuthorityInventoryCounters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthValidationAuthorityCutLine {
    counters: WorthValidationAuthorityInventoryCounters,
    ready_for_parallel_catalog_lane: bool,
    cut_line_digest: String,
}

impl WorthValidationAuthorityCutLine {
    pub(super) fn from_counters(counters: WorthValidationAuthorityInventoryCounters) -> Self {
        let ready_for_parallel_catalog_lane = counters.total_source_rows() > 0
            && counters.migrate_rows() > 0
            && counters.cap_rows() > 0;
        let cut_line_digest = format!(
            "m9-phase1-cut-line:v1:total={}:migrate={}:cap={}:gap={}",
            counters.total_source_rows(),
            counters.migrate_rows(),
            counters.cap_rows(),
            counters.query_access_gap_rows()
        );
        Self {
            counters,
            ready_for_parallel_catalog_lane,
            cut_line_digest,
        }
    }

    pub const fn counters(&self) -> &WorthValidationAuthorityInventoryCounters {
        &self.counters
    }

    pub const fn ready_for_parallel_catalog_lane(&self) -> bool {
        self.ready_for_parallel_catalog_lane
    }

    pub fn cut_line_digest(&self) -> &str {
        &self.cut_line_digest
    }
}
