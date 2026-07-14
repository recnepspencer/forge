import hashlib
import json

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_full_tree_rank_family as full_family
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-post-family-common-d-cglp.json"

MAX_ITERATIONS = 12
CUT_TOL = 1e-7
DROP_KILL = 250.0
FUND_DROP = 1000.0
FUND_OBJECTIVE = 592000.0
COSINE_REPACKAGE = 0.96


def graph_digest(edges, weights):
    payload = {
        "edges": [[int(a) + 1, int(b) + 1] for a, b in edges],
        "weights": [int(weight) for weight in weights],
    }
    return hashlib.sha256(json.dumps(payload, separators=(",", ":")).encode()).hexdigest()


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def fixed_from_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def solve_objective_lp(edges, triads, weights, root_cuts, parent_rows, objective, fixed):
    row_count = len(edges) + len(triads) + len(root_cuts) + len(parent_rows)
    matrix = lil_matrix((row_count, parent.N), dtype=float)
    upper = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1.0
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1.0
        row += 1
    for vertices, alpha in root_cuts:
        for vertex in vertices:
            matrix[row, vertex] = float(weights[vertex])
        upper[row] = float(alpha)
        row += 1
    for coeffs, rhs in parent_rows:
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


def enriched_leaves(edges, triads, weights, adj, root_cuts, parent_rows):
    source = json.loads(SOURCE.read_text())
    by_index = {leaf["leaf_index"]: leaf for leaf in source["leaves"]}
    _expanded, raw_leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    leaves = []
    for index, leaf in enumerate([row for row in raw_leaves if row["feasible"]]):
        fixed = fixed_from_leaf(leaf)
        _base, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, parent_rows, fixed, True)
        candidates = {support_hash(row["vertices"]): row for row in full_family.candidate_rows(weights, x, fixed, adj)}
        cuts = []
        for accepted in by_index[index]["accepted_rows"]:
            row = candidates[accepted["support_digest"]]
            cuts.append((row["vertices"], int(accepted["alpha_w"])))
        post_obj, post_x = bundle.leaf_rank.solve_lp(
            edges, triads, weights, root_cuts + cuts, parent_rows, fixed, True
        )
        leaves.append(
            {
                "leaf_index": index,
                "fixed": fixed,
                "included": by_index[index]["included"],
                "excluded": by_index[index]["excluded"],
                "accepted_row_count": len(cuts),
                "cuts": cuts,
                "post_first_objective": post_obj,
                "post_first_x": post_x,
            }
        )
    return leaves


def solve_master(root_x, samples):
    variable_count = parent.N + 1
    objective = np.zeros(variable_count)
    objective[: parent.N] = -root_x
    objective[-1] = 1.0
    matrix = []
    rhs = []
    for sample in samples:
        row = np.zeros(variable_count)
        row[: parent.N] = sample["x"]
        row[-1] = -1.0
        matrix.append(row)
        rhs.append(0.0)
    equality = np.zeros((1, variable_count))
    equality[0, : parent.N] = 1.0
    result = linprog(
        c=objective,
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        A_eq=equality,
        b_eq=np.array([1.0]),
        bounds=[(0.0, 1.0)] * parent.N + [(None, None)],
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return result.x[: parent.N], float(result.x[-1]), -float(result.fun)


def learn_common_d(edges, triads, weights, root_cuts, parent_rows, root_x, leaves):
    samples = [{"leaf_index": leaf["leaf_index"], "x": leaf["post_first_x"]} for leaf in leaves]
    iterations = []
    d = None
    master_b = None
    for iteration in range(MAX_ITERATIONS):
        d, master_b, master_violation = solve_master(root_x, samples)
        separated = []
        for leaf in leaves:
            value, x = solve_objective_lp(
                edges, triads, weights, root_cuts + leaf["cuts"], parent_rows, d, leaf["fixed"]
            )
            separated.append({"leaf_index": leaf["leaf_index"], "value": value, "x": x})
        worst = max(separated, key=lambda row: row["value"])
        violation = worst["value"] - master_b
        iterations.append(
            {
                "iteration": iteration,
                "master_b": master_b,
                "master_violation": master_violation,
                "worst_leaf_index": worst["leaf_index"],
                "worst_leaf_value": worst["value"],
                "separation_violation": violation,
                "sample_count": len(samples),
            }
        )
        if violation <= CUT_TOL:
            break
        samples.append({"leaf_index": worst["leaf_index"], "x": worst["x"]})
    return d, iterations


def cosine(left, right):
    denom = float(np.linalg.norm(left) * np.linalg.norm(right))
    return 0.0 if denom == 0.0 else float(np.dot(left, right) / denom)


def row_from_d(d, rhs):
    return {index: float(value) for index, value in enumerate(d) if value > 1e-10}, float(rhs)


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
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    leaves = enriched_leaves(edges, triads, weights, adj, root_cuts, parent_rows)
    top_six = sorted(leaves, key=lambda row: -row["post_first_objective"])[:6]
    d, iterations = learn_common_d(edges, triads, weights, root_cuts, parent_rows, root_x, top_six)
    all_suprema = []
    for leaf in leaves:
        value, _x = solve_objective_lp(
            edges, triads, weights, root_cuts + leaf["cuts"], parent_rows, d, leaf["fixed"]
        )
        all_suprema.append({"leaf_index": leaf["leaf_index"], "value": value})
    rhs = max(row["value"] for row in all_suprema)
    root_lhs = float(np.dot(d, root_x))
    raw_violation = root_lhs - rhs
    row = row_from_d(d, rhs)
    new_obj, new_x = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        root_cuts,
        parent_rows,
        cg_cuts=[{"coefficients": row[0], "rhs": row[1]}],
        solution=True,
    )
    scalar_bound = max(leaf["post_first_objective"] for leaf in leaves)
    scalar_obj = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        root_cuts,
        parent_rows,
        cg_cuts=[{"coefficients": {v: float(weights[v]) for v in range(parent.N)}, "rhs": scalar_bound}],
    )
    drop = root_obj - new_obj
    similarity_w = cosine(d, weights)
    status = "RetirePostFamilyCommonDCglp"
    if drop >= FUND_DROP or new_obj <= FUND_OBJECTIVE:
        status = "FundFullPostFamilyCglpReplayDesign"
    elif drop >= DROP_KILL and similarity_w <= COSINE_REPACKAGE:
        status = "KeepDiagnosticOnlyPostFamilyCommonD"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_post_family_common_d_cglp.v1",
            "authority": "diagnostic_common_d_disjunctive_row_no_replay_authority",
            "graph_digest": graph_digest(edges, weights),
            "row_system": "16_root_rank_rows_plus_two_parent_rows_plus_leaf_local_first_family_rows",
            "root_objective_before": root_obj,
            "root_x304_before": float(root_x[parent.BRANCH]),
            "top_six_leaf_indices": [leaf["leaf_index"] for leaf in top_six],
            "leaf_count": len(leaves),
            "d_normalization": "sum(d)=1,d>=0",
            "d_support_size": int(np.sum(d > 1e-10)),
            "top_d_vertices": [
                {"vertex": int(v + 1), "d": float(d[v]), "weight": float(weights[v]), "root_x": float(root_x[v])}
                for v in np.argsort(-d)[:20]
                if d[v] > 1e-10
            ],
            "cosine_to_w": similarity_w,
            "iterations": iterations,
            "all_leaf_suprema": sorted(all_suprema, key=lambda row: -row["value"]),
            "B_all_leaves": rhs,
            "root_lhs": root_lhs,
            "raw_violation": raw_violation,
            "root_objective_after_row": new_obj,
            "root_x304_after_row": float(new_x[parent.BRANCH]),
            "root_drop": drop,
            "scalar_max_leaf_bound": scalar_bound,
            "scalar_max_leaf_drop": root_obj - scalar_obj,
            "gates": {
                "max_iterations": MAX_ITERATIONS,
                "drop_kill": DROP_KILL,
                "fund_drop": FUND_DROP,
                "fund_objective": FUND_OBJECTIVE,
                "cosine_repackage": COSINE_REPACKAGE,
            },
            "failure_classification": (
                "funded"
                if status == "FundFullPostFamilyCglpReplayDesign"
                else "objective_repackage"
                if similarity_w > COSINE_REPACKAGE
                else "lp_redundant"
                if drop < DROP_KILL
                else "diagnostic_only"
            ),
            "status": status,
        }
    )
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key not in {"iterations", "all_leaf_suprema"}}, indent=2))


if __name__ == "__main__":
    main()
