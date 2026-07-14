use worth_query::facade::runtime::WorthQueryGraphReadStreamingPageReceipt;

fn main() {
    let _ = WorthQueryGraphReadStreamingPageReceipt {
        digest: String::new(),
        streaming_plan_digest: String::new(),
        snapshot_identity_digest: String::new(),
        page_ordinal: 0,
        emitted_row_count: 0,
        max_resident_frontier_observed: 0,
        max_resident_visited_observed: 0,
        next_cursor: None,
    };
}
