import hashlib
import json

import numpy as np
from scipy.optimize import linprog

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
REPLAY = CRATE / "docs" / "w607-fresh-mixed-branch-replay.json"
OUT = CRATE / "docs" / "w607-pool-affine-lift-probe.json"

POOL = [151, 221, 224, 382, 385, 455]
TARGET_GATE = 586500.0
RAW_KILL = 250.0
DROP_KILL = 250.0
FUND_DROP = 1000.0
FUND_OBJECTIVE = 593500.0
UNIFORM_TOL = 1e-7


def digest(value):
    payload = json.dumps(jsonable(value), sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(payload).hexdigest()


def jsonable(value):
    if isinstance(value, dict):
        return {str(key): jsonable(inner) for key, inner in value.items()}
    if isinstance(value, (list, tuple)):
        return [jsonable(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def graph_digest(edges, weights):
    payload = {
        "edges": [[int(a) + 1, int(b) + 1] for a, b in edges],
        "weights": [int(weight) for weight in weights],
    }
    return digest(payload)


def pool_assignment_from_child(child):
    return child["pool_assignment"]


def terminal_rows(replay):
    rows = []
    for leaf in replay["leaves"]:
        if leaf["leaf_index"] == 0:
            for index, closure in enumerate(leaf["residual_closures"]):
                if closure["triggered"]:
                    for child_index, child in enumerate(closure["children"]):
                        rows.append(
                            {
                                "leaf_index": 0,
                                "terminal_id": f"closure_{index}_child_{child_index}",
                                "U_t": float(child["bound"]),
                                "pool_assignment": pool_assignment_from_child(child),
                                "closure_source": "leaf0_residual_pair_child",
                            }
                        )
                else:
                    rows.append(
                        {
                            "leaf_index": 0,
                            "terminal_id": f"depth4_terminal_{index}",
                            "U_t": float(closure["closed_bound"]),
                            "pool_assignment": closure["terminal"]["pool_assignment"],
                            "closure_source": "leaf0_depth4_terminal",
                        }
                    )
        else:
            for index, terminal in enumerate(leaf["terminal_certificates"]):
                rows.append(
                    {
                        "leaf_index": leaf["leaf_index"],
                        "terminal_id": f"depth3_terminal_{index}",
                        "U_t": float(terminal["bound"]),
                        "pool_assignment": terminal["pool_assignment"],
                        "closure_source": "depth3_fixed_pool_terminal",
                    }
                )
    return rows


def mask_for_assignment(assignment):
    mask = []
    free = []
    for vertex in POOL:
        key = str(vertex + 1)
        if key in assignment:
            mask.append(float(assignment[key]))
        else:
            mask.append(1.0)
            free.append(vertex + 1)
    return mask, free


def annotate_masks(rows):
    annotated = []
    for row in rows:
        mask, free = mask_for_assignment(row["pool_assignment"])
        annotated.append({**row, "mask_t": mask, "free_pool_variables": free})
    return annotated


def solve_affine(root_pool_values, rows):
    variable_count = len(POOL) + 1
    objective = np.zeros(variable_count)
    objective[: len(POOL)] = -np.array(root_pool_values)
    objective[-1] = 1.0
    matrix = []
    rhs = []
    for row in rows:
        constraint = np.zeros(variable_count)
        constraint[: len(POOL)] = np.array(row["mask_t"])
        constraint[-1] = -1.0
        matrix.append(constraint)
        rhs.append(-row["U_t"])
    result = linprog(
        c=objective,
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        bounds=[(0.0, None)] * len(POOL) + [(None, None)],
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    coeffs = result.x[: len(POOL)]
    b_value = float(result.x[-1])
    raw_violation = float(np.dot(coeffs, root_pool_values) - b_value)
    slacks = []
    active = []
    for index, row in enumerate(rows):
        value = row["U_t"] + float(np.dot(coeffs, row["mask_t"]))
        slack = b_value - value
        slacks.append(slack)
        if abs(slack) <= 1e-5:
            active.append(index)
    return coeffs, b_value, raw_violation, slacks, active


def row_from_coeffs(weights, coeffs, b_value):
    row = {vertex: float(weights[vertex]) for vertex in range(parent.N)}
    for vertex, coeff in zip(POOL, coeffs):
        if coeff > 1e-10:
            row[vertex] += float(coeff)
    return row, float(b_value)


def scalar_measure(edges, triads, weights, root_cuts, parent_rows, scalar_bound):
    row = {"coefficients": {vertex: float(weights[vertex]) for vertex in range(parent.N)}, "rhs": scalar_bound}
    return branch_slack.solve_lp(edges, triads, weights, root_cuts, parent_rows, cg_cuts=[row])


def main():
    replay = json.loads(REPLAY.read_text())
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), branch_slack.p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    root_pool_values = [float(root_x[vertex]) for vertex in POOL]
    rows = annotate_masks(terminal_rows(replay))
    coeffs, b_value, raw_violation, slacks, active = solve_affine(root_pool_values, rows)
    affine_row = row_from_coeffs(weights, coeffs, b_value)
    new_obj, new_x = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        root_cuts,
        parent_rows,
        cg_cuts=[{"coefficients": affine_row[0], "rhs": affine_row[1]}],
        solution=True,
    )
    scalar_bound = max(row["U_t"] for row in rows)
    scalar_obj = scalar_measure(edges, triads, weights, root_cuts, parent_rows, scalar_bound)
    coeff_norm = float(np.linalg.norm(coeffs))
    nonzero = [float(value) for value in coeffs if value > 1e-8]
    uniform = bool(nonzero and max(nonzero) - min(nonzero) <= UNIFORM_TOL)
    zero = coeff_norm <= 1e-8
    drop = root_obj - new_obj
    status = "retire_pool_affine_lift_probe"
    if raw_violation > RAW_KILL and (drop >= FUND_DROP or new_obj <= FUND_OBJECTIVE):
        status = "fund_pool_affine_export_design"
    report = {
        "schema": "forge.hadwiger.w607_pool_affine_lift_probe.v1",
        "authority": "diagnostic_pool_affine_bridge_not_export_authority",
        "source_binding": {
            "fresh_replay_path": str(REPLAY),
            "fresh_replay_digest": digest(replay),
            "graph_digest": graph_digest(edges, weights),
            "fixed_pool_one_based": [vertex + 1 for vertex in POOL],
        },
        "second_opinion": {
            "agent": "Hume",
            "decision": "approve_bounded_diagnostic_bridge",
            "primary_failure_mode": "semantic_invalidity_or_vacuity_of_terminal_mask_abstraction",
        },
        "semantics_check": {
            "coefficients_nonnegative": bool(np.all(coeffs >= -1e-9)),
            "mask_rule": "fixed_0_to_0_fixed_1_to_1_free_or_unresolved_to_1",
            "free_variables_are_worst_case_one": True,
            "diagnostic_branch_authority_only": True,
        },
        "root_measurement": {
            "baseline_objective": root_obj,
            "baseline_pool_values": {str(vertex + 1): float(root_x[vertex]) for vertex in POOL},
            "raw_violation_at_baseline": raw_violation,
            "post_row_objective": new_obj,
            "drop": drop,
            "post_row_pool_values": {str(vertex + 1): float(new_x[vertex]) for vertex in POOL},
        },
        "lp_result": {
            "status": "optimal",
            "objective_B_minus_c_dot_x_root": -raw_violation,
            "B": b_value,
            "coefficients": {str(vertex + 1): float(value) for vertex, value in zip(POOL, coeffs)},
            "active_terminal_indices": active,
            "min_slack": float(min(slacks)),
            "max_slack": float(max(slacks)),
        },
        "degeneracy_checks": {
            "coefficient_norm": coeff_norm,
            "zero_coefficients": zero,
            "uniform_nonzero_coefficients": uniform,
            "scalar_max_bound": scalar_bound,
            "scalar_root_objective": scalar_obj,
            "scalar_root_drop": root_obj - scalar_obj,
            "drop_minus_scalar_drop": drop - (root_obj - scalar_obj),
        },
        "terminal_count": len(rows),
        "terminal_table": rows,
        "gates": {
            "target_gate": TARGET_GATE,
            "raw_kill": RAW_KILL,
            "drop_kill": DROP_KILL,
            "fund_drop": FUND_DROP,
            "fund_objective": FUND_OBJECTIVE,
        },
        "failure_reasons": [
            reason
            for reason, active_reason in [
                ("mask_semantics_failed", not bool(np.all(coeffs >= -1e-9))),
                ("zero_coefficients", zero),
                ("uniform_coefficients", uniform),
                ("raw_violation_below_gate", raw_violation <= RAW_KILL),
                ("root_drop_below_gate", drop <= DROP_KILL),
                ("scalar_cap_equivalent_or_weaker", drop <= root_obj - scalar_obj + 100.0),
            ]
            if active_reason
        ],
        "status": status,
    }
    if report["failure_reasons"]:
        report["status"] = "retire_pool_affine_lift_probe"
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "terminal_table"}, indent=2))


if __name__ == "__main__":
    main()
