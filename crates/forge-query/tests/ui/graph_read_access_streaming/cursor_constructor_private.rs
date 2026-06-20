use forge_query::facade::runtime::ForgeQueryGraphReadFrontierCursor;

fn main() {
    let _ = ForgeQueryGraphReadFrontierCursor {
        digest: String::new(),
        streaming_plan_digest: String::new(),
        snapshot_identity_digest: String::new(),
        next_page_ordinal: 1,
        prior_page_receipt_digest: String::new(),
        frontier_continuation_digest: String::new(),
        visited_set_digest: String::new(),
    };
}
