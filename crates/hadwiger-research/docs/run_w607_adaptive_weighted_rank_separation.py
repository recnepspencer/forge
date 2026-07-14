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
OUT_PATH = CRATE / "docs" / "w607-adaptive-weighted-rank-separation.json"

N = 607
KNOWN_POST16_OBJECTIVE = 641090.9615275887
LP_TOLERANCE = 1e-4
CURRENT_VIOLATION_THRESHOLD = 3000.0
MAX_JACCARD_ACCEPTED = 0.70
MIN_TOTAL_DROP = 500.0
MIN_BEST_SINGLE_DROP = 100.0

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
    if len(edges) != 3390 or len(weights) != N:
        raise ValueError("unexpected W607 data shape")
    return edges, weights


def parse_half_turn():
    tokens = re.findall(r"-?\d+(?:/\d+)?", VERTICES_PATH.read_text())
    values = [Fraction(token) for token in tokens]
    vertices = [tuple(values[i : i + 8]) for i in range(0, len(values), 8)]
    index = {row: i for i, row in enumerate(vertices)}
    image = []
    for row in vertices:
        x, y = row[:4], row[4:]
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


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def jaccard(left, right):
    left, right = set(left), set(right)
    return len(left & right) / len(left | right)


def induced_edges(vertices, adj):
    total = 0
    for i, a in enumerate(vertices):
        for b in vertices[i + 1 :]:
            total += b in adj[a]
    return total


def pocket(name, weights, adj):
    if name == "top_weight_120":
        return tuple(sorted(np.lexsort((np.arange(N), -weights))[:120]))
    kind, raw = name.rsplit("_", 1)
    center = int(raw) - 1
    if kind.startswith("twohop"):
        return clipped_twohop(center, int(kind.removeprefix("twohop")), weights, adj, weights)
    if kind.startswith("dense"):
        return dense_expand(center, int(kind.removeprefix("dense")), weights, adj, weights)
    raise ValueError(name)


def clipped_onehop(center, limit, rank, adj):
    seen = {center, *adj[center]}
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:limit]))


def clipped_twohop(center, limit, weights, adj, rank):
    seen = {center, *adj[center]}
    for vertex in list(seen):
        seen.update(adj[vertex])
    return tuple(sorted(sorted(seen, key=lambda v: (-rank[v], v))[:limit]))


def dense_expand(center, limit, weights, adj, rank):
    chosen = [center]
    chosen_set = {center}
    frontier = set(adj[center])
    while len(chosen) < limit and frontier:
        def score(v):
            contact = sum(weights[u] for u in chosen if u in adj[v])
            frac_contact = sum(rank[u] for u in chosen if u in adj[v])
            return (frac_contact * 1e6 + contact * 10 + rank[v], -v)
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


def solve_lp(edges, triads, weights, cuts, return_solution=False):
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
    objective = -float(result.fun)
    return (objective, result.x) if return_solution else objective


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
        options={"time_limit": 120, "mip_rel_gap": 0.0},
    )
    seconds = time.time() - start
    gap = getattr(result, "mip_gap", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    if not ok:
        return None, seconds, False, gap, []
    chosen = [vertices[i] for i, value in enumerate(result.x) if value > 0.5]
    return int(round(-result.fun)), seconds, True, gap, chosen


def candidate_supports(weights, adj, x):
    rank = weights * x
    order = list(np.lexsort((np.arange(N), -rank))[:20])
    supports = {}
    for limit in (80, 100, 120):
        supports[f"top_wx_{limit}"] = tuple(sorted(np.lexsort((np.arange(N), -rank))[:limit]))
    for center in order:
        for limit in (80, 100, 120):
            supports[f"onehop{limit}_{center + 1}"] = clipped_onehop(center, limit, rank, adj)
            supports[f"twohop{limit}_{center + 1}"] = clipped_twohop(center, limit, weights, adj, rank)
            supports[f"dense{limit}_{center + 1}"] = dense_expand(center, limit, weights, adj, rank)
    unique = {}
    for name, vertices in supports.items():
        unique.setdefault(support_hash(vertices), (name, vertices))
    return list(unique.values())[:250], order


def row_for_candidate(name, vertices, alpha, seconds, ok, gap, witness, weights, adj, x, image, retained):
    mirror = tuple(sorted(image[v] for v in vertices))
    current_lhs = float(np.dot(weights[list(vertices)], x[list(vertices)]))
    max_overlap = max(jaccard(vertices, support) for support in retained)
    current_violation = None if alpha is None else current_lhs - alpha
    root_violation_numerator = None if alpha is None else int(sum(weights[list(vertices)]) - 3 * alpha)
    return {
        "name": name,
        "size": len(vertices),
        "edge_count": induced_edges(vertices, adj),
        "weight_sum": int(sum(weights[list(vertices)])),
        "alpha_w": alpha,
        "current_lp_lhs": current_lhs,
        "current_lp_violation": current_violation,
        "root_violation_numerator": root_violation_numerator,
        "solver_success": ok,
        "mip_gap": gap,
        "seconds": round(seconds, 4),
        "witness_weight": int(sum(weights[witness])) if witness else 0,
        "witness_size": len(witness),
        "support_hash": support_hash(vertices),
        "half_turn_canonical_hash": min(support_hash(vertices), support_hash(mirror)),
        "max_jaccard_against_retained": max_overlap,
    }


def clean_json(value):
    if isinstance(value, dict):
        return {key: clean_json(inner) for key, inner in value.items()}
    if isinstance(value, list):
        return [clean_json(inner) for inner in value]
    if isinstance(value, tuple):
        return [clean_json(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    edges, weights = parse_edges_weights()
    adj = adjacency(edges)
    image = parse_half_turn()
    triads = triangles(adj)
    retained = [pocket(name, weights, adj) for name, _ in ACCEPTED]
    retained_cuts = list(zip(retained, [alpha for _, alpha in ACCEPTED]))
    retained_hashes = {support_hash(vertices) for vertices in retained}
    retained_orbits = {
        min(support_hash(vertices), support_hash(tuple(sorted(image[v] for v in vertices))))
        for vertices in retained
    }
    baseline_objective, x = solve_lp(edges, triads, weights, retained_cuts, True)
    if abs(baseline_objective - KNOWN_POST16_OBJECTIVE) > LP_TOLERANCE:
        raise ValueError(f"post-16 LP mismatch: {baseline_objective}")

    candidates, centers = candidate_supports(weights, adj, x)
    candidate_rows = []
    accepted = []
    violated = []
    for _, (name, vertices) in enumerate(candidates):
        orbit_hash = min(
            support_hash(vertices),
            support_hash(tuple(sorted(image[v] for v in vertices))),
        )
        if support_hash(vertices) in retained_hashes or orbit_hash in retained_orbits:
            alpha, seconds, ok, gap, witness = None, 0.0, False, None, []
        else:
            alpha, seconds, ok, gap, witness = solve_mwis(vertices, weights, adj)
        row = row_for_candidate(name, vertices, alpha, seconds, ok, gap, witness, weights, adj, x, image, retained)
        row["duplicate_retained_support"] = support_hash(vertices) in retained_hashes
        row["duplicate_retained_orbit"] = orbit_hash in retained_orbits
        row["accepted_as_new_cut"] = bool(
            ok
            and row["current_lp_violation"] is not None
            and row["current_lp_violation"] >= CURRENT_VIOLATION_THRESHOLD
            and row["max_jaccard_against_retained"] <= MAX_JACCARD_ACCEPTED
            and not row["duplicate_retained_support"]
            and not row["duplicate_retained_orbit"]
        )
        candidate_rows.append(row)
        if (
            ok
            and row["current_lp_violation"] is not None
            and row["current_lp_violation"] >= CURRENT_VIOLATION_THRESHOLD
            and not row["duplicate_retained_support"]
            and not row["duplicate_retained_orbit"]
        ):
            violated.append((vertices, alpha, name))
        if row["accepted_as_new_cut"]:
            accepted.append((vertices, alpha, name))

    accepted_cuts = [(vertices, alpha) for vertices, alpha, _ in accepted]
    violated_cuts = [(vertices, alpha) for vertices, alpha, _ in violated]
    all_new_objective = solve_lp(edges, triads, weights, retained_cuts + accepted_cuts)
    all_violated_objective = solve_lp(edges, triads, weights, retained_cuts + violated_cuts)
    total_drop = baseline_objective - all_new_objective
    diagnostic_violated_drop = baseline_objective - all_violated_objective
    single_drops = []
    for vertices, alpha, name in accepted:
        objective = solve_lp(edges, triads, weights, retained_cuts + [(vertices, alpha)])
        single_drops.append({"name": name, "drop": baseline_objective - objective})
    best_single_drop = max((row["drop"] for row in single_drops), default=0.0)
    diagnostic_single_drops = []
    for vertices, alpha, name in violated:
        objective = solve_lp(edges, triads, weights, retained_cuts + [(vertices, alpha)])
        diagnostic_single_drops.append({"name": name, "drop": baseline_objective - objective})
    diagnostic_best_single_drop = max((row["drop"] for row in diagnostic_single_drops), default=0.0)
    status = "RetireAdaptiveWeightedRankSeparation"
    if total_drop >= MIN_TOTAL_DROP and best_single_drop >= MIN_BEST_SINGLE_DROP:
        status = "FundAdaptiveWeightedRankSeparation"

    report = {
        "schema": "forge.hadwiger.w607_adaptive_weighted_rank_separation.v1",
        "baseline_post16_objective": baseline_objective,
        "known_post16_objective": KNOWN_POST16_OBJECTIVE,
        "candidate_count": len(candidate_rows),
        "diagnostic_violated_cut_count": len(violated),
        "accepted_new_cut_count": len(accepted),
        "root_lp_with_existing_plus_new_cuts": all_new_objective,
        "total_marginal_drop": total_drop,
        "root_lp_with_all_current_violated_candidates": all_violated_objective,
        "diagnostic_all_violated_drop": diagnostic_violated_drop,
        "best_single_cut_drop": best_single_drop,
        "diagnostic_best_single_violated_drop": diagnostic_best_single_drop,
        "top_wx_centers": [center + 1 for center in centers],
        "thresholds": {
            "baseline_lp_tolerance": LP_TOLERANCE,
            "min_current_lp_violation": CURRENT_VIOLATION_THRESHOLD,
            "max_jaccard_against_retained": MAX_JACCARD_ACCEPTED,
            "min_total_drop_to_fund": MIN_TOTAL_DROP,
            "min_best_single_drop_to_fund": MIN_BEST_SINGLE_DROP,
        },
        "status": status,
        "single_cut_drops": single_drops,
        "diagnostic_single_violated_drops": diagnostic_single_drops,
        "candidate_rows": candidate_rows,
    }
    report = clean_json(report)
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    summary_keys = [
        key
        for key in report
        if key not in {"candidate_rows", "single_cut_drops", "diagnostic_single_violated_drops"}
    ]
    print(json.dumps({key: report[key] for key in summary_keys}, indent=2))


if __name__ == "__main__":
    main()
