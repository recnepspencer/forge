import json
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


OUT = provenance.CRATE / "docs" / "w607-targeted-rational-terminal-repair.json"
DENOMINATOR_CAP = 10**9
MAX_DENOMINATOR = 2**32
MAX_NUMERATOR_BITS = 48
REPAIR_COST_GATE = Fraction(1, 1)
MAX_SINGLE_REPAIR_COST = Fraction(1, 2)
MAX_REPAIRED_ROWS = 16


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
    base = base_multipliers(rows, duals)
    repaired, trace = repair(rows, rhs, weights, base)
    replay = exact_replay(rows, rhs, weights, repaired)
    repair_cost = replay["objective"] - exact_replay(rows, rhs, weights, base)["objective"]
    failures = terminal_failures(selection, objective, hidden_lower, hidden_upper, base, repaired, replay, repair_cost, trace)
    return {
        **selection,
        "floating_objective": objective,
        "expected_error": objective - selection["expected_bound"],
        "hidden_solver_bound_dual_max": float(max(np.max(hidden_lower), np.max(hidden_upper))),
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "explicit_row_count": len(rows)}),
        "base_negative": base_negative_summary(rows, rhs, weights, base),
        "repair_trace": trace,
        "repair_cost": rational.frac_json(repair_cost),
        "repair_cost_float": float(repair_cost),
        "objective": rational.frac_json(replay["objective"]),
        "objective_float": float(replay["objective"]),
        "objective_over_expected": float(replay["objective"] - Fraction.from_float(selection["expected_bound"])),
        "min_slack": replay["min_slack"],
        "zero_slack_vertex_count": replay["zero_slack_vertex_count"],
        "argmin_vertices": replay["argmin_vertices"],
        "positive_row_count": replay["positive_row_count"],
        "max_denominator": max(frac.denominator for frac in repaired),
        "max_denominator_bits": max(frac.denominator.bit_length() for frac in repaired),
        "max_numerator_bits": max(abs(frac.numerator).bit_length() for frac in repaired),
        "repaired_row_count": len({step["row_index"] for step in trace if "row_index" in step}),
        "failure_reasons": failures,
        "status": "audit_only" if not selection["export_required"] else ("export_success" if not failures else "export_failed"),
    }


def base_multipliers(rows, duals):
    out = []
    for row, value in zip(rows, duals):
        if row["family"] == "variable_lower_bound" or value <= 1e-10:
            out.append(Fraction(0))
        else:
            out.append(Fraction(float(value)).limit_denominator(DENOMINATOR_CAP))
    return out


def repair(rows, rhs, weights, multipliers):
    multipliers = list(multipliers)
    trace = []
    for _ in range(64):
        slacks = slacks_for(rows, weights, multipliers)
        deficient = [index for index, value in enumerate(slacks) if value < 0]
        if not deficient:
            return multipliers, trace
        vertex = deficient[0]
        candidates = covering_candidates(rows, rhs, multipliers, vertex)
        if not candidates:
            trace.append({"failure": "no_positive_support_row_covers_deficient_vertex", "vertex": vertex + 1})
            return multipliers, trace
        _score, row_id, row_index, coeff = candidates[0]
        increment = -slacks[vertex] / coeff
        multipliers[row_index] += increment
        trace.append(
            {
                "vertex": vertex + 1,
                "row_index": row_index,
                "row_id": row_id,
                "increment": rational.frac_json(increment),
                "objective_cost": rational.frac_json(increment * int(round(float(rhs[row_index])))),
            }
        )
    trace.append({"failure": "repair_iteration_cap"})
    return multipliers, trace


def covering_candidates(rows, rhs, multipliers, vertex):
    candidates = []
    for index, (row, mult) in enumerate(zip(rows, multipliers)):
        if mult <= 0:
            continue
        coeff = row_coefficients(row).get(vertex, 0)
        if coeff <= 0:
            continue
        score = Fraction(int(round(float(rhs[index]))), coeff)
        candidates.append((score, row["id"], index, coeff))
    return sorted(candidates, key=lambda item: (item[0], item[1], item[2]))


def slacks_for(rows, weights, multipliers):
    coverage = [Fraction(0) for _ in range(provenance.parent.N)]
    for row, mult in zip(rows, multipliers):
        if mult == 0:
            continue
        for vertex, coeff in row_coefficients(row).items():
            coverage[vertex] += mult * coeff
    return [coverage[v] - int(round(float(weights[v]))) for v in range(provenance.parent.N)]


def exact_replay(rows, rhs, weights, multipliers):
    objective = Fraction(0)
    positive = 0
    for row, row_rhs, mult in zip(rows, rhs, multipliers):
        if mult == 0:
            continue
        positive += 1
        objective += mult * int(round(float(row_rhs)))
    slacks = slacks_for(rows, weights, multipliers)
    min_slack = min(slacks)
    return {
        "objective": objective,
        "min_slack": 1 if min_slack >= 0 else -1,
        "zero_slack_vertex_count": sum(1 for value in slacks if value == 0),
        "argmin_vertices": [i + 1 for i, value in enumerate(slacks) if value == min_slack][:12],
        "positive_row_count": positive,
    }


def base_negative_summary(rows, rhs, weights, multipliers):
    replay = exact_replay(rows, rhs, weights, multipliers)
    return {"min_slack": replay["min_slack"], "argmin_vertices": replay["argmin_vertices"], "objective_float": float(replay["objective"])}


def terminal_failures(selection, objective, hidden_lower, hidden_upper, base, repaired, replay, repair_cost, trace):
    allowed = Fraction.from_float(min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE))
    max_single = max((Fraction(step["objective_cost"]["num"], step["objective_cost"]["den"]) for step in trace if "objective_cost" in step), default=Fraction(0))
    changed = [index for index, (before, after) in enumerate(zip(base, repaired)) if after != before]
    return [
        reason
        for reason, active in [
            ("terminal_reproduction_error", abs(objective - selection["expected_bound"]) > 1e-6),
            ("hidden_solver_bound_dual_nonzero", float(max(np.max(hidden_lower), np.max(hidden_upper))) > 1e-8),
            ("exact_coverage_negative", replay["min_slack"] < 0),
            ("objective_gate_failed", replay["objective"] > allowed),
            ("repair_cost_exceeds_gate", repair_cost > REPAIR_COST_GATE),
            ("single_repair_cost_exceeds_gate", max_single > MAX_SINGLE_REPAIR_COST),
            ("too_many_repaired_rows", len(set(changed)) > MAX_REPAIRED_ROWS),
            ("denominator_gate_failed", max(frac.denominator for frac in repaired) > MAX_DENOMINATOR),
            ("numerator_bits_gate_failed", max(abs(frac.numerator).bit_length() for frac in repaired) > MAX_NUMERATOR_BITS),
            ("repair_trace_failure", any("failure" in step for step in trace)),
        ]
        if active
    ]


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_terminal(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row["positive_row_count"] for row in successes)
    failures = collect_failures(required, successes, total_rows)
    report = {
        "schema": "forge.hadwiger.w607_targeted_rational_terminal_repair.v1",
        "authority": "selected_terminal_support_preserving_exact_repair_preflight_only",
        "second_opinion": {"agent": "Kepler", "decision": "approve_targeted_deficit_repair"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len(required), "audit_only_count": len(terminals) - len(required), "base_denominator_cap": DENOMINATOR_CAP},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "required_total": len(required), "total_success_rows": total_rows, "worst_success_objective": max((row["objective_float"] for row in successes), default=None), "repaired_terminal_count": sum(1 for row in successes if row["repair_trace"])},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_rational_terminal_export_plumbing" if not failures else "retire_targeted_rational_terminal_repair",
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
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "base_denominator_cap": DENOMINATOR_CAP, "max_denominator": MAX_DENOMINATOR, "max_numerator_bits": MAX_NUMERATOR_BITS, "repair_objective_cost": float(REPAIR_COST_GATE), "max_single_repair_cost": float(MAX_SINGLE_REPAIR_COST), "max_repaired_rows": MAX_REPAIRED_ROWS, "total_row_budget": TOTAL_ROW_BUDGET}


if __name__ == "__main__":
    main()
