import hashlib
import json
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp

import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
FIRST_PASS = CRATE / "docs" / "w607-multileaf-conditional-rank-bundle.json"
OUT_PATH = CRATE / "docs" / "w607-full-tree-rank-family.json"

MAX_NEW_MWIS = 24
MWIS_TIME_LIMIT = 8.0
VIOLATION_GATE = 250.0
ACCEPT_DROP = 250.0
FUND_MAX = 592000.0
STRONG_MAX = 590000.0
FUND_MOVEMENT = 3000.0


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def fixed_summary(fixed):
    return {
        "included": [v + 1 for v, value in sorted(fixed.items()) if value == 1.0],
        "excluded": [v + 1 for v, value in sorted(fixed.items()) if value == 0.0],
    }


def known_alpha_table():
    artifact = json.loads(FIRST_PASS.read_text())
    out = {}
    for leaf in artifact["leaves"]:
        for row in leaf["tested_rows"]:
            if row["solver_success"] and row["alpha_w"] is not None:
                out[row["support_digest"]] = {
                    "alpha_w": int(row["alpha_w"]),
                    "source": "multileaf_first_pass_tested",
                    "name": row["name"],
                }
    return out


def solve_mwis(vertices, weights, adj):
    local = {v: i for i, v in enumerate(vertices)}
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
    return alpha, ok, gap, seconds


def fixed_from_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def banned_vertices(fixed, adj):
    banned = {v for v, value in fixed.items() if value == 0.0}
    for vertex, value in fixed.items():
        if value == 1.0:
            banned.update(adj[vertex])
    return banned


def top_centers(weights, x, fixed, adj, count=2):
    banned = banned_vertices(fixed, adj)
    centers = []
    for vertex in np.lexsort((np.arange(parent.N), -(weights * x))):
        vertex = int(vertex)
        if vertex not in banned and vertex not in centers:
            centers.append(vertex)
        if len(centers) == count:
            break
    return centers


def candidate_rows(weights, x, fixed, adj):
    centers = top_centers(weights, x, fixed, adj, 2)
    rows = []
    if centers:
        rows.append(
            {
                "template_id": "dense220_top_wx_center_1",
                "center": centers[0],
                "vertices": bundle.dense_expand([centers[0]], 220, weights, adj, weights * x, banned_vertices(fixed, adj)),
            }
        )
    if len(centers) > 1:
        rows.append(
            {
                "template_id": "dense180_top_wx_center_2",
                "center": centers[1],
                "vertices": bundle.dense_expand([centers[1]], 180, weights, adj, weights * x, banned_vertices(fixed, adj)),
            }
        )
    return rows


def analyze_leaf(leaf, leaf_index, edges, triads, weights, adj, root_cuts, rows, alpha_cache, new_mwis_left):
    fixed = fixed_from_leaf(leaf)
    baseline, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, rows, fixed, True)
    accepted = []
    tested = []
    current = baseline
    local_cuts = []
    for candidate in candidate_rows(weights, x, fixed, adj):
        vertices = candidate["vertices"]
        digest = support_hash(vertices)
        source = alpha_cache.get(digest)
        if source is None and new_mwis_left[0] > 0:
            alpha, ok, gap, seconds = solve_mwis(vertices, weights, adj)
            new_mwis_left[0] -= 1
            if ok:
                source = {"alpha_w": alpha, "source": "new_mwis", "name": candidate["template_id"]}
                alpha_cache[digest] = source
        elif source is None:
            alpha, ok, gap, seconds = None, False, None, 0.0
        else:
            alpha, ok, gap, seconds = source["alpha_w"], True, 0.0, 0.0
        lhs = float(np.dot(weights[list(vertices)], x[list(vertices)]))
        violation = lhs - alpha if ok else None
        decision = "not_solved"
        drop = 0.0
        if ok:
            decision = "low_violation"
        if ok and violation >= VIOLATION_GATE:
            trial = (vertices, alpha)
            objective = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts + local_cuts + [trial], rows, fixed)
            drop = current - objective
            if drop >= ACCEPT_DROP:
                decision = "accepted"
                local_cuts.append(trial)
                current = objective
                accepted.append(
                    {
                        "template_id": candidate["template_id"],
                        "center": candidate["center"] + 1,
                        "support_digest": digest,
                        "size": len(vertices),
                        "alpha_w": alpha,
                        "alpha_source": source["source"] if source else None,
                        "drop": drop,
                    }
                )
            else:
                decision = "tiny_lp_drop"
        tested.append(
            {
                "template_id": candidate["template_id"],
                "center": candidate["center"] + 1,
                "support_digest": digest,
                "size": len(vertices),
                "alpha_w": alpha,
                "alpha_source": source["source"] if source else None,
                "leaf_lhs": lhs,
                "violation": violation,
                "drop": drop,
                "solver_success": ok,
                "mip_gap": gap,
                "seconds": seconds,
                "decision": decision,
            }
        )
    return {
        "leaf_index": leaf_index,
        **fixed_summary(fixed),
        "baseline_objective": baseline,
        "final_objective": current,
        "drop": baseline - current,
        "accepted_count": len(accepted),
        "accepted_rows": accepted,
        "tested_rows": tested,
    }


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items()}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, tuple):
        return [clean(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    alpha_cache = known_alpha_table()
    new_mwis_left = [MAX_NEW_MWIS]
    reports = [
        analyze_leaf(leaf, index, edges, triads, weights, adj, root_cuts, rows, alpha_cache, new_mwis_left)
        for index, leaf in enumerate(finite)
    ]
    initial_max = max(row["baseline_objective"] for row in reports)
    final_max = max(row["final_objective"] for row in reports)
    top_six = sorted(reports, key=lambda row: -row["baseline_objective"])[:6]
    top_six_initial = max(row["baseline_objective"] for row in top_six)
    top_six_final = max(row["final_objective"] for row in top_six)
    accepted_leaves = sum(1 for row in reports if row["accepted_count"])
    avg_templates = sum(len(row["tested_rows"]) for row in reports) / len(reports)
    status = "RetireFullTreeRankFamily"
    if final_max <= FUND_MAX and initial_max - final_max >= FUND_MOVEMENT and accepted_leaves >= 8 and avg_templates <= 2:
        status = "FundFullTreeRankFamilyReplayDesign"
    if final_max <= STRONG_MAX:
        status = "StrongFullTreeRankFamily"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_full_tree_rank_family.v1",
            "authority": "diagnostic_template_family_schema_test",
            "template_rules": [
                "dense220_top_wx_center_1: dense_expand size 220 from highest w*x center not fixed-zero or neighbor-blocked",
                "dense180_top_wx_center_2: dense_expand size 180 from second highest w*x center not fixed-zero or neighbor-blocked",
            ],
            "leaf_count": len(reports),
            "max_new_mwis": MAX_NEW_MWIS,
            "new_mwis_used": MAX_NEW_MWIS - new_mwis_left[0],
            "initial_full_tree_max": initial_max,
            "final_full_tree_max": final_max,
            "full_tree_max_movement": initial_max - final_max,
            "initial_top_six_max": top_six_initial,
            "final_top_six_max": top_six_final,
            "top_six_max_movement": top_six_initial - top_six_final,
            "accepted_leaf_count": accepted_leaves,
            "average_templates_per_leaf": avg_templates,
            "gates": {
                "fund_max": FUND_MAX,
                "strong_max": STRONG_MAX,
                "fund_movement": FUND_MOVEMENT,
                "compact_leaf_count": 8,
                "average_template_cap": 2,
            },
            "failure_modes": [
                "remaining_leaves_do_not_accept_pattern",
                "rows_validate_but_are_lp_redundant",
                "apparent_family_duplicates_root_supports",
                "all_excluded_row_remains_exceptional",
                "new_bottleneck_shifts_outside_family",
                "mwis_cap_skips_hard_leaves",
                "template_depends_on_unstable_top_wx_centers",
            ],
            "leaves": reports,
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
