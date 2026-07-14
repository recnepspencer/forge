import json
from fractions import Fraction

import numpy as np

import run_w607_explicit_bound_terminal_dual_export_preflight as explicit
import run_w607_rational_terminal_export_probe as rational
import run_w607_terminal_dual_provenance as provenance
from w607_selected_terminal_export_support import (
    ALLOWANCE,
    DENOMINATORS,
    EXPORT_GATE,
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


OUT = provenance.CRATE / "docs" / "w607-hybrid-terminal-export-preflight.json"
RATIONAL_CAP = 2**32
INTEGER_DENOMINATORS = [1024, 4096, 16777216]
MAX_NUMERATOR_BITS = 48
PARENT_OVERHEAD_CAP = 1e-3


def mechanism_class(selection):
    if selection["selection_reason"] == "top_six_final_terminal":
        return "top_six_depth3_parent_lift"
    if selection["selection_reason"] == "leaf0_exception_child_export":
        return "leaf0_residual_child"
    return "audit_only_trigger"


def solve_terminal(selection, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows):
    klass = mechanism_class(selection)
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
    if klass == "top_six_depth3_parent_lift":
        certificate = rational_certificate(selection, rows, rhs, weights, duals)
        strategy = "per_row_rational_reconstruction"
    elif klass == "leaf0_residual_child":
        certificate = integer_certificate(selection, rows, rhs, weights, duals)
        strategy = "common_denominator_upward_integer"
    else:
        certificate = audit_certificate(selection, rows, rhs, weights, duals)
        strategy = "audit_only_no_pass_gate"
    failures = failures_for(selection, objective, hidden_lower, hidden_upper, certificate)
    return {
        **selection,
        "mechanism_class": klass,
        "selected_strategy": strategy,
        "fixed_digest": digest({str(v + 1): float(value) for v, value in sorted(fixed.items())}),
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "explicit_row_count": len(rows)}),
        "floating_objective": objective,
        "expected_error": objective - selection["expected_bound"],
        "hidden_solver_bound_dual_max": float(max(np.max(hidden_lower), np.max(hidden_upper))),
        "certificate": certificate,
        "failure_reasons": failures,
        "status": "audit_only" if not selection["export_required"] else ("export_success" if not failures else "export_failed"),
    }


def rational_certificate(selection, rows, rhs, weights, duals):
    multipliers = []
    lower_count = 0
    for row, value in zip(rows, duals):
        if row["family"] == "variable_lower_bound":
            lower_count += int(value > 1e-10)
            multipliers.append(Fraction(0))
        elif value <= 1e-10:
            multipliers.append(Fraction(0))
        else:
            multipliers.append(Fraction(float(value)).limit_denominator(RATIONAL_CAP))
    replay = rational.exact_replay(rows, rhs, weights, multipliers)
    parent = rational.parent_report(rows, rhs, duals, multipliers)
    max_den = max(frac.denominator for frac in multipliers)
    max_num_bits = max(abs(frac.numerator).bit_length() for frac in multipliers)
    return {
        "strategy": "per_row_rational_reconstruction",
        "objective_float": float(replay["objective"]),
        "objective_over_expected": float(replay["objective"] - Fraction.from_float(selection["expected_bound"])),
        "objective_margin_to_gate": float(Fraction.from_float(EXPORT_GATE) - replay["objective"]),
        "objective_margin_to_expected_plus_allowance": float(Fraction.from_float(selection["expected_bound"] + ALLOWANCE) - replay["objective"]),
        "min_slack": replay["min_slack"],
        "zero_slack_vertex_count": replay["zero_slack_vertex_count"],
        "argmin_vertices": replay["argmin_vertices"],
        "positive_row_count": replay["positive_row_count"],
        "max_denominator": max_den,
        "max_denominator_bits": max_den.bit_length(),
        "max_numerator_bits": max_num_bits,
        "lower_bound_positive_reported_not_exported": lower_count,
        "parent_lift_report": parent,
    }


def integer_certificate(selection, rows, rhs, weights, duals):
    attempts = []
    success = None
    for denominator in INTEGER_DENOMINATORS:
        attempt = rounded_attempt(rows, rhs, duals, weights, denominator, skip_lower=True)
        summary = {key: value for key, value in attempt.items() if key != "positive_rows"}
        summary["objective_over_expected"] = attempt["objective_bound"] - selection["expected_bound"]
        summary["objective_margin_to_gate"] = EXPORT_GATE - attempt["objective_bound"]
        summary["objective_margin_to_expected_plus_allowance"] = selection["expected_bound"] + ALLOWANCE - attempt["objective_bound"]
        summary["max_numerator_bits"] = max((row["numerator"].bit_length() for row in attempt["positive_rows"]), default=0)
        summary["passes"] = integer_passes(selection, summary)
        attempts.append(summary)
        if summary["passes"] and success is None:
            success = summary
    return {"strategy": "common_denominator_upward_integer", "attempts": attempts, "selected": success}


def audit_certificate(selection, rows, rhs, weights, duals):
    attempt = rounded_attempt(rows, rhs, duals, weights, 16777216, skip_lower=True)
    return {"strategy": "audit_only", "objective_float": attempt["objective_bound"], "min_slack": attempt["min_slack"]}


def integer_passes(selection, summary):
    return (
        summary["min_slack"] >= 0
        and summary["objective_bound"] <= EXPORT_GATE
        and summary["objective_bound"] <= selection["expected_bound"] + ALLOWANCE
        and summary["positive_row_count"] <= row_budget(selection)
        and summary["max_numerator_bits"] <= MAX_NUMERATOR_BITS
    )


def failures_for(selection, objective, hidden_lower, hidden_upper, certificate):
    if not selection["export_required"]:
        return []
    hidden_max = float(max(np.max(hidden_lower), np.max(hidden_upper)))
    common = [
        ("terminal_reproduction_error", abs(objective - selection["expected_bound"]) > 1e-6),
        ("hidden_solver_bound_dual_nonzero", hidden_max > 1e-8),
    ]
    if certificate["strategy"] == "per_row_rational_reconstruction":
        parent_overhead = certificate["parent_lift_report"]["max_parent_objective_overhead_abs"]
        common.extend(
            [
                ("exact_coverage_negative", certificate["min_slack"] < 0),
                ("objective_gate_failed", certificate["objective_margin_to_gate"] < 0 or certificate["objective_margin_to_expected_plus_allowance"] < 0),
                ("denominator_gate_failed", certificate["max_denominator"] > RATIONAL_CAP),
                ("numerator_bits_gate_failed", certificate["max_numerator_bits"] > MAX_NUMERATOR_BITS),
                ("parent_overhead_gate_failed", parent_overhead > PARENT_OVERHEAD_CAP),
                ("row_budget_exceeded", certificate["positive_row_count"] > row_budget(selection)),
            ]
        )
    elif certificate["strategy"] == "common_denominator_upward_integer":
        common.append(("no_integer_strategy_success", certificate["selected"] is None))
    else:
        common.append(("invalid_strategy_for_required_terminal", True))
    return [reason for reason, active in common if active]


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_terminal(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row_count(row) for row in successes)
    failures = collect_failures(required, successes, total_rows)
    report = {
        "schema": "forge.hadwiger.w607_hybrid_terminal_export_preflight.v1",
        "authority": "selected_terminal_preflight_only_no_root_or_full_export_authority",
        "second_opinion": {"agent": "Leibniz", "decision": "approve_mechanism_class_hybrid_policy"},
        "class_policy": {"top_six_depth3_parent_lift": "per_row_rational_reconstruction", "leaf0_residual_child": "common_denominator_upward_integer"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len(required), "audit_only_count": len(terminals) - len(required)},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "required_total": len(required), "total_success_rows": total_rows, "worst_success_objective": max((objective_of(row) for row in successes), default=None)},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_full_mixed_tree_terminal_export_design" if not failures else "retire_or_revise_hybrid_preflight",
    }
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps(jsonable({key: value for key, value in report.items() if key != "terminals"}), indent=2))


def row_count(row):
    cert = row["certificate"]
    if cert["strategy"] == "common_denominator_upward_integer":
        return cert["selected"]["positive_row_count"]
    return cert["positive_row_count"]


def objective_of(row):
    cert = row["certificate"]
    if cert["strategy"] == "common_denominator_upward_integer":
        return cert["selected"]["objective_bound"]
    return cert["objective_float"]


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
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "integer_denominators": INTEGER_DENOMINATORS, "rational_denominator_cap": RATIONAL_CAP, "max_numerator_bits": MAX_NUMERATOR_BITS, "parent_overhead_cap": PARENT_OVERHEAD_CAP, "total_row_budget": TOTAL_ROW_BUDGET}


if __name__ == "__main__":
    main()
