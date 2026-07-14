use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_parent_lift_readiness_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_parent_lift_readiness_checked(&handle)
        .expect("parent-lift readiness should replay");
    let (checked_rows, parent_lift_rows, theorem_authority) = report.summary();
    println!(
        "checked_rows {checked_rows} parent_lift_rows {parent_lift_rows} theorem_authority {theorem_authority} ids {:?} status {:?}",
        report.parent_lift_ids(),
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
