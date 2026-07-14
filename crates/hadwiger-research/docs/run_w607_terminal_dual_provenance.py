import hashlib
import json

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix

import run_w607_full16_micro_branch_stress as full16
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
REPLAY = CRATE / "docs" / "w607-fresh-mixed-branch-replay.json"
OUT = CRATE / "docs" / "w607-terminal-dual-provenance.json"

WINDOW = 1000.0
MAX_TERMINALS = 20
OBJECTIVE_TOL = 1e-5
POSITIVE_TOL = 1e-8
LOW_FIXED_RATIO = 0.35
HIGH_FIXED_RATIO = 0.65
MIN_COMMON_STRUCTURAL = 3


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


def fixed_from_pool(base_fixed, assignment):
    fixed = dict(base_fixed)
    fixed.update({int(vertex) - 1: float(value) for vertex, value in assignment.items()})
    return fixed


def selected_terminals(replay):
    cutoff = replay["final_mixed_max"] - WINDOW
    selected = []
    for leaf in replay["leaves"]:
        if leaf["leaf_index"] == 0:
            for closure_index, closure in enumerate(leaf["residual_closures"]):
                if closure["triggered"]:
                    for child_index, child in enumerate(closure["children"]):
                        selected.append(
                            {
                                "leaf_index": 0,
                                "terminal_id": f"closure_{closure_index}_child_{child_index}",
                                "expected_bound": float(child["bound"]),
                                "pool_assignment": child["pool_assignment"],
                                "selection_reason": "leaf0_residual_child",
                            }
                        )
                elif closure["closed_bound"] >= cutoff:
                    selected.append(
                        {
                            "leaf_index": 0,
                            "terminal_id": f"depth4_terminal_{closure_index}",
                            "expected_bound": float(closure["closed_bound"]),
                            "pool_assignment": closure["terminal"]["pool_assignment"],
                            "selection_reason": "within_window",
                        }
                    )
        else:
            for terminal_index, terminal in enumerate(leaf["terminal_certificates"]):
                if terminal["bound"] >= cutoff:
                    selected.append(
                        {
                            "leaf_index": leaf["leaf_index"],
                            "terminal_id": f"depth3_terminal_{terminal_index}",
                            "expected_bound": float(terminal["bound"]),
                            "pool_assignment": terminal["pool_assignment"],
                            "selection_reason": "within_window",
                        }
                    )
    selected.sort(key=lambda row: (-row["expected_bound"], row["leaf_index"], row["terminal_id"]))
    return selected[:MAX_TERMINALS]


def row_system_for_leaf(index, leaf, source_report, edges, triads, weights, adj, root_cuts, parent_rows):
    first_cuts, first_rows = full16.first_family_cuts(
        source_report, leaf, edges, triads, weights, adj, root_cuts, parent_rows
    )
    rows = []
    rhs = []
    for edge_index, (a, b) in enumerate(edges):
        rows.append({"family": "edges", "id": f"edge_{a + 1}_{b + 1}", "vertices": (a, b), "coeffs": None})
        rhs.append(1.0)
    for tri_index, triad in enumerate(triads):
        rows.append({"family": "triangles", "id": f"triangle_{tri_index}", "vertices": triad, "coeffs": None})
        rhs.append(1.0)
    for cut_index, (vertices, alpha) in enumerate(root_cuts):
        rows.append({"family": "root_rank", "id": f"root_rank_{cut_index}", "vertices": vertices, "coeffs": weights[list(vertices)]})
        rhs.append(float(alpha))
    for row_index, (coeffs, row_rhs) in enumerate(parent_rows):
        rows.append({"family": "parent_lifts", "id": f"parent_lift_{row_index}", "coeff_map": coeffs})
        rhs.append(float(row_rhs))
    for cut_index, (vertices, alpha) in enumerate(first_cuts):
        rows.append(
            {
                "family": "first_family_leaf_rows",
                "id": f"leaf{index}_first_family_{cut_index}",
                "vertices": vertices,
                "coeffs": weights[list(vertices)],
                "source": first_rows[cut_index],
            }
        )
        rhs.append(float(alpha))
    return rows, np.array(rhs, dtype=float), first_rows


def matrix_for(rows):
    matrix = lil_matrix((len(rows), parent.N), dtype=float)
    for row_index, row in enumerate(rows):
        if "coeff_map" in row:
            for vertex, coeff in row["coeff_map"].items():
                matrix[row_index, vertex] = float(coeff)
        elif row["coeffs"] is None:
            for vertex in row["vertices"]:
                matrix[row_index, vertex] = 1.0
        else:
            for vertex, coeff in zip(row["vertices"], row["coeffs"]):
                matrix[row_index, vertex] = float(coeff)
    return matrix.tocsr()


def solve_terminal(rows, rhs, weights, fixed):
    matrix = matrix_for(rows)
    bounds = [(0.0, 1.0)] * parent.N
    for vertex, value in fixed.items():
        bounds[vertex] = (float(value), float(value))
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix,
        b_ub=rhs,
        bounds=bounds,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result


def row_contributions(rows, rhs, result):
    multipliers = np.maximum(-np.array(result.ineqlin.marginals), 0.0)
    out = []
    for index, value in enumerate(multipliers):
        if value <= POSITIVE_TOL:
            continue
        out.append(
            {
                "family": rows[index]["family"],
                "id": rows[index]["id"],
                "marginal": float(value),
                "rhs": float(rhs[index]),
                "objective_contribution": float(value * rhs[index]),
            }
        )
    return out


def bound_contributions(result, fixed):
    lower = np.maximum(np.array(result.lower.marginals), 0.0)
    upper = np.maximum(-np.array(result.upper.marginals), 0.0)
    out = []
    for vertex, value in fixed.items():
        if value == 0.0 and lower[vertex] > POSITIVE_TOL:
            out.append(
                {
                    "family": "fixed_bounds_literals",
                    "id": f"x{vertex + 1}_lower_fixed_0",
                    "marginal": float(lower[vertex]),
                    "rhs": 0.0,
                    "objective_contribution": 0.0,
                }
            )
        if value == 1.0 and upper[vertex] > POSITIVE_TOL:
            out.append(
                {
                    "family": "fixed_bounds_literals",
                    "id": f"x{vertex + 1}_upper_fixed_1",
                    "marginal": float(upper[vertex]),
                    "rhs": 1.0,
                    "objective_contribution": float(upper[vertex]),
                }
            )
    return out


def summarize_terminal(selection, objective, result, rows, rhs, fixed, first_rows):
    row_duals = row_contributions(rows, rhs, result)
    bound_duals = bound_contributions(result, fixed)
    all_duals = row_duals + bound_duals
    mass_by_family = {}
    counts_by_family = {}
    for item in all_duals:
        family = item["family"]
        mass_by_family[family] = mass_by_family.get(family, 0.0) + item["objective_contribution"]
        counts_by_family[family] = counts_by_family.get(family, 0) + 1
    total_mass = sum(max(0.0, item["objective_contribution"]) for item in all_duals)
    fixed_mass = mass_by_family.get("fixed_bounds_literals", 0.0)
    top_rows = sorted(all_duals, key=lambda row: -abs(row["objective_contribution"]))[:12]
    return {
        **selection,
        "fixed_literals": {str(vertex + 1): value for vertex, value in sorted(fixed.items())},
        "objective": objective,
        "expected_bound": selection["expected_bound"],
        "reproduction_error": objective - selection["expected_bound"],
        "first_family_rows_used": first_rows,
        "positive_dual_counts_by_family": counts_by_family,
        "positive_dual_mass_by_family": mass_by_family,
        "fixed_bound_mass_ratio": 0.0 if total_mass <= POSITIVE_TOL else fixed_mass / total_mass,
        "top_dual_rows": top_rows,
        "positive_structural_row_ids": [
            item["id"]
            for item in row_duals
            if item["family"] != "fixed_bounds_literals" and item["objective_contribution"] > POSITIVE_TOL
        ],
    }


def common_structural_rows(summaries):
    sets = [set(row["positive_structural_row_ids"]) for row in summaries]
    common = set.intersection(*sets) if sets else set()
    return sorted(common)


def main():
    replay = json.loads(REPLAY.read_text())
    source = json.loads(SOURCE.read_text())
    source_by_index = {row["leaf_index"]: row for row in source["leaves"]}
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    selections = selected_terminals(replay)
    summaries = []
    for selection in selections:
        leaf_index = selection["leaf_index"]
        leaf = finite[leaf_index]
        base_fixed = full16.fixed_from_leaf(leaf)
        fixed = fixed_from_pool(base_fixed, selection["pool_assignment"])
        rows, rhs, first_rows = row_system_for_leaf(
            leaf_index,
            leaf,
            source_by_index[leaf_index],
            edges,
            triads,
            weights,
            adj,
            root_cuts,
            parent_rows,
        )
        objective, result = solve_terminal(rows, rhs, weights, fixed)
        summaries.append(summarize_terminal(selection, objective, result, rows, rhs, fixed, first_rows))
    max_error = max((abs(row["reproduction_error"]) for row in summaries), default=0.0)
    aggregate_mass = {}
    aggregate_counts = {}
    for row in summaries:
        for family, value in row["positive_dual_mass_by_family"].items():
            aggregate_mass[family] = aggregate_mass.get(family, 0.0) + value
        for family, value in row["positive_dual_counts_by_family"].items():
            aggregate_counts[family] = aggregate_counts.get(family, 0) + value
    total_mass = sum(aggregate_mass.values())
    fixed_ratio = 0.0 if total_mass <= POSITIVE_TOL else aggregate_mass.get("fixed_bounds_literals", 0.0) / total_mass
    common = common_structural_rows(summaries)
    conclusion = "dual_degeneracy_inconclusive"
    if max_error <= OBJECTIVE_TOL and fixed_ratio <= LOW_FIXED_RATIO and len(common) >= MIN_COMMON_STRUCTURAL:
        conclusion = "export_promising_structural_core"
    elif max_error <= OBJECTIVE_TOL and fixed_ratio >= HIGH_FIXED_RATIO:
        conclusion = "literal_bound_dominated"
    report = {
        "schema": "forge.hadwiger.w607_terminal_dual_provenance.v1",
        "authority": "diagnostic_dual_provenance_not_export_certificate",
        "second_opinion": {
            "agent": "Kant",
            "decision": "approve_narrowly",
            "primary_failure_mode": "dual_nonuniqueness",
        },
        "source_binding": {
            "fresh_replay_path": str(REPLAY),
            "fresh_replay_digest": digest(replay),
            "first_family_source": str(SOURCE),
            "first_family_digest": digest(source),
        },
        "selection_logic": {
            "window_below_mixed_max": WINDOW,
            "max_terminals": MAX_TERMINALS,
            "selected_count": len(selections),
            "mixed_max": replay["final_mixed_max"],
            "includes": "all terminals within window plus leaf0 residual children",
        },
        "gates": {
            "objective_tolerance": OBJECTIVE_TOL,
            "low_fixed_ratio": LOW_FIXED_RATIO,
            "high_fixed_ratio": HIGH_FIXED_RATIO,
            "min_common_structural_rows": MIN_COMMON_STRUCTURAL,
        },
        "aggregate_positive_dual_mass_by_family": aggregate_mass,
        "aggregate_positive_dual_counts_by_family": aggregate_counts,
        "aggregate_fixed_bound_mass_ratio": fixed_ratio,
        "common_structural_row_ids": common,
        "max_reproduction_error": max_error,
        "terminal_summaries": summaries,
        "failure_reasons": [
            reason
            for reason, active in [
                ("reproduction_error_above_tolerance", max_error > OBJECTIVE_TOL),
                ("fixed_bound_mass_high", fixed_ratio >= HIGH_FIXED_RATIO),
                ("common_structural_core_too_small", len(common) < MIN_COMMON_STRUCTURAL),
            ]
            if active
        ],
        "conclusion": conclusion,
    }
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "terminal_summaries"}, indent=2))


if __name__ == "__main__":
    main()
