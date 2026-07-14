use hadwiger_research::facade::{
    admit_hadwiger_research_handle, diagnose_g27_same_field_mwis_lp_guided_branch_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = diagnose_g27_same_field_mwis_lp_guided_branch_checked(&handle)
        .expect("same-field LP-guided branch diagnostic should run");
    let (checked, useful, worse, top_gain, top_drop, max_regression) = report.summary();
    println!(
        "checked_nodes {} useful_nodes {} worse_nodes {} top_relative_gain {} top_absolute_drop {} max_regression {} elapsed_millis {} status {:?} theorem_authority false",
        checked,
        useful,
        worse,
        top_gain,
        top_drop,
        max_regression,
        report.elapsed_millis(),
        report.status()
    );
    for (index, row) in report.rows().iter().enumerate() {
        let (
            parent_total,
            baseline_branch,
            lp_branch,
            lp_value_ppm,
            lp_score,
            baseline_worst,
            lp_worst,
            relative_gain,
            absolute_drop,
        ) = row.summary();
        println!(
            "row {} parent_total {} baseline_branch {} lp_branch {} lp_value_ppm {} lp_score {} baseline_worst_child_total {} lp_worst_child_total {} relative_gain {} absolute_drop {}",
            index,
            parent_total,
            baseline_branch,
            lp_branch,
            lp_value_ppm,
            lp_score,
            baseline_worst,
            lp_worst,
            relative_gain,
            absolute_drop
        );
    }
}
