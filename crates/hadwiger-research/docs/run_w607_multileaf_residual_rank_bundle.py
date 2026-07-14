import hashlib
import json

import numpy as np

import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
FIRST_PASS = CRATE / "docs" / "w607-multileaf-conditional-rank-bundle.json"
OUT_PATH = CRATE / "docs" / "w607-multileaf-residual-rank-bundle.json"

MAX_MWIS_CALLS_PER_LEAF = 3
MAX_CANDIDATES = 16
NEAR_DUP_JACCARD = 0.82
MWIS_TIME_LIMIT = 5.0
VIOLATION_GATE = 250.0
ACCEPT_DROP = 250.0
ADDITIONAL_MEANINGFUL_DROP = 500.0
FUND_TESTED_MAX = 592000.0
STRONG_TESTED_MAX = 590000.0
FUND_MAX_MOVEMENT = 3000.0


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right) if left or right else 1.0


def leaf_fixed(report):
    fixed = {}
    for vertex in report["included"]:
        fixed[vertex - 1] = 1.0
    for vertex in report["excluded"]:
        fixed[vertex - 1] = 0.0
    return fixed


def regenerate_first_round(report, edges, triads, weights, adj, root_cuts, rows, root_prior):
    fixed = leaf_fixed(report)
    base_obj, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, rows, fixed, True)
    raw, _seeds = bundle.candidate_supports(weights, x, adj, fixed)
    candidates = bundle.dedupe_supports(raw, root_prior, [])
    candidates = sorted(candidates, key=lambda item: -float(np.dot(weights[list(item[1])], x[list(item[1])])))
    accepted_by_digest = {row["support_digest"] for row in report["accepted_rows"]}
    accepted = []
    for name, vertices, _root_overlap, _prior_overlap in candidates:
        digest = support_hash(vertices)
        if digest in accepted_by_digest:
            match = next(row for row in report["accepted_rows"] if row["support_digest"] == digest)
            accepted.append((name, vertices, int(match["alpha_w"])))
        if len(accepted) == len(accepted_by_digest):
            break
    if len(accepted) != len(accepted_by_digest):
        missing = sorted(accepted_by_digest - {support_hash(vertices) for _name, vertices, _alpha in accepted})
        raise ValueError(f"missing first-round supports for leaf {report['leaf_index']}: {missing}")
    return fixed, base_obj, accepted


def residual_dedupe(raw, root_prior, first_supports, sibling_supports):
    kept = []
    seen = set()
    reject_counts = {
        "duplicate": 0,
        "near_first_round": 0,
        "near_sibling_residual": 0,
        "near_seen": 0,
        "shape": 0,
    }
    seen_vertices = []
    for name, vertices in raw:
        if not (bundle.leaf_rank.HARD_MIN_SIZE <= len(vertices) <= bundle.leaf_rank.HARD_MAX_SIZE):
            reject_counts["shape"] += 1
            continue
        digest = support_hash(vertices)
        if digest in seen:
            reject_counts["duplicate"] += 1
            continue
        max_first = max((jaccard(vertices, support) for support in first_supports), default=0.0)
        if max_first >= NEAR_DUP_JACCARD:
            reject_counts["near_first_round"] += 1
            continue
        max_sibling = max((jaccard(vertices, support) for support in sibling_supports), default=0.0)
        if max_sibling >= 0.90:
            reject_counts["near_sibling_residual"] += 1
            continue
        if any(jaccard(vertices, prior) >= 0.90 for prior in seen_vertices):
            reject_counts["near_seen"] += 1
            continue
        seen.add(digest)
        seen_vertices.append(vertices)
        max_root = max(jaccard(vertices, prior) for prior in root_prior)
        kept.append((name, vertices, max_root, max_first, max_sibling))
        if len(kept) >= MAX_CANDIDATES:
            break
    return kept, reject_counts


def analyze_residual_leaf(report, edges, triads, weights, adj, root_cuts, rows, root_prior, sibling_supports):
    fixed, _base_obj, first_rows = regenerate_first_round(report, edges, triads, weights, adj, root_cuts, rows, root_prior)
    first_cuts = [(vertices, alpha) for _name, vertices, alpha in first_rows]
    first_supports = [vertices for _name, vertices, _alpha in first_rows]
    post_first_obj, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts + first_cuts, rows, fixed, True)
    raw, seeds = bundle.candidate_supports(weights, x, adj, fixed)
    candidates, reject_counts = residual_dedupe(raw, root_prior, first_supports, sibling_supports)
    candidates = sorted(candidates, key=lambda item: -float(np.dot(weights[list(item[1])], x[list(item[1])])))
    accepted = []
    tested = []
    current_obj = post_first_obj
    cumulative_cuts = []
    for name, vertices, root_overlap, first_overlap, sibling_overlap in candidates[:MAX_MWIS_CALLS_PER_LEAF]:
        alpha, ok, gap, seconds = bundle.solve_mwis(vertices, weights, adj)
        lhs = float(np.dot(weights[list(vertices)], x[list(vertices)]))
        violation = lhs - alpha if ok else None
        decision = "not_solved"
        single_drop = 0.0
        if ok:
            decision = "low_violation"
        if ok and violation >= VIOLATION_GATE:
            trial = (vertices, alpha)
            trial_obj = bundle.leaf_rank.solve_lp(
                edges,
                triads,
                weights,
                root_cuts + first_cuts + cumulative_cuts + [trial],
                rows,
                fixed,
            )
            single_drop = current_obj - trial_obj
            if single_drop >= ACCEPT_DROP:
                decision = "accepted"
                cumulative_cuts.append(trial)
                current_obj = trial_obj
                accepted.append(
                    {
                        "name": name,
                        "support_digest": support_hash(vertices),
                        "size": len(vertices),
                        "alpha_w": alpha,
                        "single_drop": single_drop,
                        "cumulative_additional_drop": post_first_obj - current_obj,
                        "root_jaccard": root_overlap,
                        "first_round_jaccard": first_overlap,
                        "sibling_residual_jaccard": sibling_overlap,
                    }
                )
            else:
                decision = "tiny_lp_drop"
        tested.append(
            {
                "name": name,
                "support_digest": support_hash(vertices),
                "size": len(vertices),
                "alpha_w": alpha,
                "leaf_lhs": lhs,
                "violation": violation,
                "single_drop": single_drop,
                "root_jaccard": root_overlap,
                "first_round_jaccard": first_overlap,
                "sibling_residual_jaccard": sibling_overlap,
                "solver_success": ok,
                "mip_gap": gap,
                "seconds": seconds,
                "decision": decision,
            }
        )
    return {
        "leaf_index": report["leaf_index"],
        "included": report["included"],
        "excluded": report["excluded"],
        "original_baseline_objective": report["baseline_objective"],
        "post_first_objective": post_first_obj,
        "first_round_drop": report["drop"],
        "final_objective": current_obj,
        "additional_drop": post_first_obj - current_obj,
        "total_drop": report["baseline_objective"] - current_obj,
        "first_round_rows": [
            {"name": name, "support_digest": support_hash(vertices), "size": len(vertices), "alpha_w": alpha}
            for name, vertices, alpha in first_rows
        ],
        "accepted_residual_rows": accepted,
        "accepted_residual_count": len(accepted),
        "candidate_count": len(candidates),
        "mwis_call_count": len(tested),
        "dedupe_reject_counts": reject_counts,
        "seed_vertices": seeds,
        "tested_rows": tested,
    }, [row for row, _alpha in cumulative_cuts]


def classify(reports, initial_max, final_max):
    additional = [row for row in reports if row["additional_drop"] >= ADDITIONAL_MEANINGFUL_DROP]
    if not additional:
        return "no_residual_family"
    if len(additional) < 2:
        return "one_leaf_residual_only"
    if final_max > FUND_TESTED_MAX:
        return "residual_drops_but_worst_remains_high"
    if initial_max - final_max < FUND_MAX_MOVEMENT:
        return "max_movement_still_too_small"
    return "fundable_residual_bundle_signal"


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


def write_report(first, reports, partial):
    initial_max = first["initial_tested_max"]
    first_final_max = first["final_tested_max"]
    final_max = max(row["final_objective"] for row in reports)
    additional_meaningful = sum(1 for row in reports if row["additional_drop"] >= ADDITIONAL_MEANINGFUL_DROP)
    residual_rows = sum(row["accepted_residual_count"] for row in reports)
    status = "RetireMultileafResidualRankBundle"
    if final_max <= FUND_TESTED_MAX and initial_max - final_max >= FUND_MAX_MOVEMENT and additional_meaningful >= 2:
        status = "FundResidualLeafRankReplayDesign"
    if final_max <= STRONG_TESTED_MAX or residual_rows >= 3:
        status = "StrongResidualLeafRankSignal"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_multileaf_residual_rank_bundle.v1",
            "authority": "diagnostic_residual_branch_leaf_falsification",
            "second_agent_verdict": "run_once_after_first_family_no_export",
            "partial": partial,
            "initial_tested_max": initial_max,
            "post_first_tested_max": first_final_max,
            "final_tested_max": final_max,
            "total_max_movement": initial_max - final_max,
            "additional_max_movement": first_final_max - final_max,
            "additional_meaningful_leaf_count": additional_meaningful,
            "accepted_residual_row_count": residual_rows,
            "failure_classification": classify(reports, initial_max, final_max),
            "gates": {
                "max_mwis_calls_per_leaf": MAX_MWIS_CALLS_PER_LEAF,
                "max_candidates": MAX_CANDIDATES,
                "mwis_time_limit": MWIS_TIME_LIMIT,
                "near_duplicate_jaccard": NEAR_DUP_JACCARD,
                "violation_gate": VIOLATION_GATE,
                "accept_drop": ACCEPT_DROP,
                "additional_meaningful_drop": ADDITIONAL_MEANINGFUL_DROP,
                "fund_tested_max": FUND_TESTED_MAX,
                "strong_tested_max": STRONG_TESTED_MAX,
                "fund_max_movement": FUND_MAX_MOVEMENT,
            },
            "failure_modes": [
                "support_near_duplicate_of_first_family",
                "same_geometry_slightly_recentered",
                "large_raw_violations_tiny_lp_drops",
                "residual_drops_but_worst_leaf_remains_high",
                "mwis_timeouts_on_promising_supports",
                "bespoke_rows_without_family_structure",
            ],
            "leaves": reports,
            "status": status,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    return report


def main():
    bundle.MWIS_TIME_LIMIT = MWIS_TIME_LIMIT
    first = json.loads(FIRST_PASS.read_text())
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    root_prior = bundle.root_supports(weights, adj)
    reports = []
    sibling_supports = []
    for leaf in first["leaves"]:
        report, accepted_supports = analyze_residual_leaf(
            leaf,
            edges,
            triads,
            weights,
            adj,
            root_cuts,
            rows,
            root_prior,
            sibling_supports,
        )
        reports.append(report)
        sibling_supports.extend(accepted_supports)
        write_report(first, reports, partial=True)
    report = write_report(first, reports, partial=False)
    print(json.dumps({k: v for k, v in report.items() if k != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
