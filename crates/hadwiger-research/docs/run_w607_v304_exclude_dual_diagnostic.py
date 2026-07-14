import json
import math
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
OUT_PATH = CRATE / "docs" / "w607-v304-exclude-dual-diagnostic.json"
CERT_OUT_PATH = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"

N = 607
EXCLUDED = 303
KNOWN_EXCLUDE_BOUND = 632232.3996589432
POSITIVE_TOLERANCE = 1e-8
MAX_ROWS = 1500
MIN_CHILD_RANK_MASS_FRACTION = 0.05

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
        return twohop(center, int(kind.removeprefix("twohop")), weights, adj, weights)
    if kind.startswith("dense"):
        return dense(center, int(kind.removeprefix("dense")), weights, adj, weights)
    raise ValueError(name)


def top_by_rank(name, rank, limit):
    return name, tuple(sorted(np.lexsort((np.arange(N), -rank))[:limit]))


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


def solve_mwis(vertices, weights, adj):
    local = {v: i for i, v in enumerate(vertices)}
    constraints = []
    for i, a in enumerate(vertices):
        for b in vertices[i + 1 :]:
            if b in adj[a]:
                row = np.zeros(len(vertices))
                row[i] = 1
                row[local[b]] = 1
                constraints.append(row)
    linear = LinearConstraint(np.vstack(constraints), -np.inf, np.ones(len(constraints))) if constraints else None
    result = milp(
        c=-weights[list(vertices)],
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=linear,
        options={"time_limit": 120, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    if not result.success or (gap is not None and gap > 1e-9):
        raise ValueError("MWIS failed")
    return int(round(-result.fun))


def build_rows(edges, triads, rank_rows, weights):
    rows = []
    rhs = []
    for a, b in edges:
        rows.append(("edge", f"edge_{a + 1}_{b + 1}", (a, b), None))
        rhs.append(1)
    for i, triad in enumerate(triads):
        rows.append(("triangle", f"triangle_{i}", triad, None))
        rhs.append(1)
    for name, vertices, alpha, family in rank_rows:
        rows.append((family, name, vertices, weights[list(vertices)]))
        rhs.append(alpha)
    return rows, np.array(rhs, dtype=float)


def matrix_for(rows, active, rhs, weights):
    index = {v: i for i, v in enumerate(active)}
    matrix = lil_matrix((len(rows), len(active)), dtype=float)
    for r, (kind, _name, vertices, coeffs) in enumerate(rows):
        if kind in ("root_rank", "child_rank"):
            for v, coeff in zip(vertices, coeffs):
                if v in index:
                    matrix[r, index[v]] = coeff
        else:
            for v in vertices:
                if v in index:
                    matrix[r, index[v]] = 1
    return matrix.tocsr()


def solve_lp(rows, rhs, weights, active):
    matrix = matrix_for(rows, active, rhs, weights)
    c = -weights[active].astype(float)
    result = linprog(c=c, A_ub=matrix, b_ub=rhs, bounds=[(0, 1)] * len(active), method="highs")
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result, matrix


def child_candidate_rows(weights, adj, x_full):
    rank = weights * x_full
    rows = [top_by_rank("top_wx_120", rank, 120)]
    rows.append(("dense120_303", dense(302, 120, weights, adj, rank)))
    out = []
    for name, vertices in rows:
        alpha = solve_mwis(vertices, weights, adj)
        out.append((name, vertices, alpha, "child_rank"))
    return out


def rational_attempts(rows, rhs, multipliers, matrix, active, weights, denominators):
    positive = [(i, value) for i, value in enumerate(multipliers) if value > POSITIVE_TOLERANCE]
    attempts = []
    for den in denominators:
        coverage = np.zeros(len(active), dtype=object)
        objective_num = 0
        for row_index, value in positive:
            num = int(math.ceil(value * den - 1e-9))
            objective_num += num * int(rhs[row_index])
            row = matrix.getrow(row_index)
            for _, col in zip(*row.nonzero()):
                coverage[col] += num * int(row[0, col])
        repair = 0
        repaired = 0
        for col, vertex in enumerate(active):
            deficit = int(weights[vertex]) * den - int(coverage[col])
            if deficit > 0:
                repair += deficit
                repaired += 1
        attempts.append({
            "denominator": den,
            "objective": (objective_num + repair) / den,
            "row_count": len(positive),
            "repaired_vertices": repaired,
            "repair_objective": repair / den,
            "passes_1024_bar": den == 1024 and (objective_num + repair) / den <= 633500 and repaired == 0,
            "passes_10000_bar": den == 10000 and (objective_num + repair) / den <= 633000 and repaired == 0,
            "passes_1e6_bar": den == 1000000 and (objective_num + repair) / den <= 632500 and repaired == 0,
        })
    return attempts


def certificate_artifact(rows, rhs, multipliers, denominator):
    cert_rows = []
    objective_num = 0
    for index, value in enumerate(multipliers):
        if value <= POSITIVE_TOLERANCE:
            continue
        numerator = int(math.ceil(value * denominator - 1e-9))
        kind, name, vertices, _coeffs = rows[index]
        objective_num += numerator * int(rhs[index])
        if kind == "triangle":
            cert_rows.append({
                "kind": "parent_triangle",
                "vertices": [int(v) + 1 for v in vertices],
                "numerator": numerator,
            })
        elif kind == "child_rank":
            cert_rows.append({
                "kind": "child_weighted_rank",
                "pocket": name,
                "alpha_w": int(rhs[index]),
                "support_vertices": [int(v) + 1 for v in vertices],
                "numerator": numerator,
            })
        elif kind == "root_rank":
            cert_rows.append({
                "kind": "root_weighted_rank",
                "pocket": name,
                "alpha_w": int(rhs[index]),
                "support_vertices": [int(v) + 1 for v in vertices],
                "numerator": numerator,
            })
    return {
        "schema": "forge.hadwiger.w607_v304_exclude_dual_cover.v1",
        "graph_digest": "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad",
        "excluded_vertex": EXCLUDED + 1,
        "denominator": denominator,
        "objective_numerator": objective_num,
        "objective_bound_decimal": objective_num / denominator,
        "row_count": len(cert_rows),
        "rows": cert_rows,
        "generator": "run_w607_v304_exclude_dual_diagnostic.py",
    }


def summarize(rows, rhs, multipliers):
    counts = {}
    contrib = {}
    top = []
    for i, value in enumerate(multipliers):
        if value <= POSITIVE_TOLERANCE:
            continue
        kind, name, vertices, _coeffs = rows[i]
        counts[kind] = counts.get(kind, 0) + 1
        contrib[kind] = contrib.get(kind, 0.0) + value * rhs[i]
        top.append({
            "kind": kind,
            "name": name,
            "multiplier": value,
            "rhs": rhs[i],
            "objective_contribution": value * rhs[i],
            "size": len([v for v in vertices if v != EXCLUDED]),
        })
    top.sort(key=lambda row: row["objective_contribution"], reverse=True)
    return counts, contrib, top[:50]


def coverage_slack(matrix, multipliers, active, weights):
    coverage = matrix.transpose().dot(multipliers)
    slack = coverage - weights[active]
    return {
        "min": float(np.min(slack)),
        "median": float(np.median(slack)),
        "max": float(np.max(slack)),
        "near_tight_vertices": int(np.sum(np.abs(slack) <= 1e-5)),
        "negative_slack_vertices": int(np.sum(slack < -1e-5)),
    }


def clean(value):
    if isinstance(value, dict):
        return {k: clean(v) for k, v in value.items()}
    if isinstance(value, list):
        return [clean(v) for v in value]
    if isinstance(value, tuple):
        return [clean(v) for v in value]
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
    active = [v for v in range(N) if v != EXCLUDED]
    root_rank = [(name, pocket(name, weights, adj), alpha, "root_rank") for name, alpha in ACCEPTED]
    base_rows, base_rhs = build_rows(edges, triads, root_rank, weights)
    base_obj, base_result, _ = solve_lp(base_rows, base_rhs, weights, active)
    x_full = np.zeros(N)
    x_full[active] = base_result.x
    child_rank = child_candidate_rows(weights, adj, x_full)
    rows, rhs = build_rows(edges, triads, root_rank + child_rank, weights)
    objective, result, matrix = solve_lp(rows, rhs, weights, active)
    explicit_bounds = [(0, 1)] * N
    explicit_bounds[EXCLUDED] = (0, 0)
    explicit_matrix = matrix_for(rows, list(range(N)), rhs, weights)
    explicit = linprog(
        c=-weights.astype(float),
        A_ub=explicit_matrix,
        b_ub=rhs,
        bounds=explicit_bounds,
        method="highs",
    )
    if not explicit.success:
        raise ValueError(explicit.message)
    multipliers = np.maximum(-result.ineqlin.marginals, 0.0)
    counts, contrib, top = summarize(rows, rhs, multipliers)
    attempts = rational_attempts(rows, rhs, multipliers, matrix, active, weights, [1024, 10000, 1000000])
    total = float(np.dot(multipliers, rhs))
    child_mass = contrib.get("child_rank", 0.0)
    status = "RetireV304ExcludeDualCertificate"
    if (
        sum(counts.values()) <= MAX_ROWS
        and child_mass / total >= MIN_CHILD_RANK_MASS_FRACTION
        and any(a["passes_1024_bar"] or a["passes_10000_bar"] or a["passes_1e6_bar"] for a in attempts)
    ):
        status = "FundV304ExcludeDualCertificate"
    report = clean({
        "schema": "forge.hadwiger.w607_v304_exclude_dual_diagnostic.v1",
        "excluded_vertex": EXCLUDED + 1,
        "base_exclude_objective": base_obj,
        "final_exclude_objective": objective,
        "known_final_exclude_objective": KNOWN_EXCLUDE_BOUND,
        "explicit_fixed_objective": -float(explicit.fun),
        "deleted_vs_explicit_delta": abs(objective + float(explicit.fun)),
        "positive_dual_row_count": sum(counts.values()),
        "positive_dual_count_by_kind": counts,
        "objective_contribution_by_kind": contrib,
        "child_rank_mass_fraction": child_mass / total if total else 0,
        "coverage_slack": coverage_slack(matrix, multipliers, active, weights),
        "rationalization_attempts": attempts,
        "child_rank_rows": [
            {"name": name, "size": len(vertices), "alpha_w": alpha}
            for name, vertices, alpha, _family in child_rank
        ],
        "top_dual_rows": top,
        "status": status,
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    CERT_OUT_PATH.write_text(
        json.dumps(clean(certificate_artifact(rows, rhs, multipliers, 1024)), separators=(",", ":"))
        + "\n"
    )
    summary = {k: v for k, v in report.items() if k != "top_dual_rows"}
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
