use hadwiger_research::facade::{
    admit_hadwiger_research_handle,
    replay_g27_same_field_mwis_frontier_closure_exact_chunk_checked, HadwigerCanonicalArtifact,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = replay_g27_same_field_mwis_frontier_closure_exact_chunk_checked(&handle, 10, 14)
        .expect("same-field MWIS frontier closure exact chunk should run");
    let (
        selected_start,
        selected_end,
        checked_nodes,
        certified_nodes,
        certified_leaves,
        explicit_rows,
        positive_dual_rows,
        worst_terminal_total,
    ) = report.summary();
    let (max_denominator, min_slack_floor, max_objective_excess) = report.exact_summary();
    let (unresolved_start, unresolved_end) = report.unresolved_suffix();
    println!(
        "artifact_digest {} source_scout_digest {} selected_range {}..{} checked_nodes {} certified_nodes {} certified_leaves {} explicit_rows {} positive_dual_rows {} worst_terminal_total {} max_denominator {} min_slack_floor {} max_objective_excess {} unresolved_suffix {}..{} status {} theorem_authority {}",
        report.artifact_digest().stable_token(),
        report.scout_digest(),
        selected_start,
        selected_end,
        checked_nodes,
        certified_nodes,
        certified_leaves,
        explicit_rows,
        positive_dual_rows,
        worst_terminal_total,
        max_denominator,
        min_slack_floor,
        max_objective_excess,
        unresolved_start,
        unresolved_end,
        report.status().as_str(),
        report.admits_theorem_authority()
    );
    for node in report.nodes() {
        let (
            index,
            parent_total,
            parent_depth,
            first_branch,
            first_child_totals,
            worse_child,
            second_branch,
        ) = node.summary();
        let (
            certified_leaves,
            explicit_rows,
            positive_dual_rows,
            max_denominator,
            min_slack_floor,
            max_objective_excess,
        ) = node.exact_summary();
        println!(
            "exact_node index {} parent_total {} parent_depth {} parent_digest {} first_branch {} first_child_totals {:?} worse_child {} second_branch {} terminal_totals {:?} certified_leaves {} explicit_rows {} positive_dual_rows {} max_denominator {} min_slack_floor {} max_objective_excess {} row_digest {} status {}",
            index,
            parent_total,
            parent_depth,
            node.parent_digest(),
            first_branch,
            first_child_totals,
            worse_child,
            second_branch,
            node.terminal_totals(),
            certified_leaves,
            explicit_rows,
            positive_dual_rows,
            max_denominator,
            min_slack_floor,
            max_objective_excess,
            node.row_digest(),
            node.status().as_str()
        );
        for leaf in node.leaves() {
            let (
                leaf_index,
                terminal_total,
                certified_total,
                explicit_rows,
                positive_dual_rows,
                max_denominator,
                min_slack_floor,
                objective_excess,
            ) = leaf.summary();
            println!(
                "exact_leaf node_index {} leaf_index {} terminal_total {} certified_total {} explicit_rows {} positive_dual_rows {} max_denominator {} min_slack_floor {} objective_excess {} row_digest {} dual_digest {} status {}",
                index,
                leaf_index,
                terminal_total,
                certified_total,
                explicit_rows,
                positive_dual_rows,
                max_denominator,
                min_slack_floor,
                objective_excess,
                leaf.row_digest(),
                leaf.dual_digest(),
                leaf.status().as_str()
            );
        }
    }
}
