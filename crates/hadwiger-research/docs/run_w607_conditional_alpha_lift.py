import hashlib
import json
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, linprog, milp

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_full_tree_rank_family as full_family
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-conditional-alpha-lift.json"

TIER_A = [304, 223, 384, 302, 222, 383]
BETA_TIME_LIMIT = 5.0
SINGLE_DROP_KILL = 250.0
BUNDLE_DROP_KILL = 1000.0
FUND_DROP = 3000.0
FUND_OBJECTIVE = 592000.0
EPS = 1e-7


def graph_digest(edges, weights):
    payload = {
        "edges": [[int(a) + 1, int(b) + 1] for a, b in edges],
        "weights": [int(weight) for weight in weights],
    }
    return hashlib.sha256(json.dumps(payload, separators=(",", ":")).encode()).hexdigest()


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def fixed_from_artifact_leaf(leaf):
    fixed = {}
    for vertex in leaf["included"]:
        fixed[vertex - 1] = 1.0
    for vertex in leaf["excluded"]:
        fixed[vertex - 1] = 0.0
    return fixed


def fixed_from_tree_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def accepted_sources():
    source = json.loads(SOURCE.read_text())
    out = []
    for leaf in source["leaves"]:
        for row in leaf["accepted_rows"]:
            out.append({**row, "source_leaf_index": leaf["leaf_index"], "source_leaf": leaf})
    return out


def reconstruct_rows(edges, triads, weights, adj, root_cuts, rows):
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    by_digest = {}
    for leaf_index, leaf in enumerate(finite):
        fixed = fixed_from_tree_leaf(leaf)
        _base_obj, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, rows, fixed, True)
        for candidate in full_family.candidate_rows(weights, x, fixed, adj):
            digest = support_hash(candidate["vertices"])
            by_digest[(leaf_index, digest)] = {
                "vertices": candidate["vertices"],
                "template_id": candidate["template_id"],
                "center": candidate["center"] + 1,
            }
    accepted = []
    for source in accepted_sources():
        key = (source["source_leaf_index"], source["support_digest"])
        reconstructed = by_digest.get(key)
        if reconstructed is None:
            raise ValueError(f"could not reconstruct accepted support {key}")
        accepted.append({**source, **reconstructed})
    return accepted, finite


def leaf_decisions(fixed):
    out = {}
    for vertex in TIER_A:
        if vertex in fixed:
            out[vertex] = int(round(fixed[vertex]))
    return out


def mismatch_mask(source_fixed, target_fixed):
    source = leaf_decisions(source_fixed)
    target = leaf_decisions(target_fixed)
    mask = []
    for vertex in TIER_A:
        if vertex in source and vertex in target and source[vertex] != target[vertex]:
            mask.append(1.0)
        else:
            mask.append(0.0)
    return mask


def support_beta(vertices, weights, adj, fixed):
    vertex_set = set(vertices)
    included = [v for v, value in fixed.items() if value == 1.0 and v in vertex_set]
    excluded = {v for v, value in fixed.items() if value == 0.0 and v in vertex_set}
    for i, a in enumerate(included):
        for b in included[i + 1 :]:
            if b in adj[a]:
                return {
                    "success": True,
                    "feasible": False,
                    "beta": float("-inf"),
                    "mip_gap": 0.0,
                    "seconds": 0.0,
                    "reason": "adjacent_included_support_vertices",
                }
    forced_zero = set(excluded)
    for vertex in included:
        forced_zero.update(adj[vertex] & vertex_set)
    active = [v for v in vertices if v not in included and v not in forced_zero]
    if not active:
        return {
            "success": True,
            "feasible": True,
            "beta": int(sum(weights[v] for v in included)),
            "mip_gap": 0.0,
            "seconds": 0.0,
            "included_in_support": [v + 1 for v in included],
            "forced_zero_in_support": [v + 1 for v in sorted(forced_zero)],
        }
    local = {v: i for i, v in enumerate(active)}
    edge_rows = []
    for i, a in enumerate(active):
        for b in active[i + 1 :]:
            if b in adj[a]:
                row = np.zeros(len(active))
                row[i] = 1.0
                row[local[b]] = 1.0
                edge_rows.append(row)
    constraints = LinearConstraint(np.vstack(edge_rows), -np.inf, np.ones(len(edge_rows))) if edge_rows else None
    start = time.time()
    result = milp(
        c=-weights[active],
        integrality=np.ones(len(active)),
        bounds=Bounds(np.zeros(len(active)), np.ones(len(active))),
        constraints=constraints,
        options={"time_limit": BETA_TIME_LIMIT, "mip_rel_gap": 0.0},
    )
    seconds = time.time() - start
    gap = getattr(result, "mip_gap", None)
    success = bool(result.success and (gap is None or gap <= 1e-9))
    beta = int(round(-result.fun + sum(weights[v] for v in included))) if success else None
    return {
        "success": success,
        "feasible": bool(success),
        "beta": beta,
        "mip_gap": gap,
        "seconds": seconds,
        "included_in_support": [v + 1 for v in included],
        "forced_zero_in_support": [v + 1 for v in sorted(forced_zero)],
        "active_support_size": len(active),
    }


def fit_charges(beta_source, beta_rows, source_fixed):
    objective = np.ones(len(TIER_A))
    matrix = []
    rhs = []
    for row in beta_rows:
        if not row["success"] or not row["feasible"]:
            continue
        excess = float(row["beta"] - beta_source)
        if excess <= 0.0:
            continue
        mask = mismatch_mask(source_fixed, row["fixed"])
        if not any(mask):
            return None, "positive_excess_without_visible_mismatch"
        matrix.append([-value for value in mask])
        rhs.append(-excess)
    if not matrix:
        return np.zeros(len(TIER_A)), "no_positive_excess"
    result = linprog(
        c=objective,
        A_ub=np.array(matrix),
        b_ub=np.array(rhs),
        bounds=[(0.0, None)] * len(TIER_A),
        method="highs",
    )
    if not result.success:
        return None, result.message
    return result.x, "ok"


def row_coefficients(row, charges):
    coeffs = {vertex: float(row["weights"][vertex]) for vertex in row["vertices"]}
    rhs = float(row["beta_source"])
    source_fixed = row["source_fixed"]
    for vertex, charge in zip(TIER_A, charges):
        if charge <= EPS or vertex not in source_fixed:
            continue
        if source_fixed[vertex] == 0.0:
            coeffs[vertex] = coeffs.get(vertex, 0.0) - float(charge)
        else:
            coeffs[vertex] = coeffs.get(vertex, 0.0) + float(charge)
            rhs += float(charge)
    return coeffs, rhs


def row_root_violation(row, charges, root_x):
    coeffs, rhs = row_coefficients(row, charges)
    lhs = sum(coeff * root_x[vertex] for vertex, coeff in coeffs.items())
    return lhs - rhs


def solve_with_cg(edges, triads, weights, root_cuts, parent_rows, cg_rows):
    formatted = [{"coefficients": coeffs, "rhs": rhs} for coeffs, rhs in cg_rows]
    return branch_slack.solve_lp(
        edges,
        triads,
        weights,
        root_cuts,
        parent_rows,
        cg_cuts=formatted,
        solution=True,
    )


def analyze_row(source, leaves, weights, adj, root_x):
    source_fixed = fixed_from_artifact_leaf(source["source_leaf"])
    beta_rows = []
    for leaf_index, leaf in enumerate(leaves):
        fixed = fixed_from_tree_leaf(leaf)
        result = support_beta(source["vertices"], weights, adj, fixed)
        beta_rows.append(
            {
                **result,
                "leaf_index": leaf_index,
                "fixed": fixed,
                "included": [v + 1 for v, value in sorted(fixed.items()) if value == 1.0],
                "excluded": [v + 1 for v, value in sorted(fixed.items()) if value == 0.0],
                "mismatch_mask": mismatch_mask(source_fixed, fixed),
            }
        )
    source_beta_row = beta_rows[source["source_leaf_index"]]
    if not source_beta_row["success"] or not source_beta_row["feasible"]:
        charges = None
        charge_status = "source_beta_not_solved"
    elif any(not row["success"] for row in beta_rows):
        charges = None
        charge_status = "inexact_beta"
    else:
        charges, charge_status = fit_charges(source_beta_row["beta"], beta_rows, source_fixed)
    base = {
        "source_leaf_index": source["source_leaf_index"],
        "template_id": source["template_id"],
        "center": source["center"],
        "support_digest": source["support_digest"],
        "size": len(source["vertices"]),
        "global_alpha_w": int(source["alpha_w"]),
        "source_fixed": source_fixed,
        "beta_source": source_beta_row["beta"] if source_beta_row["success"] else None,
        "beta_reduction_vs_global_alpha": (
            int(source["alpha_w"]) - source_beta_row["beta"]
            if source_beta_row["success"] and source_beta_row["feasible"]
            else None
        ),
        "all_betas_exact": all(row["success"] for row in beta_rows),
        "charge_status": charge_status,
        "beta_rows": beta_rows,
        "weights": weights,
        "vertices": source["vertices"],
    }
    if charges is None:
        return {**base, "admissible": False}
    return {
        **base,
        "admissible": True,
        "charges": charges,
        "charges_by_tier_a": {
            str(vertex + 1): float(charge) for vertex, charge in zip(TIER_A, charges) if charge > EPS
        },
        "root_violation": row_root_violation({**base, "weights": weights}, charges, root_x),
    }


def source_beta_precheck(source, weights, adj):
    source_fixed = fixed_from_artifact_leaf(source["source_leaf"])
    result = support_beta(source["vertices"], weights, adj, source_fixed)
    reduction = (
        int(source["alpha_w"]) - result["beta"]
        if result["success"] and result["feasible"]
        else None
    )
    return {
        **result,
        "source_leaf_index": source["source_leaf_index"],
        "template_id": source["template_id"],
        "center": source["center"],
        "support_digest": source["support_digest"],
        "size": len(source["vertices"]),
        "global_alpha_w": int(source["alpha_w"]),
        "beta_reduction_vs_global_alpha": reduction,
    }


def clean(value):
    if isinstance(value, dict):
        return {
            key: clean(inner)
            for key, inner in value.items()
            if key not in {"weights", "vertices", "fixed"}
        }
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
    accepted, leaves = reconstruct_rows(edges, triads, weights, adj, root_cuts, parent_rows)
    prechecks = [source_beta_precheck(row, weights, adj) for row in accepted]
    rows_to_lift = [
        row
        for row, precheck in zip(accepted, prechecks)
        if precheck["success"] and precheck["feasible"] and precheck["beta_reduction_vs_global_alpha"] > 0
    ]
    row_reports = [analyze_row(row, leaves, weights, adj, root_x) for row in rows_to_lift]
    for precheck in prechecks:
        if precheck["support_digest"] not in {row["support_digest"] for row in row_reports}:
            row_reports.append(
                {
                    **precheck,
                    "admissible": False,
                    "all_betas_exact": False,
                    "charge_status": "no_source_beta_reduction_or_source_inexact",
                    "beta_source": precheck["beta"] if precheck["success"] else None,
                }
            )
    admissible = [row for row in row_reports if row.get("admissible")]
    single_reports = []
    for row in admissible:
        coeffs, rhs = row_coefficients(row, row["charges"])
        obj, x = solve_with_cg(edges, triads, weights, root_cuts, parent_rows, [(coeffs, rhs)])
        single_reports.append(
            {
                "support_digest": row["support_digest"],
                "source_leaf_index": row["source_leaf_index"],
                "root_violation": row["root_violation"],
                "objective": obj,
                "drop": root_obj - obj,
                "x304": float(x[parent.BRANCH]),
            }
        )
    all_rows = [row_coefficients(row, row["charges"]) for row in admissible]
    if all_rows:
        bundle_obj, bundle_x = solve_with_cg(edges, triads, weights, root_cuts, parent_rows, all_rows)
    else:
        bundle_obj, bundle_x = root_obj, root_x
    best_single = min(single_reports, key=lambda row: row["objective"]) if single_reports else None
    bundle_drop = root_obj - bundle_obj
    any_beta_reduction = any((row.get("beta_reduction_vs_global_alpha") or 0) > 0 for row in row_reports)
    any_root_violation = any(row.get("root_violation", 0.0) > 1e-6 for row in admissible)
    status = "RetireConditionalAlphaLift"
    if bundle_drop >= FUND_DROP or bundle_obj <= FUND_OBJECTIVE:
        status = "FundConditionalAlphaLiftReplayDesign"
    elif best_single and best_single["drop"] >= SINGLE_DROP_KILL:
        status = "KeepDiagnosticOnlyConditionalAlphaLift"
    elif bundle_drop >= BUNDLE_DROP_KILL:
        status = "KeepDiagnosticOnlyConditionalAlphaLiftBundle"
    failure = "funded" if status == "FundConditionalAlphaLiftReplayDesign" else "charge_eats_violation"
    if not any_beta_reduction:
        failure = "no_conditional_alpha_reduction"
    elif not admissible:
        failure = "uncertified_or_unchargeable_beta"
    elif not any_root_violation:
        failure = "lifted_rows_not_violated_at_root"
    elif bundle_drop < BUNDLE_DROP_KILL:
        failure = "lp_redundant"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_conditional_alpha_lift.v1",
            "authority": "diagnostic_conditional_face_support_mwis_no_replay_authority",
            "graph_digest": graph_digest(edges, weights),
            "row_system": "16_root_rank_rows_plus_projected_parent_lift_plus_branch_slack_parent_lift",
            "source_artifact": str(SOURCE),
            "tier_a_vertices": [v + 1 for v in TIER_A],
            "literal_convention": (
                "charges use only six Tier-A branch variables; source excluded literals use x_t, "
                "source included literals use 1-x_t"
            ),
            "root_objective_before": root_obj,
            "root_x304_before": float(root_x[parent.BRANCH]),
            "accepted_support_count": len(accepted),
            "admissible_row_count": len(admissible),
            "root_objective_after_bundle": bundle_obj,
            "root_x304_after_bundle": float(bundle_x[parent.BRANCH]),
            "bundle_drop": bundle_drop,
            "best_single": best_single,
            "gates": {
                "beta_time_limit": BETA_TIME_LIMIT,
                "single_drop_kill": SINGLE_DROP_KILL,
                "bundle_drop_kill": BUNDLE_DROP_KILL,
                "fund_drop": FUND_DROP,
                "fund_objective": FUND_OBJECTIVE,
            },
            "failure_classification": failure,
            "single_rows": single_reports,
            "rows": row_reports,
            "status": status,
        }
    )
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key not in {"rows"}}, indent=2))


if __name__ == "__main__":
    main()
