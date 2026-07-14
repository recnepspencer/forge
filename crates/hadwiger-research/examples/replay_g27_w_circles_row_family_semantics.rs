use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_row_family_semantics_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_row_family_semantics_checked(&handle)
        .expect("row-family semantics should replay");
    let (checked_rows, parent_lift_rows) = report.summary();
    println!(
        "checked_rows {checked_rows} parent_lift_rows {parent_lift_rows} status {:?}",
        report.status()
    );
    println!("conclusion {}", report.conclusion());
}
