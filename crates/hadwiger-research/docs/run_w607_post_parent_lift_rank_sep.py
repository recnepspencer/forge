import hashlib
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
EXCLUDE_CERT = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"
PARENT_LIFT = CRATE / "docs" / "w607-v304-projected-parent-lift-diagnostic.json"
ADAPTIVE = CRATE / "docs" / "w607-adaptive-weighted-rank-separation.json"
OUT_PATH = CRATE / "docs" / "w607-post-parent-lift-rank-separation.json"

N = 607
BRANCH = 303
CANDIDATE_CENTER_CAP = 12
SUPPORT_CAP = 80
VIOLATION_GATE = 3000.0
LOW_OVERLAP = 0.60
HIGH_OVERLAP_DROP = 3000.0
FUND_TOTAL_DROP = 10000.0
HYBRID_SCALE = 100000.0

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


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right)


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


def exclude_coverage(weights):
    cert = json.loads(EXCLUDE_CERT.read_text())
    coverage = np.zeros(N)
    for row in cert["rows"]:
        numerator = float(row["numerator"])
        if row["kind"] == "parent_triangle":
            for vertex in row["vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator
        elif row["kind"] == "child_weighted_rank":
            for vertex in row["support_vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator * weights[vertex - 1]
    return coverage


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
    for vertices, coeffs, rhs in cuts:
        for vertex, coeff in zip(vertices, coeffs):
            matrix[row, vertex] = coeff
        upper[row] = rhs
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = coeff
        upper[row] = rhs
        row += 1
    result = linprog(c=-weights, A_ub=matrix.tocsr(), b_ub=upper, bounds=[(0, 1)] * N, method="highs")
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


def solve_mwis(vertices, coeffs, adj):
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
        c=-coeffs[list(vertices)],
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": 60, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    return (float(-result.fun) if ok else None, ok, gap, time.time() - start)


def candidate_supports(weights, c0, x, adj):
    rank_vectors = {
        "w": weights * x,
        "c0": c0 * x,
        "frac": weights * np.minimum(x, 1 - x),
    }
    supports = {}
    for label, rank in rank_vectors.items():
        centers = [int(v) for v in np.lexsort((np.arange(N), -rank))[:CANDIDATE_CENTER_CAP]]
        for limit in (80, 100, 120):
            supports[f"{label}_top{limit}"] = tuple(sorted(np.lexsort((np.arange(N), -rank))[:limit]))
        for center in centers:
            for limit in (80, 100, 120):
                supports[f"{label}_twohop{limit}_{center + 1}"] = twohop(center, limit, weights, adj, rank)
                supports[f"{label}_dense{limit}_{center + 1}"] = dense(center, limit, weights, adj, rank)
    unique = {}
    for name, vertices in supports.items():
        unique.setdefault(support_hash(vertices), (name, vertices))
    return list(unique.values())[:SUPPORT_CAP]


def prior_support_hashes(weights, adj):
    supports = [pocket(name, weights, adj) for name, _ in ACCEPTED]
    if ADAPTIVE.exists():
        data = json.loads(ADAPTIVE.read_text())
        for row in data.get("tested_branch_rows", []):
            for side in ("include", "exclude"):
                for top in row.get(side, {}).get("top_rows", []):
                    pass
    return supports, {support_hash(support) for support in supports}


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
    c0 = exclude_coverage(weights)
    hybrid = weights + c0 / HYBRID_SCALE
    root_cuts = [(pocket(name, weights, adj), weights[list(pocket(name, weights, adj))], alpha) for name, alpha in ACCEPTED]
    lift = json.loads(PARENT_LIFT.read_text())
    parent_coeffs = {v: float(c0[v]) for v in range(N) if v != BRANCH and c0[v] > 0}
    parent_coeffs[BRANCH] = float(lift["new_lift_coefficient"])
    parent_row = (parent_coeffs, float(lift["new_rhs_numerator"]))
    base, x = solve_lp(edges, triads, weights, root_cuts, [parent_row], True)
    retained_supports, retained_hashes = prior_support_hashes(weights, adj)
    rows = []
    accepted = []
    for name, vertices in candidate_supports(weights, c0, x, adj):
        max_overlap = max(jaccard(vertices, support) for support in retained_supports)
        duplicate = support_hash(vertices) in retained_hashes
        for family, coeffs in (("w", weights), ("c0", c0), ("hybrid", hybrid)):
            alpha, ok, gap, seconds = solve_mwis(vertices, coeffs, adj)
            if not ok:
                continue
            lhs = float(np.dot(coeffs[list(vertices)], x[list(vertices)]))
            violation = lhs - alpha
            single_drop = 0.0
            accepted_row = False
            if violation >= VIOLATION_GATE and not duplicate:
                trial_cut = (vertices, coeffs[list(vertices)], alpha)
                objective = solve_lp(edges, triads, weights, root_cuts + [trial_cut], [parent_row])
                single_drop = base - objective
                accepted_row = max_overlap <= LOW_OVERLAP or single_drop >= HIGH_OVERLAP_DROP
                if accepted_row:
                    accepted.append(trial_cut)
            rows.append({
                "name": name,
                "family": family,
                "size": len(vertices),
                "alpha": alpha,
                "lhs": lhs,
                "violation": violation,
                "single_drop": single_drop,
                "max_retained_jaccard": max_overlap,
                "duplicate_retained": duplicate,
                "accepted": accepted_row,
                "seconds": seconds,
            })
    final = solve_lp(edges, triads, weights, root_cuts + accepted, [parent_row])
    total_drop = base - final
    low_overlap_accepts = sum(1 for row in rows if row["accepted"] and row["max_retained_jaccard"] <= LOW_OVERLAP)
    status = "RetirePostParentLiftRankSeparation"
    if total_drop >= FUND_TOTAL_DROP or (total_drop >= 5000 and low_overlap_accepts > 0):
        status = "FundPostParentLiftRankSeparation"
    report = clean({
        "schema": "forge.hadwiger.w607_post_parent_lift_rank_separation.v1",
        "base_parent_lift_objective": base,
        "candidate_support_count": len(candidate_supports(weights, c0, x, adj)),
        "tested_row_count": len(rows),
        "accepted_row_count": len(accepted),
        "low_overlap_accepted_row_count": low_overlap_accepts,
        "final_objective": final,
        "total_drop": total_drop,
        "status": status,
        "top_rows": sorted(rows, key=lambda row: row["violation"], reverse=True)[:30],
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "top_rows"}, indent=2))


if __name__ == "__main__":
    main()
