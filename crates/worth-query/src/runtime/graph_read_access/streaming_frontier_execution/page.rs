use super::WorthQueryGraphReadFrontierCursor;
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadStreamingPageReceipt {
    digest: String,
    streaming_plan_digest: String,
    snapshot_identity_digest: String,
    page_ordinal: usize,
    emitted_row_count: usize,
    max_resident_frontier_observed: usize,
    max_resident_visited_observed: usize,
    next_cursor: Option<WorthQueryGraphReadFrontierCursor>,
}

impl WorthQueryGraphReadStreamingPageReceipt {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn streaming_plan_digest(&self) -> &str {
        &self.streaming_plan_digest
    }

    pub fn snapshot_identity_digest(&self) -> &str {
        &self.snapshot_identity_digest
    }

    pub fn page_ordinal(&self) -> usize {
        self.page_ordinal
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

    pub fn next_cursor(&self) -> Option<&WorthQueryGraphReadFrontierCursor> {
        self.next_cursor.as_ref()
    }

    pub(crate) fn new(
        streaming_plan_digest: impl Into<String>,
        snapshot_identity_digest: impl Into<String>,
        page_ordinal: usize,
        emitted_row_count: usize,
        max_resident_frontier_observed: usize,
        max_resident_visited_observed: usize,
    ) -> Self {
        let streaming_plan_digest = streaming_plan_digest.into();
        let snapshot_identity_digest = snapshot_identity_digest.into();
        let digest = page_digest(
            &streaming_plan_digest,
            &snapshot_identity_digest,
            page_ordinal,
            emitted_row_count,
            max_resident_frontier_observed,
            max_resident_visited_observed,
        );
        Self {
            digest,
            streaming_plan_digest,
            snapshot_identity_digest,
            page_ordinal,
            emitted_row_count,
            max_resident_frontier_observed,
            max_resident_visited_observed,
            next_cursor: None,
        }
    }

    pub(crate) fn with_next_cursor(
        mut self,
        next_cursor: WorthQueryGraphReadFrontierCursor,
    ) -> Self {
        self.next_cursor = Some(next_cursor);
        self
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "page:{}:{}:{}:{}:{}:{}:{}",
            self.streaming_plan_digest,
            self.snapshot_identity_digest,
            self.page_ordinal,
            self.emitted_row_count,
            self.max_resident_frontier_observed,
            self.max_resident_visited_observed,
            self.next_cursor
                .as_ref()
                .map(WorthQueryGraphReadFrontierCursor::digest_part)
                .unwrap_or_else(|| "cursor:none".to_string())
        )
    }
}

fn page_digest(
    streaming_plan_digest: &str,
    snapshot_identity_digest: &str,
    page_ordinal: usize,
    emitted_row_count: usize,
    max_resident_frontier_observed: usize,
    max_resident_visited_observed: usize,
) -> String {
    hash_parts(&[
        "worth_query_graph_read_streaming_page_receipt_v1".to_string(),
        format!("streaming_plan:{streaming_plan_digest}"),
        format!("snapshot_identity:{snapshot_identity_digest}"),
        format!("page_ordinal:{page_ordinal}"),
        format!("emitted_row_count:{emitted_row_count}"),
        format!("max_resident_frontier_observed:{max_resident_frontier_observed}"),
        format!("max_resident_visited_observed:{max_resident_visited_observed}"),
    ])
}
