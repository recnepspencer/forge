use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_v304_include_dual_cover_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_v304_include_dual_cover_checked(&handle)
        .expect("v304 include dual cover should replay");
    let (numerator, denominator, clique_rows, min_slack) = report.summary();
    println!(
        "numerator {numerator} denominator {denominator} clique_rows {clique_rows} min_slack {min_slack} status {:?} target_authority {}",
        report.status(),
        report.admits_target_authority()
    );
    println!("conclusion {}", report.conclusion());
}
