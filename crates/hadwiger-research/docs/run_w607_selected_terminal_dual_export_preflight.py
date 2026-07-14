import json

import numpy as np

import run_w607_terminal_dual_provenance as provenance
from w607_selected_terminal_export_support import (
    ALLOWANCE,
    BOUND_DUAL_TOL,
    CANONICAL_DENOMINATOR_LIMIT,
    DENOMINATORS,
    EXPORT_GATE,
    NORMAL_ROW_BUDGET,
    REPLAY,
    SOURCE,
    TOTAL_ROW_BUDGET,
    TOP_LEAVES,
    digest,
    graph_digest,
    jsonable,
    row_budget,
    rounded_attempt,
    run_context,
    terminal_fixed,
)


OUT = provenance.CRATE / "docs" / "w607-selected-terminal-dual-export-preflight.json"
LEAF0_ROW_BUDGET = 2500


def solve_and_round(selection, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows):
    leaf_index = selection["leaf_index"]
    leaf = finite[leaf_index]
    fixed = terminal_fixed(selection, leaf)
    rows, rhs, first_rows = provenance.row_system_for_leaf(
        leaf_index, leaf, source_by_index[leaf_index], edges, triads, weights, adj, root_cuts, parent_rows
    )
    objective, result = provenance.solve_terminal(rows, rhs, weights, fixed)
    row_duals = np.maximum(-np.array(result.ineqlin.marginals), 0.0)
    lower = np.maximum(np.array(result.lower.marginals), 0.0)
    upper = np.maximum(-np.array(result.upper.marginals), 0.0)
    fixed_max = 0.0
    fixed_mass = 0.0
    all_bound_max = float(max(np.max(lower), np.max(upper)))
    all_bound_mass = float(np.sum(lower) + np.sum(upper))
    positive_bound_count = int(np.count_nonzero(lower > BOUND_DUAL_TOL) + np.count_nonzero(upper > BOUND_DUAL_TOL))
    for vertex, value in fixed.items():
        if value == 0.0:
            fixed_max = max(fixed_max, float(lower[vertex]))
        if value == 1.0:
            fixed_mass += float(upper[vertex])
            fixed_max = max(fixed_max, float(upper[vertex]))
    attempts, success = [], None
    for denominator in DENOMINATORS:
        attempt = rounded_attempt(rows, rhs, row_duals, weights, denominator)
        allowed = min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE)
        summary = {key: value for key, value in attempt.items() if key != "positive_rows"}
        summary["objective_over_expected"] = attempt["objective_bound"] - selection["expected_bound"]
        summary["passes"] = (
            attempt["min_slack"] >= 0
            and attempt["objective_bound"] <= allowed
            and attempt["positive_row_count"] <= row_budget(selection)
        )
        attempts.append(summary)
        if selection["export_required"] and summary["passes"] and success is None:
            success = attempt
    error = objective - selection["expected_bound"]
    failures = [
        reason
        for reason, active in [
            ("terminal_reproduction_error", abs(error) > 1e-5),
            ("fixed_bound_dual_mass_nonzero", fixed_max > BOUND_DUAL_TOL),
            ("nonfixed_bound_dual_mass_nonzero", all_bound_max > BOUND_DUAL_TOL),
            ("no_export_success", selection["export_required"] and success is None),
            ("success_denominator_above_canonical_limit", success is not None and success["denominator"] > CANONICAL_DENOMINATOR_LIMIT),
            ("row_budget_exceeded", success is not None and success["positive_row_count"] > row_budget(selection)),
        ]
        if active
    ]
    return {
        **selection,
        "fixed_literals": {str(v + 1): float(value) for v, value in sorted(fixed.items())},
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "row_count": len(rows)}),
        "floating_objective": objective,
        "expected_error": error,
        "fixed_bound_dual_max": fixed_max,
        "fixed_bound_dual_mass": fixed_mass,
        "all_bound_dual_max": all_bound_max,
        "all_bound_dual_mass": all_bound_mass,
        "positive_bound_dual_count": positive_bound_count,
        "row_budget": row_budget(selection),
        "attempts": attempts,
        "success_summary": None if success is None else {key: value for key, value in success.items() if key != "positive_rows"},
        "success_rows": [] if success is None else success["positive_rows"],
        "failure_reasons": failures,
        "status": "audit_only" if not selection["export_required"] else ("export_success" if not failures else "export_failed"),
    }


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_and_round(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row["success_summary"]["positive_row_count"] for row in successes if row["success_summary"])
    high_den = [row["terminal_id"] for row in successes if row["success_summary"]["denominator"] > CANONICAL_DENOMINATOR_LIMIT]
    failures = collect_failures(required, successes, total_rows, high_den)
    report = report_base(replay, source, edges, weights, root_cuts, parent_rows, terminals, successes, total_rows, high_den, failures)
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


def report_base(replay, source, edges, weights, root_cuts, parent_rows, terminals, successes, total_rows, high_den, failures):
    return {
        "schema": "forge.hadwiger.w607_selected_terminal_dual_export_preflight.v1",
        "authority": "selected_terminal_certificate_preflight_only_no_root_or_lift_authority",
        "second_opinion": {"agent": "Popper", "decision": "approve_selected_terminal_exact_dual_export_preflight"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len([t for t in terminals if t["export_required"]]), "audit_only_count": len([t for t in terminals if not t["export_required"]]), "denominators": DENOMINATORS},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "total_success_rows": total_rows, "high_denominator_successes": high_den, "worst_success_objective": max((row["success_summary"]["objective_bound"] for row in successes), default=None)},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_full_mixed_tree_terminal_export" if not failures else "retire_selected_terminal_export_preflight",
    }


def source_binding(replay, source, edges, weights, root_cuts, parent_rows):
    return {"fresh_replay_path": str(REPLAY), "fresh_replay_digest": digest(replay), "first_family_source": str(SOURCE), "first_family_digest": digest(source), "graph_digest": graph_digest(edges, weights), "root_rows_digest": digest(root_cuts), "parent_rows_digest": digest(parent_rows)}


def gates():
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "bound_dual_tolerance": BOUND_DUAL_TOL, "canonical_denominator_limit": CANONICAL_DENOMINATOR_LIMIT, "normal_row_budget": NORMAL_ROW_BUDGET, "leaf0_row_budget": LEAF0_ROW_BUDGET, "total_row_budget": TOTAL_ROW_BUDGET}


if __name__ == "__main__":
    main()
