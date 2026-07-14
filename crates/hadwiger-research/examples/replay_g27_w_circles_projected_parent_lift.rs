use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_projected_parent_lift_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_projected_parent_lift_checked(&handle)
        .expect("projected parent lift should replay");
    let (branch, gamma0, gamma1, lift) = report.summary();
    println!(
        "branch {branch} gamma0 {gamma0} gamma1 {gamma1} lift {lift} status {:?}",
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
