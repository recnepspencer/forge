use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessDenialKind, ForgeQueryGraphReadMaterializationPolicy,
    ForgeQueryGraphReadMaterializationRequest,
};

fn main() {
    let _ = ForgeQueryGraphReadMaterializationRequest {
        digest: String::new(),
        admission_digest: String::new(),
        admission_denial_kind: ForgeQueryGraphReadAccessDenialKind::BudgetExceeded,
        requirement_set_digest: String::new(),
        cost_estimate_digest: String::new(),
        estimated_touched_edges: 0,
        estimated_resident_bytes: 0,
        estimated_emitted_rows: 0,
        budget_digest: String::new(),
        inventory_match_report_digest: String::new(),
        read_graph_digest: String::new(),
        policy: ForgeQueryGraphReadMaterializationPolicy::bounded(),
    };
}
