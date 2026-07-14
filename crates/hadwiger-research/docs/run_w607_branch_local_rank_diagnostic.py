import hashlib
import json
import re
import time
from fractions import Fraction
from pathlib import Path

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, linprog, milp
from scipy.sparse import lil_matrix


ROOT = Path(__file__).resolve().parents[3]
CRATE = ROOT / "crates" / "hadwiger-research"
DATA = CRATE / "src" / "frontier_seeds" / "g27_finite_fractional"
EDGES_PATH = DATA / "W_circles_607_integers.dat"
VERTICES_PATH = DATA / "W_circles_607_vertices.sage"
OUT_PATH = CRATE / "docs" / "w607-branch-local-rank-diagnostic.json"

N = 607
KNOWN_ROOT = 641090.9615275887
LP_TOL = 1e-4
BRANCH_VERTEX_CAP = 8
SEPARATION_SPLIT_CAP = 3
CHILD_CENTER_CAP = 8
CHILD_SUPPORT_CAP = 60
VIOLATION_THRESHOLD = 3000.0
LOW_OVERLAP = 0.70
HIGH_OVERLAP_SINGLE_DROP = 2000.0
FUND_MAX_CHILD = 625000.0
RETIRE_MAX_CHILD = 631000.0

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


def half_turn():
    tokens = re.findall(r"-?\d+(?:/\d+)?", VERTICES_PATH.read_text())
    values = [Fraction(token) for token in tokens]
    vertices = [tuple(values[i : i + 8]) for i in range(0, len(values), 8)]
    lookup = {row: i for i, row in enumerate(vertices)}
    image = []
    for row in vertices:
        x, y = row[:4], row[4:]
        target = (Fraction(6) - x[0], -x[1], -x[2], -x[3], -y[0], -y[1], -y[2], -y[3])
        image.append(lookup[target])
    return image


def adjacency(edges):
    adj = [set() for _ in range(N)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    return adj


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right)


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


def onehop(center, limit, rank, adj):
    seen = {center, *adj[center]}
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:limit]))


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
        options={"time_limit": 90, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    return (int(round(-result.fun)) if ok else None, time.time() - start, ok, gap)


def candidate_vertices(weights, x, retained):
    rank_a = weights * x
    rank_b = weights * np.minimum(x, 1 - x)
    counts = np.zeros(N)
    for support in retained:
        for vertex in support:
            counts[vertex] += 1
    rank_c = counts * weights * np.maximum(x, 1e-9)
    vertices = []
    for rank in (rank_a, rank_b, rank_c):
        for vertex in np.lexsort((np.arange(N), -rank))[:10]:
            if int(vertex) not in vertices:
                vertices.append(int(vertex))
            if len(vertices) >= BRANCH_VERTEX_CAP:
                return vertices
    return vertices[:BRANCH_VERTEX_CAP]


def child_supports(weights, adj, x):
    rank = weights * x
    centers = list(np.lexsort((np.arange(N), -rank))[:CHILD_CENTER_CAP])
    supports = {}
    for limit in (80, 100, 120):
        supports[f"top_wx_{limit}"] = tuple(sorted(np.lexsort((np.arange(N), -rank))[:limit]))
    for center in centers:
        for limit in (80, 100, 120):
            supports[f"onehop{limit}_{center + 1}"] = onehop(int(center), limit, rank, adj)
            supports[f"twohop{limit}_{center + 1}"] = twohop(int(center), limit, weights, adj, rank)
            supports[f"dense{limit}_{center + 1}"] = dense(int(center), limit, weights, adj, rank)
    unique = {}
    for name, vertices in supports.items():
        unique.setdefault(support_hash(vertices), (name, vertices))
    return list(unique.values())[:CHILD_SUPPORT_CAP]


def separate_child(edges, triads, weights, adj, image, retained, retained_hashes, retained_orbits, fixed):
    base, x = solve_lp(edges, triads, weights, retained, fixed, True)
    rows = []
    cuts = []
    for _, (name, vertices) in enumerate(child_supports(weights, adj, x)):
        mirror = tuple(sorted(image[v] for v in vertices))
        orbit_hash = min(support_hash(vertices), support_hash(mirror))
        duplicate = support_hash(vertices) in retained_hashes or orbit_hash in retained_orbits
        alpha, seconds, ok, gap = (None, 0.0, False, None) if duplicate else solve_mwis(vertices, weights, adj)
        lhs = float(np.dot(weights[list(vertices)], x[list(vertices)]))
        violation = None if alpha is None else lhs - alpha
        max_overlap = max(jaccard(vertices, support) for support, _ in retained)
        single_drop = 0.0
        if ok and violation is not None and violation >= VIOLATION_THRESHOLD:
            objective = solve_lp(edges, triads, weights, retained + [(vertices, alpha)], fixed)
            single_drop = base - objective
            if max_overlap <= LOW_OVERLAP or single_drop >= HIGH_OVERLAP_SINGLE_DROP:
                cuts.append((vertices, alpha))
        rows.append({
            "name": name,
            "size": len(vertices),
            "alpha_w": alpha,
            "child_lhs": lhs,
            "child_violation": violation,
            "solver_success": ok,
            "mip_gap": gap,
            "seconds": round(seconds, 4),
            "duplicate_retained_orbit": duplicate,
            "max_jaccard_retained": max_overlap,
            "single_drop": single_drop,
        })
    final = solve_lp(edges, triads, weights, retained + cuts, fixed)
    return {
        "raw_objective": base,
        "separated_objective": final,
        "separation_drop": base - final,
        "accepted_child_cut_count": len(cuts),
        "best_single_child_cut_drop": max((row["single_drop"] for row in rows), default=0.0),
        "candidate_count": len(rows),
        "violated_candidate_count": sum(
            1 for row in rows if row["child_violation"] is not None and row["child_violation"] >= VIOLATION_THRESHOLD
        ),
        "top_rows": sorted(
            [row for row in rows if row["child_violation"] is not None],
            key=lambda row: row["child_violation"],
            reverse=True,
        )[:10],
    }


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
    start = time.time()
    edges, weights = parse_edges_weights()
    adj = adjacency(edges)
    image = half_turn()
    triads = triangles(adj)
    retained_supports = [pocket(name, weights, adj) for name, _ in ACCEPTED]
    retained = list(zip(retained_supports, [alpha for _, alpha in ACCEPTED]))
    retained_hashes = {support_hash(support) for support in retained_supports}
    retained_orbits = {
        min(support_hash(support), support_hash(tuple(sorted(image[v] for v in support))))
        for support in retained_supports
    }
    root, x = solve_lp(edges, triads, weights, retained, solution=True)
    if abs(root - KNOWN_ROOT) > LP_TOL:
        raise ValueError(f"root mismatch {root}")
    vertices = candidate_vertices(weights, x, retained_supports)
    raw_rows = []
    for vertex in vertices:
        include = solve_lp(edges, triads, weights, retained, {vertex: 1.0})
        exclude = solve_lp(edges, triads, weights, retained, {vertex: 0.0})
        raw_rows.append({
            "vertex": vertex + 1,
            "include_raw_objective": include,
            "exclude_raw_objective": exclude,
            "raw_max_child_objective": max(include, exclude),
        })
    raw_rows.sort(key=lambda row: row["raw_max_child_objective"])
    tested = []
    for row in raw_rows[:SEPARATION_SPLIT_CAP]:
        vertex = row["vertex"] - 1
        include = separate_child(edges, triads, weights, adj, image, retained, retained_hashes, retained_orbits, {vertex: 1.0})
        exclude = separate_child(edges, triads, weights, adj, image, retained, retained_hashes, retained_orbits, {vertex: 0.0})
        row = dict(row)
        row["include"] = include
        row["exclude"] = exclude
        row["separated_max_child_objective"] = max(include["separated_objective"], exclude["separated_objective"])
        tested.append(row)
    best = min((row["separated_max_child_objective"] for row in tested), default=None)
    status = "RetireBranchLocalRankDiagnostic"
    if best is not None and best <= FUND_MAX_CHILD:
        status = "FundBranchLocalRankDiagnostic"
    elif best is not None and best <= RETIRE_MAX_CHILD:
        status = "InterestingButNotFundedBranchLocalRankDiagnostic"
    report = clean({
        "schema": "forge.hadwiger.w607_branch_local_rank_diagnostic.v1",
        "root_post16_objective": root,
        "candidate_vertex_count": len(vertices),
        "separated_split_count": len(tested),
        "best_separated_max_child_objective": best,
        "status": status,
        "caps": {
            "branch_vertex_cap": BRANCH_VERTEX_CAP,
            "separation_split_cap": SEPARATION_SPLIT_CAP,
            "child_support_cap": CHILD_SUPPORT_CAP,
        },
        "thresholds": {
            "fund_max_child": FUND_MAX_CHILD,
            "interesting_max_child": RETIRE_MAX_CHILD,
            "min_child_violation": VIOLATION_THRESHOLD,
            "low_overlap_jaccard": LOW_OVERLAP,
            "high_overlap_single_drop": HIGH_OVERLAP_SINGLE_DROP,
        },
        "raw_branch_rows": raw_rows,
        "tested_branch_rows": tested,
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    summary = {key: report[key] for key in report if key not in {"raw_branch_rows", "tested_branch_rows"}}
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
