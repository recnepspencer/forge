import hashlib
import json

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_branch_slack_plateau_branch_tree as plateau
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
OUT_PATH = CRATE / "docs" / "w607-plateau-signed-cglp.json"

BOXES = [5000.0, 10000.0, 25000.0]
LARGE_BOX = 50000.0
MAX_ITERATIONS = 25
CUT_TOL = 1e-5
RAW_KILL = 250.0
DROP_KILL = 250.0
FUND_DROP = 1000.0
STRONG_DROP = 3000.0
SCALAR_MARGIN = 500.0
MEANINGFUL_OBJECTIVE = 593500.0


def graph_digest(edges, weights):
    payload = {
        "edges": [[int(a) + 1, int(b) + 1] for a, b in edges],
        "weights": [int(w) for w in weights],
    }
    return hashlib.sha256(json.dumps(payload, separators=(",", ":")).encode()).hexdigest()


def solve_objective_lp(edges, triads, weights, cuts, extra_rows, objective, fixed=None):
    fixed = fixed or {}
    row_count = len(edges) + len(triads) + len(cuts) + len(extra_rows)
    matrix = lil_matrix((row_count, parent.N), dtype=float)
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
    bounds = [(0.0, 1.0)] * parent.N
    for vertex, value in fixed.items():
        bounds[vertex] = (float(value), float(value))
    result = linprog(
        c=-np.array(objective, dtype=float),
        A_ub=matrix.tocsr(),
        b_ub=upper,
        bounds=bounds,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result.x


def objective_for(weights, tier, coeffs):
    objective = weights.astype(float).copy()
    for vertex, coeff in zip(tier, coeffs):
        objective[vertex] += coeff
    return objective


def leaf_support(edges, triads, weights, cuts, rows, tier, leaf, coeffs):
    objective = objective_for(weights, tier, coeffs)
    value, x = solve_objective_lp(edges, triads, weights, cuts, rows, objective, leaf["fixed"])
    return {
        "leaf_index": leaf["leaf_index"],
        "value": value,
        "base_weight_objective": float(np.dot(weights, x)),
        "tier_values": [float(x[v]) for v in tier],
    }


def solve_master(cuts, root_tier, box):
    variable_count = len(root_tier) + 1
    objective = np.zeros(variable_count)
    objective[: len(root_tier)] = -np.array(root_tier)
    objective[-1] = 1.0
    matrix = []
    rhs = []
    for cut in cuts:
        row = np.zeros(variable_count)
        row[: len(root_tier)] = np.array(cut["tier_values"])
        row[-1] = -1.0
        matrix.append(row)
        rhs.append(-float(cut["base_weight_objective"]))
    result = linprog(
        c=objective,
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        bounds=[(-box, box)] * len(root_tier) + [(None, None)],
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return result.x[: len(root_tier)], float(result.x[-1])


def run_box(edges, triads, weights, cuts, rows, tier, leaves, root_obj, root_x, box):
    root_tier = [float(root_x[v]) for v in tier]
    master_cuts = []
    zero = np.zeros(len(tier))
    for leaf in leaves:
        master_cuts.append(leaf_support(edges, triads, weights, cuts, rows, tier, leaf, zero))
    iterations = []
    best = None
    for iteration in range(MAX_ITERATIONS):
        coeffs, bound = solve_master(master_cuts, root_tier, box)
        supports = [
            leaf_support(edges, triads, weights, cuts, rows, tier, leaf, coeffs)
            for leaf in leaves
        ]
        worst = max(supports, key=lambda row: row["value"])
        violation = worst["value"] - bound
        raw = root_obj + float(np.dot(coeffs, root_tier)) - worst["value"]
        iterations.append(
            {
                "iteration": iteration,
                "coefficients": coeffs.tolist(),
                "master_bound": bound,
                "worst_leaf_value": worst["value"],
                "worst_leaf_index": worst["leaf_index"],
                "separation_violation": violation,
                "raw_violation": raw,
                "cut_count": len(master_cuts),
            }
        )
        best = {
            "coefficients": coeffs,
            "B": worst["value"],
            "raw_violation": raw,
            "supports": supports,
            "iterations": iterations,
            "boundary_active": any(abs(abs(c) - box) <= 1e-6 for c in coeffs),
            "box": box,
        }
        if violation <= CUT_TOL:
            break
        master_cuts.append(worst)
    return best


def row_from_coeffs(weights, tier, coeffs, bound):
    row = {v: float(weights[v]) for v in range(parent.N)}
    for vertex, coeff in zip(tier, coeffs):
        row[vertex] += float(coeff)
    return row, float(bound)


def tier_values(tier, x):
    return {str(v + 1): float(x[v]) for v in tier}


def summarize_best(best, tier):
    supports = sorted(best["supports"], key=lambda row: row["value"], reverse=True)
    return {
        "box": best["box"],
        "coefficients": {str(v + 1): float(c) for v, c in zip(tier, best["coefficients"]) if abs(c) > 1e-7},
        "B": best["B"],
        "raw_violation": best["raw_violation"],
        "boundary_active": best["boundary_active"],
        "tight_leaf_indices": [
            row["leaf_index"] for row in supports if abs(row["value"] - best["B"]) <= 1e-5
        ],
        "leaf_values": supports,
        "iterations": best["iterations"],
    }


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "x"}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, tuple):
        return [clean(inner) for inner in value]
    if isinstance(value, np.ndarray):
        return [clean(inner) for inner in value.tolist()]
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
    cuts = parent_lift.root_cuts(weights, adj)
    rows = [parent_lift.parent_row(weights), plateau.p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(edges, triads, weights, cuts, rows, solution=True)
    expanded, raw_leaves = affine.full_tree(edges, triads, weights, cuts, rows)
    leaves = [
        {**leaf, "leaf_index": index}
        for index, leaf in enumerate(raw_leaves)
        if leaf["feasible"]
    ]
    tier = list(plateau.TIER_A)
    boxes = list(BOXES)
    box_results = []
    for box in boxes:
        result = run_box(edges, triads, weights, cuts, rows, tier, leaves, root_obj, root_x, box)
        box_results.append(result)
    if box_results[-1]["boundary_active"]:
        box_results.append(run_box(edges, triads, weights, cuts, rows, tier, leaves, root_obj, root_x, LARGE_BOX))
    best = max(box_results, key=lambda row: row["raw_violation"])
    signed_row = row_from_coeffs(weights, tier, best["coefficients"], best["B"])
    new_obj, new_x = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        cuts,
        rows,
        cg_cuts=[{"coefficients": signed_row[0], "rhs": signed_row[1]}],
        solution=True,
    )
    drop = root_obj - new_obj
    scalar_bound = max(leaf["upper"] for leaf in leaves)
    scalar_obj = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        cuts,
        rows,
        cg_cuts=[{"coefficients": {v: float(weights[v]) for v in range(parent.N)}, "rhs": scalar_bound}],
    )
    scalar_drop = root_obj - scalar_obj
    status = "RetirePlateauSignedCglp"
    if best["raw_violation"] >= RAW_KILL and drop >= FUND_DROP and drop >= scalar_drop + SCALAR_MARGIN:
        status = "FundPlateauSignedLeafReplay"
    if drop >= STRONG_DROP or new_obj <= MEANINGFUL_OBJECTIVE:
        status = "StrongPlateauSignedCglp"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_plateau_signed_cglp.v1",
            "authority": "diagnostic_disjunctive_hull_cut_not_native_root_authority",
            "graph_digest": graph_digest(edges, weights),
            "row_system": "16_root_rank_rows_plus_projected_parent_lift_plus_branch_slack_parent_lift",
            "tier_a_vertices": [v + 1 for v in tier],
            "expanded_count": len(expanded),
            "leaf_count": len(leaves),
            "root_objective": root_obj,
            "root_tier_values": tier_values(tier, root_x),
            "boxes": boxes + ([LARGE_BOX] if len(box_results) > len(boxes) else []),
            "max_iterations": MAX_ITERATIONS,
            "box_results": [summarize_best(result, tier) for result in box_results],
            "best": summarize_best(best, tier),
            "new_objective": new_obj,
            "drop": drop,
            "new_tier_values": tier_values(tier, new_x),
            "scalar_max_leaf_bound": scalar_bound,
            "scalar_max_leaf_drop": scalar_drop,
            "gates": {
                "raw_kill": RAW_KILL,
                "drop_kill": DROP_KILL,
                "fund_drop": FUND_DROP,
                "strong_drop": STRONG_DROP,
                "scalar_margin": SCALAR_MARGIN,
                "meaningful_objective": MEANINGFUL_OBJECTIVE,
            },
            "failure_modes": [
                "disjunctive_hull_cut_mistaken_for_native_lp_row",
                "leaf_partition_not_complete",
                "modified_objective_leaf_lp_not_same_row_system",
                "best_coefficients_boundary_driven",
                "high_raw_violation_low_lp_drop",
                "leaf_duals_not_exported_or_replayed",
            ],
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "box_results"}, indent=2))


if __name__ == "__main__":
    main()
