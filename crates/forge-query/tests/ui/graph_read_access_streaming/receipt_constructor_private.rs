use forge_query::facade::runtime::{
    ForgeQueryGraphReadStreamingCounters, ForgeQueryGraphReadStreamingReceipt,
};

fn main() {
    let _ = ForgeQueryGraphReadStreamingReceipt {
        digest: String::new(),
        streaming_plan_digest: String::new(),
        snapshot_identity_digest: String::new(),
        convergence_result_digest: String::new(),
        page_receipts: Vec::new(),
        counters: ForgeQueryGraphReadStreamingCounters {
            page_count: 0,
            emitted_row_count: 0,
            max_resident_frontier_observed: 0,
            max_resident_visited_observed: 0,
            cursor_replay_denial_count: 0,
            cursor_identity_denial_count: 0,
        },
    };
}
