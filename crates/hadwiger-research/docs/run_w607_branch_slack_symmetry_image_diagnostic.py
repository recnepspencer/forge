import hashlib
import json

import numpy as np

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent
import run_w607_weighted_rank_pattern_preflight as rank_pattern


CRATE = parent.CRATE
OUT_PATH = CRATE / "docs" / "w607-branch-slack-symmetry-image-diagnostic.json"

SINGLE_FUND_DROP = 1000.0
BATCH_FUND_DROP = 3000.0
SINGLE_KILL_DROP = 250.0
MEANINGFUL_OBJECTIVE = 593500.0
CONTINUATION_OBJECTIVE = 590000.0
SYMMETRY_TOL = 1e-8


def row_hash(coeffs, rhs):
    items = tuple(sorted((int(v), int(c)) for v, c in coeffs.items() if c))
    return hashlib.sha256(repr((items, int(rhs))).encode()).hexdigest()


def float_row_hash(coeffs, rhs):
    items = tuple(sorted((int(v), int(round(c))) for v, c in coeffs.items() if abs(c) > 1e-9))
    return hashlib.sha256(repr((items, int(round(rhs)))).encode()).hexdigest()


def transform_row(coeffs, rhs, permutation):
    image = {}
    for vertex, coeff in coeffs.items():
        target = permutation[vertex]
        image[target] = image.get(target, 0) + coeff
    return {vertex: coeff for vertex, coeff in image.items() if coeff}, rhs


def normalize_int_row(row):
    coeffs, rhs = row
    return {int(v): int(c) for v, c in coeffs.items() if c}, int(rhs)


def normalize_float_integer_row(row):
    coeffs, rhs = row
    return {int(v): int(round(c)) for v, c in coeffs.items() if abs(c) > 1e-9}, int(round(rhs))


def exact_relation(left, right):
    left_coeffs, left_rhs = left
    right_coeffs, right_rhs = right
    all_vertices = set(left_coeffs) | set(right_coeffs)
    identical = left_rhs == right_rhs and all(left_coeffs.get(v, 0) == right_coeffs.get(v, 0) for v in all_vertices)
    left_dominates = left_rhs <= right_rhs and all(left_coeffs.get(v, 0) >= right_coeffs.get(v, 0) for v in all_vertices)
    right_dominates = right_rhs <= left_rhs and all(right_coeffs.get(v, 0) >= left_coeffs.get(v, 0) for v in all_vertices)
    ratios = []
    for vertex in all_vertices:
        a = left_coeffs.get(vertex, 0)
        b = right_coeffs.get(vertex, 0)
        if a == 0 and b == 0:
            continue
        if a == 0 or b == 0:
            return {
                "identical": identical,
                "scalar_multiple": False,
                "left_dominates_right": left_dominates,
                "right_dominates_left": right_dominates,
            }
        ratios.append((a, b))
    scalar = bool(ratios) and all(a * ratios[0][1] == ratios[0][0] * b for a, b in ratios)
    if scalar:
        scalar = left_rhs * ratios[0][1] == ratios[0][0] * right_rhs
    return {
        "identical": identical,
        "scalar_multiple": scalar,
        "left_dominates_right": left_dominates,
        "right_dominates_left": right_dominates,
    }


def activity(row, x):
    coeffs, rhs = row
    lhs = sum(float(coeff) * float(x[vertex]) for vertex, coeff in coeffs.items())
    return {"lhs": lhs, "rhs": float(rhs), "slack": float(rhs) - lhs}


def symmetry_report(weights, x, permutation):
    diffs = [abs(float(x[v]) - float(x[permutation[v]])) for v in range(parent.N)]
    weighted = [float(weights[v]) * diffs[v] for v in range(parent.N)]
    worst = max(range(parent.N), key=lambda v: diffs[v])
    worst_weighted = max(range(parent.N), key=lambda v: weighted[v])
    return {
        "max_abs_diff": diffs[worst],
        "max_abs_diff_pair": [worst + 1, permutation[worst] + 1],
        "max_weighted_diff": weighted[worst_weighted],
        "max_weighted_diff_pair": [worst_weighted + 1, permutation[worst_weighted] + 1],
        "symmetric_within_tol": max(diffs, default=0.0) <= SYMMETRY_TOL,
    }


def changed_variables(weights, before, after, permutation, limit=16):
    changes = []
    seen = set()
    for vertex in range(parent.N):
        mate = permutation[vertex]
        key = tuple(sorted((vertex, mate)))
        if key in seen:
            continue
        seen.add(key)
        delta = abs(float(after[vertex]) - float(before[vertex]))
        if mate != vertex:
            delta = max(delta, abs(float(after[mate]) - float(before[mate])))
        changes.append(
            {
                "orbit": [v + 1 for v in key],
                "weight_sum": float(sum(weights[v] for v in key)),
                "before": [float(before[v]) for v in key],
                "after": [float(after[v]) for v in key],
                "max_delta": delta,
            }
        )
    return sorted(changes, key=lambda row: (-row["max_delta"], -row["weight_sum"], row["orbit"]))[:limit]


def permutation_checks(edges, weights, permutation):
    edge_set = {tuple(sorted(edge)) for edge in edges}
    mapped_edges = {
        tuple(sorted((permutation[left], permutation[right])))
        for left, right in edges
    }
    weight_preserved = [v for v in range(parent.N) if int(weights[v]) == int(weights[permutation[v]])]
    fixed = [v for v in range(parent.N) if permutation[v] == v]
    return {
        "is_permutation": sorted(permutation) == list(range(parent.N)),
        "edge_count": len(edge_set),
        "mapped_edge_count": len(mapped_edges),
        "preserved_edge_count": len(edge_set & mapped_edges),
        "edge_preserving": edge_set == mapped_edges,
        "weight_preserved_count": len(weight_preserved),
        "weight_preserving": len(weight_preserved) == parent.N,
        "fixed_vertices": [v + 1 for v in fixed],
        "branch_vertex": parent.BRANCH + 1,
        "branch_fixed": permutation[parent.BRANCH] == parent.BRANCH,
    }


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
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    cuts = parent_lift.root_cuts(weights, adj)
    permutation = rank_pattern.half_turn(rank_pattern.parse_vertices())

    projected = normalize_float_integer_row(parent_lift.parent_row(weights))
    branch = normalize_int_row(branch_slack.p_parent_row(weights))
    projected_image = normalize_int_row(transform_row(*projected, permutation))
    branch_image = normalize_int_row(transform_row(*branch, permutation))

    baseline_rows = [projected, branch]
    baseline_obj, baseline_x = branch_slack.solve_lp(edges, triads, weights, cuts, baseline_rows, solution=True)
    projected_obj, projected_x = branch_slack.solve_lp(
        edges, triads, weights, cuts, [projected, branch, projected_image], solution=True
    )
    branch_obj, branch_x = branch_slack.solve_lp(
        edges, triads, weights, cuts, [projected, branch, branch_image], solution=True
    )
    both_obj, both_x = branch_slack.solve_lp(
        edges, triads, weights, cuts, [projected, branch, projected_image, branch_image], solution=True
    )

    projected_drop = baseline_obj - projected_obj
    branch_drop = baseline_obj - branch_obj
    both_drop = baseline_obj - both_obj
    branch_relation = exact_relation(branch, branch_image)
    projected_relation = exact_relation(projected, projected_image)
    duplicate_or_invariant = branch_relation["identical"] and projected_relation["identical"]
    status = "RetireSymmetryImageRows"
    if (
        not duplicate_or_invariant
        and (
            branch_drop >= SINGLE_FUND_DROP
            or both_drop >= BATCH_FUND_DROP
            or both_obj <= MEANINGFUL_OBJECTIVE
            or both_obj <= CONTINUATION_OBJECTIVE
        )
    ):
        status = "FundSymmetryImageFollowup"
    if duplicate_or_invariant or max(projected_drop, branch_drop, both_drop) < SINGLE_KILL_DROP:
        status = "RetireSymmetryImageRows"

    report = clean(
        {
            "schema": "forge.hadwiger.w607_branch_slack_symmetry_image_diagnostic.v1",
            "hypothesis": "automorphism images of replayed parent rows are valid by half-turn transport of final inequalities",
            "authority_note": "No new gamma certificate exported; validity is inherited only by automorphism of already replayed parent inequalities.",
            "second_agent_recommendation": "run_bounded_diagnostic_with_low_patience",
            "permutation_checks": permutation_checks(edges, weights, permutation),
            "gates": {
                "single_fund_drop": SINGLE_FUND_DROP,
                "batch_fund_drop": BATCH_FUND_DROP,
                "single_kill_drop": SINGLE_KILL_DROP,
                "meaningful_objective": MEANINGFUL_OBJECTIVE,
                "continuation_objective": CONTINUATION_OBJECTIVE,
            },
            "row_hashes": {
                "projected": float_row_hash(*projected),
                "projected_image": row_hash(*projected_image),
                "branch_slack": row_hash(*branch),
                "branch_slack_image": row_hash(*branch_image),
            },
            "row_relations": {
                "projected_vs_image": projected_relation,
                "branch_slack_vs_image": branch_relation,
            },
            "baseline": {
                "objective": baseline_obj,
                "x304": baseline_x[parent.BRANCH],
                "solution_symmetry": symmetry_report(weights, baseline_x, permutation),
                "projected_activity": activity(projected, baseline_x),
                "projected_image_activity": activity(projected_image, baseline_x),
                "branch_slack_activity": activity(branch, baseline_x),
                "branch_slack_image_activity": activity(branch_image, baseline_x),
            },
            "objectives": {
                "projected_image_only": projected_obj,
                "branch_slack_image_only": branch_obj,
                "both_images": both_obj,
            },
            "drops": {
                "projected_image_only": projected_drop,
                "branch_slack_image_only": branch_drop,
                "both_images": both_drop,
            },
            "post_solution_symmetry": {
                "projected_image_only": symmetry_report(weights, projected_x, permutation),
                "branch_slack_image_only": symmetry_report(weights, branch_x, permutation),
                "both_images": symmetry_report(weights, both_x, permutation),
            },
            "top_changed_orbits": {
                "projected_image_only": changed_variables(weights, baseline_x, projected_x, permutation),
                "branch_slack_image_only": changed_variables(weights, baseline_x, branch_x, permutation),
                "both_images": changed_variables(weights, baseline_x, both_x, permutation),
            },
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in {"top_changed_orbits"}}, indent=2))


if __name__ == "__main__":
    main()
