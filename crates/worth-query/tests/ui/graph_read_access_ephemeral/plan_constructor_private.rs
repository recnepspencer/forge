use worth_query::facade::runtime::{
    WorthQueryEphemeralGraphIndexPlan, WorthQueryEphemeralGraphIndexScopeKind,
};

fn main() {
    let _ = WorthQueryEphemeralGraphIndexPlan {
        digest: "plan".to_string(),
        admission_digest: "admission".to_string(),
        requirement_set_digest: "requirements".to_string(),
        estimated_index_bytes: 1,
        admitted_byte_budget: 1,
        estimated_touched_nodes: 1,
        estimated_touched_edges: 1,
        required_scope_kind: WorthQueryEphemeralGraphIndexScopeKind::ReadExecution,
        allocation_rows: Vec::new(),
    };
}
