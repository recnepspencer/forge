import json

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
    rounded_attempt,
    run_context,
    terminal_fixed,
)


OUT = provenance.CRATE / "docs" / "w607-bigdenom-terminal-export-probe.json"
DENOMINATORS = [2**24, 2**28, 2**32, 2**36]
MAX_NUMERATOR_BITS = 48
MAX_ROW_OBJECTIVE_BITS = 80
MAX_OBJECTIVE_BITS = 96
FRAGILE_MARGIN = 50.0
CLEAN_MARGIN = 100.0


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
    attempts, success = [], None
    for denominator in DENOMINATORS:
        attempt = rounded_attempt(rows, rhs, duals, weights, denominator, skip_lower=True)
        summary = attempt_summary(selection, attempt, denominator)
        attempts.append(summary)
        if selection["export_required"] and summary["passes"] and success is None:
            success = attempt
    failures = terminal_failures(selection, objective, hidden_lower, hidden_upper, success)
    return {
        **selection,
        "floating_objective": objective,
        "expected_error": objective - selection["expected_bound"],
        "hidden_solver_bound_dual_max": float(max(np.max(hidden_lower), np.max(hidden_upper))),
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "explicit_row_count": len(rows)}),
        "attempts": attempts,
        "success_summary": None if success is None else {key: value for key, value in success.items() if key != "positive_rows"},
        "success_row_kinds": success_kinds(success),
        "success_rows": [] if success is None else success["positive_rows"],
        "failure_reasons": failures,
        "status": "audit_only" if not selection["export_required"] else ("export_success" if not failures else "export_failed"),
    }


def attempt_summary(selection, attempt, denominator):
    allowed = min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE)
    max_numerator = max((row["numerator"] for row in attempt["positive_rows"]), default=0)
    max_row_obj = max((row["numerator"] * int(row["rhs"]) for row in attempt["positive_rows"]), default=0)
    margin = allowed - attempt["objective_bound"]
    summary = {key: value for key, value in attempt.items() if key != "positive_rows"}
    summary.update(
        {
            "objective_over_expected": attempt["objective_bound"] - selection["expected_bound"],
            "objective_margin_to_allowed": margin,
            "max_multiplier_numerator": max_numerator,
            "max_multiplier_numerator_bits": max_numerator.bit_length(),
            "max_row_objective_num_bits": int(max_row_obj).bit_length(),
            "objective_num_bits": int(attempt["objective_num"]).bit_length(),
            "passes": (
                attempt["min_slack"] >= 0
                and attempt["objective_bound"] <= allowed
                and attempt["positive_row_count"] <= row_budget(selection)
                and max_numerator.bit_length() <= MAX_NUMERATOR_BITS
                and int(max_row_obj).bit_length() <= MAX_ROW_OBJECTIVE_BITS
                and int(attempt["objective_num"]).bit_length() <= MAX_OBJECTIVE_BITS
            ),
            "fragile": denominator == 2**36 and 0.0 <= margin < FRAGILE_MARGIN,
            "clean_margin": margin >= CLEAN_MARGIN,
        }
    )
    return summary


def terminal_failures(selection, objective, hidden_lower, hidden_upper, success):
    hidden_max = float(max(np.max(hidden_lower), np.max(hidden_upper)))
    return [
        reason
        for reason, active in [
            ("terminal_reproduction_error", abs(objective - selection["expected_bound"]) > 1e-6),
            ("hidden_solver_bound_dual_nonzero", hidden_max > 1e-8),
            ("no_export_success", selection["export_required"] and success is None),
            ("fragile_bigdenom_pass", success is not None and success["denominator"] == 2**36 and (min(EXPORT_GATE, selection["expected_bound"] + ALLOWANCE) - success["objective_bound"]) < FRAGILE_MARGIN),
        ]
        if active
    ]


def success_kinds(success):
    counts = {}
    if success is None:
        return counts
    for row in success["positive_rows"]:
        counts[row["family"]] = counts.get(row["family"], 0) + 1
    return counts


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections = run_context()
    terminals = [solve_terminal(s, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for s in selections]
    required = [row for row in terminals if row["export_required"]]
    successes = [row for row in required if row["status"] == "export_success"]
    total_rows = sum(row["success_summary"]["positive_row_count"] for row in successes if row["success_summary"])
    failures = collect_failures(required, successes, total_rows)
    report = {
        "schema": "forge.hadwiger.w607_bigdenom_terminal_export_probe.v1",
        "authority": "selected_terminal_bigdenom_probe_only_no_root_or_lift_authority",
        "second_opinion": {"agent": "Maxwell", "decision": "approve_bounded_bigdenom_probe"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "scope": {"top_leaves": TOP_LEAVES, "required_export_count": len(required), "audit_only_count": len(terminals) - len(required), "denominators": DENOMINATORS},
        "gates": gates(),
        "summary": {"required_successes": len(successes), "required_total": len(required), "total_success_rows": total_rows, "worst_success_objective": max((row["success_summary"]["objective_bound"] for row in successes), default=None)},
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_bigint_terminal_export_plumbing" if not failures else "retire_bigdenom_terminal_export_probe",
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
    return {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "denominator_ladder": DENOMINATORS, "max_multiplier_numerator_bits": MAX_NUMERATOR_BITS, "max_row_objective_num_bits": MAX_ROW_OBJECTIVE_BITS, "max_objective_num_bits": MAX_OBJECTIVE_BITS, "total_row_budget": TOTAL_ROW_BUDGET, "fragile_margin": FRAGILE_MARGIN, "clean_margin": CLEAN_MARGIN}


if __name__ == "__main__":
    main()
