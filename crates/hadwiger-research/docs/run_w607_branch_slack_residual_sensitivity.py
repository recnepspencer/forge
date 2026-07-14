import json

import numpy as np
from scipy.optimize import linprog

import run_w607_gamma0_branch_tree_preflight as gamma0_tree
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
GAMMA0_LEAF = CRATE / "docs" / "w607-gamma0-leaf-dual-export.json"
GAMMA0_SLACK = CRATE / "docs" / "w607-gamma0-branch-slack-lift-diagnostic.json"
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-branch-slack-residual-sensitivity.json"

TARGET_NUM = 613_372_392 * 1024
DENOMINATOR = 1024
CANDIDATE_LIMIT = 32
VIOLATION_GATE_NUM = 1_024_000
FACE_DROP_GATE = 1000.0
SUPPORT_GATE = 12


def p_parent_row(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(c0[v]) * DENOMINATOR for v in range(parent.N) if c0[v]}
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        coeffs[int(vertex) - 1] = coeffs.get(int(vertex) - 1, 0) + int(coeff)
    coeffs[parent.BRANCH] = int(artifact["lift_coefficient_num_d1024"])
    return coeffs, int(artifact["gamma0_modified_num_d1024"])


def residual_rows(adj):
    artifact = json.loads(BRANCH_SLACK.read_text())
    leaf_artifact = json.loads(GAMMA0_LEAF.read_text())
    p = {int(v) - 1: int(c) for v, c in artifact["positive_coefficients_num_d1024"].items()}
    reduction = int(artifact["rhs_reduction_num_d1024"])
    rows = []
    for leaf in leaf_artifact["leaves"]:
        included = tuple(v - 1 for v in leaf["included"])
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = tuple(gamma0_tree.residual_vertices(adj, included, excluded))
        active_set = set(active)
        included_set = set(included)
        charge_p = sum(
            coeff
            for vertex, coeff in p.items()
            if vertex in included_set or vertex in active_set
        )
        residual = TARGET_NUM - int(leaf["success"]["objective_num"]) - charge_p - reduction
        if residual < 0:
            raise ValueError("replayed p row overspends a gamma0 leaf")
        rows.append(
            {
                "included": included,
                "excluded": excluded,
                "active": active,
                "residual_num": residual,
            }
        )
    return rows, p, reduction


def membership(vertex, row):
    return 1.0 if vertex in row["included"] or vertex in row["active"] else 0.0


def candidate_vertices(x, weights, residuals, old_branch_vertices):
    rows = []
    for vertex in range(parent.N):
        if vertex == parent.BRANCH or vertex in old_branch_vertices or x[vertex] <= 1e-7:
            continue
        caps = [row["residual_num"] for row in residuals if membership(vertex, row)]
        if not caps:
            continue
        cap = min(caps)
        rows.append(
            {
                "vertex": vertex,
                "x": float(x[vertex]),
                "weight": int(weights[vertex]),
                "singleton_capacity_num": int(cap),
                "singleton_value_num": float(cap * x[vertex]),
            }
        )
    return sorted(rows, key=lambda row: (-row["singleton_value_num"], -row["weight"], row["vertex"]))[
        :CANDIDATE_LIMIT
    ]


def solve_q_lp(candidates, residuals, x):
    matrix = []
    rhs = []
    for row in residuals:
        matrix.append([membership(candidate["vertex"], row) for candidate in candidates] + [1.0])
        rhs.append(row["residual_num"])
    objective = [-float(x[candidate["vertex"]]) for candidate in candidates] + [-1.0]
    result = linprog(
        c=np.array(objective),
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        bounds=[(0, None)] * (len(candidates) + 1),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    q = result.x[: len(candidates)]
    reduction = float(result.x[-1])
    return q, reduction, -float(result.fun)


def face_row(weights, p, q_entries, reduction1, reduction2):
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: float(c0[v]) for v in range(parent.N) if c0[v]}
    for vertex, coeff in p.items():
        coeffs[vertex] = coeffs.get(vertex, 0.0) + coeff / DENOMINATOR
    for vertex, coeff in q_entries.items():
        coeffs[vertex] = coeffs.get(vertex, 0.0) + coeff / DENOMINATOR
    return coeffs, (TARGET_NUM - reduction1 - reduction2) / DENOMINATOR


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
    weights_float = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = parent_lift.root_cuts(weights_float, adj)
    old_parent = parent_lift.parent_row(weights_float)
    p_row = p_parent_row(weights)
    base_obj, base_x = parent_lift.solve_lp(edges, triads, weights_float, cuts, [old_parent, p_row], solution=True)
    residuals, p, reduction1 = residual_rows(adj)
    slack_artifact = json.loads(GAMMA0_SLACK.read_text())
    old_branch_vertices = {vertex - 1 for vertex in slack_artifact["branch_vertices"]}
    candidates = candidate_vertices(base_x, weights, residuals, old_branch_vertices)
    q, reduction2, violation = solve_q_lp(candidates, residuals, base_x)
    q_entries = {
        candidate["vertex"]: float(coeff)
        for candidate, coeff in zip(candidates, q)
        if coeff > 1e-7
    }
    base_face = parent_lift.solve_lp(
        edges, triads, weights_float, cuts, [old_parent, p_row], fixed={parent.BRANCH: 0}
    )
    q_face = parent_lift.solve_lp(
        edges,
        triads,
        weights_float,
        cuts,
        [old_parent, p_row, face_row(weights, p, q_entries, reduction1, reduction2)],
        fixed={parent.BRANCH: 0},
    )
    min_residual_after_q = min(
        row["residual_num"]
        - reduction2
        - sum(coeff for vertex, coeff in q_entries.items() if membership(vertex, row))
        for row in residuals
    )
    support = len(q_entries)
    face_drop = base_face - q_face
    funds = (
        violation >= VIOLATION_GATE_NUM
        and face_drop >= FACE_DROP_GATE
        and support <= SUPPORT_GATE
        and min_residual_after_q >= 1.0
    )
    report = clean(
        {
            "schema": "forge.hadwiger.w607_branch_slack_residual_sensitivity.v1",
            "base_objective_with_replayed_p": base_obj,
            "base_x304": base_x[parent.BRANCH],
            "candidate_limit": CANDIDATE_LIMIT,
            "candidate_count": len(candidates),
            "old_positive_p_vertices": sorted(v + 1 for v in p),
            "q_support": [
                {
                    "vertex": vertex + 1,
                    "coefficient_num_d1024": coeff,
                    "coefficient": coeff / DENOMINATOR,
                    "x": float(base_x[vertex]),
                    "weight": int(weights[vertex]),
                }
                for vertex, coeff in q_entries.items()
            ],
            "q_support_size": support,
            "residual_reduction2_num": reduction2,
            "residual_reduction2": reduction2 / DENOMINATOR,
            "current_solution_violation_num": violation,
            "current_solution_violation": violation / DENOMINATOR,
            "base_face_objective": base_face,
            "q_face_objective": q_face,
            "q_face_drop": face_drop,
            "min_residual_after_q_num": min_residual_after_q,
            "violation_gate_num": VIOLATION_GATE_NUM,
            "face_drop_gate": FACE_DROP_GATE,
            "support_gate": SUPPORT_GATE,
            "top_singleton_candidates": [
                {
                    "vertex": row["vertex"] + 1,
                    "x": row["x"],
                    "weight": row["weight"],
                    "singleton_capacity_num": row["singleton_capacity_num"],
                    "singleton_value": row["singleton_value_num"] / DENOMINATOR,
                }
                for row in candidates[:12]
            ],
            "status": "FundResidualQGamma1Probe" if funds else "RetireResidualBranchSlackIteration",
            "authority": "diagnostic_only_no_gamma1_or_parent_replay",
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in {"top_singleton_candidates"}}, indent=2))


if __name__ == "__main__":
    main()
