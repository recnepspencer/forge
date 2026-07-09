use worth_query::facade::runtime::WorthQueryEphemeralGraphIndexCounters;

fn main() {
    let _ = WorthQueryEphemeralGraphIndexCounters {
        allocation_attempt_count: 1,
        allocation_count: 1,
        cleanup_count: 1,
        orphan_resource_count: 0,
        rejected_before_allocation_count: 0,
        touched_node_count: 1,
        touched_edge_count: 1,
    };
}
