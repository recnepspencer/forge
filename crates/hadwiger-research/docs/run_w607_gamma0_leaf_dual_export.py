import json
import math

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_gamma0_branch_tree_preflight as tree
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
TREE = CRATE / "docs" / "w607-gamma0-branch-tree-preflight.json"
EXCLUDE_CERT = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"
OUT_PATH = CRATE / "docs" / "w607-gamma0-leaf-dual-export.json"

N = parent.N
BRANCH = parent.BRANCH
TARGET = 613372392
DENOMINATORS = [1024, 4096, 16384]
ROUND_EPS = 1e-8


def graph_digest(edges, weights):
    import hashlib

    payload = {
        "edges": [[a + 1, b + 1] for a, b in edges],
        "weights": [int(w) for w in weights],
    }
    return hashlib.sha256(json.dumps(payload, separators=(",", ":")).encode()).hexdigest()


def vector_digest(values):
    import hashlib

    return hashlib.sha256(",".join(str(int(v)) for v in values).encode()).hexdigest()


def rank_rows(weights, adj):
    rows = []
    for name, alpha in parent.ACCEPTED:
        support = parent.pocket(name, weights, adj)
        rows.append({"kind": "rank", "name": name, "vertices": support, "alpha": int(alpha)})
    return rows


def all_rows(edges, triads, weights, adj, included, active):
    active_set = set(active)
    included_set = set(included)
    rows = []
    for a, b in edges:
        if a in active_set and b in active_set:
            rows.append({"kind": "edge", "vertices": (a, b), "rhs": 1, "coeff": 1})
    for a, b, c in triads:
        if a in active_set and b in active_set and c in active_set:
            rows.append({"kind": "triangle", "vertices": (a, b, c), "rhs": 1, "coeff": 1})
    for row in rank_rows(weights, adj):
        used = sum(int(weights[v]) for v in row["vertices"] if v in included_set)
        local = tuple(v for v in row["vertices"] if v in active_set)
        if not local:
            continue
        rhs = row["alpha"] - used
        if rhs < 0:
            raise ValueError(f"negative rank rhs for {row['name']}")
        rows.append(
            {
                "kind": "rank",
                "name": row["name"],
                "vertices": local,
                "rhs": rhs,
                "coeff": "w",
                "full_support_size": len(row["vertices"]),
            }
        )
    return rows


def solve_cover(active, rows, weights, c0):
    index = {v: i for i, v in enumerate(active)}
    matrix = lil_matrix((len(active), len(rows)), dtype=float)
    rhs = []
    for col, row in enumerate(rows):
        rhs.append(float(row["rhs"]))
        if row["coeff"] == 1:
            for vertex in row["vertices"]:
                matrix[index[vertex], col] = 1.0
        else:
            for vertex in row["vertices"]:
                matrix[index[vertex], col] = float(weights[vertex])
    result = linprog(
        c=np.array(rhs, dtype=float),
        A_ub=-matrix.tocsr(),
        b_ub=-np.array([float(c0[v]) for v in active]),
        bounds=[(0, None)] * len(rows),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return result.x, float(result.fun)


def rounded_certificate(active, rows, y, denominator, weights, c0, included_c0):
    nums = [max(0, int(math.ceil(value * denominator - ROUND_EPS))) for value in y]
    coverage = {v: 0 for v in active}
    objective = int(included_c0) * denominator
    positive_rows = []
    for row, numerator in zip(rows, nums):
        if numerator == 0:
            continue
        objective += numerator * int(row["rhs"])
        if row["coeff"] == 1:
            for vertex in row["vertices"]:
                if vertex in coverage:
                    coverage[vertex] += numerator
        else:
            for vertex in row["vertices"]:
                if vertex in coverage:
                    coverage[vertex] += numerator * int(weights[vertex])
        exported = {
            "kind": row["kind"],
            "vertices": [v + 1 for v in row["vertices"]],
            "rhs": int(row["rhs"]),
            "numerator": numerator,
        }
        if row["kind"] == "rank":
            exported["name"] = row["name"]
            exported["full_support_size"] = row["full_support_size"]
        positive_rows.append(exported)
    min_slack = min((coverage[v] - int(c0[v]) * denominator for v in active), default=0)
    return {
        "objective_num": objective,
        "objective_bound": objective / denominator,
        "target_num": TARGET * denominator,
        "min_slack": min_slack,
        "positive_row_count": len(positive_rows),
        "rows": positive_rows,
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
    edges, weights = parent.parse_edges_weights()
    adj = parent.adjacency(edges)
    triads = tree.triangles(adj)
    c0, _ = parent.exclude_coverage(weights)
    c0 = np.array([int(v) for v in c0], dtype=object)
    tree_artifact = json.loads(TREE.read_text())
    leaves = tree_artifact["closed_leaves"]
    leaf_reports = []
    for leaf_index, leaf in enumerate(leaves):
        included = tuple(v - 1 for v in leaf["included"])
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = tree.residual_vertices(adj, included, excluded)
        included_c0 = sum(int(c0[v]) for v in included)
        rows = all_rows(edges, triads, weights, adj, included, active)
        y, float_obj = solve_cover(active, rows, weights, c0)
        attempts = []
        success = None
        for denominator in DENOMINATORS:
            cert = rounded_certificate(active, rows, y, denominator, weights, c0, included_c0)
            passed = cert["objective_num"] <= cert["target_num"] and cert["min_slack"] >= 0
            attempts.append(
                {
                    "denominator": denominator,
                    "objective_num": cert["objective_num"],
                    "objective_bound": cert["objective_bound"],
                    "target_num": cert["target_num"],
                    "min_slack": cert["min_slack"],
                    "positive_row_count": cert["positive_row_count"],
                    "passes": passed,
                }
            )
            if passed:
                success = {**cert, "denominator": denominator}
                break
        leaf_reports.append(
            {
                "leaf_index": leaf_index,
                "included": leaf["included"],
                "excluded": leaf["excluded"],
                "active_vertices": len(active),
                "included_c0_weight": included_c0,
                "floating_residual_cover_objective": float_obj,
                "floating_total_objective": included_c0 + float_obj,
                "attempts": attempts,
                "success": success,
            }
        )
    successes = [leaf["success"] for leaf in leaf_reports if leaf["success"] is not None]
    status = "FundGamma0LeafDualReplay" if len(successes) == len(leaf_reports) else "RetireGamma0LeafDualRounding"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_gamma0_leaf_dual_export.v1",
            "graph_vertex_count": N,
            "graph_edge_count": len(edges),
            "graph_weight_sum": int(sum(weights)),
            "graph_digest": graph_digest(edges, weights),
            "c0_source_artifact": str(EXCLUDE_CERT.relative_to(CRATE)),
            "c0_vector_digest": vector_digest(c0),
            "branch_domain": "x304=0",
            "target_gamma0": TARGET,
            "denominators": DENOMINATORS,
            "leaf_count": len(leaf_reports),
            "successful_leaf_count": len(successes),
            "worst_success_objective_bound": max((leaf["objective_bound"] for leaf in successes), default=None),
            "max_success_denominator": max((leaf["denominator"] for leaf in successes), default=None),
            "status": status,
            "leaves": leaf_reports,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
