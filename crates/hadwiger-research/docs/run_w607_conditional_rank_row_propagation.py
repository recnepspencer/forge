import hashlib
import heapq
import json
import time

import numpy as np

import run_w607_all_excluded_leaf_rank_diagnostic as leaf_rank
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
LEAF_ARTIFACT = CRATE / "docs" / "w607-all-excluded-leaf-rank-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-conditional-rank-row-propagation.json"

ROW_NAME = "wx_dense220_152"
ROW_ALPHA = 362026
EXPECTED_ROOT = 594914.351525072
EXPECTED_TIER_A_WORST = 594597.0511219536
EXPECTED_LEAF_WITH_ROW = 592402.1576826534
TIER_A = leaf_rank.FIXED_ZERO
MAX_DEPTH = 6
NODE_CAP = 64
ROOT_DROP_GATE = 1000.0
WORST_GATE = 590000.0
MAX_LEAF_IMPROVEMENT_GATE = 3000.0
KILL_ROOT_DROP = 250.0
KILL_WORST_GATE = 592000.0
KILL_MAX_LEAF_IMPROVEMENT = 2500.0


def support_digest(vertices):
    text = ",".join(str(v + 1) for v in vertices)
    return hashlib.sha256(text.encode()).hexdigest()


def p_parent_row(weights):
    return leaf_rank.p_parent_row(weights)


def reconstruct_row(edges, weights, adj, triads, root_cuts, extra_rows):
    fixed = {vertex: 0.0 for vertex in TIER_A}
    leaf_obj, x = leaf_rank.solve_lp(edges, triads, weights, root_cuts, extra_rows, fixed, True)
    raw = leaf_rank.candidate_supports(weights, x, adj)
    root_prior = leaf_rank.root_supports(weights.astype(int), adj)
    candidates = leaf_rank.dedupe_supports(raw, root_prior)
    for name, vertices, _root_overlap in candidates:
        if name == ROW_NAME:
            lhs = float(np.dot(weights[list(vertices)], x[list(vertices)]))
            row = {
                "name": name,
                "vertices": vertices,
                "alpha": ROW_ALPHA,
                "leaf_lhs": lhs,
                "leaf_violation": lhs - ROW_ALPHA,
                "digest": support_digest(vertices),
            }
            return leaf_obj, row
    raise ValueError(f"failed to regenerate {ROW_NAME}")


def solve_node(edges, triads, weights, cuts, extra_rows, fixed):
    try:
        objective, x = leaf_rank.solve_lp(edges, triads, weights, cuts, extra_rows, fixed, True)
        return {"feasible": True, "upper": objective, "x": x}
    except ValueError as err:
        return {"feasible": False, "upper": float("-inf"), "error": str(err), "x": None}


def branch_score(vertex, x_value, weight):
    return min(x_value, 1.0 - x_value) * weight


def choose_branch(x, weights, allowed, fixed):
    candidates = [v for v in allowed if v not in fixed and 1e-7 < x[v] < 1.0 - 1e-7]
    if not candidates:
        return None
    return max(candidates, key=lambda v: (branch_score(v, x[v], weights[v]), abs(x[v] - 0.5), -v))


def summarize_leaf(node):
    return {
        "included": [v + 1 for v, value in sorted(node["fixed"].items()) if value == 1.0],
        "excluded": [v + 1 for v, value in sorted(node["fixed"].items()) if value == 0.0],
        "upper": node["upper"],
        "depth": node["depth"],
    }


def run_tree(edges, triads, weights, cuts, extra_rows):
    counter = 0
    root = solve_node(edges, triads, weights, cuts, extra_rows, {})
    root.update({"fixed": {}, "depth": 0, "branch_vertex": None})
    heap = [(-root["upper"], counter, root)]
    expanded = []
    leaves = []
    while heap and len(expanded) < NODE_CAP:
        _priority, _order, node = heapq.heappop(heap)
        if not node["feasible"]:
            leaves.append(node)
            continue
        branch = None if node["depth"] >= MAX_DEPTH else choose_branch(node["x"], weights, TIER_A, node["fixed"])
        if branch is None:
            leaves.append(node)
            continue
        node["branch_vertex"] = branch
        expanded.append(node)
        for value in (0.0, 1.0):
            fixed = dict(node["fixed"])
            fixed[branch] = value
            child = solve_node(edges, triads, weights, cuts, extra_rows, fixed)
            child.update({"fixed": fixed, "depth": node["depth"] + 1, "branch_vertex": None})
            counter += 1
            if child["feasible"]:
                heapq.heappush(heap, (-child["upper"], counter, child))
            else:
                leaves.append(child)
    open_nodes = [item[2] for item in heap]
    all_leaves = leaves + open_nodes
    finite = [leaf for leaf in all_leaves if leaf["feasible"]]
    return {
        "expanded_count": len(expanded),
        "closed_leaf_count": len(leaves),
        "open_leaf_count": len(open_nodes),
        "finite_leaf_count": len(finite),
        "worst_leaf_upper": max((leaf["upper"] for leaf in finite), default=float("-inf")),
        "best_leaf_upper": min((leaf["upper"] for leaf in finite), default=None),
        "max_depth": max((leaf["depth"] for leaf in all_leaves), default=0),
        "expanded_vertices": [node["branch_vertex"] + 1 for node in expanded],
        "worst_leaves": [summarize_leaf(leaf) for leaf in sorted(finite, key=lambda item: -item["upper"])[:10]],
        "open_leaves": [summarize_leaf(leaf) for leaf in sorted(open_nodes, key=lambda item: -item["upper"])[:10]],
    }


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items()}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    start = time.time()
    leaf_artifact = json.loads(LEAF_ARTIFACT.read_text())
    expected_row = next(row for row in leaf_artifact["top_rows"] if row["name"] == ROW_NAME)
    edges, weights_int = parent.parse_edges_weights()
    weights = weights_int.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    extra_rows = [parent_lift.parent_row(weights), p_parent_row(weights_int)]
    regenerated_leaf_obj, row = reconstruct_row(edges, weights, adj, triads, root_cuts, extra_rows)
    trial_cut = (row["vertices"], row["alpha"])
    fixed = {vertex: 0.0 for vertex in TIER_A}
    old_root = leaf_rank.solve_lp(edges, triads, weights, root_cuts, extra_rows)
    new_root = leaf_rank.solve_lp(edges, triads, weights, root_cuts + [trial_cut], extra_rows)
    old_leaf = leaf_rank.solve_lp(edges, triads, weights, root_cuts, extra_rows, fixed)
    new_leaf = leaf_rank.solve_lp(edges, triads, weights, root_cuts + [trial_cut], extra_rows, fixed)
    old_tree = run_tree(edges, triads, weights, root_cuts, extra_rows)
    new_tree = run_tree(edges, triads, weights, root_cuts + [trial_cut], extra_rows)
    root_drop = old_root - new_root
    max_leaf_improvement = old_tree["worst_leaf_upper"] - new_tree["worst_leaf_upper"]
    support_ok = (
        len(row["vertices"]) == int(expected_row["size"])
        and row["alpha"] == int(expected_row["alpha_w"])
        and abs(row["leaf_lhs"] - float(expected_row["leaf_lhs"])) <= 1e-6
        and abs(new_leaf - EXPECTED_LEAF_WITH_ROW) <= 1e-5
    )
    status = "RetireConditionalRankRowPropagation"
    if support_ok and (
        root_drop >= ROOT_DROP_GATE
        or new_tree["worst_leaf_upper"] <= WORST_GATE
        or max_leaf_improvement >= MAX_LEAF_IMPROVEMENT_GATE
    ):
        status = "FundConditionalRankRowPropagation"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_conditional_rank_row_propagation.v1",
            "second_agent_verdict": "bookkeeping_closure_only_add_exactly_one_regenerated_row",
            "row": {
                "name": row["name"],
                "alpha_w": row["alpha"],
                "size": len(row["vertices"]),
                "support_digest": row["digest"],
                "support_vertices": [v + 1 for v in row["vertices"]],
                "leaf_lhs": row["leaf_lhs"],
                "leaf_violation": row["leaf_violation"],
            },
            "support_regeneration_ok": support_ok,
            "old_root_objective": old_root,
            "new_root_objective": new_root,
            "root_drop": root_drop,
            "old_all_excluded_leaf": old_leaf,
            "new_all_excluded_leaf": new_leaf,
            "all_excluded_leaf_drop": old_leaf - new_leaf,
            "old_tree_report": old_tree,
            "new_tree_report": new_tree,
            "max_leaf_improvement": max_leaf_improvement,
            "baseline_checks": {
                "old_root_matches": abs(old_root - EXPECTED_ROOT) <= 1e-5,
                "old_tree_worst_matches": abs(old_tree["worst_leaf_upper"] - EXPECTED_TIER_A_WORST) <= 1e-5,
                "regenerated_leaf_matches": abs(regenerated_leaf_obj - EXPECTED_TIER_A_WORST) <= 1e-5,
            },
            "status": status,
            "gates": {
                "root_drop_gate": ROOT_DROP_GATE,
                "worst_gate": WORST_GATE,
                "max_leaf_improvement_gate": MAX_LEAF_IMPROVEMENT_GATE,
                "kill_root_drop": KILL_ROOT_DROP,
                "kill_worst_gate": KILL_WORST_GATE,
                "kill_max_leaf_improvement": KILL_MAX_LEAF_IMPROVEMENT,
                "node_cap": NODE_CAP,
                "max_depth": MAX_DEPTH,
            },
            "seconds": time.time() - start,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in {"row", "old_tree_report", "new_tree_report"}}, indent=2))


if __name__ == "__main__":
    main()
