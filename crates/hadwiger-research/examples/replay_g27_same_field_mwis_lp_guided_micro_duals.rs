use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_same_field_mwis_lp_guided_micro_duals_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_mwis_lp_guided_micro_duals_checked(&handle)
        .expect("same-field LP-guided micro dual replay should run");
    let (checked, certified, explicit_rows, positive_rows, final_worst) = report.summary();
    let (max_denominator, min_slack, max_excess) = report.exact_summary();
    println!(
        "checked_nodes {} certified_prunes {} explicit_rows {} positive_dual_rows {} final_worst_total {} max_denominator {} min_slack_floor {} max_objective_excess {} row_digest {} status {:?} theorem_authority false",
        checked,
        certified,
        explicit_rows,
        positive_rows,
        final_worst,
        max_denominator,
        min_slack,
        max_excess,
        report.row_digest(),
        report.status()
    );
}
