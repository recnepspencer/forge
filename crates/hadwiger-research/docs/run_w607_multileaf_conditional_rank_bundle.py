import hashlib
import json
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp

import run_w607_all_excluded_leaf_rank_diagnostic as leaf_rank
import run_w607_branch_slack_plateau_branch_tree as plateau
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
OUT_PATH = CRATE / "docs" / "w607-multileaf-conditional-rank-bundle.json"

LEAF_COUNT = 6
MAX_CANDIDATES = 48
MAX_MWIS_CALLS_PER_LEAF = 30
MWIS_TIME_LIMIT = 30.0
SIZES = (100, 140, 180, 220)
VIOLATION_GATE = 250.0
ACCEPT_DROP = 250.0
MEANINGFUL_DROP = 1000.0
FUND_TESTED_MAX = 592000.0
STRONG_TESTED_MAX = 590000.0
FUND_MAX_MOVEMENT = 3000.0
ROOT_JACCARD_WARN = 0.85
OLD_ROW_DIGEST = "11c64661386aafb00c3d00183cfa0e479cb224aef00aac6f3650baef74ff39c0"


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right) if left or right else 1.0


def fixed_summary(fixed):
    return {
        "included": [v + 1 for v, value in sorted(fixed.items()) if value == 1.0],
        "excluded": [v + 1 for v, value in sorted(fixed.items()) if value == 0.0],
    }


def order_by(rank, size, banned):
    ordered = [int(v) for v in np.lexsort((np.arange(parent.N), -rank)) if int(v) not in banned]
    return tuple(sorted(ordered[:size]))


def twohop(seeds, size, rank, adj, banned):
    seen = set(seeds) - banned
    for seed in list(seeds):
        seen.update(adj[seed])
    for vertex in list(seen):
        seen.update(adj[vertex])
    seen -= banned
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:size]))


def dense_expand(seeds, size, weights, adj, rank, banned):
    chosen = [v for v in seeds if v not in banned]
    chosen_set = set(chosen)
    frontier = set()
    for seed in chosen:
        frontier.update(adj[seed])
    frontier -= chosen_set | banned
    while len(chosen) < size and frontier:
        def score(vertex):
            contact = sum(weights[other] for other in chosen if other in adj[vertex])
            rank_contact = sum(rank[other] for other in chosen if other in adj[vertex])
            return (rank_contact * 1e6 + contact * 10 + rank[vertex], -vertex)

        vertex = max(frontier, key=score)
        frontier.remove(vertex)
        chosen.append(vertex)
        chosen_set.add(vertex)
        frontier.update(adj[vertex] - chosen_set - banned)
    if len(chosen) < size:
        for vertex in np.lexsort((np.arange(parent.N), -rank)):
            vertex = int(vertex)
            if vertex not in chosen_set and vertex not in banned:
                chosen.append(vertex)
                chosen_set.add(vertex)
            if len(chosen) == size:
                break
    return tuple(sorted(chosen[:size]))


def candidate_supports(weights, x, adj, fixed):
    banned = {v for v, value in fixed.items() if value == 0.0}
    included = {v for v, value in fixed.items() if value == 1.0}
    for vertex in included:
        banned.update(adj[vertex])
    rank_vectors = {
        "wx": weights * x,
        "frac": weights * np.minimum(x, 1.0 - x),
        "x": x,
    }
    boundary = sorted(
        {n for v in banned | included for n in adj[v]} - banned,
        key=lambda v: (-weights[v] * x[v], v),
    )[:10]
    supports = []
    seed_report = {}
    for label, rank in rank_vectors.items():
        top = [int(v) for v in np.lexsort((np.arange(parent.N), -rank)) if int(v) not in banned][:16]
        seed_report[label] = [v + 1 for v in top[:10]]
        for size in SIZES:
            supports.append((f"{label}_top{size}", order_by(rank, size, banned)))
        for center in top[:8]:
            for size in SIZES:
                supports.append((f"{label}_twohop{size}_{center+1}", twohop([center], size, rank, adj, banned)))
                supports.append((f"{label}_dense{size}_{center+1}", dense_expand([center], size, weights, adj, rank, banned)))
        for center in boundary[:6]:
            for size in (100, 140, 180):
                supports.append((f"{label}_boundary{size}_{center+1}", dense_expand([center], size, weights, adj, rank, banned)))
    return supports, seed_report


def root_supports(weights, adj):
    return [parent.pocket(name, weights, adj) for name, _alpha in parent.ACCEPTED]


def dedupe_supports(raw, root_prior, accepted_prior):
    kept = []
    seen = set()
    seen_vertices = []
    for name, vertices in raw:
        if not (leaf_rank.HARD_MIN_SIZE <= len(vertices) <= leaf_rank.HARD_MAX_SIZE):
            continue
        key = support_hash(vertices)
        if key in seen:
            continue
        seen.add(key)
        if any(jaccard(vertices, prior) >= 0.90 for prior in seen_vertices):
            continue
        seen_vertices.append(vertices)
        max_root = max(jaccard(vertices, prior) for prior in root_prior)
        max_prior = max((jaccard(vertices, prior) for prior in accepted_prior), default=0.0)
        kept.append((name, vertices, max_root, max_prior))
        if len(kept) >= MAX_CANDIDATES:
            break
    return kept


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


def leaf_stats(weights, x, fixed):
    fractional = [v for v in range(parent.N) if 1e-7 < x[v] < 1.0 - 1e-7]
    return {
        "fractional_count": len(fractional),
        "heavy_third_count": sum(1 for v in fractional if weights[v] >= 10000 and abs(x[v] - 1.0 / 3.0) <= 1e-6),
        "top_fractional": [
            {"vertex": v + 1, "x": x[v], "weight": weights[v], "weighted_x": weights[v] * x[v]}
            for v in sorted(fractional, key=lambda v: (-weights[v] * x[v], -weights[v], v))[:12]
        ],
        "fixed": fixed_summary(fixed),
    }


def analyze_leaf(leaf, leaf_index, edges, triads, weights, adj, root_cuts, rows, root_prior, accepted_prior):
    fixed = dict(leaf["fixed"])
    base_obj, x = leaf_rank.solve_lp(edges, triads, weights, root_cuts, rows, fixed, True)
    raw, seeds = candidate_supports(weights, x, adj, fixed)
    candidates = dedupe_supports(raw, root_prior, accepted_prior)
    candidates = sorted(candidates, key=lambda item: -float(np.dot(weights[list(item[1])], x[list(item[1])])))
    tested = []
    accepted = []
    cumulative_cuts = []
    current_obj = base_obj
    for name, vertices, root_overlap, prior_overlap in candidates[:MAX_MWIS_CALLS_PER_LEAF]:
        alpha, ok, gap, seconds = solve_mwis(vertices, weights, adj)
        lhs = float(np.dot(weights[list(vertices)], x[list(vertices)]))
        violation = lhs - alpha if ok else None
        reason = "not_solved"
        single_drop = 0.0
        cumulative_drop = 0.0
        support_digest = support_hash(vertices)
        if ok:
            reason = "low_violation"
        if ok and violation >= VIOLATION_GATE:
            trial = (vertices, alpha)
            trial_obj = leaf_rank.solve_lp(edges, triads, weights, root_cuts + cumulative_cuts + [trial], rows, fixed)
            single_drop = current_obj - trial_obj
            if single_drop >= ACCEPT_DROP:
                reason = "accepted"
                cumulative_cuts.append(trial)
                current_obj = trial_obj
                cumulative_drop = base_obj - current_obj
                accepted.append(
                    {
                        "name": name,
                        "support_digest": support_digest,
                        "size": len(vertices),
                        "alpha_w": alpha,
                        "single_drop": single_drop,
                        "cumulative_drop": cumulative_drop,
                        "old_row_jaccard": None,
                        "root_jaccard": root_overlap,
                        "prior_accepted_jaccard": prior_overlap,
                    }
                )
            else:
                reason = "tiny_lp_drop"
        tested.append(
            {
                "name": name,
                "size": len(vertices),
                "support_digest": support_digest,
                "same_as_old_row": support_digest == OLD_ROW_DIGEST,
                "alpha_w": alpha,
                "leaf_lhs": lhs,
                "violation": violation,
                "single_drop": single_drop,
                "root_jaccard": root_overlap,
                "root_overlap_warning": root_overlap >= ROOT_JACCARD_WARN,
                "prior_accepted_jaccard": prior_overlap,
                "solver_success": ok,
                "mip_gap": gap,
                "seconds": seconds,
                "decision": reason,
            }
        )
    final_obj = current_obj
    return {
        "leaf_index": leaf_index,
        **fixed_summary(fixed),
        "baseline_objective": base_obj,
        "final_objective": final_obj,
        "drop": base_obj - final_obj,
        "candidate_count": len(candidates),
        "mwis_call_count": len(tested),
        "accepted_row_count": len(accepted),
        "seed_vertices": seeds,
        "stats": leaf_stats(weights, x, fixed),
        "accepted_rows": accepted,
        "tested_rows": tested,
    }, [row for row, _alpha in cumulative_cuts]


def classify_failure(reports, initial_max, final_max):
    meaningful = [row for row in reports if row["drop"] >= MEANINGFUL_DROP]
    if not meaningful:
        return "no_multi_leaf_pockets"
    if len(meaningful) < 3:
        return "one_or_two_leaf_only_pockets"
    if final_max > FUND_TESTED_MAX:
        return "worst_tested_leaf_remains_high"
    if initial_max - final_max < FUND_MAX_MOVEMENT:
        return "max_movement_too_small"
    return "fundable_bundle_signal"


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "x"}
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
    rows = [parent_lift.parent_row(weights), plateau.p_parent_row(weights)]
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    selected = sorted(enumerate(finite), key=lambda item: -item[1]["upper"])[:LEAF_COUNT]
    root_prior = root_supports(weights, adj)
    reports = []
    accepted_prior = []
    for leaf_index, leaf in selected:
        report, accepted_supports = analyze_leaf(
            leaf,
            leaf_index,
            edges,
            triads,
            weights,
            adj,
            root_cuts,
            rows,
            root_prior,
            accepted_prior,
        )
        accepted_prior.extend(accepted_supports)
        reports.append(report)
    initial_max = max(row["baseline_objective"] for row in reports)
    final_max = max(row["final_objective"] for row in reports)
    meaningful_count = sum(1 for row in reports if row["drop"] >= MEANINGFUL_DROP)
    reusable_family_count = sum(
        1 for row in reports for accepted in row["accepted_rows"] if accepted["support_digest"] != OLD_ROW_DIGEST
    )
    status = "RetireMultileafConditionalRankBundle"
    if (
        meaningful_count >= 3
        and final_max <= FUND_TESTED_MAX
        and initial_max - final_max >= FUND_MAX_MOVEMENT
    ):
        status = "FundMultileafConditionalRankReplayDesign"
    if final_max <= STRONG_TESTED_MAX or reusable_family_count >= 3:
        status = "StrongMultileafConditionalRankSignal"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_multileaf_conditional_rank_bundle.v1",
            "authority": "diagnostic_branch_leaf_proof_object_precheck",
            "second_agent_verdict": "run_once_measure_tested_leaf_max_not_sum_of_drops",
            "leaf_count": len(reports),
            "initial_tested_max": initial_max,
            "final_tested_max": final_max,
            "tested_max_movement": initial_max - final_max,
            "meaningful_leaf_drop_count": meaningful_count,
            "reusable_non_old_accepted_count": reusable_family_count,
            "failure_classification": classify_failure(reports, initial_max, final_max),
            "gates": {
                "leaf_count": LEAF_COUNT,
                "max_candidates": MAX_CANDIDATES,
                "max_mwis_calls_per_leaf": MAX_MWIS_CALLS_PER_LEAF,
                "mwis_time_limit": MWIS_TIME_LIMIT,
                "violation_gate": VIOLATION_GATE,
                "accept_drop": ACCEPT_DROP,
                "meaningful_drop": MEANINGFUL_DROP,
                "fund_tested_max": FUND_TESTED_MAX,
                "strong_tested_max": STRONG_TESTED_MAX,
                "fund_max_movement": FUND_MAX_MOVEMENT,
            },
            "failure_modes": [
                "one_leaf_only_pocket",
                "same_old_wx_dense220_152_without_family_structure",
                "large_raw_violations_tiny_lp_drops",
                "drops_on_easy_leaves_worst_leaf_remains_high",
                "mwis_timeouts_on_promising_supports",
                "conditional_rows_not_replayable_cleanly",
            ],
            "leaves": reports,
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
