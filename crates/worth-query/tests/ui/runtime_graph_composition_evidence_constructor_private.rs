use worth_query::facade::runtime::WorthQueryGraphCompositionEvidence;

fn main() {
    let _ = WorthQueryGraphCompositionEvidence {
        graph_composition_digest: String::new(),
        graph_symbolic_resolution_digest: String::new(),
        graph_assumption_digest: None,
        counter_snapshot: String::new(),
        lifecycle_counter_snapshot: String::new(),
        symbolic_resolution_count: 0,
        affected_live_view_count: 0,
        affected_derived_view_count: 0,
        considered_computed_view_count: 0,
        assumption_summary: None,
    };
}
