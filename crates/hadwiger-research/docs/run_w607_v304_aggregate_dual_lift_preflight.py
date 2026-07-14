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
CERT_PATH = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"
OUT_PATH = CRATE / "docs" / "w607-v304-aggregate-dual-lift-preflight.json"

N = 607
EXCLUDED = 303
DENOMINATOR = 1024
U0_NUM = 647496725
U1_NUM = 618626223
W304 = 36195
KNOWN_ROOT = 641090.9615275887
KNOWN_SPLIT = U0_NUM / DENOMINATOR
VIOLATION_GATE_NUM = 1024000
DROP_GATE = 1000.0
SPLIT_IMPROVEMENT_GATE = 100.0

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
    return edges, weights.astype(int)


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
    weights = weights.astype(float)
    if name == "top_weight_120":
        return tuple(sorted(np.lexsort((np.arange(N), -weights))[:120]))
    kind, raw = name.rsplit("_", 1)
    center = int(raw) - 1
    if kind.startswith("twohop"):
        return twohop(center, int(kind.removeprefix("twohop")), weights, adj, weights)
    if kind.startswith("dense"):
        return dense(center, int(kind.removeprefix("dense")), weights, adj, weights)
    raise ValueError(name)


def certificate_coverage(weights):
    cert = json.loads(CERT_PATH.read_text())
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
                if vertex != EXCLUDED + 1:
                    coverage[vertex - 1] += numerator
        elif kind == "child_weighted_rank":
            alpha = int(row["alpha_w"])
            objective += numerator * alpha
            for vertex in row["support_vertices"]:
                if vertex != EXCLUDED + 1:
                    coverage[vertex - 1] += numerator * int(weights[vertex - 1])
        else:
            raise ValueError(kind)
    return cert, coverage, objective, counts


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
    value = incumbent if ok else None
    upper_bound = int(np.ceil(-dual_bound)) if dual_bound is not None else None
    chosen = []
    if result.x is not None:
        chosen = [vertices[i] + 1 for i, value_x in enumerate(result.x) if value_x > 0.5]
    return value, incumbent, upper_bound, ok, gap, time.time() - start, chosen


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
    cert, coverage, objective_num, counts = certificate_coverage(weights)
    root_cuts = [(pocket(name, weights, adj), alpha) for name, alpha in ACCEPTED]
    root_obj, root_x = solve_lp(edges, triads, weights, root_cuts, solution=True)
    active_slacks = [
        int(coverage[v]) - int(weights[v]) * DENOMINATOR
        for v in range(N)
        if v != EXCLUDED
    ]
    residual = [v for v in range(N) if v != EXCLUDED and v not in adj[EXCLUDED]]
    gamma, gamma_incumbent, gamma_upper, gamma_ok, gamma_gap, gamma_seconds, witness = solve_gamma(residual, coverage, adj)
    lift = U0_NUM - gamma if gamma_ok else None
    certified_lift_lower_bound = U0_NUM - gamma_upper if gamma_upper is not None else None
    lp_lift = lift if lift is not None else certified_lift_lower_bound
    comparable_split_coeff = U0_NUM - U1_NUM + DENOMINATOR * W304
    root_lhs_num = int(round(sum(float(coverage[v]) * root_x[v] for v in range(N) if v != EXCLUDED)))
    root_lhs_with_lift_num = root_lhs_num + int(round((lp_lift or 0) * root_x[EXCLUDED]))
    violation_num = root_lhs_with_lift_num - U0_NUM if lp_lift is not None else None
    root_with_aggregate = None
    root_drop = None
    root_with_aggregate_x304 = None
    split_with_aggregate = None
    split_improvement = None
    split_with_aggregate_x304 = None
    if lp_lift is not None and lp_lift > 0 and violation_num >= VIOLATION_GATE_NUM:
        aggregate_coeffs = {v: int(coverage[v]) for v in range(N) if v != EXCLUDED and int(coverage[v])}
        aggregate_coeffs[EXCLUDED] = lp_lift
        aggregate_row = (aggregate_coeffs, U0_NUM)
        root_with_aggregate, agg_x = solve_lp(edges, triads, weights, root_cuts, [aggregate_row], True)
        root_drop = root_obj - root_with_aggregate
        root_with_aggregate_x304 = float(agg_x[EXCLUDED])
        split_coeffs = {vertex: int(weights[vertex]) * DENOMINATOR for vertex in range(N)}
        split_coeffs[EXCLUDED] += U0_NUM - U1_NUM
        split_row = (split_coeffs, U0_NUM)
        split_with_aggregate, split_x = solve_lp(edges, triads, weights, root_cuts, [split_row, aggregate_row], True)
        split_improvement = KNOWN_SPLIT - split_with_aggregate
        split_with_aggregate_x304 = float(split_x[EXCLUDED])
    positive_slack = sorted(
        [
            {
                "vertex": v + 1,
                "slack_num": int(coverage[v]) - int(weights[v]) * DENOMINATOR,
                "weight": int(weights[v]),
            }
            for v in range(N)
            if v != EXCLUDED and int(coverage[v]) > int(weights[v]) * DENOMINATOR
        ],
        key=lambda row: row["slack_num"],
        reverse=True,
    )[:20]
    status = "RetireAggregateDualLiftPreflight"
    if root_drop is not None and root_drop >= DROP_GATE:
        status = "FundAggregateDualLiftRootCutWithCertifiedLift"
    if split_improvement is not None and split_improvement >= SPLIT_IMPROVEMENT_GATE:
        status = "FundAggregateDualLiftBeyondSplit"
    report = clean({
        "schema": "forge.hadwiger.w607_v304_aggregate_dual_lift_preflight.v1",
        "excluded_vertex": EXCLUDED + 1,
        "certificate_objective_numerator": objective_num,
        "expected_objective_numerator": U0_NUM,
        "denominator": DENOMINATOR,
        "row_counts": counts,
        "min_active_slack_num": min(active_slacks),
        "tight_active_vertices": sum(1 for slack in active_slacks if slack == 0),
        "positive_slack_vertices": sum(1 for slack in active_slacks if slack > 0),
        "root_objective": root_obj,
        "known_root_objective": KNOWN_ROOT,
        "residual_vertex_count": len(residual),
        "gamma_success": gamma_ok,
        "gamma_mip_gap": gamma_gap,
        "gamma_seconds": gamma_seconds,
        "gamma_numerator": gamma,
        "gamma_incumbent_numerator": gamma_incumbent,
        "gamma_upper_bound_numerator": gamma_upper,
        "lift_coefficient_l": lift,
        "certified_lift_lower_bound": certified_lift_lower_bound,
        "lp_test_lift_coefficient": lp_lift,
        "residual_objective_split_coefficient": comparable_split_coeff,
        "lift_minus_residual_split_coefficient": lift - comparable_split_coeff if lift is not None else None,
        "root_x304": float(root_x[EXCLUDED]),
        "root_aggregate_lhs_num": root_lhs_num,
        "root_aggregate_lhs_with_lift_num": root_lhs_with_lift_num if gamma_ok else None,
        "root_aggregate_violation_num": violation_num,
        "root_aggregate_violation": violation_num / DENOMINATOR if violation_num is not None else None,
        "root_with_aggregate_objective": root_with_aggregate,
        "root_aggregate_drop": root_drop,
        "root_with_aggregate_x304": root_with_aggregate_x304,
        "split_with_aggregate_objective": split_with_aggregate,
        "split_aggregate_improvement_over_split": split_improvement,
        "split_with_aggregate_x304": split_with_aggregate_x304,
        "top_positive_slack_vertices": positive_slack,
        "gamma_witness_size": len(witness),
        "gamma_witness_head": witness[:40],
        "status": status,
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    summary = {key: report[key] for key in report if key not in {"top_positive_slack_vertices", "gamma_witness_head"}}
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
