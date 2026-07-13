use worth_query::facade::runtime::{WorthQueryGraphReadAccessDenialKind, WorthQueryGraphReadMaterializationPolicy, WorthQueryGraphReadMaterializationRequest};

fn main() {
    let _ = WorthQueryGraphReadMaterializationRequest {
        digest: String::new(),
        admission_digest: String::new(),
        admission_denial_kind: WorthQueryGraphReadAccessDenialKind::BudgetExceeded,
        requirement_set_digest: String::new(),
        cost_estimate_digest: String::new(),
        estimated_touched_edges: 0,
        estimated_resident_bytes: 0,
        estimated_emitted_rows: 0,
        budget_digest: String::new(),
        inventory_match_report_digest: String::new(),
        read_graph_digest: String::new(),
        policy: WorthQueryGraphReadMaterializationPolicy::bounded(),
    };
}
