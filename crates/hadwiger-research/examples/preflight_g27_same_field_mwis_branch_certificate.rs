use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_same_field_mwis_branch_certificate_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_same_field_mwis_branch_certificate_checked(&handle)
        .expect("branch certificate preflight should replay");
    let (
        atom_mask,
        dominant_vertices,
        small_weight,
        dominant_threshold,
        root_total_upper,
        best_open_total_upper,
    ) = report.summary();
    let (nodes, open_nodes, pruned, max_depth, elapsed_millis) = report.search_summary();
    println!(
        "atom_mask {} dominant_vertices {} small {} dominant_threshold {} root_total_upper {} best_open_total_upper {} nodes {} open_nodes {} pruned {} max_depth {} elapsed_millis {} status {:?} theorem_authority {}",
        atom_mask,
        dominant_vertices,
        small_weight,
        dominant_threshold,
        root_total_upper,
        best_open_total_upper,
        nodes,
        open_nodes,
        pruned,
        max_depth,
        elapsed_millis,
        report.status(),
        report.admits_theorem_authority()
    );
}
