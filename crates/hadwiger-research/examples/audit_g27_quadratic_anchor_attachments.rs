use hadwiger_research::facade::{
    admit_hadwiger_research_handle, audit_g27_quadratic_anchor_attachments_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = audit_g27_quadratic_anchor_attachments_checked(&handle)
        .expect("quadratic anchor attachment audit should replay");
    let (audited, suppressed, outside, eligible) = report.summary();
    println!(
        "audited {} suppressed_inside_field {} outside_field {} mutation_eligible {}",
        audited, suppressed, outside, eligible
    );
    for row in report.rows() {
        println!(
            "candidate {} radicand {} status {:?} unit_targets {:?}",
            row.candidate_id(),
            row.radicand(),
            row.status(),
            row.unit_targets()
        );
    }
}
