use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_same_field_mwis_odd_cycle_branch_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_same_field_mwis_odd_cycle_branch_checked(&handle)
        .expect("odd-cycle branch preflight should replay");
    let (
        atom_mask,
        dominant_vertices,
        exact_side_weight,
        dominant_threshold,
        root_total_upper,
        best_open_total_upper,
    ) = report.summary();
    let (nodes, open_nodes, pruned, max_depth, elapsed_millis) = report.search_summary();
    let (odd_cycle_cuts, max_node_millis) = report.lp_summary();
    println!(
        "atom_mask {} dominant_vertices {} exact_side_weight {} dominant_threshold {} root_total_odd_cycle_upper {} best_open_total_odd_cycle_upper {} nodes {} open_nodes {} pruned {} max_depth {} elapsed_millis {} odd_cycle_cuts {} max_node_millis {} status {:?} theorem_authority {}",
        atom_mask,
        dominant_vertices,
        exact_side_weight,
        dominant_threshold,
        root_total_upper,
        best_open_total_upper,
        nodes,
        open_nodes,
        pruned,
        max_depth,
        elapsed_millis,
        odd_cycle_cuts,
        max_node_millis,
        report.status(),
        report.admits_theorem_authority()
    );
}
