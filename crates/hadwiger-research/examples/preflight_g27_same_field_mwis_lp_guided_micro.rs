use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_same_field_mwis_lp_guided_micro_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_same_field_mwis_lp_guided_micro_checked(&handle)
        .expect("same-field LP-guided micro preflight should run");
    let (
        parent_total,
        first_worst_total,
        second_branch,
        final_worst_total,
        additional_drop,
        elapsed_millis,
        solver_prunes,
    ) = report.summary();
    println!(
        "parent_total {} first_worst_total {} second_branch {} final_worst_total {} additional_drop {} solver_prune_candidates {} elapsed_millis {} status {:?} theorem_authority false",
        parent_total,
        first_worst_total,
        second_branch,
        final_worst_total,
        additional_drop,
        solver_prunes,
        elapsed_millis,
        report.status()
    );
}
