use forge_query::facade::runtime::ForgeQueryEphemeralGraphIndexLifecycleRegistry;

fn main() {
    let _ = ForgeQueryEphemeralGraphIndexLifecycleRegistry {
        allocation_attempt_count: 1,
        successful_allocation_count: 1,
        release_count: 1,
        active_resource_count: 0,
        rejected_before_allocation_count: 0,
        touched_node_count: 1,
        touched_edge_count: 1,
    };
}
