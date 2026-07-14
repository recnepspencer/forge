import json
from pathlib import Path

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import coo_matrix

import run_w607_gamma0_branch_tree_preflight as gamma0_tree
import run_w607_gamma1_leaf_dual_export as gamma1_export
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_branch_slack_residual_sensitivity as residual
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
GAMMA0 = CRATE / "docs" / "w607-gamma0-leaf-dual-export.json"
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT = CRATE / "docs" / "w607-branch-leaf-dual-ray-perturbation.json"

BRANCH_VERTEX = parent.BRANCH
DENOMINATOR = 1024.0
VIOLATION_GATE = 5000.0
DROP_GATE = 2000.0
REPLAY_DROP_GATE = 5000.0
OBJECTIVE_GATE = 590000.0
SIMILARITY_GATE = 0.98
OLD_VERTEX_DROP_GATE = 5000.0


def main():
    edges, weights = parent.parse_edges_weights()
    weights_float = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = parent_lift.root_cuts(weights_float, adj)
    old_parent = parent_lift.parent_row(weights_float)
    slack_parent = residual.p_parent_row(weights)
    base_obj, x = parent_lift.solve_lp(
        edges, triads, weights_float, cuts, [old_parent, slack_parent], solution=True
    )

    gamma0 = json.loads(GAMMA0.read_text())
    branch_slack = json.loads(BRANCH_SLACK.read_text())
    model = build_model(gamma0, branch_slack, adj, x, float(branch_slack["gamma0_modified"]["float"]))
    result = linprog(
        c=model["objective"],
        A_ub=model["matrix"],
        b_ub=model["upper"],
        bounds=model["bounds"],
        method="highs",
        options={"time_limit": 1200},
    )
    if not result.success:
        report = {
            "schema": "forge.hadwiger.w607_branch_leaf_dual_ray_perturbation.v1",
            "base_objective": base_obj,
            "status": "RetireBranchLeafDualRayPerturbation",
            "solver_success": False,
            "solver_message": result.message,
        }
        OUT.write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report, indent=2))
        return

    d = result.x[: parent.N]
    gamma0_bound = result.x[parent.N]
    gamma1_bound = result.x[parent.N + 1]
    lift = max(0.0, gamma0_bound - gamma1_bound)
    coeffs = {v: float(value) for v, value in enumerate(d) if value > 1e-7}
    coeffs[BRANCH_VERTEX] = coeffs.get(BRANCH_VERTEX, 0.0) + lift
    candidate_row = (coeffs, gamma0_bound)
    new_obj, new_x = parent_lift.solve_lp(
        edges,
        triads,
        weights_float,
        cuts,
        [old_parent, slack_parent, candidate_row],
        solution=True,
    )

    c0, _ = parent.exclude_coverage(weights)
    c0p = c0.astype(float)
    for vertex, coeff in branch_slack["positive_coefficients"].items():
        c0p[int(vertex) - 1] += float(coeff["float"])
    old_vertices = {int(vertex) - 1 for vertex in branch_slack["positive_coefficients"]}
    old_mass = sum(d[vertex] for vertex in old_vertices)
    total_mass = float(np.sum(d))
    violation = float(np.dot(d, x) + lift * x[BRANCH_VERTEX] - gamma0_bound)
    similarity_c0 = cosine(d, c0)
    similarity_c0p = cosine(d, c0p)
    mostly_old = total_mass > 0 and old_mass / total_mass >= 0.80
    drop = base_obj - new_obj
    clears_diagnostic = (
        violation >= VIOLATION_GATE
        and drop >= DROP_GATE
        and similarity_c0 < SIMILARITY_GATE
        and similarity_c0p < SIMILARITY_GATE
        and (not mostly_old or drop >= OLD_VERTEX_DROP_GATE)
    )
    funds_replay = clears_diagnostic and (drop >= REPLAY_DROP_GATE or new_obj <= OBJECTIVE_GATE)
    status = (
        "FundBranchLeafDualRayReplay"
        if funds_replay
        else "KeepDiagnosticOnlyBranchLeafDualRay"
        if clears_diagnostic
        else "RetireBranchLeafDualRayPerturbation"
    )
    report = {
        "schema": "forge.hadwiger.w607_branch_leaf_dual_ray_perturbation.v1",
        "base_objective": base_obj,
        "new_objective": new_obj,
        "drop": drop,
        "base_x304": float(x[BRANCH_VERTEX]),
        "new_x304": float(new_x[BRANCH_VERTEX]),
        "gamma0_bound": gamma0_bound,
        "gamma1_bound": gamma1_bound,
        "lift": lift,
        "current_violation": violation,
        "support_size": len([value for value in d if value > 1e-7]),
        "similarity_to_c0": similarity_c0,
        "similarity_to_c0_plus_p": similarity_c0p,
        "old_branch_slack_vertex_mass_fraction": old_mass / total_mass if total_mass else 0.0,
        "solver_objective": float(result.fun),
        "solver_iterations": int(result.nit),
        "violation_gate": VIOLATION_GATE,
        "drop_gate": DROP_GATE,
        "replay_drop_gate": REPLAY_DROP_GATE,
        "objective_gate": OBJECTIVE_GATE,
        "similarity_gate": SIMILARITY_GATE,
        "status": status,
        "authority": "diagnostic_only_no_rounding_or_replay",
        "top_d_vertices": top_vertices(d, weights, x),
    }
    OUT.write_text(json.dumps(clean(report), indent=2) + "\n")
    print(json.dumps({k: v for k, v in clean(report).items() if k != "top_d_vertices"}, indent=2))


def build_model(gamma0, branch_slack, adj, x, gamma0_cap):
    global Y_COUNT
    Y_COUNT = 0
    rows = []
    cols = []
    vals = []
    upper = []
    add_leaf_constraints(gamma0_leaves(gamma0, adj), 0, parent.N, rows, cols, vals, upper)
    add_leaf_constraints(gamma1_leaves(branch_slack, adj), 1, parent.N + 1, rows, cols, vals, upper)
    row = len(upper)
    rows.extend([row, row])
    cols.extend([parent.N + 1, parent.N])
    vals.extend([1.0, -1.0])
    upper.append(0.0)
    variable_count = parent.N + 2 + model_y_count()
    objective = np.zeros(variable_count)
    objective[: parent.N] = -x
    objective[parent.N] = 1.0
    bounds = [(0.0, None)] * variable_count
    bounds[BRANCH_VERTEX] = (0.0, 0.0)
    bounds[parent.N] = (0.0, gamma0_cap)
    matrix = coo_matrix((vals, (rows, cols)), shape=(len(upper), variable_count))
    return {"matrix": matrix.tocsr(), "upper": np.array(upper), "objective": objective, "bounds": bounds}


Y_COUNT = 0


def model_y_count():
    return Y_COUNT


def next_y_block(count):
    global Y_COUNT
    start = parent.N + 2 + Y_COUNT
    Y_COUNT += count
    return start


def add_leaf_constraints(leaves, branch_side, gamma_col, rows, cols, vals, upper):
    for leaf in leaves:
        y_start = next_y_block(len(leaf["rows"]))
        for active_vertex in leaf["active"]:
            row = len(upper)
            rows.append(row)
            cols.append(active_vertex)
            vals.append(1.0)
            for index, cover in leaf["coverage"].get(active_vertex, []):
                rows.append(row)
                cols.append(y_start + index)
                vals.append(-cover)
            upper.append(0.0)
        row = len(upper)
        for vertex in leaf["included"]:
            if vertex != BRANCH_VERTEX:
                rows.append(row)
                cols.append(vertex)
                vals.append(1.0)
        for index, cost in enumerate(leaf["costs"]):
            rows.append(row)
            cols.append(y_start + index)
            vals.append(cost)
        rows.append(row)
        cols.append(gamma_col)
        vals.append(-1.0)
        upper.append(0.0)


def gamma0_leaves(gamma0, adj):
    leaves = []
    for leaf in gamma0["leaves"]:
        included = tuple(v - 1 for v in leaf["included"])
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = tuple(gamma0_tree.residual_vertices(adj, included, excluded))
        leaves.append(leaf_model(included, active, leaf["success"]))
    return leaves


def gamma1_leaves(branch_slack, adj):
    leaves = []
    for leaf in branch_slack["leaf_reports"]:
        included = tuple(sorted([BRANCH_VERTEX, *(v - 1 for v in leaf["included"])]))
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = tuple(gamma1_export.tree.residual_vertices(adj, included, excluded))
        leaves.append(leaf_model(included, active, leaf["success"]))
    return leaves


def leaf_model(included, active, success):
    denominator = float(success.get("denominator", DENOMINATOR))
    active_set = set(active)
    coverage = {}
    costs = []
    rows = success["rows"]
    for index, row in enumerate(rows):
        coeff = float(row["numerator"]) / denominator
        costs.append(coeff * float(row["rhs"]))
        factor = coeff
        if row["kind"] == "rank":
            for vertex in row["vertices"]:
                v = vertex - 1
                if v in active_set:
                    coverage.setdefault(v, []).append((index, factor * row_weight(row, vertex)))
        else:
            for vertex in row["vertices"]:
                v = vertex - 1
                if v in active_set:
                    coverage.setdefault(v, []).append((index, factor))
    return {"included": included, "active": active, "rows": rows, "coverage": coverage, "costs": costs}


def row_weight(row, vertex):
    return WEIGHTS_CACHE[vertex - 1]


WEIGHTS_CACHE = None


def cosine(left, right):
    left = np.asarray(left, dtype=float)
    right = np.asarray(right, dtype=float)
    denom = float(np.linalg.norm(left) * np.linalg.norm(right))
    return 0.0 if denom == 0.0 else float(np.dot(left, right) / denom)


def top_vertices(d, weights, x):
    order = np.argsort(-d)[:20]
    return [
        {"vertex": int(vertex + 1), "d": float(d[vertex]), "weight": int(weights[vertex]), "x": float(x[vertex])}
        for vertex in order
        if d[vertex] > 1e-7
    ]


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


if __name__ == "__main__":
    edges, weights_for_cache = parent.parse_edges_weights()
    WEIGHTS_CACHE = weights_for_cache
    main()
