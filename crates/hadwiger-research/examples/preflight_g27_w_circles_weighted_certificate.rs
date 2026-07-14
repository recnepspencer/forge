use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_w_circles_weighted_certificate_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_w_circles_weighted_certificate_checked(&handle)
        .expect("W_circles_607 certificate preflight should replay");
    let (vertices, edges, weights, weight_sum, target) = report.shape_summary();
    let (witness_weight, witness_size, cover, edge_lp, clique_lp, odd_lp) = report.bound_summary();
    let (odd_cuts, odd_rounds, best_violation, cliques, cap_hit, largest_clique) =
        report.cut_summary();
    println!(
        "vertices {} edges {} weights {} weight_sum {} target {} witness_weight {} witness_size {} clique_cover {} edge_lp {} clique_lp {} odd_cycle_lp {} odd_cuts {} odd_rounds {} best_violation_ppm {} maximal_cliques {} cap_hit {} largest_clique {} status {:?} theorem_authority {}",
        vertices,
        edges,
        weights,
        weight_sum,
        target,
        witness_weight,
        witness_size,
        cover,
        edge_lp,
        clique_lp,
        odd_lp,
        odd_cuts,
        odd_rounds,
        best_violation,
        cliques,
        cap_hit,
        largest_clique,
        report.status(),
        report.admits_theorem_authority()
    );
    println!("conclusion {}", report.conclusion());
}
