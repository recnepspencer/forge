import json
import math
from fractions import Fraction

import run_w607_hybrid_terminal_export_preflight as hybrid
import run_w607_terminal_dual_provenance as provenance
from w607_selected_terminal_export_support import (
    ALLOWANCE,
    EXPORT_GATE,
    REPLAY,
    SOURCE,
    digest,
    graph_digest,
    jsonable,
    run_context,
)


OUT = provenance.CRATE / "docs" / "w607-full-terminal-export-preflight.json"
EXPECTED_TERMINALS = 135
TOTAL_ROW_BUDGET = 100000
HIGH_PARENT_LIFT_LEAVES = {1, 2, 4, 6, 9, 12}


def manifest(replay):
    items = []
    replaced = []
    for leaf in replay["leaves"]:
        leaf_index = leaf["leaf_index"]
        if leaf_index == 0:
            for index, closure in enumerate(leaf["residual_closures"]):
                if closure["triggered"]:
                    replaced.append({"leaf_index": 0, "terminal_id": f"depth4_terminal_{index}", "replaced_by": 4})
                    for child_index, child in enumerate(closure["children"]):
                        items.append(selection(0, f"closure_{index}_child_{child_index}", child["bound"], child["pool_assignment"], 5, "leaf0_residual_child", child.get("assignment")))
                else:
                    terminal = closure["terminal"]
                    items.append(selection(0, f"depth4_terminal_{index}", closure["closed_bound"], terminal["pool_assignment"], terminal["depth"], "ordinary_leaf0_closed", None))
        else:
            for index, terminal in enumerate(leaf["terminal_certificates"]):
                klass = "high_parent_lift_depth3" if leaf_index in HIGH_PARENT_LIFT_LEAVES and index == 0 else "ordinary_non_leaf0_depth3_compact"
                items.append(selection(leaf_index, f"depth3_terminal_{index}", terminal["bound"], terminal["pool_assignment"], terminal["depth"], klass, None))
    return items, replaced


def selection(leaf_index, terminal_id, bound, assignment, depth, klass, residual):
    item = {
        "leaf_index": leaf_index,
        "terminal_id": terminal_id,
        "expected_bound": float(bound),
        "pool_assignment": assignment,
        "depth": int(depth),
        "mechanism_class": klass,
        "export_required": True,
        "selection_reason": klass,
    }
    if residual is not None:
        item["residual_pair_assignment"] = residual
    return item


def strategy_for(klass):
    if klass == "high_parent_lift_depth3":
        return "per_row_rational_reconstruction"
    return "common_denominator_upward_integer"


def solve_manifest_item(item, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows):
    leaf_index = item["leaf_index"]
    leaf = finite[leaf_index]
    fixed = hybrid.terminal_fixed(item, leaf)
    rows, rhs, first_rows = provenance.row_system_for_leaf(
        leaf_index, leaf, source_by_index[leaf_index], edges, triads, weights, adj, root_cuts, parent_rows
    )
    rows, rhs = hybrid.explicit.explicit_terminal_rows(rows, rhs, fixed)
    objective, result = hybrid.explicit.solve_explicit(rows, rhs, weights)
    duals = hybrid.np.maximum(-hybrid.np.array(result.ineqlin.marginals), 0.0)
    hidden_lower = hybrid.np.maximum(hybrid.np.array(result.lower.marginals), 0.0)
    hidden_upper = hybrid.np.maximum(-hybrid.np.array(result.upper.marginals), 0.0)
    expected_strategy = strategy_for(item["mechanism_class"])
    if expected_strategy == "per_row_rational_reconstruction":
        certificate = hybrid.rational_certificate(item, rows, rhs, weights, duals)
        multipliers = rational_multipliers(rows, duals)
        certificate["positive_rows"] = positive_rational_rows(rows, rhs, multipliers)
    else:
        certificate = hybrid.integer_certificate(item, rows, rhs, weights, duals)
        certificate["positive_rows"] = positive_integer_rows(rows, rhs, duals, certificate["selected"])
    failures = hybrid.failures_for(item, objective, hidden_lower, hidden_upper, certificate)
    selected_strategy = certificate["strategy"]
    if selected_strategy != expected_strategy:
        failures.append("strategy_class_mismatch")
    return {
        **item,
        "selected_strategy": selected_strategy,
        "fixed_digest": digest({str(v + 1): float(value) for v, value in sorted(fixed.items())}),
        "row_system_digest": digest({"leaf_index": leaf_index, "first_family_rows": first_rows, "explicit_row_count": len(rows)}),
        "floating_objective": objective,
        "expected_error": objective - item["expected_bound"],
        "hidden_solver_bound_dual_max": float(max(hybrid.np.max(hidden_lower), hybrid.np.max(hidden_upper))),
        "certificate": certificate,
        "failure_reasons": failures,
        "status": "export_success" if not failures else "export_failed",
    }


def rational_multipliers(rows, duals):
    multipliers = []
    for row, value in zip(rows, duals):
        if row["family"] == "variable_lower_bound" or value <= 1e-10:
            multipliers.append(Fraction(0))
        else:
            multipliers.append(Fraction(float(value)).limit_denominator(hybrid.RATIONAL_CAP))
    return multipliers


def positive_rational_rows(rows, rhs, multipliers):
    positive = []
    for row, row_rhs, multiplier in zip(rows, rhs, multipliers):
        if multiplier == 0:
            continue
        ref = row_payload(row, row_rhs)
        ref["multiplier"] = frac_json(multiplier)
        positive.append(ref)
    return positive


def positive_integer_rows(rows, rhs, duals, selected):
    if selected is None:
        return []
    denominator = selected["denominator"]
    positive = []
    for row, row_rhs, value in zip(rows, rhs, duals):
        if row["family"] == "variable_lower_bound":
            continue
        numerator = max(0, int(math.ceil(float(value) * denominator - 1e-9)))
        if numerator == 0:
            continue
        ref = row_payload(row, row_rhs)
        ref["multiplier"] = {"num": numerator, "den": denominator}
        positive.append(ref)
    return positive


def row_payload(row, row_rhs):
    return {
        "family": row["family"],
        "id": row["id"],
        "rhs": int(round(float(row_rhs))),
        "coefficients": [[vertex + 1, coeff] for vertex, coeff in sorted(row_coefficients(row).items())],
    }


def row_coefficients(row):
    if "coeff_map" in row:
        return {int(vertex): int(round(float(coeff))) for vertex, coeff in row["coeff_map"].items()}
    if row["coeffs"] is None:
        return {int(vertex): 1 for vertex in row["vertices"]}
    return {int(vertex): int(round(float(coeff))) for vertex, coeff in zip(row["vertices"], row["coeffs"])}


def frac_json(value):
    return {"num": int(value.numerator), "den": int(value.denominator)}


def terminal_key(item):
    residual = item.get("residual_pair_assignment", {})
    return json.dumps(
        {
            "leaf": item["leaf_index"],
            "depth": item["depth"],
            "pool": item["pool_assignment"],
            "residual": residual,
        },
        sort_keys=True,
    )


def main():
    replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, _selected = run_context()
    items, replaced = manifest(replay)
    terminals = [solve_manifest_item(item, finite, source_by_index, edges, triads, weights, adj, root_cuts, parent_rows) for item in items]
    successes = [row for row in terminals if row["status"] == "export_success"]
    total_rows = sum(hybrid.row_count(row) for row in successes)
    failures = collect_failures(items, terminals, successes, total_rows, replaced, replay)
    report = {
        "schema": "forge.hadwiger.w607_full_terminal_export_preflight.v1",
        "authority": "full_terminal_export_preflight_only_no_root_or_semantic_partition_authority",
        "second_opinion": {"agent": "Tesla", "decision": "approve_full_terminal_export_preflight"},
        "source_binding": source_binding(replay, source, edges, weights, root_cuts, parent_rows),
        "class_policy": {
            "high_parent_lift_depth3": "per_row_rational_reconstruction",
            "ordinary_non_leaf0_depth3_compact": "common_denominator_upward_integer",
            "leaf0_residual_child": "common_denominator_upward_integer",
            "ordinary_leaf0_closed": "common_denominator_upward_integer",
        },
        "manifest": {"expected_terminal_count": EXPECTED_TERMINALS, "actual_terminal_count": len(items), "replaced_triggered_leaf0_terminals": replaced, "duplicate_keys": duplicate_keys(items)},
        "gates": {"export_gate": EXPORT_GATE, "terminal_allowance": ALLOWANCE, "total_row_budget": TOTAL_ROW_BUDGET},
        "summary": summary(terminals, successes, total_rows, replay),
        "class_summaries": class_summaries(terminals),
        "terminals": terminals,
        "failure_reasons": failures,
        "status": "fund_full_mixed_tree_terminal_replay_checker" if not failures else "retire_or_revise_full_terminal_export_preflight",
    }
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps(jsonable({key: value for key, value in report.items() if key != "terminals"}), indent=2))


def collect_failures(items, terminals, successes, total_rows, replaced, replay):
    failures = []
    if len(items) != EXPECTED_TERMINALS:
        failures.append("terminal_count_mismatch")
    if duplicate_keys(items):
        failures.append("duplicate_manifest_keys")
    if len(replaced) != 1:
        failures.append("unexpected_triggered_leaf0_replacement_count")
    if len(successes) != len(terminals):
        failures.append("not_all_terminals_exported")
    if total_rows > TOTAL_ROW_BUDGET:
        failures.append("total_row_budget_exceeded")
    max_obj = max((hybrid.objective_of(row) for row in successes), default=float("inf"))
    if max_obj > EXPORT_GATE:
        failures.append("max_terminal_objective_above_export_gate")
    if max_obj > replay["final_mixed_max"] + ALLOWANCE:
        failures.append("max_terminal_objective_above_mixed_plus_allowance")
    for row in terminals:
        failures.extend(f"{row['leaf_index']}:{row['terminal_id']}:{reason}" for reason in row["failure_reasons"])
    return sorted(set(failures))


def duplicate_keys(items):
    seen = set()
    dup = []
    for item in items:
        key = terminal_key(item)
        if key in seen:
            dup.append(key)
        seen.add(key)
    return dup


def summary(terminals, successes, total_rows, replay):
    objectives = [(hybrid.objective_of(row), row["leaf_index"], row["terminal_id"]) for row in successes]
    argmax = max(objectives, default=None)
    return {
        "successes": len(successes),
        "total": len(terminals),
        "total_success_rows": total_rows,
        "max_terminal_certificate_objective": None if argmax is None else argmax[0],
        "argmax_terminal": None if argmax is None else {"leaf_index": argmax[1], "terminal_id": argmax[2]},
        "fresh_replay_final_mixed_max": replay["final_mixed_max"],
        "fresh_replay_argmax_leaf": replay["argmax_leaf"],
    }


def class_summaries(terminals):
    out = {}
    for row in terminals:
        klass = row["mechanism_class"]
        data = out.setdefault(klass, {"count": 0, "success": 0, "rows": 0, "max_objective": None})
        data["count"] += 1
        if row["status"] == "export_success":
            data["success"] += 1
            data["rows"] += hybrid.row_count(row)
            objective = hybrid.objective_of(row)
            data["max_objective"] = objective if data["max_objective"] is None else max(data["max_objective"], objective)
    return out


def source_binding(replay, source, edges, weights, root_cuts, parent_rows):
    return {"fresh_replay_path": str(REPLAY), "fresh_replay_digest": digest(replay), "first_family_source": str(SOURCE), "first_family_digest": digest(source), "graph_digest": graph_digest(edges, weights), "root_rows_digest": digest(root_cuts), "parent_rows_digest": digest(parent_rows)}


if __name__ == "__main__":
    main()
