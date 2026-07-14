import json

import run_w607_full16_micro_branch_stress as full16
import run_w607_leaf0_depth4_micro_branch as leaf0
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-leaf0-augmented-pool304.json"

LEAF_INDEX = 0
OLD_POOL = [151, 221, 224, 382, 385, 455]
AUGMENTED_POOL = [151, 221, 224, 303, 382, 385, 455]
DEPTHS = [3, 4]
NODE_CAP = 80
EXPORT_MAX = 586500.0
STRONG_MAX = 586224.0
RETIRE_MAX = 587500.0


def one_based(vertices):
    return [vertex + 1 for vertex in vertices]


def vertex_map(vertices):
    return [
        {"one_based": vertex + 1, "zero_based_solver_index": vertex}
        for vertex in vertices
    ]


def branch_trace(tree):
    return [item["vertex"] for item in tree["branch_variables_by_depth"]]


def summarize_tree(tree, baseline, old_bound=None):
    row = {
        "best_bound": tree["worst_leaf_objective"],
        "movement_from_baseline": baseline - tree["worst_leaf_objective"],
        "expanded_nodes": tree["expanded_count"],
        "nodes_solved": tree["nodes_solved"],
        "closed_leaves": tree["closed_leaves"],
        "open_leaves": tree["open_leaves"],
        "hit_node_cap": tree["hit_node_cap"],
        "branch_trace_one_based": branch_trace(tree),
        "worst_terminal": tree["worst_terminal"],
        "terminal_bounds_top8": tree["terminal_bounds"][:8],
    }
    if old_bound is not None:
        row["extra_movement_vs_old_pool_same_depth"] = old_bound - tree["worst_leaf_objective"]
    return row


def run_pool(edges, triads, weights, cuts, parent_rows, fixed, pool, baseline, old_results=None):
    report = {}
    for depth in DEPTHS:
        tree = leaf0.run_tree(edges, triads, weights, cuts, parent_rows, fixed, pool, depth)
        old_bound = None
        if old_results is not None:
            old_bound = old_results[str(depth)]["best_bound"]
        report[str(depth)] = summarize_tree(tree, baseline, old_bound)
    return report


def main():
    old_cap = leaf0.NODE_CAP
    leaf0.NODE_CAP = NODE_CAP
    try:
        source = json.loads(SOURCE.read_text())
        source_by_index = {row["leaf_index"]: row for row in source["leaves"]}
        edges, weights = parent.parse_edges_weights()
        weights = weights.astype(float)
        adj = parent.adjacency(edges)
        triads = parent.triangles(adj)
        root_cuts = parent_lift.root_cuts(weights, adj)
        parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
        root_obj, _root_x = parent_lift.solve_lp(
            edges, triads, weights, root_cuts, parent_rows, solution=True
        )
        _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
        finite = [candidate for candidate in leaves if candidate["feasible"]]
        leaf = finite[LEAF_INDEX]
        fixed = full16.fixed_from_leaf(leaf)
        family_cuts, family_rows = full16.first_family_cuts(
            source_by_index[LEAF_INDEX],
            leaf,
            edges,
            triads,
            weights,
            adj,
            root_cuts,
            parent_rows,
        )
        cuts = root_cuts + family_cuts
        baseline, x = parent_lift.solve_lp(
            edges,
            triads,
            weights,
            cuts,
            parent_rows,
            fixed=fixed,
            solution=True,
        )
        old_results = run_pool(edges, triads, weights, cuts, parent_rows, fixed, OLD_POOL, baseline)
        augmented_results = run_pool(
            edges,
            triads,
            weights,
            cuts,
            parent_rows,
            fixed,
            AUGMENTED_POOL,
            baseline,
            old_results,
        )
        best_augmented = min(row["best_bound"] for row in augmented_results.values())
        any_cap_hit = any(row["hit_node_cap"] for row in old_results.values()) or any(
            row["hit_node_cap"] for row in augmented_results.values()
        )
        augmented_traces = [vertex for row in augmented_results.values() for vertex in row["branch_trace_one_based"]]
        uses_304 = 304 in augmented_traces
        status = "retire_augmented_pool304"
        if best_augmented <= STRONG_MAX and not any_cap_hit and uses_304:
            status = "strong_augmented_pool304_repair"
        elif best_augmented <= EXPORT_MAX and not any_cap_hit and uses_304:
            status = "fund_augmented_pool304_followup"
        report = {
            "schema": "forge.hadwiger.w607_leaf0_augmented_pool304.v1",
            "authority": "diagnostic_leaf0_pool_comparison_no_parent_authority",
            "second_opinion": {
                "agent": "Harvey",
                "decision": "approve_narrowly",
                "primary_failure_mode": "vertex_304_may_be_terminal_specific_decoy",
            },
            "setup_invariants": {
                "same_enriched_leaf0_row_system": True,
                "new_rows": 0,
                "new_supports": 0,
                "mwis_calls": 0,
                "global_or_all_leaf_rerun": False,
            },
            "vertex_indexing": {
                "reported_vertices": "one_based",
                "internal_solver_indices": "zero_based",
                "old_pool": vertex_map(OLD_POOL),
                "augmented_pool": vertex_map(AUGMENTED_POOL),
                "added_vertex": {"one_based": 304, "zero_based_solver_index": 303},
            },
            "baseline_root_objective": root_obj,
            "leaf_index": LEAF_INDEX,
            "tier_a_assignment": full16.fixed_summary(fixed),
            "first_family_rows_used": family_rows,
            "baseline_after_first_family": baseline,
            "baseline_augmented_pool_state": leaf0.pool_state(weights, x),
            "old_pool_one_based": one_based(OLD_POOL),
            "augmented_pool_one_based": one_based(AUGMENTED_POOL),
            "node_cap": NODE_CAP,
            "old_pool": old_results,
            "augmented_pool": augmented_results,
            "reproduction": {
                "baseline_ok": abs(baseline - 592402.1577) < 0.01,
                "old_depth3_ok": abs(old_results["3"]["best_bound"] - 589302.6440) < 0.01,
                "old_depth4_ok": abs(old_results["4"]["best_bound"] - 588378.8643) < 0.01,
            },
            "best_augmented_bound": best_augmented,
            "uses_304_in_augmented_traces": uses_304,
            "gates": {
                "export_max": EXPORT_MAX,
                "strong_max": STRONG_MAX,
                "retire_max": RETIRE_MAX,
                "node_cap": NODE_CAP,
            },
            "failure_reasons": [
                reason
                for reason, active in [
                    ("reproduction_failed", not all(
                        [
                            abs(baseline - 592402.1577) < 0.01,
                            abs(old_results["3"]["best_bound"] - 589302.6440) < 0.01,
                            abs(old_results["4"]["best_bound"] - 588378.8643) < 0.01,
                        ]
                    )),
                    ("node_cap_hit", any_cap_hit),
                    ("best_augmented_above_export_gate", best_augmented > EXPORT_MAX),
                    ("best_augmented_above_retire_max", best_augmented > RETIRE_MAX),
                    ("vertex_304_not_selected", not uses_304),
                ]
                if active
            ],
            "status": status,
        }
        OUT.write_text(json.dumps(report, indent=2) + "\n")
        print(
            json.dumps(
                {
                    key: value
                    for key, value in report.items()
                    if key not in {"old_pool", "augmented_pool"}
                },
                indent=2,
            )
        )
    finally:
        leaf0.NODE_CAP = old_cap


if __name__ == "__main__":
    main()
