import heapq
import json

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
PARENT_LIFT = CRATE / "docs" / "w607-v304-projected-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-gamma0-branch-tree-preflight.json"

N = parent.N
BRANCH = parent.BRANCH
TARGET = 613372392.0
NEAR_TARGET = TARGET * 1.005
KILL_TARGET = TARGET * 1.01
NODE_LIMIT = 64
BRANCH_EPS = 1e-7


def rank_cuts(weights, adj):
    cuts = []
    for name, alpha in parent.ACCEPTED:
        vertices = parent.pocket(name, weights, adj)
        cuts.append((vertices, sum(float(weights[v]) for v in vertices), float(alpha)))
    return cuts


def triangles(adj):
    out = []
    for a in range(N):
        for b in adj[a]:
            if b <= a:
                continue
            for c in adj[a] & adj[b]:
                if c > b:
                    out.append((a, b, c))
    return out


def residual_vertices(adj, included, excluded):
    blocked = {BRANCH, *excluded}
    for vertex in included:
        blocked.add(vertex)
        blocked.update(adj[vertex])
    return [v for v in range(N) if v not in blocked]


def solve_node_lp(edges, triads, weights, c0, cuts, adj, included, excluded):
    active = residual_vertices(adj, included, excluded)
    active_set = set(active)
    index = {v: i for i, v in enumerate(active)}
    rows = []
    rhs = []
    for a, b in edges:
        if a in active_set and b in active_set:
            rows.append((a, b))
            rhs.append(1.0)
    for a, b, c in triads:
        if a in active_set and b in active_set and c in active_set:
            rows.append((a, b, c))
            rhs.append(1.0)
    included_set = set(included)
    for vertices, support_weight, alpha in cuts:
        used = sum(float(weights[v]) for v in vertices if v in included_set)
        local = tuple(v for v in vertices if v in active_set)
        if not local:
            if used > alpha + 1e-7:
                return None
            continue
        local_rhs = alpha - used
        if local_rhs < -1e-7:
            return None
        rows.append(local)
        rhs.append(local_rhs)
    matrix = lil_matrix((len(rows), len(active)), dtype=float)
    for row, vertices in enumerate(rows):
        for vertex in vertices:
            matrix[row, index[vertex]] = float(weights[vertex]) if len(vertices) > 3 else 1.0
    result = linprog(
        c=-np.array([float(c0[v]) for v in active]),
        A_ub=matrix.tocsr(),
        b_ub=np.array(rhs, dtype=float),
        bounds=[(0, 1)] * len(active),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    x = np.zeros(N)
    for vertex, value in zip(active, result.x):
        x[vertex] = value
    included_weight = float(sum(c0[v] for v in included))
    return {
        "active": active,
        "included_weight": included_weight,
        "residual_upper": -float(result.fun),
        "upper": included_weight - float(result.fun),
        "x": x,
        "row_count": len(rows),
    }


def branch_vertex(node, c0):
    fractional = [
        v
        for v in node["active"]
        if BRANCH_EPS < node["x"][v] < 1.0 - BRANCH_EPS and c0[v] > 0
    ]
    if not fractional:
        return None
    return max(fractional, key=lambda v: (float(c0[v]) * node["x"][v], float(c0[v]), -v))


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "x"}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, tuple):
        return [clean(inner) for inner in value]
    if isinstance(value, set):
        return sorted(clean(inner) for inner in value)
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = triangles(adj)
    c0, _ = parent.exclude_coverage(weights)
    c0 = np.array([float(v) for v in c0])
    lift = json.loads(PARENT_LIFT.read_text())
    if int(lift["gamma0_upper_numerator"]) != int(TARGET):
        raise ValueError("gamma0 target mismatch")
    cuts = rank_cuts(weights, adj)
    root = solve_node_lp(edges, triads, weights, c0, cuts, adj, (), ())
    heap = []
    counter = 0
    root_item = ((), (), root)
    heapq.heappush(heap, (-root["upper"], counter, root_item))
    expanded = []
    closed_leaves = []
    terminal_leaves = []
    max_depth = 0
    while heap and len(expanded) < NODE_LIMIT:
        _, _, (included, excluded, node) = heapq.heappop(heap)
        depth = len(included) + len(excluded)
        max_depth = max(max_depth, depth)
        if node["upper"] <= TARGET + 1e-6:
            closed_leaves.append((included, excluded, node, "target"))
            continue
        vertex = branch_vertex(node, c0)
        if vertex is None:
            terminal_leaves.append((included, excluded, node))
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
                "branch_c0": c0[vertex],
                "branch_contribution": c0[vertex] * node["x"][vertex],
            }
        )
        for value in (1, 0):
            child_included = tuple(sorted((*included, vertex))) if value else included
            child_excluded = tuple(sorted((*excluded, vertex))) if not value else excluded
            child = solve_node_lp(edges, triads, weights, c0, cuts, adj, child_included, child_excluded)
            if child is None:
                continue
            counter += 1
            heapq.heappush(heap, (-child["upper"], counter, (child_included, child_excluded, child)))
    open_nodes = [(included, excluded, node) for _, _, (included, excluded, node) in heap]
    all_open = open_nodes + terminal_leaves
    max_open_upper = max((node["upper"] for _, _, node in all_open), default=0.0)
    best_open_leaf = max(all_open, key=lambda item: item[2]["upper"], default=None)
    worst_closed_leaf = max(closed_leaves, key=lambda item: item[2]["upper"], default=None)
    status = "RetireGamma0BranchTreePreflight"
    if not all_open and all(node["upper"] <= TARGET + 1e-6 for _, _, node, _ in closed_leaves):
        status = "CloseGamma0BranchTree"
    elif max_open_upper <= NEAR_TARGET:
        status = "NearCloseGamma0BranchTree"
    elif max_open_upper <= KILL_TARGET:
        status = "MarginalGamma0BranchTree"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_gamma0_branch_tree_preflight.v1",
            "branch_vertex": BRANCH + 1,
            "target_gamma0": TARGET,
            "near_close_threshold": NEAR_TARGET,
            "kill_threshold": KILL_TARGET,
            "node_limit": NODE_LIMIT,
            "root_upper": root["upper"],
            "root_relative_gap": (root["upper"] - TARGET) / TARGET,
            "expanded_node_count": len(expanded),
            "closed_leaf_count": len(closed_leaves),
            "open_leaf_count": len(all_open),
            "max_depth": max_depth,
            "max_open_upper": max_open_upper,
            "max_open_relative_gap": (max_open_upper - TARGET) / TARGET if max_open_upper else None,
            "best_open_leaf": leaf_summary(best_open_leaf),
            "worst_closed_leaf": leaf_summary(worst_closed_leaf[:3] if worst_closed_leaf else None),
            "closed_leaves": [
                {**leaf_summary((included, excluded, node)), "closed_by": reason}
                for included, excluded, node, reason in sorted(
                    closed_leaves, key=lambda item: item[2]["upper"], reverse=True
                )
            ],
            "expanded_nodes": expanded,
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "expanded_nodes"}, indent=2))


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


if __name__ == "__main__":
    main()
