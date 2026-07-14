use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadFrontierCursor {
    digest: String,
    streaming_plan_digest: String,
    snapshot_identity_digest: String,
    next_page_ordinal: usize,
    prior_page_receipt_digest: String,
    frontier_continuation_digest: String,
    visited_set_digest: String,
}

impl WorthQueryGraphReadFrontierCursor {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn streaming_plan_digest(&self) -> &str {
        &self.streaming_plan_digest
    }

    pub fn snapshot_identity_digest(&self) -> &str {
        &self.snapshot_identity_digest
    }

    pub fn next_page_ordinal(&self) -> usize {
        self.next_page_ordinal
    }

    pub fn prior_page_receipt_digest(&self) -> &str {
        &self.prior_page_receipt_digest
    }

    pub fn frontier_continuation_digest(&self) -> &str {
        &self.frontier_continuation_digest
    }

    pub fn visited_set_digest(&self) -> &str {
        &self.visited_set_digest
    }

    pub(crate) fn new(
        streaming_plan_digest: impl Into<String>,
        snapshot_identity_digest: impl Into<String>,
        next_page_ordinal: usize,
        prior_page_receipt_digest: impl Into<String>,
        frontier_continuation_digest: impl Into<String>,
        visited_set_digest: impl Into<String>,
    ) -> Self {
        let streaming_plan_digest = streaming_plan_digest.into();
        let snapshot_identity_digest = snapshot_identity_digest.into();
        let prior_page_receipt_digest = prior_page_receipt_digest.into();
        let frontier_continuation_digest = frontier_continuation_digest.into();
        let visited_set_digest = visited_set_digest.into();
        let digest = hash_parts(&[
            "worth_query_graph_read_frontier_cursor_v1".to_string(),
            format!("streaming_plan:{streaming_plan_digest}"),
            format!("snapshot_identity:{snapshot_identity_digest}"),
            format!("next_page_ordinal:{next_page_ordinal}"),
            format!("prior_page_receipt:{prior_page_receipt_digest}"),
            format!("frontier_continuation:{frontier_continuation_digest}"),
            format!("visited_set:{visited_set_digest}"),
        ]);
        Self {
            digest,
            streaming_plan_digest,
            snapshot_identity_digest,
            next_page_ordinal,
            prior_page_receipt_digest,
            frontier_continuation_digest,
            visited_set_digest,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "cursor:{}:{}:{}:{}:{}:{}",
            self.streaming_plan_digest,
            self.snapshot_identity_digest,
            self.next_page_ordinal,
            self.prior_page_receipt_digest,
            self.frontier_continuation_digest,
            self.visited_set_digest
        )
    }
}
