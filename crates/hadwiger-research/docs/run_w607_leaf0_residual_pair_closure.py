import itertools
import json

import run_w607_full16_micro_branch_stress as full16
import run_w607_leaf0_depth4_micro_branch as leaf0
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-leaf0-residual-pair-closure.json"

LEAF_INDEX = 0
POOL = [151, 221, 224, 382, 385, 455]
PAIR = [221, 455]
DEPTH_THREE = 3
DEPTH_FOUR = 4
NODE_CAP = 80
EXPORT_MAX = 586500.0
STRONG_MAX = 586224.0


def one_based(vertices):
    return [vertex + 1 for vertex in vertices]


def assignment_for(vertices, fixed):
    return {
        str(vertex + 1): float(fixed[vertex])
        for vertex in vertices
        if vertex in fixed
    }


def pair_state(weights, node):
    x = node.get("x")
    return [
        {
            "vertex": vertex + 1,
            "fixed": vertex in node["fixed"],
            "fixed_value": float(node["fixed"][vertex]) if vertex in node["fixed"] else None,
            "lp_value": None if x is None else float(x[vertex]),
            "fractional": bool(x is not None and 1e-7 < x[vertex] < 1.0 - 1e-7),
            "score": None if x is None else float(weights[vertex] * min(x[vertex], 1.0 - x[vertex])),
        }
        for vertex in PAIR
    ]


def terminal_summary(weights, node):
    return {
        "objective": float(node["upper"]),
        "depth": int(node["depth"]),
        "pool_assignment": assignment_for(POOL, node["fixed"]),
        "pair_state": pair_state(weights, node),
    }


def collect_depth4_terminals(tree):
    terminals = []
    for row in tree["terminal_bounds"]:
        copied = dict(row)
        copied["objective"] = float(row["upper"])
        pool_states = {state["vertex"]: state for state in row["pool_state"]}
        assignment = row["pool_assignment"]
        copied["pair_state"] = [
            {
                "vertex": vertex + 1,
                "fixed": str(vertex + 1) in assignment,
                "fixed_value": assignment.get(str(vertex + 1)),
                "lp_value": pool_states[vertex + 1]["lp_value"],
                "fractional": pool_states[vertex + 1]["fractional"],
                "score": pool_states[vertex + 1]["score"],
            }
            for vertex in PAIR
        ]
        terminals.append(copied)
    return terminals


def pair_unresolved(row):
    states = {state["vertex"]: state for state in row["pair_state"]}
    return all(not states[vertex]["fixed"] and states[vertex]["fractional"] for vertex in one_based(PAIR))


def solve_pair_children(edges, triads, weights, cuts, parent_rows, base_fixed):
    children = []
    for values in itertools.product([0.0, 1.0], repeat=len(PAIR)):
        fixed = dict(base_fixed)
        fixed.update({vertex: value for vertex, value in zip(PAIR, values)})
        child = leaf0.solve_node(edges, triads, weights, cuts, parent_rows, fixed)
        child.update({"fixed": fixed, "depth": len(fixed), "branch_vertex": None})
        children.append(
            {
                "assignment": {str(vertex + 1): float(value) for vertex, value in zip(PAIR, values)},
                "feasible": child["feasible"],
                "objective": float(child["upper"]),
                "pool_assignment": assignment_for(POOL, fixed),
            }
        )
    return children


def triggered_closures(edges, triads, weights, cuts, parent_rows, base_fixed, terminals):
    closures = []
    for row in terminals:
        trigger = row["objective"] > EXPORT_MAX and pair_unresolved(row)
        reason = []
        if row["objective"] <= EXPORT_MAX:
            reason.append("below_export_gate")
        if not pair_unresolved(row):
            reason.append("pair_not_unresolved_fractional")
        if trigger:
            fixed = dict(base_fixed)
            fixed.update({int(vertex) - 1: value for vertex, value in row["pool_assignment"].items()})
            children = solve_pair_children(edges, triads, weights, cuts, parent_rows, fixed)
            max_child = max(child["objective"] for child in children if child["feasible"])
        else:
            children = []
            max_child = row["objective"]
        closures.append(
            {
                "terminal": row,
                "triggered": trigger,
                "reason": "triggered" if trigger else ",".join(reason),
                "children": children,
                "closed_objective": max_child,
            }
        )
    return closures


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
        depth3 = leaf0.run_tree(edges, triads, weights, cuts, parent_rows, fixed, POOL, DEPTH_THREE)
        depth4 = leaf0.run_tree(edges, triads, weights, cuts, parent_rows, fixed, POOL, DEPTH_FOUR)
        terminals = collect_depth4_terminals(depth4)
        closures = triggered_closures(edges, triads, weights, cuts, parent_rows, fixed, terminals)
        final_closed_max = max(row["closed_objective"] for row in closures)
        triggered_count = sum(1 for row in closures if row["triggered"])
        extra_solves = 4 * triggered_count
        status = "retire_residual_pair_closure"
        if final_closed_max <= STRONG_MAX and triggered_count > 0:
            status = "strong_residual_pair_closure"
        elif final_closed_max <= EXPORT_MAX and triggered_count > 0:
            status = "fund_residual_pair_closure_followup"
        report = {
            "schema": "forge.hadwiger.w607_leaf0_residual_pair_closure.v1",
            "authority": "diagnostic_leaf0_residual_pair_closure_no_parent_authority",
            "second_opinion": {
                "agent": "Schrodinger",
                "decision": "approve_one_bounded_diagnostic",
                "primary_failure_mode": "non_exportable_leaf_surgery",
            },
            "setup_invariants": {
                "same_enriched_leaf0_row_system": True,
                "new_rows": 0,
                "new_supports": 0,
                "mwis_calls": 0,
                "augmented_variables": 0,
                "global_or_all_leaf_rerun": False,
            },
            "solver_notes": {
                "lp_backend": "same parent_lift.solve_lp path as prior diagnostics",
                "timeout_hit": False,
                "node_cap_hit": depth3["hit_node_cap"] or depth4["hit_node_cap"],
            },
            "baseline_root_objective": root_obj,
            "leaf_index": LEAF_INDEX,
            "tier_a_assignment": full16.fixed_summary(fixed),
            "old_pool_one_based": one_based(POOL),
            "residual_pair_one_based": one_based(PAIR),
            "first_family_rows_used": family_rows,
            "baseline_after_first_family": baseline,
            "baseline_pool_state": leaf0.pool_state(weights, x),
            "depth3_bound": depth3["worst_leaf_objective"],
            "depth4_bound": depth4["worst_leaf_objective"],
            "depth4_branch_trace_one_based": [
                item["vertex"] for item in depth4["branch_variables_by_depth"]
            ],
            "depth4_terminals": terminals,
            "closures": closures,
            "resource_usage": {
                "node_cap": NODE_CAP,
                "depth3_nodes_solved": depth3["nodes_solved"],
                "depth4_nodes_solved": depth4["nodes_solved"],
                "triggered_residual_nodes": triggered_count,
                "extra_lp_solves": extra_solves,
                "total_lp_solves_estimate": depth3["nodes_solved"] + depth4["nodes_solved"] + extra_solves,
            },
            "final_closed_leaf0_max": final_closed_max,
            "reproduction": {
                "baseline_ok": abs(baseline - 592402.1577) < 0.01,
                "depth3_ok": abs(depth3["worst_leaf_objective"] - 589302.6440) < 0.01,
                "depth4_ok": abs(depth4["worst_leaf_objective"] - 588378.8643) < 0.01,
            },
            "gates": {
                "export_max": EXPORT_MAX,
                "strong_max": STRONG_MAX,
                "node_cap": NODE_CAP,
            },
            "failure_reasons": [
                reason
                for reason, active in [
                    ("reproduction_failed", not all(
                        [
                            abs(baseline - 592402.1577) < 0.01,
                            abs(depth3["worst_leaf_objective"] - 589302.6440) < 0.01,
                            abs(depth4["worst_leaf_objective"] - 588378.8643) < 0.01,
                        ]
                    )),
                    ("node_cap_hit", depth3["hit_node_cap"] or depth4["hit_node_cap"]),
                    ("no_residual_nodes_triggered", triggered_count == 0),
                    ("final_closed_max_above_export_gate", final_closed_max > EXPORT_MAX),
                    ("incomplete_pair_responsibility", any(
                        row["terminal"]["objective"] > EXPORT_MAX
                        and not row["triggered"]
                        for row in closures
                    )),
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
                    if key not in {"depth4_terminals", "closures"}
                },
                indent=2,
            )
        )
    finally:
        leaf0.NODE_CAP = old_cap


if __name__ == "__main__":
    main()
