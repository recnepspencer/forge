use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_w_circles_weighted_rank_cuts_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_w_circles_weighted_rank_cuts_checked(&handle)
        .expect("weighted rank cuts should replay");
    let (root, cut_floor) = report.lp_summary();
    println!(
        "cuts {} root_lp {} weighted_rank_lp_floor {} status {:?} theorem_authority {}",
        report.rows().len(),
        root,
        cut_floor,
        report.status(),
        report.admits_theorem_authority()
    );
    for row in report.rows() {
        let (name, vertices, edges, weight_sum, alpha, violation, witness_size) = row.summary();
        println!(
            "cut {} vertices {} edges {} weight_sum {} alpha_w {} violation_numerator {} witness_size {}",
            name, vertices, edges, weight_sum, alpha, violation, witness_size
        );
    }
    println!("conclusion {}", report.conclusion());
}
