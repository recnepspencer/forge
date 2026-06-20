use forge_query::facade::runtime::{
    ForgeQueryGraphReadMaterializationAdmittedLimits, ForgeQueryGraphReadMaterializationCounters,
    ForgeQueryGraphReadMaterializationProgress,
};

fn main() {
    let _ = ForgeQueryGraphReadMaterializationProgress {
        digest: String::new(),
        request_digest: String::new(),
        admitted_limits: ForgeQueryGraphReadMaterializationAdmittedLimits {
            digest: String::new(),
            max_resident_bytes: 0,
            max_touched_edges: 0,
        },
        counters: ForgeQueryGraphReadMaterializationCounters {
            digest: String::new(),
            touched_edges: 0,
            frontier_pages: 0,
            allocated_bytes: 0,
            emitted_rows: 0,
            checkpoint_count: 0,
            cancellation_poll_count: 0,
        },
    };
}
