use hadwiger_research::facade::{
    admit_hadwiger_research_handle, diagnose_g27_same_field_mwis_full_frontier_shape_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = diagnose_g27_same_field_mwis_full_frontier_shape_checked(&handle)
        .expect("same-field full frontier shape diagnostic should run");
    let (open, tied_band, gap_to_second, best_open) = report.summary();
    println!(
        "open_frontier_nodes {} tied_band_nodes {} gap_to_second {} best_open_total_bound {} frontier_totals {:?} frontier_depths {:?} status {:?} theorem_authority false",
        open,
        tied_band,
        gap_to_second,
        best_open,
        report.top_open_totals(),
        report.top_open_depths(),
        report.status()
    );
}
