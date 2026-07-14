use hadwiger_research::facade::{
    admit_hadwiger_research_handle, replay_g27_same_field_mwis_odd_cycle_rows_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_mwis_odd_cycle_rows_checked(&handle)
        .expect("odd-cycle row replay should run");
    let (root_total, pruned, checked, clique_rows, odd_cycle_rows) = report.summary();
    let (max_cycle, metadata_bytes, digest) = report.metadata_summary();
    println!(
        "root_total_odd_cycle_bound {} pruned_nodes {} checked_nodes {} clique_rows {} odd_cycle_rows {} max_odd_cycle_length {} metadata_bytes {} row_digest {} status {:?} theorem_authority {}",
        root_total,
        pruned,
        checked,
        clique_rows,
        odd_cycle_rows,
        max_cycle,
        metadata_bytes,
        digest,
        report.status(),
        report.admits_theorem_authority()
    );
}
