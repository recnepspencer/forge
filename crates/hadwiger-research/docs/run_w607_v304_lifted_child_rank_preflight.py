import json
import re
import time
from pathlib import Path

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, linprog, milp
from scipy.sparse import lil_matrix


ROOT = Path(__file__).resolve().parents[3]
CRATE = ROOT / "crates" / "hadwiger-research"
DATA = CRATE / "src" / "frontier_seeds" / "g27_finite_fractional"
EDGES_PATH = DATA / "W_circles_607_integers.dat"
OUT_PATH = CRATE / "docs" / "w607-v304-lifted-child-rank-preflight.json"

N = 607
EXCLUDED = 303
KNOWN_ROOT = 641090.9615275887
KNOWN_EXCLUDE = 632232.3996589432
VIOLATION_GATE = 1000.0
DENOMINATOR = 1024
INCLUDE_BOUND_NUM = 618626223
EXCLUDE_BOUND_NUM = 647496725

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
    weights = np.array([int(float(x.strip())) for x in weight_blob.split(",") if x.strip()], dtype=float)
    return edges, weights


def adjacency(edges):
    adj = [set() for _ in range(N)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    return adj


def triangles(adj):
    rows = []
    for a in range(N):
        for b in adj[a]:
            if b <= a:
                continue
            for c in adj[a] & adj[b]:
                if c > b:
                    rows.append((a, b, c))
    return rows


def top_by_rank(rank, limit):
    return tuple(sorted(np.lexsort((np.arange(N), -rank))[:limit]))


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
    if name == "top_weight_120":
        return tuple(sorted(np.lexsort((np.arange(N), -weights))[:120]))
    kind, raw = name.rsplit("_", 1)
    center = int(raw) - 1
    if kind.startswith("twohop"):
        return twohop(center, int(kind.removeprefix("twohop")), weights, adj, weights)
    if kind.startswith("dense"):
        return dense(center, int(kind.removeprefix("dense")), weights, adj, weights)
    raise ValueError(name)


def solve_lp(edges, triads, weights, cuts, fixed=None, solution=False):
    row_count = len(edges) + len(triads) + len(cuts)
    matrix = lil_matrix((row_count, N), dtype=float)
    upper = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = 1
        matrix[row, b] = 1
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1
        row += 1
    for vertices, alpha in cuts:
        for v in vertices:
            matrix[row, v] = weights[v]
        upper[row] = alpha
        row += 1
    bounds = [(0, 1)] * N
    for vertex, value in (fixed or {}).items():
        bounds[vertex] = (value, value)
    result = linprog(c=-weights, A_ub=matrix.tocsr(), b_ub=upper, bounds=bounds, method="highs")
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


def solve_lp_with_extra_rows(edges, triads, weights, cuts, extra_rows, fixed=None, solution=False):
    row_count = len(edges) + len(triads) + len(cuts) + len(extra_rows)
    matrix = lil_matrix((row_count, N), dtype=float)
    upper = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = 1
        matrix[row, b] = 1
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1
        row += 1
    for vertices, alpha in cuts:
        for v in vertices:
            matrix[row, v] = weights[v]
        upper[row] = alpha
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = coeff
        upper[row] = rhs
        row += 1
    bounds = [(0, 1)] * N
    for vertex, value in (fixed or {}).items():
        bounds[vertex] = (value, value)
    result = linprog(c=-weights, A_ub=matrix.tocsr(), b_ub=upper, bounds=bounds, method="highs")
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


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
    result = milp(
        c=-weights[list(vertices)],
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": 90, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    if not result.success or (gap is not None and gap > 1e-9):
        raise ValueError(f"MWIS failed: {result.message}")
    return int(round(-result.fun))


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
    start = time.time()
    edges, weights = parse_edges_weights()
    adj = adjacency(edges)
    triads = triangles(adj)
    root_cuts = [(pocket(name, weights, adj), alpha) for name, alpha in ACCEPTED]
    root_obj, root_x = solve_lp(edges, triads, weights, root_cuts, solution=True)
    base_exclude, base_x = solve_lp(edges, triads, weights, root_cuts, {EXCLUDED: 0.0}, True)
    rank = weights * base_x
    child_rows = [
        ("top_wx_120", top_by_rank(rank, 120), 312868),
        ("dense120_303", dense(302, 120, weights, adj, rank), 287232),
    ]
    exclude_obj, exclude_x = solve_lp(
        edges,
        triads,
        weights,
        root_cuts + [(vertices, alpha) for _name, vertices, alpha in child_rows],
        {EXCLUDED: 0.0},
        True,
    )
    dense_support = next(vertices for name, vertices, _alpha in child_rows if name == "dense120_303")
    active_support = tuple(v for v in dense_support if v != EXCLUDED)
    include_active_support = tuple(v for v in active_support if v not in adj[EXCLUDED])
    alpha0 = solve_mwis(active_support, weights, adj)
    beta = solve_mwis(include_active_support, weights, adj)
    lift_m = max(0, beta - alpha0)
    lhs_root = float(np.dot(weights[list(active_support)], root_x[list(active_support)]))
    lhs_exclude = float(np.dot(weights[list(active_support)], exclude_x[list(active_support)]))
    violation = lhs_root - alpha0 - lift_m * root_x[EXCLUDED]
    lifted_extra = None
    root_with_lift = None
    root_lift_drop = None
    root_with_lift_x304 = None
    split_with_lift = None
    split_lift_improvement = None
    split_with_lift_x304 = None
    if violation >= VIOLATION_GATE:
        if lift_m == 0:
            root_with_lift, root_lift_x = solve_lp(edges, triads, weights, root_cuts + [(active_support, alpha0)], solution=True)
            split_coeffs = {vertex: float(weights[vertex]) for vertex in range(N)}
            split_coeffs[EXCLUDED] += (EXCLUDE_BOUND_NUM - INCLUDE_BOUND_NUM) / DENOMINATOR
            split_rhs = EXCLUDE_BOUND_NUM / DENOMINATOR
            split_with_lift, split_lift_x = solve_lp_with_extra_rows(
                edges,
                triads,
                weights,
                root_cuts + [(active_support, alpha0)],
                [(split_coeffs, split_rhs)],
                solution=True,
            )
        else:
            coeffs = {vertex: float(weights[vertex]) for vertex in active_support}
            coeffs[EXCLUDED] = -float(lift_m)
            lifted_extra = (coeffs, float(alpha0))
            root_with_lift, root_lift_x = solve_lp_with_extra_rows(
                edges, triads, weights, root_cuts, [lifted_extra], solution=True
            )
            split_coeffs = {vertex: float(weights[vertex]) for vertex in range(N)}
            split_coeffs[EXCLUDED] += (EXCLUDE_BOUND_NUM - INCLUDE_BOUND_NUM) / DENOMINATOR
            split_rhs = EXCLUDE_BOUND_NUM / DENOMINATOR
            split_with_lift, split_lift_x = solve_lp_with_extra_rows(
                edges, triads, weights, root_cuts, [lifted_extra, (split_coeffs, split_rhs)], solution=True
            )
        root_lift_drop = root_obj - root_with_lift
        root_with_lift_x304 = float(root_lift_x[EXCLUDED])
        split_lift_improvement = EXCLUDE_BOUND_NUM / DENOMINATOR - split_with_lift
        split_with_lift_x304 = float(split_lift_x[EXCLUDED])
    report = clean({
        "schema": "forge.hadwiger.w607_v304_lifted_child_rank_preflight.v1",
        "excluded_vertex": EXCLUDED + 1,
        "root_objective": root_obj,
        "known_root_objective": KNOWN_ROOT,
        "base_exclude_before_child_rows": base_exclude,
        "v304_exclude_objective": exclude_obj,
        "known_v304_exclude_objective": KNOWN_EXCLUDE,
        "source_child_row": "dense120_303",
        "full_support_size": len(dense_support),
        "active_support_size": len(active_support),
        "include_active_support_size": len(include_active_support),
        "artifact_child_alpha": 287232,
        "alpha0_active_support": alpha0,
        "beta_include_branch_support": beta,
        "lift_coefficient_m": lift_m,
        "root_x304": float(root_x[EXCLUDED]),
        "root_lhs_active_support": lhs_root,
        "exclude_lhs_active_support": lhs_exclude,
        "lifted_row_violation_at_root": violation,
        "root_with_lifted_row_objective": root_with_lift,
        "root_lifted_row_drop": root_lift_drop,
        "root_with_lifted_row_x304": root_with_lift_x304,
        "split_with_lifted_row_objective": split_with_lift,
        "split_lifted_row_improvement_over_split": split_lift_improvement,
        "split_with_lifted_row_x304": split_with_lift_x304,
        "violation_gate": VIOLATION_GATE,
        "status": "FundLiftedChildRankReplay" if root_lift_drop is not None and root_lift_drop >= 1000 else "RetireLiftedChildRankPreflight",
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
