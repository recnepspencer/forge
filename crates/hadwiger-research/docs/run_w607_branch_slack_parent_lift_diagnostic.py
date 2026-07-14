import json
import math
from fractions import Fraction

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_gamma1_leaf_dual_export as gamma1_export
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SLACK = CRATE / "docs" / "w607-gamma0-branch-slack-lift-diagnostic.json"
GAMMA1_TREE = CRATE / "docs" / "w607-gamma1-branch-tree-preflight.json"
OUT_PATH = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"

GAMMA0 = 613_372_392
DENOMINATORS = [1024, 4096, 16384]
DROP_GATE = 1000.0
SUPPORT_GATE = 6
LIFT_MARGIN_NUM = 1024
ROUND_EPS = 1e-8


def rational(value):
    return Fraction(str(value)).limit_denominator(1024)


def solve_cover(active, rows, weights, cmod):
    index = {v: i for i, v in enumerate(active)}
    matrix = lil_matrix((len(active), len(rows)), dtype=float)
    rhs = []
    for col, row in enumerate(rows):
        rhs.append(float(row["rhs"]))
        for vertex in row["vertices"]:
            matrix[index[vertex], col] = float(weights[vertex]) if row["coeff"] == "w" else 1.0
    result = linprog(
        c=np.array(rhs, dtype=float),
        A_ub=-matrix.tocsr(),
        b_ub=-np.array([float(cmod[v]) for v in active]),
        bounds=[(0, None)] * len(rows),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return result.x, float(result.fun)


def rounded_certificate(active, rows, y, denominator, weights, cmod, included_weight):
    nums = [max(0, int(math.ceil(value * denominator - ROUND_EPS))) for value in y]
    coverage = {v: 0 for v in active}
    objective = int(included_weight * denominator)
    positive_rows = 0
    exported_rows = []
    for row, numerator in zip(rows, nums):
        if numerator == 0:
            continue
        positive_rows += 1
        objective += numerator * int(row["rhs"])
        for vertex in row["vertices"]:
            if vertex in coverage:
                coverage[vertex] += numerator * (int(weights[vertex]) if row["coeff"] == "w" else 1)
        exported = {
            "kind": row["kind"],
            "vertices": [v + 1 for v in row["vertices"]],
            "rhs": int(row["rhs"]),
            "numerator": numerator,
        }
        if row["kind"] == "rank":
            exported["name"] = row["name"]
            exported["full_support_size"] = row["full_support_size"]
        exported_rows.append(exported)
    min_slack = min((coverage[v] - int(cmod[v] * denominator) for v in active), default=0)
    return {
        "denominator": denominator,
        "objective_num": objective,
        "objective_bound": objective / denominator,
        "min_slack": min_slack,
        "positive_row_count": positive_rows,
        "rows": exported_rows,
    }


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items()}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, Fraction):
        return {"num": value.numerator, "den": value.denominator, "float": float(value)}
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    edges, weights = parent.parse_edges_weights()
    adj = parent.adjacency(edges)
    triads = gamma1_export.tree.triangles(adj)
    c0, _ = parent.exclude_coverage(weights)
    c0 = [Fraction(int(v), 1) for v in c0]
    slack = json.loads(SLACK.read_text())
    p = {row["vertex"] - 1: rational(row["coefficient"]) for row in slack["positive_coefficients"]}
    reduction = rational(slack["rhs_reduction"])
    gamma0_modified = Fraction(GAMMA0, 1) - reduction
    cmod = list(c0)
    for vertex, coeff in p.items():
        cmod[vertex] += coeff
    tree = json.loads(GAMMA1_TREE.read_text())
    leaf_reports = []
    for leaf_index, leaf in enumerate(tree["closed_leaves"]):
        included = tuple(sorted([parent.BRANCH, *(v - 1 for v in leaf["included"])]))
        excluded = tuple(v - 1 for v in leaf["excluded"])
        active = gamma1_export.tree.residual_vertices(adj, included, excluded)
        included_weight = sum(cmod[v] for v in included)
        rows = gamma1_export.all_rows(edges, triads, weights, adj, included, active)
        y, float_obj = solve_cover(active, rows, weights, cmod)
        attempts = []
        success = None
        for denominator in DENOMINATORS:
            cert = rounded_certificate(active, rows, y, denominator, weights, cmod, included_weight)
            passes = cert["min_slack"] >= 0
            cert["passes"] = passes
            attempts.append(cert)
            if passes:
                success = cert
                break
        leaf_reports.append(
            {
                "leaf_index": leaf_index,
                "included": leaf["included"],
                "excluded": leaf["excluded"],
                "active_vertices": len(active),
                "included_modified_weight": included_weight,
                "floating_objective": float_obj,
                "floating_total_objective": included_weight + Fraction(str(float_obj)),
                "attempts": attempts,
                "success": success,
            }
        )
    successes = [leaf["success"] for leaf in leaf_reports if leaf["success"] is not None]
    worst = max((Fraction(leaf["objective_num"], leaf["denominator"]) for leaf in successes), default=None)
    gamma1_modified = worst
    lift = gamma0_modified - gamma1_modified if gamma1_modified is not None else None
    cuts = parent_lift.root_cuts(weights.astype(float), adj)
    old_parent_row = parent_lift.parent_row(weights.astype(float))
    cmod_float = {v: float(value) for v, value in enumerate(cmod) if value}
    if lift is not None:
        cmod_float[parent.BRANCH] = cmod_float.get(parent.BRANCH, 0.0) + float(lift)
    new_row = (cmod_float, float(gamma0_modified))
    base_obj, base_x = parent_lift.solve_lp(edges, triads, weights.astype(float), cuts, [old_parent_row], solution=True)
    parent_obj = parent_lift.solve_lp(edges, triads, weights.astype(float), cuts, [old_parent_row, new_row]) if lift is not None else None
    parent_drop = base_obj - parent_obj if parent_obj is not None else None
    exact_pass = (
        len(successes) == len(leaf_reports)
        and lift is not None
        and lift * 1024 >= LIFT_MARGIN_NUM
        and max(leaf["denominator"] for leaf in successes) <= max(DENOMINATORS)
    )
    status = "RetireBranchSlackParentLift"
    if exact_pass and parent_drop is not None and parent_drop >= DROP_GATE and len(p) <= SUPPORT_GATE:
        status = "FundBranchSlackParentLift"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_branch_slack_parent_lift_diagnostic.v1",
            "canonical_denominator": 1024,
            "gamma0_modified": gamma0_modified,
            "gamma0_modified_num_d1024": int(gamma0_modified * 1024),
            "rhs_reduction": reduction,
            "rhs_reduction_num_d1024": int(reduction * 1024),
            "positive_coefficients": {str(v + 1): coeff for v, coeff in p.items()},
            "positive_coefficients_num_d1024": {str(v + 1): int(coeff * 1024) for v, coeff in p.items()},
            "coefficient_support": len(p),
            "leaf_count": len(leaf_reports),
            "successful_leaf_count": len(successes),
            "worst_gamma1_modified": gamma1_modified,
            "worst_gamma1_modified_num_d1024": int(gamma1_modified * 1024) if gamma1_modified is not None else None,
            "lift_coefficient": lift,
            "lift_coefficient_num_d1024": int(lift * 1024) if lift is not None else None,
            "base_parent_lift_objective": base_obj,
            "new_parent_objective": parent_obj,
            "parent_drop": parent_drop,
            "base_x304": base_x[parent.BRANCH],
            "drop_gate": DROP_GATE,
            "support_gate": SUPPORT_GATE,
            "lift_margin_num": LIFT_MARGIN_NUM,
            "leaf_reports": leaf_reports,
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "leaf_reports"}, indent=2))


if __name__ == "__main__":
    main()
