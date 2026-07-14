use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_gamma1_leaf_dual_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_gamma1_leaf_dual_checked(&handle)
        .expect("gamma1 leaf dual should replay");
    let (leaves, rows, worst_objective, min_slack) = report.summary();
    println!(
        "leaves {leaves} rows {rows} worst_objective_num {worst_objective} min_slack {min_slack} status {:?}",
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
