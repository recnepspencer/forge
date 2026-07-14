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
OUT_PATH = CRATE / "docs" / "w607-weighted-rank-pattern-preflight.json"

N = 607
THRESHOLD = 3000
MIN_MARGINAL_DROP = 1000.0

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


def parse_edges_and_weights():
    text = EDGES_PATH.read_text()
    edge_blob = text.split("Edges = {", 1)[1].split("};", 1)[0]
    edges = sorted((int(a) - 1, int(b) - 1) for a, b in re.findall(r"<(\d+),(\d+)>", edge_blob))
    weight_blob = text.split("w = [", 1)[1].split("];", 1)[0]
    weights = np.array([int(float(x.strip())) for x in weight_blob.split(",") if x.strip()], dtype=float)
    if len(edges) != 3390 or len(weights) != N:
        raise ValueError("unexpected W607 edge/weight shape")
    return edges, weights


def parse_vertices():
    tokens = re.findall(r"-?\d+(?:/\d+)?", VERTICES_PATH.read_text())
    values = [Fraction(token) for token in tokens]
    if len(values) != N * 8:
        raise ValueError("unexpected W607 vertex shape")
    return [tuple(values[i : i + 8]) for i in range(0, len(values), 8)]


def half_turn(vertices):
    index = {row: i for i, row in enumerate(vertices)}
    image = []
    for row in vertices:
        x = list(row[:4])
        y = list(row[4:])
        target = (Fraction(6) - x[0], -x[1], -x[2], -x[3], -y[0], -y[1], -y[2], -y[3])
        image.append(index[target])
    if sorted(image) != list(range(N)):
        raise ValueError("half-turn is not a permutation")
    return image


def adjacency(edges):
    adj = [set() for _ in range(N)]
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
    return adj


def pocket(name, weights, adj):
    if name == "top_weight_120":
        return tuple(sorted(np.lexsort((np.arange(N), -weights))[:120]))
    kind, raw_center = name.rsplit("_", 1)
    center = int(raw_center) - 1
    if kind.startswith("twohop"):
        limit = int(kind.removeprefix("twohop"))
        seen = {center}
        seen.update(adj[center])
        for vertex in list(seen):
            seen.update(adj[vertex])
        ordered = sorted(seen, key=lambda v: (-weights[v], v))[:limit]
        return tuple(sorted(ordered))
    if kind.startswith("dense"):
        limit = int(kind.removeprefix("dense"))
        chosen = [center]
        frontier = set(adj[center])
        while len(chosen) < limit and frontier:
            def score(v):
                return (sum(weights[u] for u in chosen if u in adj[v]) * 10000 + weights[v], -v)
            vertex = max(frontier, key=score)
            frontier.remove(vertex)
            chosen.append(vertex)
            frontier.update(adj[vertex] - set(chosen))
        return tuple(sorted(chosen))
    raise ValueError(f"unknown pocket {name}")


def support_hash(vertices):
    text = ",".join(str(v + 1) for v in vertices)
    return hashlib.sha256(text.encode()).hexdigest()


def jaccard(left, right):
    left = set(left)
    right = set(right)
    return len(left & right) / len(left | right)


def induced_edges(vertices, adj):
    vertices = list(vertices)
    total = 0
    for i, a in enumerate(vertices):
        for b in vertices[i + 1 :]:
            total += b in adj[a]
    return total


def solve_mwis(vertices, weights, adj):
    local = {v: i for i, v in enumerate(vertices)}
    edge_rows = []
    for i, a in enumerate(vertices):
        for b in vertices[i + 1 :]:
            if b in adj[a]:
                row = np.zeros(len(vertices))
                row[i] = 1
                row[local[b]] = 1
                edge_rows.append(row)
    if edge_rows:
        constraints = LinearConstraint(np.vstack(edge_rows), -np.inf, np.ones(len(edge_rows)))
    else:
        constraints = None
    start = time.time()
    result = milp(
        c=-weights[list(vertices)],
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": 300},
    )
    seconds = time.time() - start
    if not result.success:
        return None, seconds, False, []
    chosen = [vertices[i] for i, value in enumerate(result.x) if value > 0.5]
    return int(round(-result.fun)), seconds, True, chosen


def triangles(adj):
    rows = []
    for a in range(N):
        for b in adj[a]:
            if b <= a:
                continue
            common = adj[a] & adj[b]
            for c in common:
                if c > b:
                    rows.append((a, b, c))
    return rows


def solve_cut_lp(edges, triads, weights, cuts):
    row_count = len(edges) + len(triads) + len(cuts)
    matrix = lil_matrix((row_count, N), dtype=float)
    upper = np.ones(row_count)
    row = 0
    for a, b in edges:
        matrix[row, a] = 1
        matrix[row, b] = 1
        row += 1
    for a, b, c in triads:
        matrix[row, a] = 1
        matrix[row, b] = 1
        matrix[row, c] = 1
        row += 1
    for vertices, alpha in cuts:
        for v in vertices:
            matrix[row, v] = weights[v]
        upper[row] = alpha
        row += 1
    result = linprog(
        c=-weights,
        A_ub=matrix.tocsr(),
        b_ub=upper,
        bounds=[(0, 1)] * N,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun)


def centered_kind(name):
    if name.startswith(("twohop80_", "twohop120_", "dense80_", "dense120_")):
        return name.rsplit("_", 1)
    return None


def main():
    edges, weights = parse_edges_and_weights()
    adj = adjacency(edges)
    image = half_turn(parse_vertices())
    triads = triangles(adj)
    accepted_rows = []
    accepted_supports = {}
    canonical_seen = set()
    for name, alpha in ACCEPTED:
        vertices = pocket(name, weights, adj)
        mirror = tuple(sorted(image[v] for v in vertices))
        row = {
            "name": name,
            "size": len(vertices),
            "edge_count": induced_edges(vertices, adj),
            "weight_sum": int(sum(weights[list(vertices)])),
            "alpha_w": alpha,
            "violation_numerator": int(sum(weights[list(vertices)]) - 3 * alpha),
            "support_hash": support_hash(vertices),
            "half_turn_support_hash": support_hash(mirror),
            "half_turn_canonical_hash": min(support_hash(vertices), support_hash(mirror)),
        }
        accepted_rows.append(row)
        accepted_supports[name] = vertices
        canonical_seen.add(row["half_turn_canonical_hash"])

    candidates = []
    candidate_names = set()
    for name, _ in ACCEPTED:
        parsed = centered_kind(name)
        if not parsed:
            continue
        kind, raw_center = parsed
        mirror_center = image[int(raw_center) - 1] + 1
        candidate_name = f"{kind}_{mirror_center}"
        if candidate_name in {n for n, _ in ACCEPTED} or candidate_name in candidate_names:
            continue
        candidate_names.add(candidate_name)
        candidates.append(candidate_name)

    candidate_rows = []
    new_cuts = []
    existing_hashes = {row["support_hash"] for row in accepted_rows}
    for name in sorted(candidates):
        vertices = pocket(name, weights, adj)
        mirror = tuple(sorted(image[v] for v in vertices))
        alpha, seconds, ok, witness = solve_mwis(vertices, weights, adj)
        weight_sum = int(sum(weights[list(vertices)]))
        violation = None if alpha is None else weight_sum - 3 * alpha
        max_overlap = max(jaccard(vertices, support) for support in accepted_supports.values())
        row = {
            "name": name,
            "center": int(name.rsplit("_", 1)[1]),
            "half_turn_center": image[int(name.rsplit("_", 1)[1]) - 1] + 1,
            "size": len(vertices),
            "edge_count": induced_edges(vertices, adj),
            "weight_sum": weight_sum,
            "alpha_w": alpha,
            "violation_numerator": violation,
            "solver_success": ok,
            "seconds": round(seconds, 4),
            "witness_weight": int(sum(weights[witness])) if witness else 0,
            "witness_size": len(witness),
            "support_hash": support_hash(vertices),
            "half_turn_support_hash": support_hash(mirror),
            "half_turn_canonical_hash": min(support_hash(vertices), support_hash(mirror)),
            "duplicate_support": support_hash(vertices) in existing_hashes,
            "duplicate_orbit": min(support_hash(vertices), support_hash(mirror)) in canonical_seen,
            "max_jaccard_against_accepted": max_overlap,
        }
        accepted = ok and violation >= THRESHOLD and not row["duplicate_support"]
        accepted = accepted and not row["duplicate_orbit"] and max_overlap < 0.90
        row["accepted_as_new_cut"] = bool(accepted)
        candidate_rows.append(row)
        if accepted:
            new_cuts.append((vertices, alpha))

    accepted_cuts = [(accepted_supports[name], alpha) for name, alpha in ACCEPTED]
    lp_existing = solve_cut_lp(edges, triads, weights, accepted_cuts)
    lp_new = solve_cut_lp(edges, triads, weights, accepted_cuts + new_cuts)
    marginal_drop = lp_existing - lp_new
    distinct_nonorbit = len({row["half_turn_canonical_hash"] for row in candidate_rows if row["accepted_as_new_cut"]})
    status = "RetirePatternOrbitExpansion"
    if distinct_nonorbit >= 4 or marginal_drop >= MIN_MARGINAL_DROP:
        status = "FundPatternOrbitExpansion"

    report = {
        "schema": "forge.hadwiger.w607_weighted_rank_pattern_preflight.v1",
        "accepted_cut_count": len(ACCEPTED),
        "candidate_count": len(candidate_rows),
        "accepted_new_cut_count": len(new_cuts),
        "distinct_nonorbit_new_cut_count": distinct_nonorbit,
        "root_lp_with_existing_cuts": lp_existing,
        "root_lp_with_existing_plus_new_cuts": lp_new,
        "marginal_drop_from_new_cuts": marginal_drop,
        "thresholds": {
            "min_violation_numerator": THRESHOLD,
            "max_jaccard_for_new_cut": 0.90,
            "min_distinct_nonorbit_new_cuts": 4,
            "min_marginal_lp_drop": MIN_MARGINAL_DROP,
        },
        "status": status,
        "accepted_rows": accepted_rows,
        "candidate_rows": candidate_rows,
    }
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: report[k] for k in report if k not in {"accepted_rows", "candidate_rows"}}, indent=2))


if __name__ == "__main__":
    main()
