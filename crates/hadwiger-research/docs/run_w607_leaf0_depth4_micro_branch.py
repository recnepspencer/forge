import heapq
import json

import numpy as np

import run_w607_full16_micro_branch_stress as full16
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-leaf0-depth4-micro-branch.json"

LEAF_INDEX = 0
POOL = [151, 221, 224, 382, 385, 455]
NODE_CAP = 64
DEPTH_THREE = 3
DEPTH_FOUR = 4
EXPORT_MAX = 586500.0
STRONG_MAX = 586224.0
KILL_MOVEMENT = 1000.0
KILL_MAX = 588000.0


def solve_node(edges, triads, weights, cuts, parent_rows, fixed):
    try:
        objective, x = parent_lift.solve_lp(
            edges, triads, weights, cuts, parent_rows, fixed=fixed, solution=True
        )
        return {"feasible": True, "upper": objective, "x": x}
    except ValueError as error:
        return {"feasible": False, "upper": float("-inf"), "error": str(error), "x": None}


def choose_branch(node, weights, allowed):
    candidates = [
        vertex
        for vertex in allowed
        if vertex not in node["fixed"]
        and node["x"] is not None
        and 1e-7 < node["x"][vertex] < 1.0 - 1e-7
    ]
    if not candidates:
        return None
    return max(candidates, key=lambda vertex: (weights[vertex] * min(node["x"][vertex], 1.0 - node["x"][vertex]), -vertex))


def pool_state(weights, x):
    if x is None:
        return None
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


def pool_assignment(fixed):
    return {
        str(vertex + 1): float(fixed[vertex])
        for vertex in POOL
        if vertex in fixed
    }


def summarize_terminal(node, weights):
    return {
        "feasible": bool(node["feasible"]),
        "upper": float(node["upper"]),
        "depth": int(node["depth"]),
        "pool_assignment": pool_assignment(node["fixed"]),
        "pool_state": pool_state(weights, node["x"]),
    }


def run_tree(edges, triads, weights, cuts, parent_rows, base_fixed, allowed, max_depth):
    counter = 0
    root = solve_node(edges, triads, weights, cuts, parent_rows, base_fixed)
    root.update({"fixed": dict(base_fixed), "depth": 0, "branch_vertex": None})
    heap = [(-root["upper"], counter, root)]
    expanded = []
    closed = []
    while heap and len(expanded) < NODE_CAP:
        _priority, _order, node = heapq.heappop(heap)
        if not node["feasible"]:
            closed.append(node)
            continue
        branch = None if node["depth"] >= max_depth else choose_branch(node, weights, allowed)
        if branch is None:
            closed.append(node)
            continue
        node["branch_vertex"] = branch
        expanded.append(node)
        for value in (0.0, 1.0):
            fixed = dict(node["fixed"])
            fixed[branch] = value
            child = solve_node(edges, triads, weights, cuts, parent_rows, fixed)
            child.update({"fixed": fixed, "depth": node["depth"] + 1, "branch_vertex": None})
            counter += 1
            if child["feasible"]:
                heapq.heappush(heap, (-child["upper"], counter, child))
            else:
                closed.append(child)
    open_nodes = [item[2] for item in heap]
    terminals = closed + open_nodes
    feasible = [node for node in terminals if node["feasible"]]
    worst = max(feasible, key=lambda node: node["upper"], default=None)
    terminal_bounds = sorted(
        [summarize_terminal(node, weights) for node in feasible],
        key=lambda row: -row["upper"],
    )
    return {
        "max_depth": max_depth,
        "nodes_solved": len(expanded) * 2 + 1,
        "expanded_count": len(expanded),
        "closed_leaves": len(closed),
        "open_leaves": len(open_nodes),
        "hit_node_cap": bool(heap and len(expanded) >= NODE_CAP),
        "worst_leaf_objective": float(worst["upper"]) if worst is not None else float("-inf"),
        "branch_variables_by_depth": [
            {"depth": int(node["depth"]), "vertex": node["branch_vertex"] + 1, "upper": float(node["upper"])}
            for node in expanded
        ],
        "worst_terminal": summarize_terminal(worst, weights) if worst is not None else None,
        "terminal_bounds": terminal_bounds,
    }


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
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    root_obj, _root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    leaf = finite[LEAF_INDEX]
    fixed = full16.fixed_from_leaf(leaf)
    family_cuts, family_rows = full16.first_family_cuts(
        source_by_index[LEAF_INDEX], leaf, edges, triads, weights, adj, root_cuts, parent_rows
    )
    cuts = root_cuts + family_cuts
    baseline, x = parent_lift.solve_lp(edges, triads, weights, cuts, parent_rows, fixed=fixed, solution=True)
    depth3 = run_tree(edges, triads, weights, cuts, parent_rows, fixed, POOL, DEPTH_THREE)
    depth4 = run_tree(edges, triads, weights, cuts, parent_rows, fixed, POOL, DEPTH_FOUR)
    extra_movement = depth3["worst_leaf_objective"] - depth4["worst_leaf_objective"]
    no_cap = not depth3["hit_node_cap"] and not depth4["hit_node_cap"]
    status = "retire_leaf0_depth4"
    if depth4["worst_leaf_objective"] <= STRONG_MAX and no_cap:
        status = "strong_leaf0_depth4_repair"
    elif depth4["worst_leaf_objective"] <= EXPORT_MAX and extra_movement >= KILL_MOVEMENT and no_cap:
        status = "fund_exceptional_leaf_depth_mix"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_leaf0_depth4_micro_branch.v1",
            "authority": "diagnostic_leaf0_depth4_no_parent_authority",
            "second_opinion": {
                "agent": "Halley",
                "decision": "approve_narrow_falsification_pass",
                "primary_failure_mode": "apparent movement from asymmetric repeated branching may not export",
            },
            "baseline_root_objective": root_obj,
            "leaf_index": LEAF_INDEX,
            "tier_a_assignment": full16.fixed_summary(fixed),
            "fixed_pool": [vertex + 1 for vertex in POOL],
            "first_family_rows_used": family_rows,
            "baseline_after_first_family": baseline,
            "baseline_pool_state": pool_state(weights, x),
            "depth3_tree": depth3,
            "depth4_tree": depth4,
            "depth3_movement": baseline - depth3["worst_leaf_objective"],
            "depth4_movement": baseline - depth4["worst_leaf_objective"],
            "depth3_to_depth4_movement": extra_movement,
            "reproduces_full16_leaf0_depth3": abs(depth3["worst_leaf_objective"] - 589302.6440) < 0.01,
            "gates": {
                "node_cap": NODE_CAP,
                "export_max": EXPORT_MAX,
                "strong_max": STRONG_MAX,
                "kill_extra_movement": KILL_MOVEMENT,
                "kill_max": KILL_MAX,
            },
            "failure_reasons": [
                reason
                for reason, active in [
                    ("depth3_reproduction_failed", abs(depth3["worst_leaf_objective"] - 589302.6440) >= 0.01),
                    ("node_cap_hit", not no_cap),
                    ("depth4_above_export_gate", depth4["worst_leaf_objective"] > EXPORT_MAX),
                    ("extra_movement_below_gate", extra_movement < KILL_MOVEMENT),
                    ("depth4_above_kill_max", depth4["worst_leaf_objective"] > KILL_MAX),
                ]
                if active
            ],
            "status": status,
        }
    )
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if not key.endswith("_tree")}, indent=2))


if __name__ == "__main__":
    main()
