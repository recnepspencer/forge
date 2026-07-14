import json

import numpy as np
from scipy.optimize import linprog

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


REPLAY = "crates/hadwiger-research/docs/w607-fresh-mixed-branch-replay.json"
OUT = "crates/hadwiger-research/docs/w607-tier-a-mixed-affine-lift.json"

TIER_A = [222, 223, 302, 304, 383, 384]
SCALAR_CAP = 586224.2382592546
RAW_KILL = 250.0
DROP_KILL = 250.0
FUND_DROP = 1000.0
FUND_OBJECTIVE = 593500.0
TOL = 1e-6


def semantic_mask(leaf):
    included = {vertex - 1 for vertex in leaf["tier_a_assignment"]["included"]}
    excluded = {vertex - 1 for vertex in leaf["tier_a_assignment"]["excluded"]}
    mask = []
    states = {}
    for vertex in TIER_A:
        if vertex in included:
            mask.append(1.0)
            states[str(vertex + 1)] = "included"
        elif vertex in excluded:
            mask.append(0.0)
            states[str(vertex + 1)] = "excluded_or_forced_zero"
        else:
            mask.append(1.0)
            states[str(vertex + 1)] = "free_worst_case_one"
    return mask, states


def source_kind(leaf):
    if leaf["exceptional_rule"] != "none":
        return leaf["exceptional_rule"]
    if len(leaf["tier_a_assignment"]["included"]) == 1:
        return "singleton_dense180_branch_bound"
    return "mixed_depth3_branch_bound"


def leaf_rows(replay):
    rows = []
    for leaf in replay["leaves"]:
        mask, states = semantic_mask(leaf)
        rows.append(
            {
                "leaf_index": leaf["leaf_index"],
                "U_leaf": leaf["final_mixed_bound"],
                "source": source_kind(leaf),
                "included": leaf["tier_a_assignment"]["included"],
                "excluded": leaf["tier_a_assignment"]["excluded"],
                "mask": mask,
                "states": states,
            }
        )
    return sorted(rows, key=lambda row: row["leaf_index"])


def solve_fit(root_values, leaves):
    variable_count = len(TIER_A) + 1
    objective = np.zeros(variable_count)
    objective[: len(TIER_A)] = -np.array(root_values)
    objective[-1] = 1.0
    matrix = []
    rhs = []
    for leaf in leaves:
        row = np.zeros(variable_count)
        row[: len(TIER_A)] = np.array(leaf["mask"])
        row[-1] = -1.0
        matrix.append(row)
        rhs.append(-leaf["U_leaf"])
    result = linprog(
        c=objective,
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        bounds=[(0.0, None)] * len(TIER_A) + [(None, None)],
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    coeffs = result.x[: len(TIER_A)]
    b_value = float(result.x[-1])
    slacks = []
    active = []
    for leaf in leaves:
        value = leaf["U_leaf"] + float(np.dot(coeffs, leaf["mask"]))
        slack = b_value - value
        slacks.append(slack)
        if abs(slack) <= 1e-5:
            active.append(leaf["leaf_index"])
    return coeffs, b_value, slacks, active


def row_from_coeffs(weights, coeffs, b_value):
    row = {vertex: float(weights[vertex]) for vertex in range(parent.N)}
    for vertex, coeff in zip(TIER_A, coeffs):
        if coeff > 1e-10:
            row[vertex] += float(coeff)
    return row, b_value


def scalar_objective(edges, triads, weights, root_cuts, parent_rows):
    row = {"coefficients": {vertex: float(weights[vertex]) for vertex in range(parent.N)}, "rhs": SCALAR_CAP}
    return branch_slack.solve_lp(edges, triads, weights, root_cuts, parent_rows, cg_cuts=[row])


def main():
    replay = json.load(open(REPLAY))
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), branch_slack.p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    root_values = [float(root_x[vertex]) for vertex in TIER_A]
    leaves = leaf_rows(replay)
    leaves_by_index = {leaf["leaf_index"]: leaf for leaf in leaves}
    coeffs, b_value, slacks, active = solve_fit(root_values, leaves)
    raw_violation = float(root_obj + np.dot(coeffs, root_values) - b_value)
    row = row_from_coeffs(weights, coeffs, b_value)
    new_obj, new_x = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        root_cuts,
        parent_rows,
        cg_cuts=[{"coefficients": row[0], "rhs": row[1]}],
        solution=True,
    )
    scalar_obj = scalar_objective(edges, triads, weights, root_cuts, parent_rows)
    singleton_faces = {
        next(iter(leaf["included"])) for leaf in leaves if len(leaf["included"]) == 1
    }
    active_singleton_only = all(
        len(leaves_by_index[index]["included"]) == 1 for index in active
    )
    non_active_tight = [
        leaf["leaf_index"]
        for leaf, slack in zip(leaves, slacks)
        if leaf["leaf_index"] not in active and slack <= 100.0
    ]
    zero = bool(np.linalg.norm(coeffs) <= 1e-8)
    nonzero = [value for value in coeffs if value > 1e-8]
    uniform = bool(nonzero and max(nonzero) - min(nonzero) <= 1e-7)
    drop = root_obj - new_obj
    scalar_drop = root_obj - scalar_obj
    failure_reasons = [
        reason
        for reason, active_reason in [
            ("zero_coefficients", zero),
            ("uniform_coefficients", uniform),
            ("raw_violation_below_gate", raw_violation <= RAW_KILL),
            ("root_drop_below_gate", drop <= DROP_KILL),
            ("scalar_cap_equivalent_or_weaker", drop <= scalar_drop + 100.0),
            ("singleton_only_active_control", active_singleton_only),
        ]
        if active_reason
    ]
    status = "fund_tier_a_affine_lift_export"
    if failure_reasons or not (drop >= FUND_DROP or new_obj <= FUND_OBJECTIVE):
        status = "retire_tier_a_affine_lift"
    report = {
        "schema": "forge.hadwiger.w607_tier_a_mixed_affine_lift.v1",
        "authority": "diagnostic_all16_tier_a_affine_lift_not_exact_export",
        "second_opinion": {
            "agent": "Aquinas",
            "decision": "approve_one_bounded_diagnostic",
            "primary_failure_mode": "scalar_cap_or_singleton_only_separator",
        },
        "tier_a_variables": [
            {"one_based": vertex + 1, "zero_based": vertex} for vertex in TIER_A
        ],
        "source_replay": REPLAY,
        "root_measurement": {
            "baseline_objective": root_obj,
            "root_tier_a_values": {str(vertex + 1): float(root_x[vertex]) for vertex in TIER_A},
            "raw_violation": raw_violation,
            "post_row_objective": new_obj,
            "drop": drop,
            "post_row_tier_a_values": {str(vertex + 1): float(new_x[vertex]) for vertex in TIER_A},
        },
        "fit_result": {
            "B": b_value,
            "objective_B_minus_root_row_lhs": b_value
            - float(root_obj + np.dot(coeffs, root_values)),
            "coefficients": {str(vertex + 1): float(coeff) for vertex, coeff in zip(TIER_A, coeffs)},
            "active_leaf_indices": active,
            "min_slack": float(min(slacks)),
            "max_slack": float(max(slacks)),
        },
        "validity_check": [
            {
                **leaf,
                "value_U_plus_c_mask": leaf["U_leaf"] + float(np.dot(coeffs, leaf["mask"])),
                "slack_to_B": float(slack),
            }
            for leaf, slack in zip(leaves, slacks)
        ],
        "scalar_comparison": {
            "scalar_cap": SCALAR_CAP,
            "scalar_objective": scalar_obj,
            "scalar_drop": scalar_drop,
            "drop_minus_scalar_drop": drop - scalar_drop,
            "zero_coefficients": zero,
            "uniform_coefficients": uniform,
        },
        "singleton_control_check": {
            "singleton_faces": sorted(singleton_faces),
            "active_singleton_only": active_singleton_only,
            "non_active_near_tight_leaves": non_active_tight,
        },
        "gates": {
            "raw_kill": RAW_KILL,
            "drop_kill": DROP_KILL,
            "fund_drop": FUND_DROP,
            "fund_objective": FUND_OBJECTIVE,
        },
        "failure_reasons": failure_reasons,
        "status": status,
    }
    with open(OUT, "w") as handle:
        json.dump(report, handle, indent=2)
        handle.write("\n")
    print(json.dumps({key: value for key, value in report.items() if key != "validity_check"}, indent=2))


if __name__ == "__main__":
    main()
