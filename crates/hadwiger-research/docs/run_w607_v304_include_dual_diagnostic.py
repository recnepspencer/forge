import json
import math
import re
import time
from pathlib import Path

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix


ROOT = Path(__file__).resolve().parents[3]
CRATE = ROOT / "crates" / "hadwiger-research"
DATA = CRATE / "src" / "frontier_seeds" / "g27_finite_fractional"
EDGES_PATH = DATA / "W_circles_607_integers.dat"
OUT_PATH = CRATE / "docs" / "w607-v304-include-dual-diagnostic.json"
CERT_OUT_PATH = CRATE / "docs" / "w607-v304-include-dual-cover-den1024.json"

N = 607
INCLUDED = 303
KNOWN_INCLUDE_BOUND = 604127.0
POSITIVE_TOLERANCE = 1e-8

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
    rows = []
    for a in range(N):
        for b in adj[a]:
            if b <= a:
                continue
            for c in adj[a] & adj[b]:
                if c > b:
                    rows.append((a, b, c))
    return rows


def build_rows(edges, triads, rank_rows, weights, active):
    active_set = set(active)
    rows = []
    rhs = []
    for a, b in edges:
        if a in active_set and b in active_set:
            rows.append(("edge", f"edge_{a + 1}_{b + 1}", (a, b), None))
            rhs.append(1)
    for i, triad in enumerate(triads):
        kept = tuple(v for v in triad if v in active_set)
        if len(kept) >= 2:
            rows.append(("triangle", f"triangle_{i}", kept, None))
            rhs.append(1)
    for name, vertices, alpha in rank_rows:
        fixed_weight = weights[INCLUDED] if INCLUDED in vertices else 0
        kept = tuple(v for v in vertices if v in active_set)
        if kept:
            rows.append(("root_rank", name, kept, weights[list(kept)]))
            rhs.append(alpha - fixed_weight)
    return rows, np.array(rhs, dtype=float)


def build_full_rows(edges, triads, rank_rows, weights):
    rows = []
    rhs = []
    for a, b in edges:
        rows.append(("edge", f"edge_{a + 1}_{b + 1}", (a, b), None))
        rhs.append(1)
    for i, triad in enumerate(triads):
        rows.append(("triangle", f"triangle_{i}", triad, None))
        rhs.append(1)
    for name, vertices, alpha in rank_rows:
        rows.append(("root_rank", name, vertices, weights[list(vertices)]))
        rhs.append(alpha)
    return rows, np.array(rhs, dtype=float)


def matrix_for(rows, active):
    index = {v: i for i, v in enumerate(active)}
    matrix = lil_matrix((len(rows), len(active)), dtype=float)
    for r, (kind, _name, vertices, coeffs) in enumerate(rows):
        if kind == "root_rank":
            for v, coeff in zip(vertices, coeffs):
                matrix[r, index[v]] = coeff
        else:
            for v in vertices:
                matrix[r, index[v]] = 1
    return matrix.tocsr()


def solve_lp(rows, rhs, weights, active):
    matrix = matrix_for(rows, active)
    result = linprog(
        c=-weights[active].astype(float),
        A_ub=matrix,
        b_ub=rhs,
        bounds=[(0, 1)] * len(active),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result, matrix


def solve_explicit(rows, rhs, weights):
    matrix = lil_matrix((len(rows), N), dtype=float)
    for r, (kind, _name, vertices, coeffs) in enumerate(rows):
        if kind == "root_rank":
            for v, coeff in zip(vertices, coeffs):
                matrix[r, v] = coeff
        else:
            for v in vertices:
                matrix[r, v] = 1
    bounds = [(0, 1)] * N
    bounds[INCLUDED] = (1, 1)
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix.tocsr(),
        b_ub=rhs,
        bounds=bounds,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun)


def summarize(rows, rhs, multipliers, constant):
    counts = {"included_vertex": 1}
    contrib = {"included_vertex": constant}
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
            "size": len(vertices),
        })
    top.sort(key=lambda row: row["objective_contribution"], reverse=True)
    return counts, contrib, top[:50]


def rational_attempts(rows, rhs, multipliers, matrix, active, weights, constant, denominators):
    positive = [(i, value) for i, value in enumerate(multipliers) if value > POSITIVE_TOLERANCE]
    attempts = []
    for den in denominators:
        coverage = np.zeros(len(active), dtype=object)
        objective_num = int(constant) * den
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
            "row_count": len(positive) + 1,
            "repaired_vertices": repaired,
            "repair_objective": repair / den,
            "passes_1024_bar": den == 1024 and (objective_num + repair) / den <= 604500 and repaired == 0,
            "passes_10000_bar": den == 10000 and (objective_num + repair) / den <= 604200 and repaired == 0,
        })
    return attempts


def certificate(rows, rhs, multipliers, denominator, constant):
    out = [{
        "kind": "included_vertex",
        "vertex": INCLUDED + 1,
        "weight": int(constant),
        "numerator": denominator,
    }]
    objective_num = int(constant) * denominator
    for i, value in enumerate(multipliers):
        if value <= POSITIVE_TOLERANCE:
            continue
        num = int(math.ceil(value * denominator - 1e-9))
        kind, name, vertices, _coeffs = rows[i]
        objective_num += num * int(rhs[i])
        out.append({
            "kind": kind,
            "name": name,
            "vertices": [int(v) + 1 for v in vertices],
            "rhs": int(rhs[i]),
            "numerator": num,
        })
    return {
        "schema": "forge.hadwiger.w607_v304_include_dual_cover.v1",
        "graph_digest": "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad",
        "included_vertex": INCLUDED + 1,
        "denominator": denominator,
        "objective_numerator": objective_num,
        "objective_bound_decimal": objective_num / denominator,
        "row_count": len(out),
        "rows": out,
        "generator": "run_w607_v304_include_dual_diagnostic.py",
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
    closed = {INCLUDED, *adj[INCLUDED]}
    active = [v for v in range(N) if v not in closed]
    rank_rows = [(name, pocket(name, weights, adj), alpha) for name, alpha in ACCEPTED]
    rows, rhs = build_rows(edges, triangles(adj), rank_rows, weights, active)
    residual, result, matrix = solve_lp(rows, rhs, weights, active)
    objective = residual + weights[INCLUDED]
    full_rows, full_rhs = build_full_rows(edges, triangles(adj), rank_rows, weights)
    explicit = solve_explicit(full_rows, full_rhs, weights)
    multipliers = np.maximum(-result.ineqlin.marginals, 0.0)
    counts, contrib, top = summarize(rows, rhs, multipliers, int(weights[INCLUDED]))
    attempts = rational_attempts(rows, rhs, multipliers, matrix, active, weights, int(weights[INCLUDED]), [1024, 10000])
    status = "RetireV304IncludeDualCertificate"
    if any(a["passes_1024_bar"] or a["passes_10000_bar"] for a in attempts):
        status = "FundV304IncludeDualCertificate"
    report = clean({
        "schema": "forge.hadwiger.w607_v304_include_dual_diagnostic.v1",
        "included_vertex": INCLUDED + 1,
        "included_weight": int(weights[INCLUDED]),
        "active_vertex_count": len(active),
        "residual_objective": residual,
        "include_objective": objective,
        "known_include_objective": KNOWN_INCLUDE_BOUND,
        "explicit_fixed_objective": explicit,
        "residual_vs_explicit_delta": abs(objective - explicit),
        "positive_dual_count_by_kind": counts,
        "objective_contribution_by_kind": contrib,
        "rationalization_attempts": attempts,
        "top_dual_rows": top,
        "status": status,
        "seconds": time.time() - start,
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    CERT_OUT_PATH.write_text(json.dumps(clean(certificate(rows, rhs, multipliers, 1024, int(weights[INCLUDED]))), separators=(",", ":")) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "top_dual_rows"}, indent=2))


if __name__ == "__main__":
    main()
