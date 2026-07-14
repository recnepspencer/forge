use hadwiger_research::facade::{
    admit_hadwiger_research_handle, preflight_g27_same_field_mwis_lp_guided_final_top_pair_checked,
    HadwigerResearchOperatingContext,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("Hadwiger handle admits");
    let report = preflight_g27_same_field_mwis_lp_guided_final_top_pair_checked(&handle)
        .expect("same-field LP-guided final top-pair preflight should run");
    println!(
        "checked_nodes {} certified_nodes {} certified_leaves {} remaining_best_open_total {} status {:?} theorem_authority false",
        report.checked_nodes,
        report.certified_nodes,
        report.certified_leaves,
        report.remaining_best_open_total,
        report.status
    );
    for node in report.nodes {
        println!(
            "node_index {} parent_total {} parent_depth {} first_branch {} first_child_totals {:?} second_branch {} terminal_totals {:?} certified_leaves {} explicit_rows {} positive_dual_rows {} max_denominator {} min_slack_floor {} max_objective_excess {} parent_digest {} row_digest {} status {:?}",
            node.index,
            node.parent_total,
            node.parent_depth,
            node.first_branch,
            node.first_child_totals,
            node.second_branch,
            node.terminal_totals,
            node.certified_leaves,
            node.explicit_rows,
            node.positive_dual_rows,
            node.max_denominator,
            node.min_slack_floor,
            node.max_objective_excess,
            node.parent_digest,
            node.row_digest,
            node.status
        );
    }
}
