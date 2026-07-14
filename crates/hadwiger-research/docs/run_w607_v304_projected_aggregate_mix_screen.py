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
EXCLUDE_CERT = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"
EXCLUDE_AGG = CRATE / "docs" / "w607-v304-aggregate-dual-lift-preflight.json"
OUT_PATH = CRATE / "docs" / "w607-v304-projected-aggregate-mix-screen.json"

N = 607
BRANCH = 303
DENOMINATOR = 1024
U0_NUM = 647496725
U1_NUM = 618626223
KNOWN_ROOT = 641090.9615275887
CURRENT_BEST = 632232.3996589413
INCLUDE_CERT_BOUND = 618626223 / DENOMINATOR
VIOLATION_GATE_NUM = 102400
LP_IMPROVEMENT_GATE = 100.0
GRID = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0]
MWIS_TEST_CAP = 1
MWIS_TIME_LIMIT_SECONDS = 180

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


def coverage_from_cert(path, weights):
    cert = json.loads(path.read_text())
    coverage = np.zeros(N, dtype=object)
    objective = 0
    for row in cert["rows"]:
        kind = row["kind"]
        numerator = int(row["numerator"])
        if kind == "included_vertex":
            vertex = int(row["vertex"]) - 1
            coverage[vertex] += numerator * int(row["weight"])
            objective += numerator * int(row["weight"])
        elif kind in ("edge", "triangle", "parent_triangle"):
            objective += numerator * int(row.get("rhs", 1))
            for vertex in row["vertices"]:
                if not (kind == "parent_triangle" and vertex == BRANCH + 1):
                    coverage[vertex - 1] += numerator
        elif kind == "child_weighted_rank":
            alpha = int(row["alpha_w"])
            objective += numerator * alpha
            for vertex in row["support_vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator * int(weights[vertex - 1])
        else:
            raise ValueError(kind)
    return coverage, objective


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


def greedy_lower(vertices, coeffs, adj):
    chosen = []
    blocked = set()
    for vertex in sorted(vertices, key=lambda v: (-coeffs[v], v)):
        if coeffs[vertex] <= 0 or vertex in blocked:
            continue
        chosen.append(vertex)
        blocked.add(vertex)
        blocked.update(adj[vertex])
    return int(sum(coeffs[v] for v in chosen)), [v + 1 for v in chosen]


def solve_gamma(vertices, coeffs, adj):
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
        c=-np.array([float(coeffs[v]) for v in vertices]),
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": MWIS_TIME_LIMIT_SECONDS, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    dual_bound = getattr(result, "mip_dual_bound", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    incumbent = int(round(-result.fun)) if result.fun is not None else None
    upper = int(np.ceil(-dual_bound)) if dual_bound is not None else None
    return incumbent if ok else None, incumbent, upper, ok, gap, time.time() - start


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
    c0, u0 = coverage_from_cert(EXCLUDE_CERT, weights)
    c1, u1 = coverage_from_cert(INCLUDE_CERT, weights)
    exclude_artifact = json.loads(EXCLUDE_AGG.read_text())
    exclude_lift = int(exclude_artifact["lp_test_lift_coefficient"])
    exclude_coeffs = {v: int(c0[v]) for v in range(N) if v != BRANCH and int(c0[v])}
    exclude_coeffs[BRANCH] = exclude_lift
    exclude_row = (exclude_coeffs, U0_NUM)
    split_coeffs = {v: int(weights[v]) * DENOMINATOR for v in range(N)}
    split_coeffs[BRANCH] += U0_NUM - U1_NUM
    split_row = (split_coeffs, U0_NUM)
    current_obj, y = solve_lp(edges, triads, weights, root_cuts, [split_row, exclude_row], True)
    if abs(current_obj - CURRENT_BEST) > 1e-3:
        raise ValueError(f"current best mismatch {current_obj}")
    active = [v for v in range(N) if v != BRANCH]
    rows = []
    for lam in GRID:
        scale = 100
        lam_i = int(round(lam * scale))
        coeffs = np.array([lam_i * int(c0[v]) + (scale - lam_i) * int(c1[v]) for v in range(N)], dtype=object)
        lhs_y = int(round(sum(float(coeffs[v]) * y[v] for v in active)))
        greedy, greedy_vertices = greedy_lower(active, coeffs, adj)
        margin_over_greedy = lhs_y - greedy
        row = {
            "lambda_num": lam_i,
            "lambda_den": scale,
            "lhs_current_solution": lhs_y,
            "greedy_stable_lower_bound": greedy,
            "margin_over_greedy": margin_over_greedy,
            "passes_greedy_screen": margin_over_greedy >= VIOLATION_GATE_NUM,
            "greedy_witness_size": len(greedy_vertices),
            "greedy_witness_head": greedy_vertices[:20],
        }
        row["coeffs"] = coeffs
        rows.append(row)
    candidates = sorted(
        [row for row in rows if row["passes_greedy_screen"]],
        key=lambda row: row["margin_over_greedy"],
        reverse=True,
    )[:MWIS_TEST_CAP]
    tested = []
    for row in candidates:
        coeffs = row["coeffs"]
        gamma, incumbent, upper, ok, gap, seconds = solve_gamma(active, coeffs, adj)
        rhs = gamma if ok else upper
        violation = row["lhs_current_solution"] - rhs if rhs is not None else None
        row.update({
            "gamma_success": ok,
            "gamma_incumbent": incumbent,
            "gamma_upper_bound": upper,
            "gamma_mip_gap": gap,
            "gamma_seconds": seconds,
            "projected_rhs_used": rhs,
            "projected_violation_num": violation,
            "projected_violation": violation / (scale * DENOMINATOR) if violation is not None else None,
        })
        if violation is not None and violation >= VIOLATION_GATE_NUM:
            mix_coeffs = {v: int(coeffs[v]) for v in active if int(coeffs[v])}
            mix_row = (mix_coeffs, rhs)
            fixed_exclude_row = ({BRANCH: 1}, 0)
            obj, x = solve_lp(
                edges,
                triads,
                weights,
                root_cuts,
                [split_row, exclude_row, fixed_exclude_row, mix_row],
                True,
            )
            row.update({
                "lp_objective_with_projected_mix": obj,
                "lp_improvement_over_current_best": CURRENT_BEST - obj,
                "lp_x304": float(x[BRANCH]),
                "lp_fixed_x304_zero": True,
            })
            tested.append(row)
    for row in rows:
        row.pop("coeffs", None)
    best_improvement = max((row.get("lp_improvement_over_current_best", 0.0) for row in tested), default=0.0)
    best_projected_objective = min(
        (row["lp_objective_with_projected_mix"] for row in tested if "lp_objective_with_projected_mix" in row),
        default=None,
    )
    diagnostic_branch_max = max(INCLUDE_CERT_BOUND, best_projected_objective) if best_projected_objective is not None else None
    status = "RetireProjectedAggregateMixScreen"
    if best_improvement >= LP_IMPROVEMENT_GATE:
        status = "FundProjectedAggregateMixCut"
    report = clean({
        "schema": "forge.hadwiger.w607_v304_projected_aggregate_mix_screen.v1",
        "u0_numerator": u0,
        "u1_numerator": u1,
        "denominator": DENOMINATOR,
        "current_best_objective": current_obj,
        "current_best_x304": float(y[BRANCH]),
        "grid": GRID,
        "violation_gate_num": VIOLATION_GATE_NUM,
        "lp_improvement_gate": LP_IMPROVEMENT_GATE,
        "greedy_pass_count": sum(1 for row in rows if row["passes_greedy_screen"]),
        "attempted_mwis_count": len(candidates),
        "mwis_test_cap": MWIS_TEST_CAP,
        "mwis_time_limit_seconds": MWIS_TIME_LIMIT_SECONDS,
        "tested_lp_count": len(tested),
        "best_lp_improvement": best_improvement,
        "best_projected_exclude_objective": best_projected_objective,
        "include_certificate_bound": INCLUDE_CERT_BOUND,
        "diagnostic_one_node_branch_max": diagnostic_branch_max,
        "rows": rows,
        "status": status,
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    summary = {k: v for k, v in report.items() if k != "rows"}
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
