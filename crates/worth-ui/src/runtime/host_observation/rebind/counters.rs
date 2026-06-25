#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiHostObservationRebindCounters {
    changed_fact_count: usize,
    preserved_fact_count: usize,
    consuming_projection_count: usize,
    source_reparse_count: usize,
    artifact_scan_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiHostObservationRebindCounters {
    pub(super) fn new(
        changed_fact_count: usize,
        preserved_fact_count: usize,
        consuming_projection_count: usize,
    ) -> Self {
        Self {
            changed_fact_count,
            preserved_fact_count,
            consuming_projection_count,
            source_reparse_count: 0,
            artifact_scan_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn changed_fact_count(self) -> usize {
        self.changed_fact_count
    }

    pub fn preserved_fact_count(self) -> usize {
        self.preserved_fact_count
    }

    pub fn consuming_projection_count(self) -> usize {
        self.consuming_projection_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn artifact_scan_count(self) -> usize {
        self.artifact_scan_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
