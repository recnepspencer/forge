import hashlib
import json
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, linprog, milp
from scipy.sparse import lil_matrix

import run_w607_post_parent_lift_rank_sep as old_sep
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-branch-slack-global-rank-separation.json"

MAX_CANDIDATES = 200
MAX_MWIS_CALLS = 80
PRIMARY_SIZES = (120, 150, 180)
LARGE_SIZES = (210, 240)
LARGE_CANDIDATE_LIMIT = 20
VIOLATION_GATE = 250.0
ROOT_JACCARD_REJECT = 0.70
OLD_LOCAL_JACCARD_REJECT = 0.80
NEW_JACCARD_REJECT = 0.85
SINGLE_DROP_GATE = 500.0
KEEP_INVESTIGATING_DROP = 2500.0
PROMISING_DROP = 10000.0
KILL_DROP = 1000.0
MWIS_TIME_LIMIT = 120.0
HEAVY_PLATEAU = [304, 223, 384, 302, 222, 383]


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right)


def p_parent_row(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(c0[v]) * parent.DENOMINATOR for v in range(parent.N) if c0[v]}
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        index = int(vertex) - 1
        coeffs[index] = coeffs.get(index, 0) + int(coeff)
    coeffs[parent.BRANCH] = int(artifact["lift_coefficient_num_d1024"])
    return coeffs, int(artifact["gamma0_modified_num_d1024"])


def solve_lp(edges, triads, weights, cuts, extra_rows, solution=False):
    row_count = len(edges) + len(triads) + len(cuts) + len(extra_rows)
    matrix = lil_matrix((row_count, parent.N), dtype=float)
    upper = np.ones(row_count)
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
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix.tocsr(),
        b_ub=upper,
        bounds=[(0, 1)] * parent.N,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


def root_supports(weights, adj):
    return [parent.pocket(name, weights, adj) for name, _ in parent.ACCEPTED]


def prior_local_supports(edges, triads, weights, adj):
    c0 = old_sep.exclude_coverage(weights.astype(float))
    root_cuts = [
        (old_sep.pocket(name, weights.astype(float), adj), weights[list(old_sep.pocket(name, weights.astype(float), adj))], alpha)
        for name, alpha in old_sep.ACCEPTED
    ]
    lift = json.loads(old_sep.PARENT_LIFT.read_text())
    coeffs = {v: float(c0[v]) for v in range(parent.N) if v != parent.BRANCH and c0[v] > 0}
    coeffs[parent.BRANCH] = float(lift["new_lift_coefficient"])
    _, x = old_sep.solve_lp(edges, triads, weights.astype(float), root_cuts, [(coeffs, float(lift["new_rhs_numerator"]))], True)
    return [vertices for _, vertices in old_sep.candidate_supports(weights.astype(float), c0, x, adj)]


def dense_expand(seeds, limit, weights, adj, rank):
    chosen = list(dict.fromkeys(seeds))
    chosen_set = set(chosen)
    frontier = set()
    for seed in chosen:
        frontier.update(adj[seed])
    frontier -= chosen_set
    while len(chosen) < limit and frontier:
        def score(vertex):
            contact = sum(weights[other] for other in chosen if other in adj[vertex])
            rank_contact = sum(rank[other] for other in chosen if other in adj[vertex])
            return (rank_contact * 1e6 + contact * 10 + rank[vertex], -vertex)

        vertex = max(frontier, key=score)
        frontier.remove(vertex)
        chosen.append(vertex)
        chosen_set.add(vertex)
        frontier.update(adj[vertex] - chosen_set)
    if len(chosen) < limit:
        for vertex in np.lexsort((np.arange(parent.N), -rank)):
            if vertex not in chosen_set:
                chosen.append(int(vertex))
                chosen_set.add(int(vertex))
            if len(chosen) == limit:
                break
    return tuple(sorted(chosen[:limit]))


def closure(seeds, limit, rank, adj):
    seen = set(seeds)
    for seed in list(seeds):
        seen.update(adj[seed])
    for vertex in list(seen):
        seen.update(adj[vertex])
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:limit]))


def anti_neighborhood(seeds, limit, rank, adj):
    blocked = set(seeds)
    for seed in seeds:
        blocked.update(adj[seed])
    pool = [v for v in np.lexsort((np.arange(parent.N), -rank)) if v not in blocked]
    chosen = list(seeds) + [int(v) for v in pool[: max(0, limit - len(seeds))]]
    return tuple(sorted(chosen[:limit]))


def raw_supports(weights, cmod, x, adj):
    rank_vectors = {
        "wx": weights * x,
        "x": x,
        "cmod": cmod * x,
        "hybrid": (weights + cmod / 100000.0) * x,
    }
    supports = []
    heavy = sorted(HEAVY_PLATEAU, key=lambda v: (-weights[v] * x[v], v))
    for label, rank in rank_vectors.items():
        order = [int(v) for v in np.lexsort((np.arange(parent.N), -rank))[:24]]
        for size in PRIMARY_SIZES:
            supports.append((f"{label}_top{size}", tuple(sorted(order_by_rank(rank, size)))))
        for center in order[:12]:
            for size in PRIMARY_SIZES:
                supports.append((f"{label}_closure{size}_{center+1}", closure([center], size, rank, adj)))
                supports.append((f"{label}_dense{size}_{center+1}", dense_expand([center], size, weights, adj, rank)))
        for index, left in enumerate(heavy):
            for right in heavy[index + 1 :]:
                for size in PRIMARY_SIZES:
                    name = f"{label}_mixed{size}_{left+1}_{right+1}"
                    supports.append((name, dense_expand([left, right], size, weights, adj, rank)))
                    supports.append((name + "_anti", anti_neighborhood([left, right], size, rank, adj)))
        for size in LARGE_SIZES:
            supports.append((f"{label}_large_top{size}", tuple(sorted(order_by_rank(rank, size)))))
            for center in heavy[:4]:
                supports.append((f"{label}_large_dense{size}_{center+1}", dense_expand([center], size, weights, adj, rank)))
    return supports


def order_by_rank(rank, limit):
    return [int(v) for v in np.lexsort((np.arange(parent.N), -rank))[:limit]]


def filtered_supports(raw, root_prior, local_prior):
    kept = []
    seen = []
    large_count = 0
    for name, vertices in raw:
        size = len(vertices)
        if size < 120 or size > 240:
            continue
        if size > 180:
            if large_count >= LARGE_CANDIDATE_LIMIT:
                continue
            large_count += 1
        if any(jaccard(vertices, prior) >= ROOT_JACCARD_REJECT for prior in root_prior):
            continue
        if any(jaccard(vertices, prior) >= OLD_LOCAL_JACCARD_REJECT for prior in local_prior):
            continue
        if any(jaccard(vertices, prior) >= NEW_JACCARD_REJECT for prior in seen):
            continue
        seen.append(vertices)
        kept.append((name, vertices))
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
                row[i] = 1
                row[local[b]] = 1
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


def plateau_count(x, weights):
    heavy = [v for v in range(parent.N) if weights[v] >= 10000]
    return sum(1 for v in heavy if abs(x[v] - (1.0 / 3.0)) <= 1e-6)


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
    old_parent = parent_lift.parent_row(weights_float)
    p_row = p_parent_row(weights)
    root_cuts = [(parent.pocket(name, weights, adj), alpha) for name, alpha in parent.ACCEPTED]
    base_obj, x = solve_lp(edges, triads, weights_float, root_cuts, [old_parent, p_row], True)
    c0, _ = parent.exclude_coverage(weights)
    cmod = np.array([int(v) * parent.DENOMINATOR for v in c0], dtype=float)
    branch_slack = json.loads(BRANCH_SLACK.read_text())
    for vertex, coeff in branch_slack["positive_coefficients_num_d1024"].items():
        cmod[int(vertex) - 1] += int(coeff)
    root_prior = root_supports(weights, adj)
    local_prior = prior_local_supports(edges, triads, weights, adj)
    candidates = filtered_supports(raw_supports(weights_float, cmod, x, adj), root_prior, local_prior)
    candidates = sorted(candidates, key=lambda item: -float(np.dot(weights_float[list(item[1])], x[list(item[1])])))
    rows = []
    accepted = []
    for name, vertices in candidates[:MAX_MWIS_CALLS]:
        alpha, ok, gap, seconds = solve_mwis(vertices, weights_float, adj)
        lhs = float(np.dot(weights_float[list(vertices)], x[list(vertices)]))
        violation = lhs - alpha if ok else None
        root_overlap = max(jaccard(vertices, prior) for prior in root_prior)
        local_overlap = max(jaccard(vertices, prior) for prior in local_prior)
        single_drop = 0.0
        accept = False
        if ok and violation >= VIOLATION_GATE and root_overlap < 0.60 and local_overlap < OLD_LOCAL_JACCARD_REJECT:
            trial = (vertices, alpha)
            objective = solve_lp(edges, triads, weights_float, root_cuts + [trial], [old_parent, p_row])
            single_drop = base_obj - objective
            accept = single_drop >= SINGLE_DROP_GATE
            if accept:
                accepted.append(trial)
        rows.append(
            {
                "name": name,
                "size": len(vertices),
                "alpha_w": alpha,
                "lhs": lhs,
                "violation": violation,
                "single_drop": single_drop,
                "root_jaccard": root_overlap,
                "old_local_jaccard": local_overlap,
                "solver_success": ok,
                "mip_gap": gap,
                "seconds": seconds,
                "accepted": accept,
            }
        )
    final_obj, final_x = solve_lp(edges, triads, weights_float, root_cuts + accepted, [old_parent, p_row], True)
    total_drop = base_obj - final_obj
    status = "RetireBranchSlackGlobalRankSeparation"
    if total_drop >= PROMISING_DROP or any(row["single_drop"] >= 5000 for row in rows):
        status = "FundGlobalRankReplay"
    elif total_drop >= KEEP_INVESTIGATING_DROP and accepted:
        status = "FundGlobalRankFollowup"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_branch_slack_global_rank_separation.v1",
            "base_objective": base_obj,
            "candidate_count": len(candidates),
            "mwis_call_count": len(rows),
            "accepted_row_count": len(accepted),
            "final_objective": final_obj,
            "total_drop": total_drop,
            "base_heavy_third_plateau_count": plateau_count(x, weights),
            "final_heavy_third_plateau_count": plateau_count(final_x, weights),
            "status": status,
            "gates": {
                "max_candidates": MAX_CANDIDATES,
                "max_mwis_calls": MAX_MWIS_CALLS,
                "violation_gate": VIOLATION_GATE,
                "single_drop_gate": SINGLE_DROP_GATE,
                "kill_drop": KILL_DROP,
                "keep_investigating_drop": KEEP_INVESTIGATING_DROP,
                "promising_drop": PROMISING_DROP,
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
