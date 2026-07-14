import json
from fractions import Fraction

import numpy as np

import run_w607_explicit_bound_terminal_dual_export_preflight as explicit
import run_w607_terminal_dual_provenance as provenance
from w607_selected_terminal_export_support import (
    ALLOWANCE,
    EXPORT_GATE,
    REPLAY,
    SOURCE,
    TOTAL_ROW_BUDGET,
    TOP_LEAVES,
    digest,
    graph_digest,
    jsonable,
    row_budget,
    row_coefficients,
    run_context,
    terminal_fixed,
)


OUT = provenance.CRATE / "docs" / "w607-rational-terminal-export-probe.json"
DENOMINATOR_CAPS = [10**6, 10**8, 10**9, 2**32]
MAX_DENOMINATOR = 2**32
MAX_NUMERATOR_BITS = 48
MAX_DENOMINATOR_BITS = 32
PARENT_OVERHEAD_CAP = 50.0


def solve_terminal(selection, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows):
    leaf_index = selection["leaf_index"]
    leaf = finite[leaf_index]
    fixed = terminal_fixed(selection, leaf)
    rows, rhs, first_rows = provenance.row_system_for_leaf(
        leaf_index, leaf, source_by_index[leaf_index], edges, triads, weights, adj, root_cuts, parent_rows
    )
    rows, rhs = explicit.explicit_terminal_rows(rows, rhs, fixed)
    objective, result = explicit.solve_explicit(rows, rhs, weights)
    duals = np.maximum(-np.array(result.ineqlin.marginals), 0.0)
    hidden_lower = np.maximum(np.array(result.lower.marginals), 0.0)
    hidden_upper = np.maximum(-np.array(result.upper.marginals), 0.0)
    attempts = [attempt(selection, rows, rhs, weights, duals, cap) for cap in DENOMINATOR_CAPS]
    passing = [row for row in attempts if row["passes"]]
    success = min(passing, key=lambda row: (row["objective_float"], row["max_numerator_bits"], row["max_denominator"], row["positive_row_count"]), default=None)
    failures = terminal_failures(selection, objective, hidden_lower, hidden_upper, success)
    return {
        **selection,
        "floating_objective": objective,
        "expected_error": objective - selection["expected_bound"],
        "hidden_solver_bound_dual_max": float(max(np.max(hidden_lower), np.max(hidden_upper))),
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "explicit_row_count": len(rows)}),
        "attempts": attempts,
        "success_summary": success,
        "failure_reasons": failures,
        "status": "audit_only" if not selection["export_required"] else ("export_success" if not failures else "export_failed"),
    }


def attempt(selection, rows, rhs, weights, duals, cap):
    multipliers = []
    lower_report = {"count": 0, "mass": 0.0}
    for row, value in zip(rows, duals):
        if row["family"] == "variable_lower_bound":
            if value > 1e-10:
                lower_report["count"] += 1
                lower_report["mass"] += float(value)
            multipliers.append(Fraction(0))
        elif value <= 1e-10:
            multipliers.append(Fraction(0))
        else:
            multipliers.append(Fraction(float(value)).limit_denominator(cap))
    replay = exact_replay(rows, rhs, weights, multipliers)
    allowed = min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE)
    max_num_bits = max((abs(frac.numerator).bit_length() for frac in multipliers), default=0)
    max_den = max((frac.denominator for frac in multipliers), default=1)
    parent = parent_report(rows, rhs, duals, multipliers)
    passes = (
        replay["min_slack"] >= 0
        and replay["objective"] <= Fraction.from_float(allowed)
        and replay["positive_row_count"] <= row_budget(selection)
        and max_den <= MAX_DENOMINATOR
        and max_den.bit_length() <= MAX_DENOMINATOR_BITS
        and max_num_bits <= MAX_NUMERATOR_BITS
        and parent["max_parent_objective_overhead_abs"] <= PARENT_OVERHEAD_CAP
    )
    return {
        "denominator_cap": cap,
        "objective": frac_json(replay["objective"]),
        "objective_float": float(replay["objective"]),
        "objective_over_expected": float(replay["objective"] - Fraction.from_float(selection["expected_bound"])),
        "objective_margin_to_allowed": float(Fraction.from_float(allowed) - replay["objective"]),
        "min_slack": replay["min_slack"],
        "zero_slack_vertex_count": replay["zero_slack_vertex_count"],
        "argmin_vertices": replay["argmin_vertices"],
        "positive_row_count": replay["positive_row_count"],
        "max_denominator": max_den,
        "max_denominator_bits": max_den.bit_length(),
        "max_numerator_bits": max_num_bits,
        "lower_bound_positive_reported_not_exported": lower_report,
        "parent_lift_report": parent,
        "passes": passes,
    }


def exact_replay(rows, rhs, weights, multipliers):
    coverage = [Fraction(0) for _ in range(provenance.parent.N)]
    objective = Fraction(0)
    positive = 0
    for row, row_rhs, mult in zip(rows, rhs, multipliers):
        if mult == 0:
            continue
        positive += 1
        objective += mult * int(round(float(row_rhs)))
        for vertex, coeff in row_coefficients(row).items():
            coverage[vertex] += mult * coeff
    slacks = [coverage[v] - int(round(float(weights[v]))) for v in range(provenance.parent.N)]
    min_slack = min(slacks)
    return {
        "objective": objective,
        "min_slack": 1 if min_slack >= 0 else -1,
        "min_slack_exact": frac_json(min_slack),
        "zero_slack_vertex_count": sum(1 for value in slacks if value == 0),
        "argmin_vertices": [index + 1 for index, value in enumerate(slacks) if value == min_slack][:12],
        "positive_row_count": positive,
    }


def parent_report(rows, rhs, duals, multipliers):
    reports = []
    max_overhead = 0.0
    for row, row_rhs, dual, mult in zip(rows, rhs, duals, multipliers):
        if row["family"] != "parent_lifts" or mult == 0:
            continue
        rhs_int = int(round(float(row_rhs)))
        overhead = float(mult) * rhs_int - float(dual) * rhs_int
        max_overhead = max(max_overhead, abs(overhead))
        reports.append(
            {
                "id": row["id"],
                "float_multiplier": float(dual),
                "rational_multiplier": frac_json(mult),
                "denominator": mult.denominator,
                "numerator_bits": abs(mult.numerator).bit_length(),
                "objective_contribution": float(mult * rhs_int),
                "objective_overhead": overhead,
            }
        )
    return {"rows": reports, "max_parent_objective_overhead_abs": max_overhead}


def terminal_failures(selection, objective, hidden_lower, hidden_upper, success):
    hidden_max = float(max(np.max(hidden_lower), np.max(hidden_upper)))
    return [
        reason
        for reason, active in [
            ("terminal_reproduction_error", abs(objective - selection["expected_bound"]) > 1e-6),
            ("hidden_solver_bound_dual_nonzero", hidden_max > 1e-8),
            ("no_export_success", selection["export_required"] and success is None),
        ]
        if active
    ]


def frac_json(value):
    return {"num": int(value.numerator), "den": int(value.denominator), "float": float(value)}


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_terminal(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row["success_summary"]["positive_row_count"] for row in successes if row["success_summary"])
    failures = collect_failures(required, successes, total_rows)
    report = {
        "schema": "forge.hadwiger.w607_rational_terminal_export_probe.v1",
        "authority": "selected_terminal_exact_rational_preflight_only_no_root_or_lift_authority",
        "second_opinion": {"agent": "McClintock", "decision": "approve_exact_replay_filter"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len(required), "audit_only_count": len(terminals) - len(required), "denominator_caps": DENOMINATOR_CAPS},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "required_total": len(required), "total_success_rows": total_rows, "worst_success_objective": max((row["success_summary"]["objective_float"] for row in successes), default=None)},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_rational_terminal_export_plumbing" if not failures else "retire_rational_terminal_export_probe",
    }
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps(jsonable({key: value for key, value in report.items() if key != "terminals"}), indent=2))


def collect_failures(required, successes, total_rows):
    failures = ["not_all_required_terminals_exported"] if len(successes) != len(required) else []
    if total_rows > TOTAL_ROW_BUDGET:
        failures.append("total_row_budget_exceeded")
    for row in required:
        failures.extend(f"{row['terminal_id']}:{reason}" for reason in row["failure_reasons"])
    return sorted(set(failures))


def source_binding(replay, source, edges, weights, root_cuts, parent_rows):
    return {"fresh_replay_path": str(REPLAY), "fresh_replay_digest": digest(replay), "first_family_source": str(SOURCE), "first_family_digest": digest(source), "graph_digest": graph_digest(edges, weights), "root_rows_digest": digest(root_cuts), "parent_rows_digest": digest(parent_rows)}


def gates():
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "denominator_caps": DENOMINATOR_CAPS, "max_denominator": MAX_DENOMINATOR, "max_numerator_bits": MAX_NUMERATOR_BITS, "max_denominator_bits": MAX_DENOMINATOR_BITS, "parent_objective_overhead_cap": PARENT_OVERHEAD_CAP, "total_row_budget": TOTAL_ROW_BUDGET}


if __name__ == "__main__":
    main()
