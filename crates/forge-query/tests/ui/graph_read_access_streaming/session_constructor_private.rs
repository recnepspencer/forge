use std::collections::BTreeSet;

use forge_query::facade::runtime::ForgeQueryGraphReadStreamingCursorSession;

fn main() {
    let _ = ForgeQueryGraphReadStreamingCursorSession {
        streaming_plan_digest: String::new(),
        snapshot_identity_digest: String::new(),
        expected_next_page_ordinal: 1,
        page_receipts: Vec::new(),
        consumed_cursor_digests: BTreeSet::new(),
    };
}
