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
OUT_PATH = CRATE / "docs" / "w607-v304-split-cut-diagnostic.json"

N = 607
V304 = 303
DEN = 1024
U_INCLUDE_NUM = 618626223
U_EXCLUDE_NUM = 647496725
KNOWN_ROOT = 641090.9615275887

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


def pocket(name, weights, adj):
    if name == "top_weight_120":
        return tuple(sorted(np.lexsort((np.arange(N), -weights))[:120]))
    kind, raw = name.rsplit("_", 1)
    center = int(raw) - 1
    if kind.startswith("twohop"):
        limit = int(kind.removeprefix("twohop"))
        seen = {center, *adj[center]}
        for vertex in list(seen):
            seen.update(adj[vertex])
        return tuple(sorted(sorted(seen, key=lambda v: (-weights[v], v))[:limit]))
    if kind.startswith("dense"):
        return dense(center, int(kind.removeprefix("dense")), weights, adj)
    raise ValueError(name)


def dense(seed, limit, weights, adj):
    chosen = [seed]
    chosen_set = {seed}
    frontier = set(adj[seed])
    while len(chosen) < limit and frontier:
        def score(v):
            return (sum(weights[u] for u in chosen if u in adj[v]) * 10000 + weights[v], -v)
        vertex = max(frontier, key=score)
        frontier.remove(vertex)
        chosen.append(vertex)
        chosen_set.add(vertex)
        frontier.update(adj[vertex] - chosen_set)
    return tuple(sorted(chosen))


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


def solve_lp(edges, triads, rank_rows, weights, split_cut=False):
    row_count = len(edges) + len(triads) + len(rank_rows) + int(split_cut)
    matrix = lil_matrix((row_count, N), dtype=float)
    rhs = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1
        row += 1
    for triad in triads:
        for v in triad:
            matrix[row, v] = 1
        row += 1
    for vertices, alpha in rank_rows:
        for v in vertices:
            matrix[row, v] = weights[v]
        rhs[row] = alpha
        row += 1
    if split_cut:
        for v, weight in enumerate(weights):
            matrix[row, v] = weight * DEN
        matrix[row, V304] += U_EXCLUDE_NUM - U_INCLUDE_NUM
        rhs[row] = U_EXCLUDE_NUM
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix.tocsr(),
        b_ub=rhs,
        bounds=[(0, 1)] * N,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result


def main():
    edges, weights = parse_edges_weights()
    adj = adjacency(edges)
    rank_rows = [(pocket(name, weights, adj), alpha) for name, alpha in ACCEPTED]
    triads = triangles(adj)
    root, root_result = solve_lp(edges, triads, rank_rows, weights, False)
    cut, cut_result = solve_lp(edges, triads, rank_rows, weights, True)
    lhs_num = int(round(np.dot(weights, root_result.x) * DEN + (U_EXCLUDE_NUM - U_INCLUDE_NUM) * root_result.x[V304]))
    violation = lhs_num - U_EXCLUDE_NUM
    report = {
        "schema": "forge.hadwiger.w607_v304_split_cut_diagnostic.v1",
        "root_objective": root,
        "known_root_objective": KNOWN_ROOT,
        "root_x304": float(root_result.x[V304]),
        "include_bound_num": U_INCLUDE_NUM,
        "exclude_bound_num": U_EXCLUDE_NUM,
        "denominator": DEN,
        "split_cut_rhs_num": U_EXCLUDE_NUM,
        "split_cut_lhs_at_root_num_rounded": lhs_num,
        "split_cut_violation_num_rounded": violation,
        "split_cut_violation": violation / DEN,
        "root_with_split_cut_objective": cut,
        "root_drop": root - cut,
        "new_x304": float(cut_result.x[V304]),
        "status": "SplitObjectiveCutMovedRoot" if root - cut >= 5000 else "SplitObjectiveCutWeak",
    }
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
