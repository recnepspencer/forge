use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_semantic_partition_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_semantic_partition_checked(&handle)
        .expect("semantic partition should replay");
    let (tier_assignments, terminals, rows) = report.summary();
    println!(
        "tier_assignments {tier_assignments} terminals {terminals} rows {rows} status {:?}",
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
