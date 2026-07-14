import json

import numpy as np
from scipy.optimize import linprog

import run_w607_gamma0_branch_tree_preflight as gamma0_tree
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
GAMMA0_TREE = CRATE / "docs" / "w607-gamma0-branch-tree-preflight.json"
GAMMA0_LEAF = CRATE / "docs" / "w607-gamma0-leaf-dual-export.json"
OUT_PATH = CRATE / "docs" / "w607-gamma0-branch-slack-lift-diagnostic.json"

TARGET = 613_372_392.0
DENOMINATOR = 1024.0
VIOLATION_GATE = 1024_000.0
DROP_GATE = 1000.0
SUPPORT_GATE = 6


def branch_vertices(tree):
    vertices = set()
    for node in tree["expanded_nodes"]:
        vertices.add(node["branch_vertex"] - 1)
    return sorted(vertices)


def leaf_bound_map(leaf_artifact):
    out = {}
    for leaf in leaf_artifact["leaves"]:
        key = (tuple(v - 1 for v in leaf["included"]), tuple(v - 1 for v in leaf["excluded"]))
        out[key] = float(leaf["success"]["objective_bound"])
    return out


def conservative_membership(branch, included, excluded, active):
    active_set = set(active)
    included_set = set(included)
    excluded_set = set(excluded)
    return [1.0 if vertex in included_set or (vertex in active_set and vertex not in excluded_set) else 0.0 for vertex in branch]


def solve_slack_lp(tree, leaf_artifact, branch, adj):
    bounds_by_leaf = leaf_bound_map(leaf_artifact)
    constraints = []
    rhs = []
    rows = []
    for leaf in tree["closed_leaves"]:
        included = tuple(v - 1 for v in leaf["included"])
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = gamma0_tree.residual_vertices(adj, included, excluded)
        bound = bounds_by_leaf[(included, excluded)]
        slack = TARGET - bound
        coeff = conservative_membership(branch, included, excluded, active)
        constraints.append([*coeff, 1.0])
        rhs.append(slack)
        rows.append(
            {
                "included": leaf["included"],
                "excluded": leaf["excluded"],
                "leaf_bound": bound,
                "slack": slack,
                "charged_branch_vertices": [branch[i] + 1 for i, value in enumerate(coeff) if value],
            }
        )
    return constraints, rhs, rows


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
    tree = json.loads(GAMMA0_TREE.read_text())
    leaf_artifact = json.loads(GAMMA0_LEAF.read_text())
    branch = branch_vertices(tree)
    if len(branch) > 8:
        raise ValueError("branch support too large for diagnostic")
    c0, _ = parent.exclude_coverage(weights)
    c0 = np.array([float(v) for v in c0])
    cuts = parent_lift.root_cuts(weights, adj)
    parent_row = parent_lift.parent_row(weights)
    base_obj, base_x = parent_lift.solve_lp(edges, triads, weights, cuts, [parent_row], solution=True)
    constraints, rhs, leaf_rows = solve_slack_lp(tree, leaf_artifact, branch, adj)
    objective = [-float(base_x[v]) for v in branch] + [-1.0]
    result = linprog(
        c=np.array(objective),
        A_ub=np.array(constraints),
        b_ub=np.array(rhs),
        bounds=[(0, None)] * (len(branch) + 1),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    coeffs = result.x[: len(branch)]
    reduction = result.x[-1]
    current_violation_num = -float(result.fun)
    face_coeffs = {v: float(c0[v]) for v in range(parent.N) if c0[v]}
    for vertex, coeff in zip(branch, coeffs):
        if coeff > 1e-7:
            face_coeffs[vertex] = face_coeffs.get(vertex, 0.0) + float(coeff)
    face_rhs = TARGET - reduction
    face_row = (face_coeffs, face_rhs)
    face_obj = parent_lift.solve_lp(edges, triads, weights, cuts, [parent_row, face_row], fixed={parent.BRANCH: 0})
    drop = base_obj - face_obj
    positive = [
        {"vertex": vertex + 1, "coefficient": coeff}
        for vertex, coeff in zip(branch, coeffs)
        if coeff > 1e-7
    ]
    status = "RetireGamma0BranchSlackLift"
    if current_violation_num >= VIOLATION_GATE and drop >= DROP_GATE and len(positive) <= SUPPORT_GATE:
        status = "FundGamma0BranchSlackLift"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_gamma0_branch_slack_lift_diagnostic.v1",
            "target_gamma0": TARGET,
            "branch_vertices": [v + 1 for v in branch],
            "base_parent_lift_objective": base_obj,
            "base_x304": base_x[parent.BRANCH],
            "base_branch_values": {str(v + 1): base_x[v] for v in branch},
            "current_solution_violation_num": current_violation_num,
            "current_solution_violation": current_violation_num / DENOMINATOR,
            "rhs_reduction": reduction,
            "positive_coefficients": positive,
            "coefficient_support": len(positive),
            "face_objective_with_cut": face_obj,
            "face_drop": drop,
            "violation_gate_num": VIOLATION_GATE,
            "drop_gate": DROP_GATE,
            "support_gate": SUPPORT_GATE,
            "leaf_constraints": leaf_rows,
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "leaf_constraints"}, indent=2))


if __name__ == "__main__":
    main()
