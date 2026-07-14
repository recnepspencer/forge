import hashlib
import json

import numpy as np
from scipy.optimize import linprog

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_branch_slack_plateau_branch_tree as plateau
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
OUT_PATH = CRATE / "docs" / "w607-plateau-affine-disjunction.json"

RAW_KILL = 250.0
DROP_KILL = 250.0
FUND_DROP = 1000.0
STRONG_DROP = 3000.0
MEANINGFUL_OBJECTIVE = 593500.0
KNOWN_MAX_LEAF_DROP = 317.3004031183664
REPACKAGE_MARGIN = 100.0


def graph_digest(edges, weights):
    payload = {
        "edges": [[int(a) + 1, int(b) + 1] for a, b in edges],
        "weights": [int(w) for w in weights],
    }
    return hashlib.sha256(json.dumps(payload, separators=(",", ":")).encode()).hexdigest()


def full_tree(edges, triads, weights, cuts, rows):
    leaves = []
    expanded = []

    def visit(fixed, depth):
        node = plateau.solve_node(edges, triads, weights, cuts, rows, fixed)
        node.update({"fixed": dict(fixed), "depth": depth})
        if not node["feasible"]:
            leaves.append(node)
            return
        branch = None
        if depth < plateau.MAX_DEPTH_TIER_A:
            branch = plateau.choose_branch(node["x"], weights, plateau.TIER_A, fixed)
        if branch is None:
            leaves.append(node)
            return
        expanded.append({**node, "branch_vertex": branch})
        for value in (0.0, 1.0):
            child_fixed = dict(fixed)
            child_fixed[branch] = value
            visit(child_fixed, depth + 1)

    visit({}, 0)
    return expanded, leaves


def semantic_mask(tier, fixed, adj):
    fixed_one = {v for v, value in fixed.items() if value == 1.0}
    fixed_zero = {v for v, value in fixed.items() if value == 0.0}
    neighbor_zero = set()
    verified_edges = []
    for included in fixed_one:
        for vertex in tier:
            if vertex in adj[included]:
                neighbor_zero.add(vertex)
                verified_edges.append([included + 1, vertex + 1])
    forced_zero = fixed_zero | neighbor_zero
    mask = []
    for vertex in tier:
        if vertex in fixed_one:
            mask.append(1.0)
        elif vertex in forced_zero:
            mask.append(0.0)
        else:
            mask.append(1.0)
    return {
        "fixed_one": sorted(v + 1 for v in fixed_one & set(tier)),
        "fixed_zero": sorted(v + 1 for v in fixed_zero & set(tier)),
        "neighbor_forced_zero": sorted(v + 1 for v in neighbor_zero - fixed_zero),
        "verified_include_edges": verified_edges,
        "mask": mask,
    }


def fit_affine_row(tier, leaves, adj, root_obj, root_x):
    masks = []
    for leaf in leaves:
        masks.append(semantic_mask(tier, leaf["fixed"], adj))
    variable_count = len(tier) + 1
    objective = np.zeros(variable_count)
    for i, vertex in enumerate(tier):
        objective[i] = -float(root_x[vertex])
    objective[-1] = 1.0
    matrix = []
    rhs = []
    for leaf, mask in zip(leaves, masks):
        row = np.zeros(variable_count)
        for i, value in enumerate(mask["mask"]):
            row[i] = value
        row[-1] = -1.0
        matrix.append(row)
        rhs.append(-float(leaf["upper"]))
    result = linprog(
        c=objective,
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        bounds=[(0, None)] * len(tier) + [(None, None)],
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    coeffs = result.x[: len(tier)]
    bound = result.x[-1]
    lhs_root = root_obj + sum(coeffs[i] * root_x[vertex] for i, vertex in enumerate(tier))
    tight = []
    for index, (leaf, mask) in enumerate(zip(leaves, masks)):
        value = float(leaf["upper"]) + sum(coeffs[i] * mask["mask"][i] for i in range(len(tier)))
        if abs(value - bound) <= 1e-5:
            tight.append(index)
    return coeffs, bound, lhs_root - bound, masks, tight


def row_from_affine(weights, tier, coeffs, bound):
    row_coeffs = {v: float(weights[v]) for v in range(parent.N)}
    for vertex, coeff in zip(tier, coeffs):
        if coeff > 1e-7:
            row_coeffs[vertex] += float(coeff)
    return row_coeffs, float(bound)


def leaf_summary(leaf, mask, coeffs, bound, index):
    value = float(leaf["upper"]) + sum(coeffs[i] * mask["mask"][i] for i in range(len(coeffs)))
    return {
        "leaf_index": index,
        "included": [v + 1 for v, value in sorted(leaf["fixed"].items()) if value == 1.0],
        "excluded": [v + 1 for v, value in sorted(leaf["fixed"].items()) if value == 0.0],
        "upper": leaf["upper"],
        "depth": leaf["depth"],
        "fixed_one": mask["fixed_one"],
        "fixed_zero": mask["fixed_zero"],
        "neighbor_forced_zero": mask["neighbor_forced_zero"],
        "charged_mask": mask["mask"],
        "affine_leaf_bound": value,
        "slack_to_B": bound - value,
    }


def tier_values(tier, x):
    return {str(v + 1): float(x[v]) for v in tier}


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "x"}
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
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = parent_lift.root_cuts(weights, adj)
    rows = [parent_lift.parent_row(weights), plateau.p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(edges, triads, weights, cuts, rows, solution=True)
    expanded, leaves = full_tree(edges, triads, weights, cuts, rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    tier = list(plateau.TIER_A)
    coeffs, bound, raw_violation, masks, tight = fit_affine_row(tier, finite, adj, root_obj, root_x)
    affine_row = row_from_affine(weights, tier, coeffs, bound)
    new_obj, new_x = branch_slack.solve_lp(edges, triads, weights, cuts, rows, cg_cuts=[{"coefficients": affine_row[0], "rhs": affine_row[1]}], solution=True)
    drop = root_obj - new_obj
    scalar_bound = max(leaf["upper"] for leaf in finite)
    scalar_row = ({v: float(weights[v]) for v in range(parent.N)}, scalar_bound)
    scalar_obj = branch_slack.solve_lp(edges, triads, weights, cuts, rows, cg_cuts=[{"coefficients": scalar_row[0], "rhs": scalar_row[1]}])
    scalar_drop = root_obj - scalar_obj
    status = "RetirePlateauAffineDisjunction"
    if raw_violation >= RAW_KILL and drop >= FUND_DROP and drop >= scalar_drop + REPACKAGE_MARGIN:
        status = "FundPlateauAffineLeafReplay"
    if drop >= STRONG_DROP or new_obj <= MEANINGFUL_OBJECTIVE:
        status = "StrongPlateauAffineDisjunction"
    leaf_rows = [leaf_summary(leaf, mask, coeffs, bound, i) for i, (leaf, mask) in enumerate(zip(finite, masks))]
    report = clean(
        {
            "schema": "forge.hadwiger.w607_plateau_affine_disjunction.v1",
            "authority": "diagnostic_affine_disjunctive_aggregation_not_replay_authority",
            "graph_digest": graph_digest(edges, weights),
            "row_system": "16_root_rank_rows_plus_projected_parent_lift_plus_branch_slack_parent_lift",
            "tier_a_vertices": [v + 1 for v in tier],
            "expanded_count": len(expanded),
            "leaf_count": len(finite),
            "root_objective": root_obj,
            "root_tier_values": tier_values(tier, root_x),
            "coefficients": {str(v + 1): coeff for v, coeff in zip(tier, coeffs) if coeff > 1e-7},
            "B": bound,
            "raw_violation": raw_violation,
            "new_objective": new_obj,
            "drop": drop,
            "new_tier_values": tier_values(tier, new_x),
            "scalar_max_leaf_bound": scalar_bound,
            "scalar_max_leaf_drop": scalar_drop,
            "known_max_leaf_drop": KNOWN_MAX_LEAF_DROP,
            "active_leaf_indices": tight,
            "gates": {
                "raw_kill": RAW_KILL,
                "drop_kill": DROP_KILL,
                "fund_drop": FUND_DROP,
                "strong_drop": STRONG_DROP,
                "meaningful_objective": MEANINGFUL_OBJECTIVE,
                "repackage_margin": REPACKAGE_MARGIN,
            },
            "leaves": leaf_rows,
            "failure_modes": [
                "leaf_bounds_not_full_parent_objective",
                "leaves_not_semantic_partition",
                "neighbor_zero_not_graph_verified",
                "signed_coefficients_used_without_exact_leaf_supremum",
                "diagnostic_aggregation_mistaken_for_replay_authority",
            ],
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
