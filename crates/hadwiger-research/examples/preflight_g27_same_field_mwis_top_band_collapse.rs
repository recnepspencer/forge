use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_same_field_mwis_top_band_collapse_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_same_field_mwis_top_band_collapse_checked(&handle)
        .expect("same-field top-band collapse preflight should run");
    let (initial, final_best, tied, selected, expanded, prunes) = report.summary();
    let (open, elapsed) = report.search_summary();
    println!(
        "initial_best_total {} final_best_total {} final_tied_band_nodes {} selected_origin_count {} expanded_nodes {} solver_pruned_descendants {} open_frontier_nodes {} elapsed_millis {} status {:?} theorem_authority false",
        initial,
        final_best,
        tied,
        selected,
        expanded,
        prunes,
        open,
        elapsed,
        report.status()
    );
}
