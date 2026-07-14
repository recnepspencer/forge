import hashlib
import json

import numpy as np
from scipy.optimize import linprog

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_tailored_gamma0_branch_tree as tailored
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
TREE_PATH = CRATE / "docs" / "w607-tailored-gamma0-branch-tree.json"
OUT_PATH = CRATE / "docs" / "w607-tailored-gamma0-slack-face-diagnostic.json"

DEN = 1024.0
VIOLATION_KILL_NUM = 256_000.0
VIOLATION_FUND_NUM = 1_024_000.0
FACE_DROP_KILL = 250.0
FACE_DROP_FUND = 1000.0
SUPPORT_FUND = 8


def file_digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def branch_vertices(tree):
    out = []
    for node in tree["tree"]["expanded_nodes"]:
        vertex = int(node["branch_vertex"]) - 1
        if vertex not in out:
            out.append(vertex)
    return out


def residual_vertices(adj, included, excluded):
    blocked = {parent.BRANCH, *excluded}
    for vertex in included:
        blocked.add(vertex)
        blocked.update(adj[vertex])
    return [v for v in range(parent.N) if v not in blocked]


def membership(branch, included, excluded, active):
    included_set = set(included)
    active_set = set(active)
    excluded_set = set(excluded)
    return [
        1.0 if vertex in included_set or (vertex in active_set and vertex not in excluded_set) else 0.0
        for vertex in branch
    ]


def solve_slack_lp(tree, branch, adj):
    gamma0_d = float(tree["bounds"]["diagnostic_gamma0"])
    constraints = []
    rhs = []
    rows = []
    for leaf in tree["tree"]["closed_leaves"]:
        included = tuple(v - 1 for v in leaf["included"])
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = residual_vertices(adj, included, excluded)
        slack = gamma0_d - float(leaf["upper"])
        coeff = membership(branch, included, excluded, active)
        constraints.append([*coeff, 1.0])
        rhs.append(slack)
        rows.append(
            {
                "included": leaf["included"],
                "excluded": leaf["excluded"],
                "leaf_upper": leaf["upper"],
                "slack_before": slack,
                "charged_branch_vertices": [branch[i] + 1 for i, value in enumerate(coeff) if value],
            }
        )
    return constraints, rhs, rows


def face_row(d_num, q, branch, gamma0_num, reduction_num):
    coeffs = {v: float(d_num[v]) for v in range(parent.N) if v != parent.BRANCH and abs(d_num[v]) > 1e-9}
    for vertex, coeff in zip(branch, q):
        if coeff > 1e-7:
            coeffs[vertex] = coeffs.get(vertex, 0.0) + float(coeff * DEN)
    return coeffs, float(gamma0_num - reduction_num)


def solve_face(edges, triads, weights, cuts, rows, solution=False):
    return branch_slack.solve_lp(
        edges,
        triads,
        weights,
        cuts,
        rows,
        solution=solution,
    )


def solve_face_fixed(edges, triads, weights, cuts, rows, solution=False):
    return parent_lift.solve_lp(
        edges,
        triads,
        weights,
        cuts,
        rows,
        fixed={parent.BRANCH: 0},
        solution=solution,
    )


def top_changed(before, after, weights, limit=12):
    rows = []
    for vertex in range(parent.N):
        delta = abs(float(after[vertex]) - float(before[vertex]))
        if delta > 1e-8:
            rows.append(
                {
                    "vertex": vertex + 1,
                    "weight": weights[vertex],
                    "before": before[vertex],
                    "after": after[vertex],
                    "delta": delta,
                }
            )
    return sorted(rows, key=lambda row: (-row["delta"], -row["weight"], row["vertex"]))[:limit]


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items()}
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
    tree = json.loads(TREE_PATH.read_text())
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = parent_lift.root_cuts(weights, adj)
    d_num, _ = tailored.modified_vector(weights)
    branch = branch_vertices(tree)
    constraints, rhs, leaf_rows = solve_slack_lp(tree, branch, adj)
    projected = parent_lift.parent_row(weights)
    old_branch = branch_slack.p_parent_row(weights)
    baseline_obj, baseline_x = solve_face_fixed(edges, triads, weights, cuts, [projected, old_branch], solution=True)
    objective = [-float(baseline_x[v]) for v in branch] + [-1.0]
    result = linprog(
        c=np.array(objective),
        A_ub=np.array(constraints),
        b_ub=np.array(rhs),
        bounds=[(0, None)] * (len(branch) + 1),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    q = result.x[: len(branch)]
    reduction = float(result.x[-1])
    violation = -float(result.fun)
    gamma0_num = float(tree["bounds"]["diagnostic_gamma0_num_d1024"])
    row = face_row(d_num, q, branch, gamma0_num, reduction * DEN)
    face_obj, face_x = solve_face_fixed(edges, triads, weights, cuts, [projected, old_branch, row], solution=True)
    face_drop = baseline_obj - face_obj
    positive = [
        {"vertex": vertex + 1, "coefficient": coeff, "coefficient_num_d1024": coeff * DEN}
        for vertex, coeff in zip(branch, q)
        if coeff > 1e-7
    ]
    pure_reduction_share = reduction / violation if violation > 0 else None
    status = "RetireTailoredGamma0SlackFace"
    if (
        positive
        and violation >= VIOLATION_FUND_NUM / DEN
        and face_drop >= FACE_DROP_FUND
        and len(positive) <= SUPPORT_FUND
        and (pure_reduction_share is None or pure_reduction_share < 0.9)
    ):
        status = "FundTailoredGamma1ForSlackRow"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_tailored_gamma0_slack_face_diagnostic.v1",
            "authority": "face_valid_only",
            "parent_valid": False,
            "requires_gamma1_for_parent": True,
            "tested_with_x304_fixed": 0,
            "tree_artifact": str(TREE_PATH.relative_to(CRATE)),
            "tree_artifact_digest": file_digest(TREE_PATH),
            "branch_vertex": parent.BRANCH + 1,
            "branch_variables": [v + 1 for v in branch],
            "gamma0_d": tree["bounds"]["diagnostic_gamma0"],
            "gamma0_d_num_d1024": tree["bounds"]["diagnostic_gamma0_num_d1024"],
            "baseline_face_objective": baseline_obj,
            "baseline_x304": baseline_x[parent.BRANCH],
            "q_support": positive,
            "q_support_size": len(positive),
            "rhs_reduction": reduction,
            "rhs_reduction_num_d1024": reduction * DEN,
            "current_face_violation": violation,
            "current_face_violation_num_d1024": violation * DEN,
            "pure_reduction_share": pure_reduction_share,
            "face_objective_with_row": face_obj,
            "face_drop": face_drop,
            "post_x304": face_x[parent.BRANCH],
            "top_changed_vertices": top_changed(baseline_x, face_x, weights),
            "gates": {
                "violation_kill_num": VIOLATION_KILL_NUM,
                "violation_fund_num": VIOLATION_FUND_NUM,
                "face_drop_kill": FACE_DROP_KILL,
                "face_drop_fund": FACE_DROP_FUND,
                "support_fund": SUPPORT_FUND,
                "pure_reduction_share_kill": 0.9,
            },
            "leaf_constraints": leaf_rows,
            "failure_modes": [
                "face_row_used_as_parent_row",
                "undercharged_undecided_branch_variable",
                "stale_leaf_bound_for_wrong_vector",
                "support_leakage_beyond_tailored_branch_variables",
                "diagnostic_slack_mistaken_for_exact_replay_slack",
                "future_gamma1_erases_face_gain",
            ],
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "leaf_constraints"}, indent=2))


if __name__ == "__main__":
    main()
