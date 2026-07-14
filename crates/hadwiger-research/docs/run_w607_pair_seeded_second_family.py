import hashlib
import json
import time

import numpy as np

import run_w607_full_tree_rank_family as full_family
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-pair-seeded-second-family.json"

LEAF_COUNT = 6
MAX_MWIS_PER_LEAF = 6
MAX_TOTAL_MWIS = 36
TOP_SEEDS = 6
MAX_GENERATED_PER_LEAF = 24
MWIS_TIME_LIMIT = 3.0
SIZES = (180, 220)
VIOLATION_GATE = 500.0
ACCEPT_DROP = 500.0
DROP_RATIO_GATE = 0.25
JACCARD_GATE = 0.65
SINGLE_JACCARD_GATE = 0.75
SUCCESS_LEAVES = 3
SUCCESS_MOVEMENT = 3000.0
SUCCESS_MAX = 590200.0
STRONG_MAX = 590000.0


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right) if left or right else 1.0


def fixed_from_tree_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def fixed_from_report(report):
    fixed = {}
    for vertex in report["included"]:
        fixed[vertex - 1] = 1.0
    for vertex in report["excluded"]:
        fixed[vertex - 1] = 0.0
    return fixed


def banned_vertices(fixed, adj):
    banned = {v for v, value in fixed.items() if value == 0.0}
    for vertex, value in fixed.items():
        if value == 1.0:
            banned.update(adj[vertex])
    return banned


def balanced_pair_expand(seed_a, seed_b, size, weights, adj, rank, banned):
    chosen = []
    chosen_set = set()
    frontiers = []
    for seed in (seed_a, seed_b):
        if seed in banned:
            return ()
        chosen.append(seed)
        chosen_set.add(seed)
        frontiers.append(set(adj[seed]) - banned - chosen_set)
    turn = 0
    while len(chosen) < size and (frontiers[0] or frontiers[1]):
        side = turn % 2
        if not frontiers[side]:
            side = 1 - side
        if not frontiers[side]:
            break

        def score(vertex):
            contact = sum(weights[other] for other in chosen if other in adj[vertex])
            rank_contact = sum(rank[other] for other in chosen if other in adj[vertex])
            other_seed = seed_b if side == 0 else seed_a
            bridge = weights[other_seed] if other_seed in adj[vertex] else 0.0
            return (rank_contact * 1e6 + contact * 10 + bridge + rank[vertex], -vertex)

        vertex = max(frontiers[side], key=score)
        for frontier in frontiers:
            frontier.discard(vertex)
        chosen.append(vertex)
        chosen_set.add(vertex)
        for frontier in frontiers:
            frontier.update(adj[vertex] - chosen_set - banned)
        turn += 1
    if len(chosen) < size:
        for vertex in np.lexsort((np.arange(parent.N), -rank)):
            vertex = int(vertex)
            if vertex not in chosen_set and vertex not in banned:
                chosen.append(vertex)
                chosen_set.add(vertex)
            if len(chosen) == size:
                break
    return tuple(sorted(chosen[:size]))


def contribution_metrics(vertices, seed_a, seed_b, adj):
    left = {seed_a} | (adj[seed_a] & set(vertices))
    right = {seed_b} | (adj[seed_b] & set(vertices))
    return {
        "seed_a_neighborhood_share": len(left & set(vertices)) / len(vertices),
        "seed_b_neighborhood_share": len(right & set(vertices)) / len(vertices),
        "both_seed_neighborhood_share": len(left & right & set(vertices)) / len(vertices),
    }


def first_family_for_leaf(report, leaf, edges, triads, weights, adj, root_cuts, parent_rows):
    fixed = fixed_from_tree_leaf(leaf)
    _base, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, parent_rows, fixed, True)
    candidates = full_family.candidate_rows(weights, x, fixed, adj)
    by_digest = {support_hash(row["vertices"]): row for row in candidates}
    cuts = []
    supports = []
    for accepted in report["accepted_rows"]:
        row = by_digest[accepted["support_digest"]]
        cuts.append((row["vertices"], int(accepted["alpha_w"])))
        supports.append(row["vertices"])
    return cuts, supports


def generate_pair_candidates(weights, x, adj, fixed, first_supports, root_prior, prior_supports):
    banned = banned_vertices(fixed, adj)
    rank_vectors = {
        "wx": weights * x,
        "frac": weights * np.minimum(x, 1.0 - x),
    }
    out = []
    rejections = {"duplicate": 0, "jaccard": 0, "single_like": 0, "unbalanced": 0}
    seen = set()
    for label, rank in rank_vectors.items():
        top = [int(v) for v in np.lexsort((np.arange(parent.N), -rank)) if int(v) not in banned][:TOP_SEEDS]
        single = {}
        for seed in top:
            for size in SIZES:
                single[(seed, size)] = bundle.dense_expand([seed], size, weights, adj, rank, banned)
        for i, seed_a in enumerate(top):
            for seed_b in top[i + 1 :]:
                for size in SIZES:
                    vertices = balanced_pair_expand(seed_a, seed_b, size, weights, adj, rank, banned)
                    if not vertices:
                        continue
                    digest = support_hash(vertices)
                    if digest in seen:
                        rejections["duplicate"] += 1
                        continue
                    metrics = contribution_metrics(vertices, seed_a, seed_b, adj)
                    if min(metrics["seed_a_neighborhood_share"], metrics["seed_b_neighborhood_share"]) < 0.08:
                        rejections["unbalanced"] += 1
                        continue
                    single_j = max(jaccard(vertices, single[(seed_a, size)]), jaccard(vertices, single[(seed_b, size)]))
                    if single_j > SINGLE_JACCARD_GATE:
                        rejections["single_like"] += 1
                        continue
                    all_prior = root_prior + first_supports + prior_supports
                    max_j = max((jaccard(vertices, prior) for prior in all_prior), default=0.0)
                    if max_j > JACCARD_GATE:
                        rejections["jaccard"] += 1
                        continue
                    seen.add(digest)
                    out.append(
                        {
                            "name": f"{label}_pair_dense{size}_{seed_a+1}_{seed_b+1}",
                            "vertices": vertices,
                            "seed_a": seed_a,
                            "seed_b": seed_b,
                            "size": size,
                            "rank_label": label,
                            "seed_a_score": float(rank[seed_a]),
                            "seed_b_score": float(rank[seed_b]),
                            "single_center_jaccard": single_j,
                            "max_prior_jaccard": max_j,
                            **metrics,
                        }
                    )
                    if len(out) >= MAX_GENERATED_PER_LEAF:
                        return sorted(
                            out,
                            key=lambda row: -float(np.dot(weights[list(row["vertices"])], x[list(row["vertices"])])),
                        ), rejections
    return sorted(out, key=lambda row: -float(np.dot(weights[list(row["vertices"])], x[list(row["vertices"])]))), rejections


def solve_candidate(candidate, weights, adj):
    bundle.MWIS_TIME_LIMIT = MWIS_TIME_LIMIT
    return bundle.solve_mwis(candidate["vertices"], weights, adj)


def analyze_leaf(report, leaf, leaf_index, edges, triads, weights, adj, root_cuts, parent_rows, root_prior, prior_supports):
    fixed = fixed_from_tree_leaf(leaf)
    first_cuts, first_supports = first_family_for_leaf(report, leaf, edges, triads, weights, adj, root_cuts, parent_rows)
    post_first, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts + first_cuts, parent_rows, fixed, True)
    candidates, pre_rejections = generate_pair_candidates(weights, x, adj, fixed, first_supports, root_prior, prior_supports)
    tested = []
    accepted = []
    current = post_first
    local_cuts = []
    for candidate in candidates[:MAX_MWIS_PER_LEAF]:
        alpha, ok, gap, seconds = solve_candidate(candidate, weights, adj)
        lhs = float(np.dot(weights[list(candidate["vertices"])], x[list(candidate["vertices"])]))
        violation = lhs - alpha if ok else None
        drop = 0.0
        decision = "not_solved"
        if ok:
            decision = "low_violation"
        if ok and violation >= VIOLATION_GATE:
            trial = (candidate["vertices"], alpha)
            objective = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts + first_cuts + local_cuts + [trial], parent_rows, fixed)
            drop = current - objective
            ratio = drop / violation if violation > 0 else 0.0
            if drop >= ACCEPT_DROP and ratio >= DROP_RATIO_GATE:
                decision = "accepted"
                local_cuts.append(trial)
                current = objective
                accepted.append({**candidate, "alpha_w": alpha, "violation": violation, "drop": drop})
            else:
                decision = "tiny_or_inefficient_drop"
        tested.append(
            {
                **{key: value for key, value in candidate.items() if key != "vertices"},
                "support_digest": support_hash(candidate["vertices"]),
                "alpha_w": alpha,
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
        "included": report["included"],
        "excluded": report["excluded"],
        "post_first_objective": post_first,
        "final_objective": current,
        "additional_drop": post_first - current,
        "candidate_count": len(candidates),
        "pre_mwis_rejections": pre_rejections,
        "tested_rows": tested,
        "accepted_rows": [
            {
                **{key: value for key, value in row.items() if key != "vertices"},
                "support_digest": support_hash(row["vertices"]),
            }
            for row in accepted
        ],
        "accepted_count": len(accepted),
    }, [row["vertices"] for row in accepted]


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


def failure_classification(status, accepted_total, reports):
    if status != "RetirePairSeededSecondFamily":
        return "funded"
    if all(row["candidate_count"] == 0 for row in reports):
        return "no_genuine_pair_shaped_candidates"
    if accepted_total == 0:
        return "no_pair_rows"
    return "pair_drops_but_ceiling_remains_high"


def make_report(reports, total_mwis, partial):
    initial_max = max(row["post_first_objective"] for row in reports)
    final_max = max(row["final_objective"] for row in reports)
    accepted_leaf_count = sum(1 for row in reports if row["accepted_count"])
    accepted_total = sum(row["accepted_count"] for row in reports)
    status = "RetirePairSeededSecondFamily"
    if accepted_leaf_count >= SUCCESS_LEAVES and initial_max - final_max >= SUCCESS_MOVEMENT and final_max <= SUCCESS_MAX:
        status = "FundPairSeededSecondFamilyFollowup"
    if final_max <= STRONG_MAX or (accepted_leaf_count >= 4 and initial_max - final_max >= 2500):
        status = "StrongPairSeededSecondFamilySignal"
    return clean(
        {
            "schema": "forge.hadwiger.w607_pair_seeded_second_family.v1",
            "authority": "diagnostic_pair_seeded_leaf_rank_falsifier",
            "second_agent_verdict": "run_once_no_rescue_no_export",
            "partial": partial,
            "leaf_count": len(reports),
            "mwis_call_count": total_mwis,
            "initial_top_six_post_first_max": initial_max,
            "final_top_six_max": final_max,
            "top_six_max_movement": initial_max - final_max,
            "accepted_leaf_count": accepted_leaf_count,
            "accepted_row_count": accepted_total,
            "failure_classification": failure_classification(status, accepted_total, reports),
            "gates": {
                "leaf_count": LEAF_COUNT,
                "max_mwis_per_leaf": MAX_MWIS_PER_LEAF,
                "max_total_mwis": MAX_TOTAL_MWIS,
                "mwis_time_limit": MWIS_TIME_LIMIT,
                "violation_gate": VIOLATION_GATE,
                "accept_drop": ACCEPT_DROP,
                "drop_ratio_gate": DROP_RATIO_GATE,
                "prior_jaccard_gate": JACCARD_GATE,
                "single_center_jaccard_gate": SINGLE_JACCARD_GATE,
                "success_leaves": SUCCESS_LEAVES,
                "success_movement": SUCCESS_MOVEMENT,
                "success_max": SUCCESS_MAX,
                "strong_max": STRONG_MAX,
            },
            "leaves": reports,
            "status": status,
        }
    )


def main():
    source = json.loads(SOURCE.read_text())
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    selected_reports = sorted(source["leaves"], key=lambda row: -row["final_objective"])[:LEAF_COUNT]
    by_index = {index: leaf for index, leaf in enumerate(finite)}
    root_prior = bundle.root_supports(weights, adj)
    reports = []
    prior_pair_supports = []
    total_mwis = 0
    for report in selected_reports:
        if total_mwis >= MAX_TOTAL_MWIS:
            break
        leaf_report, accepted_supports = analyze_leaf(
            report,
            by_index[report["leaf_index"]],
            report["leaf_index"],
            edges,
            triads,
            weights,
            adj,
            root_cuts,
            parent_rows,
            root_prior,
            prior_pair_supports,
        )
        total_mwis += len(leaf_report["tested_rows"])
        prior_pair_supports.extend(accepted_supports)
        reports.append(leaf_report)
        OUT.write_text(json.dumps(make_report(reports, total_mwis, partial=True), indent=2) + "\n")
    report = make_report(reports, total_mwis, partial=False)
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
