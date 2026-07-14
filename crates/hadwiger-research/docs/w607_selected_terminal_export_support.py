import hashlib
import json
import math

import numpy as np

import run_w607_terminal_dual_provenance as provenance
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
REPLAY = CRATE / "docs" / "w607-fresh-mixed-branch-replay.json"
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"

TOP_LEAVES = [1, 6, 12, 2, 4, 9]
DENOMINATORS = [1024, 4096, 16384, 1048576, 16777216]
CANONICAL_DENOMINATOR_LIMIT = 4096
EXPORT_GATE = 586500.0
ALLOWANCE = 300.0
POSITIVE_TOL = 1e-8
BOUND_DUAL_TOL = 1e-7
NORMAL_ROW_BUDGET = 1500
LEAF0_ROW_BUDGET = 2500
TOTAL_ROW_BUDGET = 15000


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
    if isinstance(value, np.ndarray):
        return [jsonable(inner) for inner in value.tolist()]
    return value


def graph_digest(edges, weights):
    return digest(
        {
            "edges": [[a + 1, b + 1] for a, b in sorted(edges)],
            "weights": [int(value) for value in weights],
        }
    )


def selected_terminals(replay):
    selections = []
    for leaf_index in TOP_LEAVES:
        leaf = replay["leaves"][leaf_index]
        terminal = leaf["terminal_certificates"][0]
        selections.append(
            selection(
                leaf_index,
                "depth3_terminal_0",
                terminal["bound"],
                terminal["pool_assignment"],
                terminal["depth"],
                "none",
                True,
                "top_six_final_terminal",
            )
        )
    leaf0 = replay["leaves"][0]
    for closure_index, closure in enumerate(leaf0["residual_closures"]):
        if not closure["triggered"]:
            continue
        terminal = closure["terminal"]
        selections.append(
            selection(
                0,
                f"closure_{closure_index}_trigger_terminal",
                terminal["bound"],
                terminal["pool_assignment"],
                terminal["depth"],
                "leaf0_depth4_residual_pair_closure",
                False,
                "leaf0_exception_trigger_audit",
            )
        )
        for child_index, child in enumerate(closure["children"]):
            item = selection(
                0,
                f"closure_{closure_index}_child_{child_index}",
                child["bound"],
                child["pool_assignment"],
                terminal["depth"] + 1,
                "leaf0_residual_pair_child",
                True,
                "leaf0_exception_child_export",
            )
            item["residual_pair_assignment"] = child["assignment"]
            selections.append(item)
    return selections


def selection(leaf_index, terminal_id, bound, assignment, depth, rule, required, reason):
    return {
        "leaf_index": leaf_index,
        "terminal_id": terminal_id,
        "expected_bound": float(bound),
        "pool_assignment": assignment,
        "depth": int(depth),
        "exceptional_rule": rule,
        "export_required": required,
        "selection_reason": reason,
    }


def row_coefficients(row):
    coeffs = {}
    if "coeff_map" in row:
        for vertex, coeff in row["coeff_map"].items():
            coeffs[int(vertex)] = int(round(float(coeff)))
    elif row["coeffs"] is None:
        for vertex in row["vertices"]:
            coeffs[int(vertex)] = 1
    else:
        for vertex, coeff in zip(row["vertices"], row["coeffs"]):
            coeffs[int(vertex)] = int(round(float(coeff)))
    return coeffs


def row_reference(row):
    out = {"family": row["family"], "id": row["id"]}
    if "vertices" in row:
        out["vertices"] = [int(vertex) + 1 for vertex in row["vertices"]]
    if row["family"] in {"root_rank", "first_family_leaf_rows"}:
        out["support_digest"] = digest(out.get("vertices", []))
    if row["family"] == "first_family_leaf_rows":
        out["source"] = row["source"]
    return out


def explicit_bound_rows(rows, rhs):
    explicit_rows = list(rows)
    explicit_rhs = [float(value) for value in rhs]
    for vertex in range(parent.N):
        explicit_rows.append({"family": "variable_upper_bound", "id": f"x{vertex + 1}_upper_bound", "vertices": (vertex,), "coeffs": (1.0,)})
        explicit_rhs.append(1.0)
    for vertex in range(parent.N):
        explicit_rows.append({"family": "variable_lower_bound", "id": f"x{vertex + 1}_lower_bound", "vertices": (vertex,), "coeffs": (-1.0,)})
        explicit_rhs.append(0.0)
    return explicit_rows, np.array(explicit_rhs, dtype=float)


def rounded_attempt(rows, rhs, multipliers, weights, denominator, skip_lower=False):
    coverage = [0] * parent.N
    objective_num = 0
    positive = []
    upper_count = 0
    upper_objective_num = 0
    lower_count = 0
    lower_mass = 0.0
    for row, row_rhs, value in zip(rows, rhs, multipliers):
        if skip_lower and row["family"] == "variable_lower_bound":
            if value > POSITIVE_TOL:
                lower_count += 1
                lower_mass += float(value)
            continue
        numerator = max(0, int(math.ceil(float(value) * denominator - 1e-9)))
        if numerator == 0:
            continue
        row_rhs = int(round(float(row_rhs)))
        objective_num += numerator * row_rhs
        if row["family"] == "variable_upper_bound":
            upper_count += 1
            upper_objective_num += numerator
        for vertex, coeff in row_coefficients(row).items():
            coverage[vertex] += numerator * coeff
        ref = row_reference(row)
        ref["rhs"] = row_rhs
        ref["numerator"] = numerator
        positive.append(ref)
    slacks = [coverage[v] - int(round(float(weights[v]))) * denominator for v in range(parent.N)]
    min_slack = min(slacks)
    return {
        "denominator": denominator,
        "objective_num": objective_num,
        "objective_bound": objective_num / denominator,
        "min_slack": min_slack,
        "argmin_vertices": [i + 1 for i, value in enumerate(slacks) if value == min_slack][:12],
        "positive_row_count": len(positive),
        "variable_upper_row_count": upper_count,
        "variable_upper_objective": upper_objective_num / denominator,
        "lower_bound_positive_count_reported_not_exported": lower_count,
        "lower_bound_positive_mass_reported_not_exported": lower_mass,
        "positive_rows": positive,
    }


def row_budget(selection):
    return LEAF0_ROW_BUDGET if selection["leaf_index"] == 0 else NORMAL_ROW_BUDGET


def terminal_fixed(selection, finite_leaf):
    base = provenance.full16.fixed_from_leaf(finite_leaf)
    return provenance.fixed_from_pool(base, selection["pool_assignment"])


def run_context():
    replay = json.loads(REPLAY.read_text())
    source = json.loads(SOURCE.read_text())
    source_by_index = {row["leaf_index"]: row for row in source["leaves"]}
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = provenance.parent.triangles(adj)
    root_cuts = provenance.parent_lift.root_cuts(weights, adj)
    parent_rows = [provenance.parent_lift.parent_row(weights), provenance.bundle.plateau.p_parent_row(weights)]
    _expanded, leaves = provenance.affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    selections = selected_terminals(replay)
    return replay, source, source_by_index, edges, weights, adj, triads, root_cuts, parent_rows, finite, selections
