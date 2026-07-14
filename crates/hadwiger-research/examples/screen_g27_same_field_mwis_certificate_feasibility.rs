use hadwiger_research::facade::{
    admit_hadwiger_research_handle, screen_g27_same_field_mwis_certificate_feasibility_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = screen_g27_same_field_mwis_certificate_feasibility_checked(&handle)
        .expect("certificate feasibility screen should replay");
    let (g27_anchor, w_anchor) = report.alignment();
    println!(
        "alignment g27 {} w {} target {} theorem_authority {}",
        g27_anchor,
        w_anchor,
        report.target_weight(),
        report.admits_theorem_authority()
    );
    for channel in report.channels() {
        let (
            dominant_vertices,
            dominant_edges,
            small_weight,
            dominant_threshold,
            replayed_best_total,
            clique_cover_upper,
            clique_lp_total,
            odd_cycle_lp_total,
        ) = channel.summary();
        let (odd_cuts, odd_rounds) = channel.cut_summary();
        println!(
            "channel atom_mask {} dominant_vertices {} dominant_edges {} small {} dominant_threshold {} replayed_best_total {} clique_cover_upper {} clique_lp_total {} odd_cycle_lp_total {} odd_cuts {} odd_rounds {} status {:?}",
            channel.atom_mask(),
            dominant_vertices,
            dominant_edges,
            small_weight,
            dominant_threshold,
            replayed_best_total,
            clique_cover_upper,
            clique_lp_total,
            odd_cycle_lp_total,
            odd_cuts,
            odd_rounds,
            channel.status()
        );
    }
}
