use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_branch_slack_lift_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_branch_slack_lift_checked(&handle)
        .expect("branch-slack lift should replay");
    let (gamma0, gamma1, lift, rows) = report.summary();
    println!(
        "gamma0_num {gamma0} gamma1_num {gamma1} lift_num {lift} rows {rows} status {:?}",
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
