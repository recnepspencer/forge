use worth_query::facade::runtime::{WorthQueryGraphReadStreamingCounters, WorthQueryGraphReadStreamingReceipt};

fn main() {
    let _ = WorthQueryGraphReadStreamingReceipt {
        digest: String::new(),
        streaming_plan_digest: String::new(),
        snapshot_identity_digest: String::new(),
        convergence_result_digest: String::new(),
        page_receipts: Vec::new(),
        counters: WorthQueryGraphReadStreamingCounters {
            page_count: 0,
            emitted_row_count: 0,
            max_resident_frontier_observed: 0,
            max_resident_visited_observed: 0,
            cursor_replay_denial_count: 0,
            cursor_identity_denial_count: 0,
        },
    };
}
