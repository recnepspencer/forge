import json
import math

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
OUT_PATH = CRATE / "docs" / "w607-two-variable-lift-diagnostic.json"

EXPECTED_BASE = 594914.351525072
PAIRS = [
    (304, 223),
    (304, 384),
    (223, 384),
    (222, 223),
    (222, 383),
    (302, 383),
    (302, 384),
]
VIOLATION_GATE = 1000.0
DROP_GATE = 1000.0
STRONG_ROOT_GATE = 593500.0
KILL_DROP = 250.0


def build_matrix(edges, triads, weights, rank_cuts, extra_rows, extra_cut=None):
    extra = 1 if extra_cut is not None else 0
    rows = len(edges) + len(triads) + len(rank_cuts) + len(extra_rows) + extra
    matrix = lil_matrix((rows, parent.N), dtype=float)
    upper = np.ones(rows)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1.0
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1.0
        row += 1
    for vertices, alpha in rank_cuts:
        for vertex in vertices:
            matrix[row, vertex] = float(weights[vertex])
        upper[row] = float(alpha)
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = float(coeff)
        upper[row] = float(rhs)
        row += 1
    if extra_cut is not None:
        for vertex, coeff in extra_cut["coefficients"].items():
            matrix[row, vertex] = float(coeff)
        upper[row] = float(extra_cut["rhs"])
    return matrix.tocsr(), upper


def solve_lp(edges, triads, weights, rank_cuts, extra_rows, objective, fixed=None, extra_cut=None, solution=False):
    matrix, upper = build_matrix(edges, triads, weights, rank_cuts, extra_rows, extra_cut)
    bounds = [(0.0, 1.0)] * parent.N
    for vertex, value in (fixed or {}).items():
        bounds[vertex] = (float(value), float(value))
    result = linprog(
        c=-np.asarray(objective, dtype=float),
        A_ub=matrix,
        b_ub=upper,
        bounds=bounds,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    value = -float(result.fun)
    return (value, result.x) if solution else value


def try_solve_lp(edges, triads, weights, rank_cuts, extra_rows, objective, fixed):
    try:
        return solve_lp(edges, triads, weights, rank_cuts, extra_rows, objective, fixed=fixed)
    except ValueError as err:
        if "infeasible" in str(err).lower():
            return None
        raise


def c0_vector(weights):
    c0, _ = parent.exclude_coverage(weights)
    return np.array([int(value) for value in c0], dtype=float)


def c0_plus_p_vector(weights):
    coeffs, _rhs = branch_slack.p_parent_row(weights)
    vector = np.zeros(parent.N, dtype=float)
    for vertex, coeff in coeffs.items():
        vector[vertex] = float(coeff)
    return vector


def best_lift_coefficients(xa, xb, caps):
    rows = []
    upper = []
    if caps.get("a") is not None:
        rows.append([1.0, 0.0])
        upper.append(caps["a"])
    if caps.get("b") is not None:
        rows.append([0.0, 1.0])
        upper.append(caps["b"])
    if caps.get("sum") is not None:
        rows.append([1.0, 1.0])
        upper.append(caps["sum"])
    result = linprog(
        c=np.array([-xa, -xb]),
        A_ub=np.array(rows),
        b_ub=np.array(upper),
        bounds=[(None, None), (None, None)],
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return float(result.x[0]), float(result.x[1])


def lift_for_pair(name, d, pair, root_x, edges, triads, weights, rank_cuts, extra_rows):
    a, b = pair
    gammas = {}
    feasible = {}
    for av in (0, 1):
        for bv in (0, 1):
            key = f"{av}{bv}"
            gammas[key] = try_solve_lp(
                edges,
                triads,
                weights,
                rank_cuts,
                extra_rows,
                d,
                fixed={a: float(av), b: float(bv)},
            )
            feasible[key] = gammas[key] is not None
    if gammas["00"] is None:
        return {
            "vector": name,
            "pair": [a + 1, b + 1],
            "gamma": gammas,
            "face_feasible": feasible,
            "root_violation": float("-inf"),
            "root_drop": 0.0,
            "new_root_objective": None,
            "coefficients": {},
            "rejected": "face_00_infeasible",
        }
    gamma00 = gammas["00"]
    if gammas["10"] is None or gammas["01"] is None:
        return {
            "vector": name,
            "pair": [a + 1, b + 1],
            "gamma": gammas,
            "face_feasible": feasible,
            "root_violation": float("-inf"),
            "root_drop": 0.0,
            "new_root_objective": None,
            "coefficients": {},
            "rejected": "singleton_include_face_infeasible",
        }
    caps = {
        "a": gamma00 - gammas["10"],
        "b": gamma00 - gammas["01"],
        "sum": None if gammas["11"] is None else gamma00 - gammas["11"],
    }
    lift_a, lift_b = best_lift_coefficients(root_x[a], root_x[b], caps)
    lhs = float(np.dot(d, root_x) + lift_a * root_x[a] + lift_b * root_x[b])
    violation = lhs - gamma00
    coefficients = {i: float(value) for i, value in enumerate(d) if abs(value) > 1e-9}
    coefficients[a] = coefficients.get(a, 0.0) + lift_a
    coefficients[b] = coefficients.get(b, 0.0) + lift_b
    return {
        "vector": name,
        "pair": [a + 1, b + 1],
        "gamma": gammas,
        "face_feasible": feasible,
        "lift_a": lift_a,
        "lift_b": lift_b,
        "cap_a": caps["a"],
        "cap_b": caps["b"],
        "cap_sum": caps["sum"],
        "root_lhs": lhs,
        "rhs": gamma00,
        "root_violation": violation,
        "coefficients": coefficients,
    }


def cosine_with_weights(coefficients, weights):
    vector = np.zeros(parent.N, dtype=float)
    for vertex, coeff in coefficients.items():
        vector[int(vertex)] = float(coeff)
    denom = np.linalg.norm(vector) * np.linalg.norm(weights)
    return float(np.dot(vector, weights) / denom) if denom else 0.0


def cut_from_row(row):
    return {"coefficients": row["coefficients"], "rhs": row["rhs"]}


def clean(value):
    if isinstance(value, dict):
        return {str(key): clean(inner) for key, inner in value.items() if key != "coefficients"}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    if isinstance(value, float) and (math.isinf(value) or math.isnan(value)):
        return str(value)
    return value


def main():
    edges, weights = parent.parse_edges_weights()
    weights_float = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    rank_cuts = parent_lift.root_cuts(weights_float, adj)
    extra_rows = [parent_lift.parent_row(weights_float), branch_slack.p_parent_row(weights)]
    root_obj, root_x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, weights_float, solution=True)
    vectors = [("c0", c0_vector(weights)), ("c0_plus_p", c0_plus_p_vector(weights))]
    rows = []
    for name, d in vectors:
        for pair in PAIRS:
            row = lift_for_pair(name, d, pair, root_x, edges, triads, weights_float, rank_cuts, extra_rows)
            row["cosine_with_weights"] = cosine_with_weights(row["coefficients"], weights_float)
            if row["root_violation"] >= VIOLATION_GATE:
                new_obj = solve_lp(
                    edges,
                    triads,
                    weights_float,
                    rank_cuts,
                    extra_rows,
                    weights_float,
                    extra_cut=cut_from_row(row),
                )
                row["root_drop"] = root_obj - new_obj
                row["new_root_objective"] = new_obj
            else:
                row["root_drop"] = 0.0
                row["new_root_objective"] = None
            rows.append(row)
    rows = sorted(rows, key=lambda item: (item["root_drop"], item["root_violation"]), reverse=True)
    best = rows[0] if rows else None
    status = "RetireTwoVariableLiftDiagnostic"
    if best and best["root_violation"] >= VIOLATION_GATE and best["root_drop"] >= DROP_GATE:
        status = "FundTwoVariableLiftReplay"
    if best and best.get("new_root_objective") is not None and best["new_root_objective"] <= STRONG_ROOT_GATE:
        status = "StrongFundTwoVariableLiftReplay"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_two_variable_lift_diagnostic.v1",
            "authority": "diagnostic_only_lp_face_bounds_no_leaf_replay",
            "second_agent_verdict": "coherent_only_if_each_gamma_maximizes_same_d_vector",
            "base_objective": root_obj,
            "baseline_reproduced": abs(root_obj - EXPECTED_BASE) <= 1e-5,
            "candidate_pair_count": len(PAIRS),
            "vector_count": len(vectors),
            "face_solve_count": len(PAIRS) * len(vectors) * 4,
            "best_root_violation": best["root_violation"] if best else 0.0,
            "best_root_drop": best["root_drop"] if best else 0.0,
            "best_new_root_objective": best.get("new_root_objective") if best else None,
            "status": status,
            "gates": {
                "violation_gate": VIOLATION_GATE,
                "drop_gate": DROP_GATE,
                "strong_root_gate": STRONG_ROOT_GATE,
                "kill_drop": KILL_DROP,
                "pairs": [[a + 1, b + 1] for a, b in PAIRS],
                "vectors": [name for name, _d in vectors],
            },
            "top_rows": rows[:20],
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "top_rows"}, indent=2))


if __name__ == "__main__":
    main()
