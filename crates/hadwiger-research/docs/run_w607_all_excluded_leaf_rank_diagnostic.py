import hashlib
import json
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, linprog, milp
from scipy.sparse import lil_matrix

import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-all-excluded-leaf-rank-diagnostic.json"

FIXED_ZERO = [304, 223, 384, 302, 222, 383]
EXPECTED_LEAF = 594597.0511219536
MAX_CANDIDATES = 64
MAX_MWIS_CALLS = 40
EARLY_CALLS = 20
MWIS_TIME_LIMIT = 60.0
SIZES = (100, 140, 180, 220, 260)
HARD_MIN_SIZE = 80
HARD_MAX_SIZE = 260
VIOLATION_GATE = 250.0
SINGLE_ACCEPT_DROP = 250.0
MEANINGFUL_SINGLE_DROP = 750.0
FUND_TOTAL_DROP = 3000.0
STRONG_LEAF_GATE = 590000.0
KILL_TOTAL_DROP = 1000.0
KILL_LEAF_GATE = 592000.0
ROOT_JACCARD_WARN = 0.85


def p_parent_row(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(c0[v]) * parent.DENOMINATOR for v in range(parent.N) if c0[v]}
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        index = int(vertex) - 1
        coeffs[index] = coeffs.get(index, 0) + int(coeff)
    coeffs[parent.BRANCH] = int(artifact["lift_coefficient_num_d1024"])
    return coeffs, int(artifact["gamma0_modified_num_d1024"])


def solve_lp(edges, triads, weights, cuts, extra_rows, fixed=None, solution=False):
    fixed = fixed or {}
    rows = len(edges) + len(triads) + len(cuts) + len(extra_rows)
    matrix = lil_matrix((rows, parent.N), dtype=float)
    upper = np.ones(rows)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1.0
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1.0
        row += 1
    for vertices, alpha in cuts:
        for vertex in vertices:
            matrix[row, vertex] = float(weights[vertex])
        upper[row] = float(alpha)
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = float(coeff)
        upper[row] = float(rhs)
        row += 1
    bounds = [(0.0, 1.0)] * parent.N
    for vertex, value in fixed.items():
        bounds[vertex] = (float(value), float(value))
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix.tocsr(),
        b_ub=upper,
        bounds=bounds,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right)


def order_by(rank, size, banned=()):
    banned = set(banned)
    return tuple(sorted(int(v) for v in np.lexsort((np.arange(parent.N), -rank)) if v not in banned)[:size])


def twohop(seeds, size, rank, adj, banned):
    seen = set(seeds) - set(banned)
    for seed in list(seeds):
        seen.update(adj[seed])
    for vertex in list(seen):
        seen.update(adj[vertex])
    seen -= set(banned)
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:size]))


def dense_expand(seeds, size, weights, adj, rank, banned):
    banned = set(banned)
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


def root_supports(weights, adj):
    return [parent.pocket(name, weights, adj) for name, _alpha in parent.ACCEPTED]


def candidate_supports(weights, x, adj):
    banned = set(FIXED_ZERO)
    rank_vectors = {
        "wx": weights * x,
        "frac": weights * np.minimum(x, 1.0 - x),
        "x": x,
    }
    boundary = sorted({n for v in FIXED_ZERO for n in adj[v]} - banned, key=lambda v: (-weights[v] * x[v], v))[:12]
    supports = []
    for label, rank in rank_vectors.items():
        top = [int(v) for v in np.lexsort((np.arange(parent.N), -rank)) if v not in banned][:18]
        for size in SIZES:
            supports.append((f"{label}_top{size}", order_by(rank, size, banned)))
        for center in top[:10]:
            for size in SIZES[:-1]:
                supports.append((f"{label}_twohop{size}_{center+1}", twohop([center], size, rank, adj, banned)))
                supports.append((f"{label}_dense{size}_{center+1}", dense_expand([center], size, weights, adj, rank, banned)))
        for center in boundary[:8]:
            for size in (100, 140, 180):
                supports.append((f"{label}_boundary{size}_{center+1}", dense_expand([center], size, weights, adj, rank, banned)))
    return supports


def dedupe_supports(raw, root_prior):
    kept = []
    seen = set()
    seen_vertices = []
    for name, vertices in raw:
        if not (HARD_MIN_SIZE <= len(vertices) <= HARD_MAX_SIZE):
            continue
        key = support_hash(vertices)
        if key in seen:
            continue
        seen.add(key)
        max_root = max(jaccard(vertices, prior) for prior in root_prior)
        if any(jaccard(vertices, prior) >= 0.90 for prior in seen_vertices):
            continue
        seen_vertices.append(vertices)
        kept.append((name, vertices, max_root))
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


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items()}
    if isinstance(value, list):
        return [clean(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    edges, weights = parent.parse_edges_weights()
    weights_float = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights_float, adj)
    extra_rows = [parent_lift.parent_row(weights_float), p_parent_row(weights)]
    fixed = {vertex: 0.0 for vertex in FIXED_ZERO}
    base_obj, x = solve_lp(edges, triads, weights_float, root_cuts, extra_rows, fixed, True)
    root_prior = root_supports(weights, adj)
    candidates = dedupe_supports(candidate_supports(weights_float, x, adj), root_prior)
    candidates = sorted(candidates, key=lambda item: -float(np.dot(weights_float[list(item[1])], x[list(item[1])])))
    rows = []
    accepted = []
    for index, (name, vertices, root_overlap) in enumerate(candidates[:MAX_MWIS_CALLS]):
        alpha, ok, gap, seconds = solve_mwis(vertices, weights_float, adj)
        lhs = float(np.dot(weights_float[list(vertices)], x[list(vertices)]))
        violation = lhs - alpha if ok else None
        single_drop = 0.0
        accept = False
        if ok and violation >= VIOLATION_GATE:
            trial = (vertices, alpha)
            objective = solve_lp(edges, triads, weights_float, root_cuts + [trial], extra_rows, fixed)
            single_drop = base_obj - objective
            accept = single_drop >= SINGLE_ACCEPT_DROP
            if accept:
                accepted.append(trial)
        rows.append(
            {
                "name": name,
                "size": len(vertices),
                "alpha_w": alpha,
                "leaf_lhs": lhs,
                "violation": violation,
                "single_drop": single_drop,
                "root_jaccard": root_overlap,
                "root_overlap_warning": root_overlap >= ROOT_JACCARD_WARN,
                "solver_success": ok,
                "mip_gap": gap,
                "seconds": seconds,
                "accepted": accept,
            }
        )
        if index + 1 == EARLY_CALLS and not accepted:
            break
    final_obj, final_x = solve_lp(edges, triads, weights_float, root_cuts + accepted, extra_rows, fixed, True)
    total_drop = base_obj - final_obj
    best_single = max((row["single_drop"] for row in rows), default=0.0)
    status = "RetireAllExcludedLeafRankDiagnostic"
    if (total_drop >= FUND_TOTAL_DROP or final_obj <= STRONG_LEAF_GATE) and accepted:
        status = "FundAllExcludedLeafRankFollowup"
    elif accepted and best_single >= MEANINGFUL_SINGLE_DROP and final_obj <= KILL_LEAF_GATE:
        status = "InterestingAllExcludedLeafRankDiagnostic"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_all_excluded_leaf_rank_diagnostic.v1",
            "second_agent_verdict": "sound_but_strategically_weak_one_leaf_falsification_only",
            "fixed_zero_vertices": [v + 1 for v in FIXED_ZERO],
            "base_objective": base_obj,
            "expected_leaf_objective": EXPECTED_LEAF,
            "baseline_reproduced": abs(base_obj - EXPECTED_LEAF) <= 1e-5,
            "candidate_count": len(candidates),
            "mwis_call_count": len(rows),
            "accepted_row_count": len(accepted),
            "best_single_drop": best_single,
            "final_objective": final_obj,
            "total_drop": total_drop,
            "status": status,
            "gates": {
                "max_candidates": MAX_CANDIDATES,
                "max_mwis_calls": MAX_MWIS_CALLS,
                "early_calls_without_accept": EARLY_CALLS,
                "mwis_time_limit": MWIS_TIME_LIMIT,
                "violation_gate": VIOLATION_GATE,
                "single_accept_drop": SINGLE_ACCEPT_DROP,
                "meaningful_single_drop": MEANINGFUL_SINGLE_DROP,
                "fund_total_drop": FUND_TOTAL_DROP,
                "strong_leaf_gate": STRONG_LEAF_GATE,
                "kill_total_drop": KILL_TOTAL_DROP,
                "kill_leaf_gate": KILL_LEAF_GATE,
            },
            "top_rows": sorted(
                rows,
                key=lambda row: (
                    row["single_drop"],
                    row["violation"] if row["violation"] is not None else -1e100,
                ),
                reverse=True,
            )[:30],
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "top_rows"}, indent=2))


if __name__ == "__main__":
    main()
