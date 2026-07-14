import json

import numpy as np

import run_w607_post_family_micro_branch as micro
import run_w607_full_tree_rank_family as full_family
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-full16-micro-branch-stress.json"

POOL = [151, 221, 224, 382, 385, 455]
DEPTH = 3
NODE_CAP = 24
EXPORT_MAX = 586500.0
MATERIAL_MOVEMENT = 5000.0


def support_hash(vertices):
    import hashlib

    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def fixed_from_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def fixed_summary(fixed):
    return {
        "included": [v + 1 for v, value in sorted(fixed.items()) if value == 1.0],
        "excluded": [v + 1 for v, value in sorted(fixed.items()) if value == 0.0],
    }


def first_family_cuts(report, leaf, edges, triads, weights, adj, root_cuts, parent_rows):
    fixed = fixed_from_leaf(leaf)
    _base, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, parent_rows, fixed, True)
    candidates = {support_hash(row["vertices"]): row for row in full_family.candidate_rows(weights, x, fixed, adj)}
    cuts = []
    rows = []
    for accepted in report["accepted_rows"]:
        row = candidates[accepted["support_digest"]]
        cuts.append((row["vertices"], int(accepted["alpha_w"])))
        rows.append(
            {
                "template_id": accepted["template_id"],
                "center": accepted["center"],
                "support_digest": accepted["support_digest"],
                "size": accepted["size"],
                "alpha_w": int(accepted["alpha_w"]),
            }
        )
    return cuts, rows


def pool_state(weights, x):
    return [
        {
            "vertex": vertex + 1,
            "weight": float(weights[vertex]),
            "lp_value": float(x[vertex]),
            "score": float(weights[vertex] * min(x[vertex], 1.0 - x[vertex])),
            "fractional": bool(1e-7 < x[vertex] < 1.0 - 1e-7),
        }
        for vertex in POOL
    ]


def worst_terminal_assignment(tree):
    candidates = [row for row in tree["child_objectives"] if row["feasible"]]
    if not candidates:
        return None
    return max(candidates, key=lambda row: row["upper"])


def branch_trace(tree):
    return [item["vertex"] for item in tree["branch_variables_by_depth"]]


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "x"}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, tuple):
        return [clean(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    source = json.loads(SOURCE.read_text())
    source_by_index = {row["leaf_index"]: row for row in source["leaves"]}
    top_six_ids = {row["leaf_index"] for row in sorted(source["leaves"], key=lambda row: -row["final_objective"])[:6]}
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    root_obj, _root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    reports = []
    for index, leaf in enumerate(finite):
        report = source_by_index[index]
        fixed = fixed_from_leaf(leaf)
        first_cuts, first_rows = first_family_cuts(report, leaf, edges, triads, weights, adj, root_cuts, parent_rows)
        cuts = root_cuts + first_cuts
        baseline, x = parent_lift.solve_lp(edges, triads, weights, cuts, parent_rows, fixed=fixed, solution=True)
        tree = micro.run_tree(edges, triads, weights, cuts, parent_rows, fixed, POOL, DEPTH)
        reports.append(
            {
                "leaf_index": index,
                "top_six": index in top_six_ids,
                "tier_a_assignment": fixed_summary(fixed),
                "first_family_rows_used": first_rows,
                "baseline_after_first_family": baseline,
                "pool_state": pool_state(weights, x),
                "depth3_worst": tree["worst_leaf_objective"],
                "movement": baseline - tree["worst_leaf_objective"],
                "nodes": tree["nodes_solved"],
                "closed_leaves": tree["closed_leaves"],
                "open_leaves": tree["open_leaves"],
                "hit_cap": tree["hit_node_cap"],
                "branch_variables_ordered": branch_trace(tree),
                "worst_terminal_assignment": worst_terminal_assignment(tree),
                "tree": tree,
            }
        )
    pre_max = max(row["baseline_after_first_family"] for row in reports)
    post_max = max(row["depth3_worst"] for row in reports)
    worst_before = max(reports, key=lambda row: row["baseline_after_first_family"])
    worst_after = max(reports, key=lambda row: row["depth3_worst"])
    cap_hit = any(row["hit_cap"] for row in reports)
    high_failures = [
        row["leaf_index"]
        for row in reports
        if row["baseline_after_first_family"] > 590000.0
        and row["movement"] < MATERIAL_MOVEMENT
        and row["baseline_after_first_family"] > EXPORT_MAX
    ]
    outside_bottlenecks = [
        row["leaf_index"]
        for row in reports
        if (not row["top_six"]) and row["depth3_worst"] > EXPORT_MAX
    ]
    passed = post_max <= EXPORT_MAX and not cap_hit and not high_failures and not outside_bottlenecks
    report = clean(
        {
            "schema": "forge.hadwiger.w607_full16_micro_branch_stress.v1",
            "authority": "diagnostic_fixed_pool_full_partition_stress_no_parent_authority",
            "baseline_root_objective": root_obj,
            "tier_a_partition_leaf_count": len(reports),
            "fixed_pool": [vertex + 1 for vertex in POOL],
            "depth": DEPTH,
            "node_cap": NODE_CAP,
            "pre_microbranch_full_16_max": pre_max,
            "post_microbranch_full_16_max": post_max,
            "full_16_movement": pre_max - post_max,
            "worst_leaf_before": worst_before["leaf_index"],
            "worst_leaf_after": worst_after["leaf_index"],
            "total_nodes": sum(row["nodes"] for row in reports),
            "any_cap_hit": cap_hit,
            "outside_top_six_bottlenecks": outside_bottlenecks,
            "high_leaf_material_movement_failures": high_failures,
            "branch_pattern_consensus": {
                "unique_ordered_traces": len({tuple(row["branch_variables_ordered"]) for row in reports}),
                "all_use_pool_only": all(set(row["branch_variables_ordered"]).issubset({v + 1 for v in POOL}) for row in reports),
            },
            "decision": "fund_export_lift_design" if passed else "retain_top_six_only",
            "failure_reasons": []
            if passed
            else [
                reason
                for reason, active in [
                    ("post_max_above_export_gate", post_max > EXPORT_MAX),
                    ("node_cap_hit", cap_hit),
                    ("outside_top_six_bottleneck", bool(outside_bottlenecks)),
                    ("high_leaf_insufficient_movement", bool(high_failures)),
                ]
                if active
            ],
            "gates": {
                "export_max": EXPORT_MAX,
                "node_cap": NODE_CAP,
                "material_movement": MATERIAL_MOVEMENT,
            },
            "leaves": reports,
        }
    )
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
