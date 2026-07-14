import heapq
import hashlib
import json

import numpy as np

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_gamma0_branch_tree_preflight as gamma0_tree
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
OLD_TREE = CRATE / "docs" / "w607-gamma0-branch-tree-preflight.json"
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-tailored-gamma0-branch-tree.json"

DEN = 1024.0
NODE_LIMIT = 96
BRANCH_EPS = 1e-7
CHEAP_GAIN_NUM = 256_000.0
EXPORT_GAIN_NUM = 1_024_000.0
STRONG_GAIN_NUM = 3_072_000.0
CHEAP_ROOT_DROP = 250.0
EXPORT_ROOT_DROP = 1000.0
STRONG_ROOT_DROP = 3000.0
GAMMA1_MODIFIED_NUM = 559_085_319_025.0


def digest_ints(values):
    return hashlib.sha256(",".join(str(int(v)) for v in values).encode()).hexdigest()


def modified_vector(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    d_num = np.array([int(v) * int(DEN) for v in c0], dtype=float)
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        d_num[int(vertex) - 1] += int(coeff)
    return d_num, artifact


def solve_node(edges, triads, weights, d, cuts, adj, included, excluded):
    return gamma0_tree.solve_node_lp(edges, triads, weights, d, cuts, adj, included, excluded)


def branch_vertex(node, d):
    fractional = [
        v
        for v in node["active"]
        if BRANCH_EPS < node["x"][v] < 1.0 - BRANCH_EPS and d[v] > 0
    ]
    if not fractional:
        return None
    return max(fractional, key=lambda v: (float(d[v]) * node["x"][v], float(d[v]), -v))


def old_leaf_precheck(tree_artifact, edges, triads, weights, d, cuts, adj):
    rows = []
    for leaf in tree_artifact["closed_leaves"]:
        included = tuple(v - 1 for v in leaf["included"])
        excluded = tuple(v - 1 for v in leaf["excluded"])
        node = solve_node(edges, triads, weights, d, cuts, adj, included, excluded)
        rows.append(
            {
                "included": leaf["included"],
                "excluded": leaf["excluded"],
                "upper": node["upper"],
                "included_weight": node["included_weight"],
                "residual_upper": node["residual_upper"],
                "active_vertices": len(node["active"]),
            }
        )
    worst = max(rows, key=lambda row: row["upper"])
    return rows, worst


def flat_root_compare(edges, triads, weights, d, cuts, adj):
    c0, _ = parent.exclude_coverage(weights)
    c0 = np.array([float(v) for v in c0])
    c0_root = solve_node(edges, triads, weights, c0, cuts, adj, (), ())
    d_root = solve_node(edges, triads, weights, d, cuts, adj, (), ())
    return {
        "c0_root_upper": c0_root["upper"],
        "d_root_upper": d_root["upper"],
        "c0_top_fractional": top_fractional(weights, c0, c0_root["x"]),
        "d_top_fractional": top_fractional(weights, d, d_root["x"]),
    }


def top_fractional(weights, objective, x, limit=12):
    fractional = [v for v in range(parent.N) if BRANCH_EPS < x[v] < 1.0 - BRANCH_EPS]
    chosen = sorted(fractional, key=lambda v: (-objective[v] * x[v], -objective[v], v))[:limit]
    return [
        {
            "vertex": v + 1,
            "x": x[v],
            "weight": weights[v],
            "objective_coeff": objective[v],
            "objective_contribution": objective[v] * x[v],
        }
        for v in chosen
    ]


def run_tree(edges, triads, weights, d, cuts, adj, close_target):
    root = solve_node(edges, triads, weights, d, cuts, adj, (), ())
    heap = []
    counter = 0
    heapq.heappush(heap, (-root["upper"], counter, ((), (), root)))
    expanded = []
    closed = []
    terminal = []
    max_depth = 0
    while heap and len(expanded) < NODE_LIMIT:
        _, _, (included, excluded, node) = heapq.heappop(heap)
        depth = len(included) + len(excluded)
        max_depth = max(max_depth, depth)
        if node["upper"] <= close_target + 1e-6:
            closed.append((included, excluded, node, "close_target"))
            continue
        vertex = branch_vertex(node, d)
        if vertex is None:
            terminal.append((included, excluded, node))
            continue
        expanded.append(
            {
                "depth": depth,
                "branch_vertex": vertex + 1,
                "upper": node["upper"],
                "included_weight": node["included_weight"],
                "residual_upper": node["residual_upper"],
                "active_vertices": len(node["active"]),
                "branch_x": node["x"][vertex],
                "branch_d": d[vertex],
                "branch_contribution": d[vertex] * node["x"][vertex],
            }
        )
        for value in (1, 0):
            child_included = tuple(sorted((*included, vertex))) if value else included
            child_excluded = tuple(sorted((*excluded, vertex))) if not value else excluded
            child = solve_node(edges, triads, weights, d, cuts, adj, child_included, child_excluded)
            if child is None:
                continue
            counter += 1
            heapq.heappush(heap, (-child["upper"], counter, (child_included, child_excluded, child)))
    open_nodes = [(included, excluded, node) for _, _, (included, excluded, node) in heap]
    all_remaining = open_nodes + terminal
    worst_open = max(all_remaining, key=lambda item: item[2]["upper"], default=None)
    worst_closed = max(closed, key=lambda item: item[2]["upper"], default=None)
    max_open_upper = max((node["upper"] for _, _, node in all_remaining), default=0.0)
    max_closed_upper = max((node["upper"] for _, _, node, _ in closed), default=0.0)
    diagnostic_upper = max(max_open_upper, max_closed_upper)
    return {
        "root_upper": root["upper"],
        "close_target": close_target,
        "expanded_node_count": len(expanded),
        "closed_leaf_count": len(closed),
        "open_leaf_count": len(all_remaining),
        "max_depth": max_depth,
        "max_open_upper": max_open_upper,
        "max_closed_upper": max_closed_upper,
        "diagnostic_upper": diagnostic_upper,
        "worst_open_leaf": leaf_summary(worst_open),
        "worst_closed_leaf": leaf_summary(worst_closed[:3] if worst_closed else None),
        "closed_leaves": [
            {**leaf_summary((included, excluded, node)), "closed_by": reason}
            for included, excluded, node, reason in sorted(closed, key=lambda item: item[2]["upper"], reverse=True)
        ],
        "expanded_nodes": expanded,
    }


def parent_row_from_gamma(d_num, gamma0_num):
    lift_num = gamma0_num - GAMMA1_MODIFIED_NUM
    coeffs = {v: float(d_num[v]) for v in range(parent.N) if v != parent.BRANCH and abs(d_num[v]) > 1e-9}
    coeffs[parent.BRANCH] = float(lift_num)
    return coeffs, float(gamma0_num)


def root_effect(edges, triads, weights, cuts, old_branch_row, new_row):
    projected = parent_lift.parent_row(weights)
    baseline_obj, baseline_x = branch_slack.solve_lp(
        edges, triads, weights, cuts, [projected, old_branch_row], solution=True
    )
    new_obj, new_x = branch_slack.solve_lp(
        edges, triads, weights, cuts, [projected, old_branch_row, new_row], solution=True
    )
    return {
        "baseline_objective": baseline_obj,
        "new_objective": new_obj,
        "root_drop": baseline_obj - new_obj,
        "baseline_x304": baseline_x[parent.BRANCH],
        "new_x304": new_x[parent.BRANCH],
    }


def leaf_summary(leaf):
    if leaf is None:
        return None
    included, excluded, node = leaf
    return {
        "included": [v + 1 for v in included],
        "excluded": [v + 1 for v in excluded],
        "upper": node["upper"],
        "included_weight": node["included_weight"],
        "residual_upper": node["residual_upper"],
        "active_vertices": len(node["active"]),
    }


def status_for(gain_num, root_drop, tree):
    if gain_num >= STRONG_GAIN_NUM and root_drop >= STRONG_ROOT_DROP:
        return "StrongTailoredGamma0Tree"
    if gain_num >= EXPORT_GAIN_NUM and root_drop >= EXPORT_ROOT_DROP and tree["open_leaf_count"] == 0:
        return "FundTailoredGamma0LeafExport"
    if gain_num >= CHEAP_GAIN_NUM and root_drop >= CHEAP_ROOT_DROP:
        return "CheapTailoredGamma0Success"
    return "RetireTailoredGamma0Tree"


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
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = gamma0_tree.triangles(adj)
    cuts = gamma0_tree.rank_cuts(weights, adj)
    d_num, artifact = modified_vector(weights)
    d = d_num / DEN
    charged_num = float(artifact["gamma0_modified_num_d1024"])
    charged = charged_num / DEN
    close_target = (charged_num - CHEAP_GAIN_NUM) / DEN
    tree_artifact = json.loads(OLD_TREE.read_text())
    old_leaf_rows, old_leaf_worst = old_leaf_precheck(tree_artifact, edges, triads, weights, d, cuts, adj)
    flat = flat_root_compare(edges, triads, weights, d, cuts, adj)
    tree = run_tree(edges, triads, weights, d, cuts, adj, close_target)
    diagnostic_gamma0 = tree["diagnostic_upper"]
    diagnostic_gamma0_num = diagnostic_gamma0 * DEN
    gain_num = charged_num - diagnostic_gamma0_num
    old_branch_row = branch_slack.p_parent_row(weights)
    new_row = parent_row_from_gamma(d_num, diagnostic_gamma0_num)
    effect = root_effect(edges, triads, weights, parent_lift.root_cuts(weights, adj), old_branch_row, new_row)
    report = clean(
        {
            "schema": "forge.hadwiger.w607_tailored_gamma0_branch_tree.v1",
            "authority": "diagnostic_only_not_proof_authority_until_rounded_leaf_duals_replay",
            "second_agent_recommendation": "run_bounded_diagnostic_not_export_yet",
            "branch_domain": "x304=0",
            "row_language": "edge_triangle_replayed_weighted_rank_rows",
            "node_limit": NODE_LIMIT,
            "vector": {
                "name": "d=c0+p",
                "denominator": int(DEN),
                "d_num_digest": digest_ints(d_num),
                "p_support": sorted(int(v) for v in artifact["positive_coefficients_num_d1024"]),
                "p_num_d1024": artifact["positive_coefficients_num_d1024"],
            },
            "bounds": {
                "charged_gamma0_num_d1024": charged_num,
                "charged_gamma0": charged,
                "gamma1_modified_num_d1024": GAMMA1_MODIFIED_NUM,
                "cheap_close_target": close_target,
                "diagnostic_gamma0": diagnostic_gamma0,
                "diagnostic_gamma0_num_d1024": diagnostic_gamma0_num,
                "gain_num_d1024": gain_num,
                "gain_objective": gain_num / DEN,
                "new_lift_num_d1024": diagnostic_gamma0_num - GAMMA1_MODIFIED_NUM,
            },
            "gates": {
                "cheap_gain_num": CHEAP_GAIN_NUM,
                "export_gain_num": EXPORT_GAIN_NUM,
                "strong_gain_num": STRONG_GAIN_NUM,
                "cheap_root_drop": CHEAP_ROOT_DROP,
                "export_root_drop": EXPORT_ROOT_DROP,
                "strong_root_drop": STRONG_ROOT_DROP,
            },
            "prechecks": {
                "old_leaf_worst": old_leaf_worst,
                "old_leaf_worst_gain_num": charged_num - old_leaf_worst["upper"] * DEN,
                "flat_root_compare": flat,
            },
            "tree": tree,
            "root_effect": effect,
            "status": status_for(gain_num, effect["root_drop"], tree),
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in {"tree"}}, indent=2))


if __name__ == "__main__":
    main()
