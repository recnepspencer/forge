use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_same_field_mwis_branch_prefix_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_mwis_branch_prefix_checked(&handle)
        .expect("same-field branch prefix replay should run");
    let (expanded, pruned, open, best_open_total, root_total, h32c_prunes) = report.summary();
    println!(
        "expanded_nodes {} pruned_nodes {} open_frontier_nodes {} best_open_total_bound {} root_total_bound {} h32c_certified_prunes {} status {:?} theorem_authority false",
        expanded,
        pruned,
        open,
        best_open_total,
        root_total,
        h32c_prunes,
        report.status()
    );
}
