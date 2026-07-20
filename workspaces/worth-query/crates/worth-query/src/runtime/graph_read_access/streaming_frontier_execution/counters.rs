#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadStreamingCounters {
    page_count: usize,
    emitted_row_count: usize,
    max_resident_frontier_observed: usize,
    max_resident_visited_observed: usize,
    cursor_replay_denial_count: usize,
    cursor_identity_denial_count: usize,
}

impl WorthQueryGraphReadStreamingCounters {
    pub fn page_count(&self) -> usize {
        self.page_count
    }

    pub fn emitted_row_count(&self) -> usize {
        self.emitted_row_count
    }

    pub fn max_resident_frontier_observed(&self) -> usize {
        self.max_resident_frontier_observed
    }

    pub fn max_resident_visited_observed(&self) -> usize {
        self.max_resident_visited_observed
    }

    pub fn cursor_replay_denial_count(&self) -> usize {
        self.cursor_replay_denial_count
    }

    pub fn cursor_identity_denial_count(&self) -> usize {
        self.cursor_identity_denial_count
    }

    pub(crate) fn from_execution(
        page_count: usize,
        emitted_row_count: usize,
        max_resident_frontier_observed: usize,
        max_resident_visited_observed: usize,
    ) -> Self {
        Self {
            page_count,
            emitted_row_count,
            max_resident_frontier_observed,
            max_resident_visited_observed,
            cursor_replay_denial_count: 0,
            cursor_identity_denial_count: 0,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "streaming_counters:pages:{}:rows:{}:frontier:{}:visited:{}:replay_denials:{}:identity_denials:{}",
            self.page_count,
            self.emitted_row_count,
            self.max_resident_frontier_observed,
            self.max_resident_visited_observed,
            self.cursor_replay_denial_count,
            self.cursor_identity_denial_count
        )
    }
}
