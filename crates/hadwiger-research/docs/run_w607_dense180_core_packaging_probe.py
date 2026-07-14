import itertools
import json
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
ALIGN = CRATE / "docs" / "w607-first-family-alignment-probe.json"
OUT = CRATE / "docs" / "w607-dense180-core-packaging-probe.json"

MAX_MWIS_SOLVES = 8
MWIS_TIME_LIMIT = 20.0
CORE_DROP_GATE = 250.0
VARIANT_ALPHA_SPREAD_GATE = 5000.0


def solve_mwis(vertices, weights, adj):
    local = {vertex: index for index, vertex in enumerate(vertices)}
    rows = []
    for i, a in enumerate(vertices):
        for b in vertices[i + 1 :]:
            if b in adj[a]:
                row = np.zeros(len(vertices))
                row[i] = 1.0
                row[local[b]] = 1.0
                rows.append(row)
    constraints = LinearConstraint(np.vstack(rows), -np.inf, np.ones(len(rows))) if rows else None
    start = time.time()
    result = milp(
        c=-weights[list(vertices)],
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": MWIS_TIME_LIMIT, "mip_rel_gap": 0.0},
    )
    seconds = time.time() - start
    gap = getattr(result, "mip_gap", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    alpha = int(round(-result.fun)) if ok else None
    return {"alpha_w": alpha, "success": ok, "mip_gap": gap, "seconds": seconds}


def row_effect(edges, triads, weights, root_cuts, parent_rows, vertices, alpha):
    before, root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    lhs = float(np.dot(weights[list(vertices)], root_x[list(vertices)]))
    row = {"coefficients": {vertex: float(weights[vertex]) for vertex in vertices}, "rhs": float(alpha)}
    after, new_x = branch_slack.solve_lp(
        edges,
        triads,
        weights,
        root_cuts,
        parent_rows,
        cg_cuts=[row],
        solution=True,
    )
    return {
        "root_objective_before": before,
        "root_lhs": lhs,
        "alpha_w": alpha,
        "raw_violation": lhs - alpha,
        "root_objective_after": after,
        "root_drop": before - after,
        "post_row_x304": float(new_x[parent.BRANCH]),
    }


def overlap_matrix(items):
    rows = []
    for left, right in itertools.combinations(items, 2):
        a = set(left["vertices"])
        b = set(right["vertices"])
        rows.append(
            {
                "left_leaf": left["leaf_index"],
                "right_leaf": right["leaf_index"],
                "overlap": len(a & b),
                "union": len(a | b),
                "jaccard": len(a & b) / len(a | b),
            }
        )
    return rows


def main():
    align = json.loads(ALIGN.read_text())
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), branch_slack.p_parent_row(weights)]
    supports = [
        {
            "leaf_index": row["leaf_index"],
            "center": row["center"],
            "alpha_w": row["alpha_w"],
            "support_digest": row["support_digest"],
            "vertices": [vertex - 1 for vertex in row["support_vertices"]],
        }
        for row in align["support_signatures"]
    ]
    core = sorted(set.intersection(*(set(row["vertices"]) for row in supports)))
    union = sorted(set.union(*(set(row["vertices"]) for row in supports)))
    variants = [
        {
            "leaf_index": row["leaf_index"],
            "center": row["center"],
            "vertices": sorted(set(row["vertices"]) - set(core)),
        }
        for row in supports
    ]
    mwis_used = 0
    core_mwis = solve_mwis(core, weights, adj)
    mwis_used += 1
    union_mwis = solve_mwis(union, weights, adj)
    mwis_used += 1
    variant_results = []
    for variant in variants:
        if mwis_used >= MAX_MWIS_SOLVES:
            break
        result = solve_mwis(variant["vertices"], weights, adj)
        mwis_used += 1
        variant_results.append({**variant, "mwis": result})
    core_effect = row_effect(edges, triads, weights, root_cuts, parent_rows, core, core_mwis["alpha_w"]) if core_mwis["success"] else None
    union_effect = row_effect(edges, triads, weights, root_cuts, parent_rows, union, union_mwis["alpha_w"]) if union_mwis["success"] else None
    variant_alphas = [row["mwis"]["alpha_w"] for row in variant_results if row["mwis"]["success"]]
    alpha_allowances = [
        {
            "leaf_index": row["leaf_index"],
            "variant_size": len(row["vertices"]),
            "variant_alpha_w": row["mwis"]["alpha_w"],
            "core_plus_variant_alpha_sum": None
            if not core_mwis["success"] or not row["mwis"]["success"]
            else core_mwis["alpha_w"] + row["mwis"]["alpha_w"],
            "target_support_alpha": 258701,
        }
        for row in variant_results
    ]
    variant_spread = max(variant_alphas) - min(variant_alphas) if variant_alphas else None
    status = "retire_common_core_packaging"
    if core_effect and core_effect["root_drop"] >= CORE_DROP_GATE:
        status = "fund_common_core_export"
    elif variant_spread is not None and variant_spread <= VARIANT_ALPHA_SPREAD_GATE and len(variant_results) == len(variants):
        status = "fund_variant_disjunction"
    report = {
        "schema": "forge.hadwiger.w607_dense180_core_packaging_probe.v1",
        "authority": "diagnostic_common_core_packaging_not_export_authority",
        "second_opinion": {
            "agent": "Dewey",
            "decision": "approve_bounded_diagnostic",
            "primary_failure_mode": "common_core_visually_large_but_polyhedrally_inert",
        },
        "source_binding": {
            "alignment_artifact": str(ALIGN),
            "active_template": align["active_template"],
            "active_leaf_ids": align["active_leaf_ids"],
        },
        "mwis_budget": {"max": MAX_MWIS_SOLVES, "used": mwis_used, "time_limit_seconds": MWIS_TIME_LIMIT},
        "core": {
            "size": len(core),
            "vertices": [vertex + 1 for vertex in core],
            "mwis": core_mwis,
            "root_lp_effect": core_effect,
        },
        "union": {
            "size": len(union),
            "vertices": [vertex + 1 for vertex in union],
            "mwis": union_mwis,
            "root_lp_effect": union_effect,
        },
        "variants": [
            {**row, "vertices": [vertex + 1 for vertex in row["vertices"]]}
            for row in variant_results
        ],
        "variant_overlap_matrix": overlap_matrix(variant_results),
        "support_overlap_matrix": align["pairwise_support_comparisons"],
        "alpha_decomposition": {
            "target_support_alpha": 258701,
            "core_alpha_w": core_mwis["alpha_w"],
            "variant_alpha_spread": variant_spread,
            "rows": alpha_allowances,
        },
        "gates": {
            "core_drop_gate": CORE_DROP_GATE,
            "variant_alpha_spread_gate": VARIANT_ALPHA_SPREAD_GATE,
        },
        "failure_reasons": [
            reason
            for reason, active in [
                ("core_mwis_failed", not core_mwis["success"]),
                ("union_mwis_failed", not union_mwis["success"]),
                ("core_row_root_drop_below_gate", not core_effect or core_effect["root_drop"] < CORE_DROP_GATE),
                ("variant_alpha_spread_large", variant_spread is None or variant_spread > VARIANT_ALPHA_SPREAD_GATE),
            ]
            if active
        ],
        "status": status,
    }
    if report["failure_reasons"] and status != "fund_variant_disjunction":
        report["status"] = "retire_common_core_packaging"
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key not in {"core", "union", "variants", "variant_overlap_matrix", "support_overlap_matrix"}}, indent=2))


if __name__ == "__main__":
    main()
