import heapq
import json
import time

import numpy as np

import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-branch-slack-plateau-branch-tree.json"

TIER_A = [304, 223, 384, 302, 222, 383]
FIRST_NODE_CAP = 64
HARD_NODE_CAP = 128
MAX_DEPTH_TIER_A = 6
MAX_DEPTH_WITH_TIER_B = 7
CONTINUE_GATE = 590000.0
STALL_GATE = 592000.0
EXPORT_GATE = 586000.0
REPLAY_DROP_GATE = 8000.0
MAYBE_DROP_GATE = 5000.0
NINETY_PERCENT_GATE = 584000.0


def p_parent_row(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(c0[v]) * parent.DENOMINATOR for v in range(parent.N) if c0[v]}
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        index = int(vertex) - 1
        coeffs[index] = coeffs.get(index, 0) + int(coeff)
    coeffs[parent.BRANCH] = int(artifact["lift_coefficient_num_d1024"])
    return coeffs, int(artifact["gamma0_modified_num_d1024"])


def solve_node(edges, triads, weights, cuts, extra_rows, fixed):
    try:
        objective, x = parent_lift.solve_lp(
            edges,
            triads,
            weights.astype(float),
            cuts,
            extra_rows,
            fixed=fixed,
            solution=True,
        )
        return {"feasible": True, "upper": objective, "x": x}
    except ValueError as err:
        return {"feasible": False, "upper": float("-inf"), "error": str(err), "x": None}


def tier_b_vertices(weights, x):
    excluded = set(TIER_A + [parent.BRANCH])
    candidates = [
        v
        for v in range(parent.N)
        if v not in excluded and 0.31 <= x[v] <= 0.35 and weights[v] >= 10000
    ]
    return sorted(candidates, key=lambda v: (-weights[v], -weights[v] * x[v], v))[:6]


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


def run_tree(edges, triads, weights, cuts, extra_rows, allowed, node_cap, max_depth):
    counter = 0
    root = solve_node(edges, triads, weights, cuts, extra_rows, {})
    root.update({"fixed": {}, "depth": 0, "branch_vertex": None})
    heap = [(-root["upper"], counter, root)]
    expanded = []
    leaves = []
    while heap and len(expanded) < node_cap:
        _priority, _order, node = heapq.heappop(heap)
        if not node["feasible"]:
            leaves.append(node)
            continue
        branch = (
            None
            if node["depth"] >= max_depth
            else choose_branch(node["x"], weights, allowed, node["fixed"])
        )
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
    worst = max((leaf["upper"] for leaf in finite), default=float("-inf"))
    below_584 = sum(1 for leaf in finite if leaf["upper"] <= NINETY_PERCENT_GATE)
    return {
        "expanded_count": len(expanded),
        "closed_leaf_count": len(leaves),
        "open_leaf_count": len(open_nodes),
        "finite_leaf_count": len(finite),
        "worst_leaf_upper": worst,
        "best_leaf_upper": min((leaf["upper"] for leaf in finite), default=None),
        "max_depth": max((leaf["depth"] for leaf in all_leaves), default=0),
        "ninety_percent_below_gate": below_584 / len(finite) if finite else 0.0,
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
    edges, weights = parent.parse_edges_weights()
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = parent_lift.root_cuts(weights.astype(float), adj)
    rows = [parent_lift.parent_row(weights.astype(float)), p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(
        edges, triads, weights.astype(float), cuts, rows, solution=True
    )
    tier_b = tier_b_vertices(weights, root_x)
    tier_a = list(TIER_A)
    tier_a_report = run_tree(
        edges, triads, weights, cuts, rows, tier_a, FIRST_NODE_CAP, MAX_DEPTH_TIER_A
    )
    continue_to_b = (
        tier_a_report["worst_leaf_upper"] <= CONTINUE_GATE
        and tier_a_report["expanded_count"] >= FIRST_NODE_CAP
    )
    tier_ab_report = None
    if continue_to_b:
        tier_ab_report = run_tree(
            edges,
            triads,
            weights,
            cuts,
            rows,
            tier_a + tier_b,
            HARD_NODE_CAP,
            MAX_DEPTH_WITH_TIER_B,
        )
    final_report = tier_ab_report or tier_a_report
    improvement = root_obj - final_report["worst_leaf_upper"]
    status = "RetirePlateauBranchTree"
    if (
        final_report["worst_leaf_upper"] <= EXPORT_GATE
        and improvement >= REPLAY_DROP_GATE
        and final_report["finite_leaf_count"] <= 64
        and final_report["ninety_percent_below_gate"] >= 0.90
    ):
        status = "FundPlateauLeafDualExport"
    elif final_report["worst_leaf_upper"] <= CONTINUE_GATE and improvement >= MAYBE_DROP_GATE:
        status = "InterestingPlateauBranchTree"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_branch_slack_plateau_branch_tree.v1",
            "root_objective": root_obj,
            "tier_a_vertices": [v + 1 for v in tier_a],
            "tier_b_vertices": [v + 1 for v in tier_b],
            "tier_a_report": tier_a_report,
            "tier_ab_report": tier_ab_report,
            "final_worst_leaf_upper": final_report["worst_leaf_upper"],
            "final_improvement": improvement,
            "status": status,
            "gates": {
                "first_node_cap": FIRST_NODE_CAP,
                "hard_node_cap": HARD_NODE_CAP,
                "continue_gate": CONTINUE_GATE,
                "stall_gate": STALL_GATE,
                "export_gate": EXPORT_GATE,
                "replay_drop_gate": REPLAY_DROP_GATE,
                "maybe_drop_gate": MAYBE_DROP_GATE,
                "ninety_percent_gate": NINETY_PERCENT_GATE,
            },
            "seconds": time.time() - start,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in {"tier_a_report", "tier_ab_report"}}, indent=2))


if __name__ == "__main__":
    main()
