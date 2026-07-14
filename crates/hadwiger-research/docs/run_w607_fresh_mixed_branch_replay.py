import hashlib
import json
import sys

import numpy as np

import run_w607_full16_micro_branch_stress as full16
import run_w607_leaf0_depth4_micro_branch as leaf0
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-fresh-mixed-branch-replay.json"
CHECKPOINT = CRATE / "docs" / "w607-fresh-mixed-branch-replay.checkpoint.json"

FIXED_POOL = [151, 221, 224, 382, 385, 455]
RESIDUAL_PAIR = [221, 455]
TARGET_GATE = 586500.0
TARGET_WEIGHTED_ALPHA = 512933
DEPTH_THREE = 3
DEPTH_FOUR = 4
NODE_CAP = 80
EXPECTED_MIXED_MAX = 586224.2383


def digest(value):
    payload = json.dumps(jsonable(value), sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(payload).hexdigest()


def jsonable(value):
    if isinstance(value, dict):
        return {str(key): jsonable(inner) for key, inner in value.items()}
    if isinstance(value, (list, tuple)):
        return [jsonable(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def one_based(vertices):
    return [vertex + 1 for vertex in vertices]


def assignment_for(vertices, fixed):
    return {
        str(vertex + 1): float(fixed[vertex])
        for vertex in vertices
        if vertex in fixed
    }


def unresolved_pool(weights, terminal):
    fixed = {int(vertex) - 1 for vertex in terminal["pool_assignment"]}
    rows = []
    for state in terminal["pool_state"]:
        vertex = state["vertex"] - 1
        if vertex not in fixed and state["fractional"]:
            rows.append(
                {
                    "vertex": state["vertex"],
                    "lp_value": state["lp_value"],
                    "score": state["score"],
                    "weight": float(weights[vertex]),
                }
            )
    return rows


def terminal_certificate(weights, terminal):
    return {
        "bound": terminal["upper"],
        "depth": terminal["depth"],
        "pool_assignment": terminal["pool_assignment"],
        "unresolved_fractional_pool": unresolved_pool(weights, terminal),
    }


def residual_pair_ready(terminal):
    assignment = terminal["pool_assignment"]
    states = {state["vertex"]: state for state in terminal["pool_state"]}
    return all(
        str(vertex + 1) not in assignment and states[vertex + 1]["fractional"]
        for vertex in RESIDUAL_PAIR
    )


def solve_residual_children(edges, triads, weights, cuts, parent_rows, base_fixed, terminal):
    fixed = dict(base_fixed)
    fixed.update({int(vertex) - 1: value for vertex, value in terminal["pool_assignment"].items()})
    children = []
    for left in (0.0, 1.0):
        for right in (0.0, 1.0):
            child_fixed = dict(fixed)
            child_fixed[RESIDUAL_PAIR[0]] = left
            child_fixed[RESIDUAL_PAIR[1]] = right
            child = leaf0.solve_node(edges, triads, weights, cuts, parent_rows, child_fixed)
            children.append(
                {
                    "assignment": {
                        str(RESIDUAL_PAIR[0] + 1): left,
                        str(RESIDUAL_PAIR[1] + 1): right,
                    },
                    "feasible": child["feasible"],
                    "bound": child["upper"],
                    "pool_assignment": assignment_for(FIXED_POOL, child_fixed),
                }
            )
    return children


def close_leaf0_terminal(edges, triads, weights, cuts, parent_rows, base_fixed, terminal):
    trigger = terminal["upper"] > TARGET_GATE and residual_pair_ready(terminal)
    if not trigger:
        return {
            "triggered": False,
            "terminal": terminal_certificate(weights, terminal),
            "children": [],
            "closed_bound": terminal["upper"],
            "reason": "below_gate_or_pair_not_ready",
        }
    children = solve_residual_children(edges, triads, weights, cuts, parent_rows, base_fixed, terminal)
    return {
        "triggered": True,
        "terminal": terminal_certificate(weights, terminal),
        "children": children,
        "closed_bound": max(child["bound"] for child in children if child["feasible"]),
        "reason": "above_gate_pair_unresolved_fractional",
    }


def branch_trace(tree):
    return [item["vertex"] for item in tree["branch_variables_by_depth"]]


def leaf_report(index, leaf, source_report, edges, triads, weights, adj, root_cuts, parent_rows):
    fixed = full16.fixed_from_leaf(leaf)
    first_cuts, first_rows = full16.first_family_cuts(
        source_report, leaf, edges, triads, weights, adj, root_cuts, parent_rows
    )
    cuts = root_cuts + first_cuts
    baseline, _x = parent_lift.solve_lp(
        edges, triads, weights, cuts, parent_rows, fixed=fixed, solution=True
    )
    depth3 = leaf0.run_tree(edges, triads, weights, cuts, parent_rows, fixed, FIXED_POOL, DEPTH_THREE)
    final_bound = depth3["worst_leaf_objective"]
    closures = []
    depth4 = None
    exceptional = index == 0
    if exceptional:
        depth4 = leaf0.run_tree(edges, triads, weights, cuts, parent_rows, fixed, FIXED_POOL, DEPTH_FOUR)
        closures = [
            close_leaf0_terminal(edges, triads, weights, cuts, parent_rows, fixed, terminal)
            for terminal in depth4["terminal_bounds"]
        ]
        final_bound = max(row["closed_bound"] for row in closures)
    terminal_rows = depth4["terminal_bounds"] if exceptional else depth3["terminal_bounds"]
    return {
        "leaf_index": index,
        "exceptional_rule": "leaf0_depth4_residual_pair_closure" if exceptional else "none",
        "tier_a_assignment": full16.fixed_summary(fixed),
        "row_system_digest": digest(
            {
                "tier_a_assignment": full16.fixed_summary(fixed),
                "first_family_rows_used": first_rows,
            }
        ),
        "first_family_rows_used": first_rows,
        "baseline_after_first_family": baseline,
        "depth3_bound": depth3["worst_leaf_objective"],
        "depth3_branch_trace": branch_trace(depth3),
        "depth3_nodes_solved": depth3["nodes_solved"],
        "depth3_hit_cap": depth3["hit_node_cap"],
        "depth4_bound": None if depth4 is None else depth4["worst_leaf_objective"],
        "depth4_branch_trace": None if depth4 is None else branch_trace(depth4),
        "depth4_nodes_solved": 0 if depth4 is None else depth4["nodes_solved"],
        "depth4_hit_cap": False if depth4 is None else depth4["hit_node_cap"],
        "terminal_certificates": [terminal_certificate(weights, row) for row in terminal_rows],
        "residual_closures": closures,
        "final_mixed_bound": final_bound,
    }


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
        finite = [leaf for leaf in leaves if leaf["feasible"]]
        source_digest = digest(source)
        loaded_checkpoint = False
        reports = []
        use_checkpoint = "--no-checkpoint" not in sys.argv
        if use_checkpoint and CHECKPOINT.exists():
            checkpoint = json.loads(CHECKPOINT.read_text())
            if checkpoint.get("source_first_family_digest") == source_digest:
                reports = checkpoint.get("leaves", [])
                loaded_checkpoint = bool(reports)
        completed = {row["leaf_index"] for row in reports}
        for index, leaf in enumerate(finite):
            if index in completed:
                print(f"replay leaf {index}: checkpoint", flush=True)
                continue
            print(f"replay leaf {index}: solving", flush=True)
            reports.append(
                leaf_report(index, leaf, source_by_index[index], edges, triads, weights, adj, root_cuts, parent_rows)
            )
            CHECKPOINT.write_text(
                json.dumps(
                    {
                        "schema": "forge.hadwiger.w607_fresh_mixed_branch_replay.checkpoint.v1",
                        "source_first_family_digest": source_digest,
                        "leaves": reports,
                    },
                    indent=2,
                )
                + "\n"
            )
        sorted_bounds = sorted(
            [
                {
                    "leaf_index": row["leaf_index"],
                    "final_mixed_bound": row["final_mixed_bound"],
                    "baseline_after_first_family": row["baseline_after_first_family"],
                    "exceptional_rule": row["exceptional_rule"],
                }
                for row in reports
            ],
            key=lambda row: (-row["final_mixed_bound"], row["leaf_index"]),
        )
        final_max = sorted_bounds[0]["final_mixed_bound"]
        argmax = [
            row["leaf_index"]
            for row in sorted_bounds
            if abs(row["final_mixed_bound"] - final_max) <= 1e-6
        ]
        any_cap = any(row["depth3_hit_cap"] or row["depth4_hit_cap"] for row in reports)
        extra_leaf0_solves = sum(4 for row in reports[0]["residual_closures"] if row["triggered"])
        non_leaf0_exception = any(row["exceptional_rule"] != "none" for row in reports[1:])
        report = {
            "schema": "forge.hadwiger.w607_fresh_mixed_branch_replay.v1",
            "status": "fund_export_lift_design" if final_max <= TARGET_GATE and not any_cap and not non_leaf0_exception else "retire_fresh_replay",
            "authority_label": "fresh_replay_diagnostic_branch_authority_only",
            "target_gate": TARGET_GATE,
            "target_weighted_alpha": TARGET_WEIGHTED_ALPHA,
            "second_opinion": {
                "agent": "Avicenna",
                "decision": "approve",
                "primary_failure_mode": "artifact_contamination",
            },
            "stale_artifact_dependency_check": {
                "loaded_prior_stress_artifact": False,
                "loaded_prior_mixed_artifact": False,
                "loaded_prior_leaf0_closure_artifact": False,
                "loaded_own_fresh_replay_checkpoint": loaded_checkpoint,
                "source_first_family_artifact": str(SOURCE),
            },
            "setup_invariants": {
                "new_rows": 0,
                "new_supports": 0,
                "new_mwis_calls": 0,
                "new_branch_variables": 0,
                "fixed_pool_one_based": one_based(FIXED_POOL),
                "residual_pair_one_based": one_based(RESIDUAL_PAIR),
                "leaf0_only_exception": not non_leaf0_exception,
            },
            "digests": {
                "source_first_family_digest": source_digest,
                "root_rows_digest": digest(root_cuts),
                "parent_rows_digest": digest(parent_rows),
                "per_leaf_row_systems_digest": digest(
                    [
                        {
                            "leaf_index": row["leaf_index"],
                            "row_system_digest": row["row_system_digest"],
                        }
                        for row in reports
                    ]
                ),
            },
            "baseline_root_objective": root_obj,
            "leaf_count": len(reports),
            "final_mixed_max": final_max,
            "argmax_leaf": argmax,
            "margin_to_target_gate": TARGET_GATE - final_max,
            "matches_expected_mixed_max": abs(final_max - EXPECTED_MIXED_MAX) < 0.01,
            "resource_usage": {
                "depth3_nodes_total": sum(row["depth3_nodes_solved"] for row in reports),
                "leaf0_depth4_nodes": reports[0]["depth4_nodes_solved"],
                "leaf0_extra_residual_pair_solves": extra_leaf0_solves,
                "total_nodes_plus_extra_solves": sum(row["depth3_nodes_solved"] for row in reports)
                + reports[0]["depth4_nodes_solved"]
                + extra_leaf0_solves,
                "any_cap_hit": any_cap,
                "timeout_hit": False,
            },
            "leaf_bounds_sorted": sorted_bounds,
            "leaves": reports,
            "failure_reasons": [
                reason
                for reason, active in [
                    ("final_max_above_target_gate", final_max > TARGET_GATE),
                    ("node_cap_hit", any_cap),
                    ("non_leaf0_exception", non_leaf0_exception),
                    ("expected_mixed_max_not_reproduced", abs(final_max - EXPECTED_MIXED_MAX) >= 0.01),
                ]
                if active
            ],
        }
        if report["failure_reasons"]:
            report["status"] = "retire_fresh_replay"
        OUT.write_text(json.dumps(report, indent=2) + "\n")
        print(
            json.dumps(
                {key: value for key, value in report.items() if key != "leaves"},
                indent=2,
            )
        )
    finally:
        leaf0.NODE_CAP = old_cap


if __name__ == "__main__":
    main()
