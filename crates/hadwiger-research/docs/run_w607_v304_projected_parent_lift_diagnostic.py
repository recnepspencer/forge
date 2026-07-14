import json
import re
from pathlib import Path

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix


ROOT = Path(__file__).resolve().parents[3]
CRATE = ROOT / "crates" / "hadwiger-research"
DATA = CRATE / "src" / "frontier_seeds" / "g27_finite_fractional"
EDGES_PATH = DATA / "W_circles_607_integers.dat"
EXCLUDE_CERT = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"
EXCLUDE_AGG = CRATE / "docs" / "w607-v304-aggregate-dual-lift-preflight.json"
PROJECTED = CRATE / "docs" / "w607-v304-projected-aggregate-mix-screen.json"
OUT_PATH = CRATE / "docs" / "w607-v304-projected-parent-lift-diagnostic.json"

N = 607
BRANCH = 303
DENOMINATOR = 1024
U0_NUM = 647496725
U1_NUM = 618626223
KNOWN_ROOT = 641090.9615275887
CURRENT_BEST = 632232.3996589413
DROP_GATE = 1000.0
IMPROVEMENT_GATE = 100.0

ACCEPTED = [
    ("top_weight_120", 316539),
    ("twohop80_304", 255387),
    ("twohop120_304", 306879),
    ("twohop120_152", 262126),
    ("twohop120_222", 262126),
    ("twohop120_225", 262126),
    ("twohop120_383", 262126),
    ("twohop120_386", 262126),
    ("twohop120_456", 262126),
    ("twohop80_223", 216958),
    ("twohop80_224", 216958),
    ("dense80_304", 202259),
    ("dense80_223", 235789),
    ("dense120_223", 315855),
    ("dense80_224", 235789),
    ("dense120_224", 315855),
]


def parse_edges_weights():
    text = EDGES_PATH.read_text()
    edge_blob = text.split("Edges = {", 1)[1].split("};", 1)[0]
    edges = sorted((int(a) - 1, int(b) - 1) for a, b in re.findall(r"<(\d+),(\d+)>", edge_blob))
    weight_blob = text.split("w = [", 1)[1].split("];", 1)[0]
    weights = np.array([int(float(x.strip())) for x in weight_blob.split(",") if x.strip()], dtype=int)
    return edges, weights


def adjacency(edges):
    adj = [set() for _ in range(N)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    return adj


def triangles(adj):
    out = []
    for a in range(N):
        for b in adj[a]:
            if b <= a:
                continue
            for c in adj[a] & adj[b]:
                if c > b:
                    out.append((a, b, c))
    return out


def twohop(center, limit, weights, adj, rank):
    seen = {center, *adj[center]}
    for vertex in list(seen):
        seen.update(adj[vertex])
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:limit]))


def dense(center, limit, weights, adj, rank):
    chosen = [center]
    chosen_set = {center}
    frontier = set(adj[center])
    while len(chosen) < limit and frontier:
        def score(v):
            return (
                sum(rank[u] for u in chosen if u in adj[v]) * 1e6
                + sum(weights[u] for u in chosen if u in adj[v]) * 10
                + rank[v],
                -v,
            )

        vertex = max(frontier, key=score)
        frontier.remove(vertex)
        chosen.append(vertex)
        chosen_set.add(vertex)
        frontier.update(adj[vertex] - chosen_set)
    return tuple(sorted(chosen))


def pocket(name, weights, adj):
    rank = weights.astype(float)
    if name == "top_weight_120":
        return tuple(sorted(np.lexsort((np.arange(N), -rank))[:120]))
    kind, raw = name.rsplit("_", 1)
    center = int(raw) - 1
    if kind.startswith("twohop"):
        return twohop(center, int(kind.removeprefix("twohop")), rank, adj, rank)
    if kind.startswith("dense"):
        return dense(center, int(kind.removeprefix("dense")), rank, adj, rank)
    raise ValueError(name)


def exclude_coverage(weights):
    cert = json.loads(EXCLUDE_CERT.read_text())
    coverage = np.zeros(N, dtype=object)
    objective = 0
    for row in cert["rows"]:
        numerator = int(row["numerator"])
        if row["kind"] == "parent_triangle":
            objective += numerator
            for vertex in row["vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator
        elif row["kind"] == "child_weighted_rank":
            alpha = int(row["alpha_w"])
            objective += numerator * alpha
            for vertex in row["support_vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator * int(weights[vertex - 1])
        else:
            raise ValueError(row["kind"])
    return coverage, objective


def solve_lp(edges, triads, weights, cuts, extra_rows=None, solution=False):
    extra_rows = extra_rows or []
    row_count = len(edges) + len(triads) + len(cuts) + len(extra_rows)
    matrix = lil_matrix((row_count, N), dtype=float)
    upper = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1
        row += 1
    for vertices, alpha in cuts:
        for vertex in vertices:
            matrix[row, vertex] = float(weights[vertex])
        upper[row] = alpha
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = float(coeff)
        upper[row] = float(rhs)
        row += 1
    result = linprog(c=-weights.astype(float), A_ub=matrix.tocsr(), b_ub=upper, bounds=[(0, 1)] * N, method="highs")
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


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
    edges, weights = parse_edges_weights()
    adj = adjacency(edges)
    triads = triangles(adj)
    root_cuts = [(pocket(name, weights, adj), alpha) for name, alpha in ACCEPTED]
    coverage, objective_num = exclude_coverage(weights)
    projected = json.loads(PROJECTED.read_text())
    aggregate = json.loads(EXCLUDE_AGG.read_text())
    gamma0_scaled = int(projected["rows"][-1]["gamma_upper_bound"])
    if projected["rows"][-1]["lambda_num"] != 100:
        raise ValueError("projected artifact last row is not pure exclude")
    gamma0 = int(np.ceil(gamma0_scaled / projected["rows"][-1]["lambda_den"]))
    gamma1 = int(aggregate["gamma_upper_bound_numerator"])
    lift = gamma0 - gamma1
    if objective_num != U0_NUM or lift <= 0:
        raise ValueError("invalid lifted row inputs")
    root_obj, root_x = solve_lp(edges, triads, weights, root_cuts, solution=True)
    old_lift = int(aggregate["lp_test_lift_coefficient"])
    old_coeffs = {v: int(coverage[v]) for v in range(N) if v != BRANCH and int(coverage[v])}
    old_coeffs[BRANCH] = old_lift
    old_row = (old_coeffs, U0_NUM)
    new_coeffs = {v: int(coverage[v]) for v in range(N) if v != BRANCH and int(coverage[v])}
    new_coeffs[BRANCH] = lift
    new_row = (new_coeffs, gamma0)
    split_coeffs = {v: int(weights[v]) * DENOMINATOR for v in range(N)}
    split_coeffs[BRANCH] += U0_NUM - U1_NUM
    split_row = (split_coeffs, U0_NUM)
    new_obj, new_x = solve_lp(edges, triads, weights, root_cuts, [new_row], True)
    split_new_obj, split_new_x = solve_lp(edges, triads, weights, root_cuts, [split_row, new_row], True)
    old_new_obj, old_new_x = solve_lp(edges, triads, weights, root_cuts, [old_row, new_row], True)
    split_old_new_obj, split_old_new_x = solve_lp(edges, triads, weights, root_cuts, [split_row, old_row, new_row], True)
    report = clean({
        "schema": "forge.hadwiger.w607_v304_projected_parent_lift_diagnostic.v1",
        "branch_vertex": BRANCH + 1,
        "coverage_objective_numerator": objective_num,
        "denominator": DENOMINATOR,
        "gamma0_upper_numerator": gamma0,
        "gamma0_scaled_source": gamma0_scaled,
        "gamma1_upper_numerator": gamma1,
        "new_lift_coefficient": lift,
        "old_lift_coefficient": old_lift,
        "new_rhs_numerator": gamma0,
        "old_rhs_numerator": U0_NUM,
        "dominates_old_for_x304_lt_1": gamma0 < U0_NUM and gamma0 - lift == U0_NUM - old_lift,
        "root_objective": root_obj,
        "known_root_objective": KNOWN_ROOT,
        "root_x304": float(root_x[BRANCH]),
        "new_parent_lift_objective": new_obj,
        "new_parent_lift_drop": root_obj - new_obj,
        "new_parent_lift_x304": float(new_x[BRANCH]),
        "split_plus_new_objective": split_new_obj,
        "split_plus_new_improvement_over_current_best": CURRENT_BEST - split_new_obj,
        "split_plus_new_x304": float(split_new_x[BRANCH]),
        "old_plus_new_objective": old_new_obj,
        "old_plus_new_improvement_over_current_best": CURRENT_BEST - old_new_obj,
        "old_plus_new_x304": float(old_new_x[BRANCH]),
        "split_old_new_objective": split_old_new_obj,
        "split_old_new_improvement_over_current_best": CURRENT_BEST - split_old_new_obj,
        "split_old_new_x304": float(split_old_new_x[BRANCH]),
        "funds_parent_lift": root_obj - new_obj >= DROP_GATE and CURRENT_BEST - split_old_new_obj >= IMPROVEMENT_GATE,
        "status": "FundProjectedParentLift" if CURRENT_BEST - split_old_new_obj >= IMPROVEMENT_GATE else "RetireProjectedParentLift",
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
