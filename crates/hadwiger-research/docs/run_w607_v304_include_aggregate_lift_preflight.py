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
INCLUDE_CERT = CRATE / "docs" / "w607-v304-include-dual-cover-den1024.json"
EXCLUDE_AGG = CRATE / "docs" / "w607-v304-aggregate-dual-lift-preflight.json"
OUT_PATH = CRATE / "docs" / "w607-v304-include-aggregate-lift-preflight.json"

N = 607
BRANCH = 303
DENOMINATOR = 1024
U1_NUM = 618626223
KNOWN_ROOT = 641090.9615275887
CURRENT_BEST = 632232.3996589413
VIOLATION_GATE_NUM = 1024000
NOVELTY_GATE = 100.0

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


def include_coverage(weights):
    cert = json.loads(INCLUDE_CERT.read_text())
    coverage = np.zeros(N, dtype=object)
    objective = 0
    counts = {}
    for row in cert["rows"]:
        kind = row["kind"]
        counts[kind] = counts.get(kind, 0) + 1
        numerator = int(row["numerator"])
        if kind == "included_vertex":
            vertex = int(row["vertex"]) - 1
            coverage[vertex] += numerator * int(row["weight"])
            objective += numerator * int(row["weight"])
        elif kind in ("edge", "triangle"):
            objective += numerator * int(row.get("rhs", 1))
            for vertex in row["vertices"]:
                coverage[vertex - 1] += numerator
        else:
            raise ValueError(kind)
    return coverage, objective, counts


def solve_lp(edges, triads, weights, cuts, extra_rows=None, solution=False):
    extra_rows = extra_rows or []
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
            matrix[row, v] = float(weights[v])
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


def solve_gamma(vertices, coverage, adj):
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
        c=-np.array([float(coverage[v]) for v in vertices]),
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": 180, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    dual_bound = getattr(result, "mip_dual_bound", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    incumbent = int(round(-result.fun)) if result.fun is not None else None
    upper = int(np.ceil(-dual_bound)) if dual_bound is not None else None
    witness = []
    if result.x is not None:
        witness = [vertices[i] + 1 for i, value in enumerate(result.x) if value > 0.5]
    return incumbent if ok else None, incumbent, upper, ok, gap, time.time() - start, witness


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
    coverage, objective_num, counts = include_coverage(weights)
    closed = {BRANCH, *adj[BRANCH]}
    active = [v for v in range(N) if v not in closed]
    neighbor_coverage = sum(int(coverage[v]) for v in adj[BRANCH])
    active_slacks = [int(coverage[v]) - int(weights[v]) * DENOMINATOR for v in active]
    c_root = int(round(sum(float(coverage[v]) * root_x[v] for v in range(N))))
    root_slack_fraction = 1.0 - float(root_x[BRANCH])
    m_gate_root = (c_root - U1_NUM - VIOLATION_GATE_NUM) / root_slack_fraction
    gamma = gamma_incumbent = gamma_upper = gamma_gap = gamma_seconds = None
    witness = []
    m_conservative = None
    root_with_include = root_drop = root_with_include_x304 = None
    split_with_include = split_improvement = split_with_include_x304 = None
    both_with_include = both_improvement = both_with_include_x304 = None
    status = "RetireIncludeAggregateLiftPrecheck"
    if m_gate_root > 0:
        positive_vertices = [v for v in active if int(coverage[v]) > 0]
        gamma, gamma_incumbent, gamma_upper, gamma_ok, gamma_gap, gamma_seconds, witness = solve_gamma(
            positive_vertices, coverage, adj
        )
        m_conservative = max(0, gamma_upper - U1_NUM) if gamma_upper is not None else None
        if m_conservative is not None and m_conservative <= m_gate_root:
            include_coeffs = {v: int(coverage[v]) for v in range(N) if int(coverage[v])}
            include_coeffs[BRANCH] = int(coverage[BRANCH]) + m_conservative
            include_row = (include_coeffs, U1_NUM + m_conservative)
            root_with_include, include_x = solve_lp(edges, triads, weights, root_cuts, [include_row], True)
            root_drop = root_obj - root_with_include
            root_with_include_x304 = float(include_x[BRANCH])
            split_coeffs = {v: int(weights[v]) * DENOMINATOR for v in range(N)}
            split_coeffs[BRANCH] += 647496725 - U1_NUM
            split_row = (split_coeffs, 647496725)
            split_with_include, split_x = solve_lp(edges, triads, weights, root_cuts, [split_row, include_row], True)
            split_improvement = 647496725 / DENOMINATOR - split_with_include
            split_with_include_x304 = float(split_x[BRANCH])
            exclude = json.loads(EXCLUDE_AGG.read_text())
            exclude_lift = int(exclude["lp_test_lift_coefficient"])
            exclude_coeffs = {v: int(weights[v]) * 0 for v in range(N)}
            # Reuse the previous aggregate row exactly by parsing its source certificate.
            exclude_cov, _, _ = exclude_coverage(weights)
            exclude_coeffs = {v: int(exclude_cov[v]) for v in range(N) if v != BRANCH and int(exclude_cov[v])}
            exclude_coeffs[BRANCH] = exclude_lift
            exclude_row = (exclude_coeffs, 647496725)
            both_with_include, both_x = solve_lp(edges, triads, weights, root_cuts, [exclude_row, include_row], True)
            both_improvement = CURRENT_BEST - both_with_include
            both_with_include_x304 = float(both_x[BRANCH])
            status = "RetireIncludeAggregateLiftAfterLp"
            if split_improvement is not None and split_improvement >= NOVELTY_GATE:
                status = "FundIncludeAggregateLiftBeyondSplit"
            if both_improvement is not None and both_improvement >= NOVELTY_GATE:
                status = "FundPairedAggregateLift"
        else:
            status = "RetireIncludeAggregateLiftWeakConservativeM"
    report = clean({
        "schema": "forge.hadwiger.w607_v304_include_aggregate_lift_preflight.v1",
        "branch_vertex": BRANCH + 1,
        "certificate_objective_numerator": objective_num,
        "expected_objective_numerator": U1_NUM,
        "denominator": DENOMINATOR,
        "row_counts": counts,
        "included_vertex_coverage_num": int(coverage[BRANCH]),
        "expected_included_vertex_coverage_num": int(weights[BRANCH]) * DENOMINATOR,
        "neighbor_coverage_num": neighbor_coverage,
        "min_active_slack_num": min(active_slacks),
        "tight_active_vertices": sum(1 for slack in active_slacks if slack == 0),
        "positive_active_slack_vertices": sum(1 for slack in active_slacks if slack > 0),
        "root_objective": root_obj,
        "known_root_objective": KNOWN_ROOT,
        "root_x304": float(root_x[BRANCH]),
        "root_include_coverage_lhs_num": c_root,
        "m_gate_root": m_gate_root,
        "gamma_numerator": gamma,
        "gamma_incumbent_numerator": gamma_incumbent,
        "gamma_upper_bound_numerator": gamma_upper,
        "gamma_mip_gap": gamma_gap,
        "gamma_seconds": gamma_seconds,
        "m_conservative": m_conservative,
        "root_with_include_aggregate_objective": root_with_include,
        "root_include_aggregate_drop": root_drop,
        "root_with_include_aggregate_x304": root_with_include_x304,
        "split_with_include_aggregate_objective": split_with_include,
        "split_include_aggregate_improvement": split_improvement,
        "split_with_include_aggregate_x304": split_with_include_x304,
        "paired_aggregate_objective": both_with_include,
        "paired_aggregate_improvement_over_current_best": both_improvement,
        "paired_aggregate_x304": both_with_include_x304,
        "gamma_witness_size": len(witness),
        "gamma_witness_head": witness[:40],
        "status": status,
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    summary = {k: v for k, v in report.items() if k != "gamma_witness_head"}
    print(json.dumps(summary, indent=2))


def exclude_coverage(weights):
    cert = json.loads((CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json").read_text())
    coverage = np.zeros(N, dtype=object)
    objective = 0
    counts = {}
    for row in cert["rows"]:
        kind = row["kind"]
        counts[kind] = counts.get(kind, 0) + 1
        numerator = int(row["numerator"])
        if kind == "parent_triangle":
            objective += numerator
            for vertex in row["vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator
        elif kind == "child_weighted_rank":
            alpha = int(row["alpha_w"])
            objective += numerator * alpha
            for vertex in row["support_vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator * int(weights[vertex - 1])
        else:
            raise ValueError(kind)
    return coverage, objective, counts


if __name__ == "__main__":
    main()
