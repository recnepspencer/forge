use hadwiger_research::facade::{
    admit_hadwiger_research_handle, scout_g27_same_field_mwis_frontier_closure_campaign_checked,
    HadwigerCanonicalArtifact, HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = scout_g27_same_field_mwis_frontier_closure_campaign_checked(&handle)
        .expect("same-field MWIS frontier closure campaign scout should run");
    let (frontier_nodes, scout_rows, ready, failing, worst_terminal, continuation_max) =
        report.summary();
    println!(
        "artifact_digest {} frontier_nodes {} scout_rows {} ready_count {} failing_count {} worst_terminal_total {} continuation_max_total {} status {} theorem_authority {}",
        report.artifact_digest().stable_token(),
        frontier_nodes,
        scout_rows,
        ready,
        failing,
        worst_terminal,
        continuation_max,
        report.status().as_str(),
        report.admits_theorem_authority()
    );
    for node in report.frontier_nodes() {
        let (index, total, depth, previously_closed) = node.summary();
        println!(
            "frontier_node index {} total {} depth {} digest {} previously_closed {}",
            index,
            total,
            depth,
            node.digest(),
            previously_closed
        );
    }
    for row in report.scout_rows() {
        let (
            index,
            parent_total,
            parent_depth,
            first_branch,
            first_child_totals,
            worse_child,
            second_branch,
        ) = row.summary();
        println!(
            "scout_row index {} parent_total {} parent_depth {} parent_digest {} first_branch {} first_child_totals {:?} worse_child {} second_branch {} terminal_totals {:?} row_class {}",
            index,
            parent_total,
            parent_depth,
            row.parent_digest(),
            first_branch,
            first_child_totals,
            worse_child,
            second_branch,
            row.terminal_totals(),
            row.row_class().as_str()
        );
    }
}
