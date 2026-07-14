import json
from pathlib import Path

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
PARENT_LIFT = CRATE / "docs" / "w607-v304-projected-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-post-parent-lift-branch-prescreen.json"

N = parent.N
BRANCH = parent.BRANCH
CANDIDATE_COUNT = 8
RAW_MAX_GATE = 590000.0
STRONG_RAW_MAX_GATE = 585000.0
BALANCE_REJECT_GATE = 595000.0
SPLIT_MOVEMENT_GATE = 5000.0


def root_cuts(weights, adj):
    return [(parent.pocket(name, weights, adj), alpha) for name, alpha in parent.ACCEPTED]


def parent_row(weights):
    lift = json.loads(PARENT_LIFT.read_text())
    coverage, _ = parent.exclude_coverage(weights)
    coeffs = {v: float(coverage[v]) for v in range(N) if v != BRANCH and coverage[v]}
    coeffs[BRANCH] = float(lift["new_lift_coefficient"])
    return coeffs, float(lift["new_rhs_numerator"])


def solve_lp(edges, triads, weights, cuts, extra_rows, fixed=None, solution=False):
    fixed = fixed or {}
    row_count = len(edges) + len(triads) + len(cuts) + len(extra_rows)
    matrix = lil_matrix((row_count, N), dtype=float)
    upper = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1.0
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1.0
        row += 1
    for vertices, alpha in cuts:
        for vertex in vertices:
            matrix[row, vertex] = float(weights[vertex])
        upper[row] = float(alpha)
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = float(coeff)
        upper[row] = float(rhs)
        row += 1
    bounds = [(0.0, 1.0)] * N
    for vertex, value in fixed.items():
        bounds[vertex] = (float(value), float(value))
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix.tocsr(),
        b_ub=upper,
        bounds=bounds,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


def candidate_vertices(weights, x):
    fractional = [v for v in range(N) if 1e-7 < x[v] < 1.0 - 1e-7]
    return sorted(fractional, key=lambda v: (-weights[v] * x[v], -weights[v], v))[:CANDIDATE_COUNT]


def split_row(weights, vertex, exclude_obj, include_obj):
    coeffs = {v: float(weights[v]) for v in range(N)}
    coeffs[vertex] += exclude_obj - include_obj
    return coeffs, exclude_obj


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
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = root_cuts(weights, adj)
    lift_row = parent_row(weights)
    base_obj, base_x = solve_lp(edges, triads, weights, cuts, [lift_row], solution=True)
    rows = []
    for vertex in candidate_vertices(weights, base_x):
        exclude_obj = solve_lp(edges, triads, weights, cuts, [lift_row], fixed={vertex: 0})
        include_obj = solve_lp(edges, triads, weights, cuts, [lift_row], fixed={vertex: 1})
        max_child = max(exclude_obj, include_obj)
        split = split_row(weights, vertex, exclude_obj, include_obj)
        split_obj, split_x = solve_lp(edges, triads, weights, cuts, [lift_row, split], solution=True)
        movement = base_obj - split_obj
        rows.append(
            {
                "vertex": vertex + 1,
                "weight": weights[vertex],
                "base_x": base_x[vertex],
                "base_weighted_x": weights[vertex] * base_x[vertex],
                "exclude_objective": exclude_obj,
                "include_objective": include_obj,
                "max_child_objective": max_child,
                "child_gap": abs(exclude_obj - include_obj),
                "split_objective": split_obj,
                "split_movement": movement,
                "split_x_vertex": split_x[vertex],
                "passes_raw_gate": max_child <= RAW_MAX_GATE,
                "passes_strong_raw_gate": max_child <= STRONG_RAW_MAX_GATE,
                "passes_balance_gate": max_child <= BALANCE_REJECT_GATE,
                "passes_split_movement_gate": movement >= SPLIT_MOVEMENT_GATE,
                "funds_second_aggregate": max_child <= RAW_MAX_GATE and movement >= SPLIT_MOVEMENT_GATE,
            }
        )
    best = min(rows, key=lambda row: row["max_child_objective"])
    funded = [row for row in rows if row["funds_second_aggregate"]]
    report = clean(
        {
            "schema": "forge.hadwiger.w607_post_parent_lift_branch_prescreen.v1",
            "base_objective": base_obj,
            "base_x304": base_x[BRANCH],
            "candidate_count": len(rows),
            "raw_max_gate": RAW_MAX_GATE,
            "strong_raw_max_gate": STRONG_RAW_MAX_GATE,
            "balance_reject_gate": BALANCE_REJECT_GATE,
            "split_movement_gate": SPLIT_MOVEMENT_GATE,
            "best_by_raw_max": best,
            "funded_candidate_count": len(funded),
            "status": "FundSecondAggregateLift" if funded else "RetireSecondAggregateLift",
            "candidates": rows,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "candidates"}, indent=2))


if __name__ == "__main__":
    main()
