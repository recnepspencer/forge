use std::collections::BTreeSet;

use worth_query::facade::runtime::WorthQueryGraphReadStreamingCursorSession;

fn main() {
    let _ = WorthQueryGraphReadStreamingCursorSession {
        streaming_plan_digest: String::new(),
        snapshot_identity_digest: String::new(),
        expected_next_page_ordinal: 1,
        page_receipts: Vec::new(),
        consumed_cursor_digests: BTreeSet::new(),
    };
}
