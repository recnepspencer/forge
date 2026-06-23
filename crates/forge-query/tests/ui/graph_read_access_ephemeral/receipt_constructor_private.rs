use forge_query::facade::runtime::{
    ForgeQueryEphemeralGraphIndexCounters, ForgeQueryEphemeralGraphIndexReceipt,
    ForgeQueryEphemeralGraphIndexScopeKind,
};

fn main() {
    let _ = ForgeQueryEphemeralGraphIndexReceipt {
        digest: "receipt".to_string(),
        plan_digest: "plan".to_string(),
        scope_digest: "scope".to_string(),
        index_digest: "index".to_string(),
        scope_kind: ForgeQueryEphemeralGraphIndexScopeKind::ReadExecution,
        actual_allocated_bytes: 1,
        admitted_byte_budget: 1,
        active_resource_count_after_scope: 0,
        counters: ForgeQueryEphemeralGraphIndexCounters {
            allocation_attempt_count: 1,
            allocation_count: 1,
            cleanup_count: 1,
            orphan_resource_count: 0,
            rejected_before_allocation_count: 0,
            touched_node_count: 1,
            touched_edge_count: 1,
        },
    };
}
