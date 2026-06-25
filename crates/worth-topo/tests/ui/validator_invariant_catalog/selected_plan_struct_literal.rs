use topology::facade::WorthTopologySelectedLegalityObligationPlan;

fn main() {
    let _ = WorthTopologySelectedLegalityObligationPlan {
        catalog_digest: String::new(),
        query_catalog_digest: String::new(),
        routing_closure_digest: String::new(),
        query_selection_digest: String::new(),
        selected_obligation_rows: Vec::new(),
        denial_rows: Vec::new(),
        counters: panic!("private selection counters unavailable"),
        phase_four_seed: panic!("private Phase 4 seed unavailable"),
        selected_plan_digest: String::new(),
    };
}
