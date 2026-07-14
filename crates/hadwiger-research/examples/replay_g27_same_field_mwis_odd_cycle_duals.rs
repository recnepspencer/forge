use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_same_field_mwis_odd_cycle_duals_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_mwis_odd_cycle_duals_checked(&handle)
        .expect("odd-cycle dual replay should run");
    let (checked, pruned, rows, positive_rows, root_total) = report.summary();
    let (max_denominator, min_slack_floor, max_objective_excess) = report.exact_summary();
    println!(
        "checked_nodes {} certified_pruned_nodes {} explicit_rows {} positive_dual_rows {} root_total_bound {} max_denominator {} min_slack_floor {} max_objective_excess {} status {:?} theorem_authority {}",
        checked,
        pruned,
        rows,
        positive_rows,
        root_total,
        max_denominator,
        min_slack_floor,
        max_objective_excess,
        report.status(),
        report.admits_theorem_authority()
    );
}
