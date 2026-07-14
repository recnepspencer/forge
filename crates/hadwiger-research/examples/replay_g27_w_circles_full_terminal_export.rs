use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_full_terminal_export_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_full_terminal_export_checked(&handle)
        .expect("full-terminal export should replay");
    let (terminals, rows, worst_objective_floor, min_slack_floor) = report.summary();
    println!(
        "terminals {terminals} rows {rows} worst_objective_floor {worst_objective_floor} min_slack_floor {min_slack_floor} status {:?}",
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
