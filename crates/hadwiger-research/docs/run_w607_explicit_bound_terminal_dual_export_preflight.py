import json

import numpy as np
from scipy.optimize import linprog

import run_w607_terminal_dual_provenance as provenance
from w607_selected_terminal_export_support import (
    ALLOWANCE,
    CANONICAL_DENOMINATOR_LIMIT,
    DENOMINATORS,
    EXPORT_GATE,
    NORMAL_ROW_BUDGET,
    REPLAY,
    SOURCE,
    TOTAL_ROW_BUDGET,
    TOP_LEAVES,
    digest,
    explicit_bound_rows,
    graph_digest,
    jsonable,
    row_budget,
    rounded_attempt,
    run_context,
    terminal_fixed,
)


OUT = provenance.CRATE / "docs" / "w607-explicit-bound-terminal-dual-export-preflight.json"
LEAF0_ROW_BUDGET = 2500


def explicit_terminal_rows(rows, rhs, fixed):
    rows, rhs = explicit_bound_rows(rows, rhs)
    rhs = [float(value) for value in rhs]
    for vertex, value in sorted(fixed.items()):
        if value == 0.0:
            rows.append({"family": "fixed_zero_literal", "id": f"x{vertex + 1}_fixed_zero", "vertices": (vertex,), "coeffs": (1.0,)})
            rhs.append(0.0)
        elif value == 1.0:
            rows.append({"family": "fixed_one_literal", "id": f"x{vertex + 1}_fixed_one", "vertices": (vertex,), "coeffs": (-1.0,)})
            rhs.append(-1.0)
        else:
            raise ValueError(f"unexpected fixed value {value}")
    return rows, np.array(rhs, dtype=float)


def solve_explicit(rows, rhs, weights):
    result = linprog(
        c=-weights.astype(float),
        A_ub=provenance.matrix_for(rows),
        b_ub=rhs,
        bounds=[(None, None)] * provenance.parent.N,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result


def solve_and_round(selection, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows):
    leaf_index = selection["leaf_index"]
    leaf = finite[leaf_index]
    fixed = terminal_fixed(selection, leaf)
    rows, rhs, first_rows = provenance.row_system_for_leaf(
        leaf_index, leaf, source_by_index[leaf_index], edges, triads, weights, adj, root_cuts, parent_rows
    )
    rows, rhs = explicit_terminal_rows(rows, rhs, fixed)
    objective, result = solve_explicit(rows, rhs, weights)
    duals = np.maximum(-np.array(result.ineqlin.marginals), 0.0)
    hidden_lower = np.maximum(np.array(result.lower.marginals), 0.0)
    hidden_upper = np.maximum(-np.array(result.upper.marginals), 0.0)
    hidden_max = float(max(np.max(hidden_lower), np.max(hidden_upper)))
    attempts, success = [], None
    for denominator in DENOMINATORS:
        attempt = rounded_attempt(rows, rhs, duals, weights, denominator, skip_lower=True)
        allowed = min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE)
        summary = {key: value for key, value in attempt.items() if key != "positive_rows"}
        summary["objective_over_expected"] = attempt["objective_bound"] - selection["expected_bound"]
        summary["passes"] = (
            attempt["min_slack"] >= 0
            and attempt["objective_bound"] <= allowed
            and attempt["positive_row_count"] <= row_budget(selection)
            and denominator <= CANONICAL_DENOMINATOR_LIMIT
        )
        attempts.append(summary)
        if selection["export_required"] and summary["passes"] and success is None:
            success = attempt
    error = objective - selection["expected_bound"]
    failures = [
        reason
        for reason, active in [
            ("terminal_reproduction_error", abs(error) > 1e-6),
            ("hidden_solver_bound_dual_nonzero", hidden_max > 1e-8),
            ("no_export_success", selection["export_required"] and success is None),
            ("row_budget_exceeded", success is not None and success["positive_row_count"] > row_budget(selection)),
        ]
        if active
    ]
    return {
        **selection,
        "fixed_literals_represented_as_explicit_rows": {str(v + 1): float(value) for v, value in sorted(fixed.items())},
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "explicit_row_count": len(rows)}),
        "floating_objective": objective,
        "expected_error": error,
        "hidden_solver_bound_dual_max": hidden_max,
        "hidden_solver_bound_dual_mass": float(np.sum(hidden_lower) + np.sum(hidden_upper)),
        "row_budget": row_budget(selection),
        "attempts": attempts,
        "success_summary": None if success is None else {key: value for key, value in success.items() if key != "positive_rows"},
        "success_row_kinds": success_kinds(success),
        "success_rows": [] if success is None else success["positive_rows"],
        "failure_reasons": failures,
        "status": "audit_only" if not selection["export_required"] else ("export_success" if not failures else "export_failed"),
    }


def success_kinds(success):
    counts = {}
    if success is None:
        return counts
    for row in success["positive_rows"]:
        counts[row["family"]] = counts.get(row["family"], 0) + 1
    return counts


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_and_round(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row["success_summary"]["positive_row_count"] for row in successes if row["success_summary"])
    high_den = [row["terminal_id"] for row in successes if row["success_summary"]["denominator"] > CANONICAL_DENOMINATOR_LIMIT]
    failures = collect_failures(required, successes, total_rows, high_den)
    report = {
        "schema": "forge.hadwiger.w607_explicit_bound_terminal_dual_export_preflight.v1",
        "authority": "selected_terminal_certificate_preflight_only_no_root_or_lift_authority",
        "second_opinion": {"agent": "Euclid", "decision": "approve_one_explicit_bound_row_preflight"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len(required), "audit_only_count": len(terminals) - len(required), "denominators": DENOMINATORS},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "required_total": len(required), "total_success_rows": total_rows, "high_denominator_successes": high_den, "worst_success_objective": max((row["success_summary"]["objective_bound"] for row in successes), default=None)},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_full_selected_terminal_box_row_export" if not failures else "retire_explicit_bound_terminal_export_preflight",
    }
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps(jsonable({key: value for key, value in report.items() if key != "terminals"}), indent=2))


def collect_failures(required, successes, total_rows, high_den):
    failures = [
        reason
        for reason, active in [
            ("not_all_required_terminals_exported", len(successes) != len(required)),
            ("total_row_budget_exceeded", total_rows > TOTAL_ROW_BUDGET),
            ("canonical_denominator_gate_failed", bool(high_den)),
        ]
        if active
    ]
    for row in required:
        failures.extend(f"{row['terminal_id']}:{reason}" for reason in row["failure_reasons"])
    return sorted(set(failures))


def source_binding(replay, source, edges, weights, root_cuts, parent_rows):
    return {"fresh_replay_path": str(REPLAY), "fresh_replay_digest": digest(replay), "first_family_source": str(SOURCE), "first_family_digest": digest(source), "graph_digest": graph_digest(edges, weights), "root_rows_digest": digest(root_cuts), "parent_rows_digest": digest(parent_rows)}


def gates():
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "canonical_denominator_limit": CANONICAL_DENOMINATOR_LIMIT, "normal_row_budget": NORMAL_ROW_BUDGET, "leaf0_row_budget": LEAF0_ROW_BUDGET, "total_row_budget": TOTAL_ROW_BUDGET, "lower_bound_rows": "reported_but_not_upward_rounded"}


if __name__ == "__main__":
    main()
