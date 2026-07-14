import json
import math
from fractions import Fraction

import numpy as np

import run_w607_explicit_bound_terminal_dual_export_preflight as explicit
import run_w607_rational_terminal_export_probe as rational
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


OUT = provenance.CRATE / "docs" / "w607-upward-rational-terminal-export-probe.json"
DENOMINATOR_CAPS = [10**6, 10**8, 10**9, 2**32]
MAX_DENOMINATOR = 2**32
MAX_NUMERATOR_BITS = 48
REPAIR_COST_GATE = 1.0
PARENT_OVERHEAD_CAP = 50.0


def upward_fraction(value, denominator):
    return Fraction(max(0, math.ceil(float(value) * denominator - 1e-18)), denominator)


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
    success = min(passing, key=lambda row: (row["objective_float"], row["max_numerator_bits"], row["denominator_cap"]), default=None)
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


def attempt(selection, rows, rhs, weights, duals, denominator):
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
            multipliers.append(upward_fraction(value, denominator))
    replay = rational.exact_replay(rows, rhs, weights, multipliers)
    parent = rational.parent_report(rows, rhs, duals, multipliers)
    allowed = min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE)
    objective = replay["objective"]
    float_obj = float(objective)
    repair_cost = max(0.0, float_obj - selection["expected_bound"])
    max_num_bits = max((abs(frac.numerator).bit_length() for frac in multipliers), default=0)
    max_den = max((frac.denominator for frac in multipliers), default=1)
    passes = (
        replay["min_slack"] >= 0
        and objective <= Fraction.from_float(allowed)
        and replay["positive_row_count"] <= row_budget(selection)
        and max_den <= MAX_DENOMINATOR
        and max_num_bits <= MAX_NUMERATOR_BITS
        and repair_cost <= REPAIR_COST_GATE
        and parent["max_parent_objective_overhead_abs"] <= PARENT_OVERHEAD_CAP
    )
    return {
        "denominator_cap": denominator,
        "objective": rational.frac_json(objective),
        "objective_float": float_obj,
        "objective_over_expected": float(objective - Fraction.from_float(selection["expected_bound"])),
        "objective_margin_to_allowed": float(Fraction.from_float(allowed) - objective),
        "repair_objective_cost": repair_cost,
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


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_terminal(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row["success_summary"]["positive_row_count"] for row in successes if row["success_summary"])
    failures = collect_failures(required, successes, total_rows)
    report = {
        "schema": "forge.hadwiger.w607_upward_rational_terminal_export_probe.v1",
        "authority": "selected_terminal_support_preserving_rational_repair_preflight_only",
        "second_opinion": {"agent": "Linnaeus", "decision": "approve_support_preserving_rational_repair"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len(required), "audit_only_count": len(terminals) - len(required), "denominator_caps": DENOMINATOR_CAPS},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "required_total": len(required), "total_success_rows": total_rows, "worst_success_objective": max((row["success_summary"]["objective_float"] for row in successes), default=None)},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_rational_terminal_export_plumbing" if not failures else "retire_upward_rational_terminal_export_probe",
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
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "denominator_caps": DENOMINATOR_CAPS, "max_denominator": MAX_DENOMINATOR, "max_numerator_bits": MAX_NUMERATOR_BITS, "repair_objective_cost": REPAIR_COST_GATE, "parent_objective_overhead_cap": PARENT_OVERHEAD_CAP, "total_row_budget": TOTAL_ROW_BUDGET}


if __name__ == "__main__":
    main()
