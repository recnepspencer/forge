use worth_query::facade::runtime::WorthQueryEphemeralGraphIndex;

fn main() {
    let _ = WorthQueryEphemeralGraphIndex {
        index_digest: "index".to_string(),
        plan_digest: "plan".to_string(),
        scope_digest: "scope".to_string(),
        rebuild_basis_digest: "rebuild".to_string(),
        allocated_bytes: 1,
        allocation_row_count: 1,
        touched_node_count: 1,
        touched_edge_count: 1,
    };
}
