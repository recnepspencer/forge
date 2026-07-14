use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_certificate_admission_gap_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_certificate_admission_gap_checked(&handle)
        .expect("certificate admission gap should replay");
    let (floor, ceil, target, target_pass, theorem_authority) = report.summary();
    println!(
        "status {:?} admitted_scope {} admitted_bound_floor {floor} admitted_bound_ceil {ceil} target_bound {target} target_pass {target_pass} theorem_authority {theorem_authority}",
        report.status(),
        report.admitted_scope()
    );
    println!("blockers {:?}", report.blockers());
    println!("conclusion {}", report.conclusion());
}
