use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_same_field_mwis_lp_guided_second_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_same_field_mwis_lp_guided_second_checked(&handle)
        .expect("same-field LP-guided second-node preflight should run");
    let (
        parent_total,
        first_branch,
        first_worst_total,
        second_branch,
        final_worst_total,
        solver_prunes,
        certified_prunes,
    ) = report.summary();
    let (explicit_rows, positive_rows, max_denominator, min_slack, max_excess) =
        report.exact_summary();
    println!(
        "parent_total {} first_branch {} first_worst_total {} second_branch {} final_worst_total {} solver_prune_candidates {} certified_prunes {} explicit_rows {} positive_dual_rows {} max_denominator {} min_slack_floor {} max_objective_excess {} parent_digest {} row_digest {} status {:?} theorem_authority false",
        parent_total,
        first_branch,
        first_worst_total,
        second_branch,
        final_worst_total,
        solver_prunes,
        certified_prunes,
        explicit_rows,
        positive_rows,
        max_denominator,
        min_slack,
        max_excess,
        report.parent_digest(),
        report.row_digest(),
        report.status()
    );
}
