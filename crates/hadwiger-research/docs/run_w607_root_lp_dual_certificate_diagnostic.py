import json
import math
import re
from fractions import Fraction
from pathlib import Path

import numpy as np
from scipy.optimize import linprog
from scipy.sparse import lil_matrix


ROOT = Path(__file__).resolve().parents[3]
CRATE = ROOT / "crates" / "hadwiger-research"
DATA = CRATE / "src" / "frontier_seeds" / "g27_finite_fractional"
EDGES_PATH = DATA / "W_circles_607_integers.dat"
OUT_PATH = CRATE / "docs" / "w607-root-lp-dual-certificate-diagnostic.json"
CERT_OUT_PATH = CRATE / "docs" / "w607-root-rank-triangle-dual-cover-den1024.json"

N = 607
KNOWN_OBJECTIVE = 641090.9615275887
LP_TOLERANCE = 1e-4
POSITIVE_TOLERANCE = 1e-8
MAX_ROWS_TO_FUND = 5000
MAX_CERT_OBJECTIVE = 641500.0

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
        return dense_expand(center, int(kind.removeprefix("dense")), weights, adj)
    raise ValueError(name)


def dense_expand(seed, limit, weights, adj):
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


def build_rows(edges, triads, rank_cuts, weights):
    rows = []
    rhs = []
    for i, (a, b) in enumerate(edges):
        rows.append(("edge", f"edge_{a + 1}_{b + 1}", (a, b), None))
        rhs.append(1)
    for i, triad in enumerate(triads):
        rows.append(("triangle", f"triangle_{i}", triad, None))
        rhs.append(1)
    for name, vertices, alpha in rank_cuts:
        rows.append(("rank", name, vertices, weights[list(vertices)]))
        rhs.append(alpha)
    matrix = lil_matrix((len(rows), N), dtype=float)
    for r, (kind, _name, vertices, coeffs) in enumerate(rows):
        if kind == "rank":
            for v, coeff in zip(vertices, coeffs):
                matrix[r, v] = coeff
        else:
            for v in vertices:
                matrix[r, v] = 1
    return rows, np.array(rhs, dtype=float), matrix.tocsr()


def solve_root_lp(matrix, rhs, weights):
    result = linprog(
        c=-weights.astype(float),
        A_ub=matrix,
        b_ub=rhs,
        bounds=[(0, 1)] * N,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    return -float(result.fun), result


def row_objective_by_kind(rows, rhs, multipliers, upper):
    totals = {"edge": 0.0, "triangle": 0.0, "rank": 0.0, "singleton_bound": float(np.sum(upper))}
    counts = {"edge": 0, "triangle": 0, "rank": 0, "singleton_bound": int(np.sum(upper > POSITIVE_TOLERANCE))}
    for index, value in enumerate(multipliers):
        if value > POSITIVE_TOLERANCE:
            kind = rows[index][0]
            totals[kind] += float(value * rhs[index])
            counts[kind] += 1
    return counts, totals


def rational_replay(rows, rhs, multipliers, upper, weights, denominators):
    attempts = []
    positive = [(i, value) for i, value in enumerate(multipliers) if value > POSITIVE_TOLERANCE]
    positive_upper = [(i, value) for i, value in enumerate(upper) if value > POSITIVE_TOLERANCE]
    for den in denominators:
        coverage = np.zeros(N, dtype=object)
        objective_num = 0
        positive_nums = []
        for row_index, value in positive:
            num = int(math.ceil(value * den - 1e-9))
            positive_nums.append(num)
            kind, _name, vertices, coeffs = rows[row_index]
            objective_num += num * int(rhs[row_index])
            if kind == "rank":
                for v, coeff in zip(vertices, coeffs):
                    coverage[v] += num * int(coeff)
            else:
                for v in vertices:
                    coverage[v] += num
        for vertex, value in positive_upper:
            num = int(math.ceil(value * den - 1e-9))
            objective_num += num
            coverage[vertex] += num
        repair = 0
        repaired_vertices = 0
        for vertex, weight in enumerate(weights):
            deficit = int(weight) * den - int(coverage[vertex])
            if deficit > 0:
                repair += deficit
                repaired_vertices += 1
        objective = (objective_num + repair) / den
        attempts.append({
            "denominator": den,
            "objective": objective,
            "row_count": len(positive) + len(positive_upper),
            "repaired_vertices": repaired_vertices,
            "repair_objective": repair / den,
            "max_multiplier_numerator": max(positive_nums, default=0),
            "passes": objective <= MAX_CERT_OBJECTIVE and repaired_vertices == 0,
        })
    return attempts


def certificate_artifact(rows, rhs, multipliers, denominator):
    cert_rows = []
    objective_num = 0
    for index, value in enumerate(multipliers):
        if value <= POSITIVE_TOLERANCE:
            continue
        numerator = int(math.ceil(value * denominator - 1e-9))
        kind, name, vertices, coeffs = rows[index]
        objective_num += numerator * int(rhs[index])
        if kind == "triangle":
            cert_rows.append({
                "kind": "triangle",
                "vertices": [int(v) + 1 for v in vertices],
                "numerator": numerator,
            })
        elif kind == "rank":
            cert_rows.append({
                "kind": "weighted_rank",
                "pocket": name,
                "alpha_w": int(rhs[index]),
                "support_vertices": [int(v) + 1 for v in vertices],
                "numerator": numerator,
            })
    return {
        "schema": "forge.hadwiger.w607_rank_triangle_dual_cover.v1",
        "graph_digest": "sha256:be181cad41b7156208a583235ab6937c51eb2292b7bed952bb98f68e0b1b4dad",
        "weight_sum": 1999983,
        "denominator": denominator,
        "objective_numerator": objective_num,
        "objective_bound_decimal": objective_num / denominator,
        "row_count": len(cert_rows),
        "rows": cert_rows,
        "generator": "run_w607_root_lp_dual_certificate_diagnostic.py",
    }


def slack_stats(matrix, multipliers, upper, weights):
    coverage = matrix.transpose().dot(multipliers) + upper
    slack = coverage - weights
    return {
        "min": float(np.min(slack)),
        "median": float(np.median(slack)),
        "max": float(np.max(slack)),
        "near_tight_vertices": int(np.sum(np.abs(slack) <= 1e-5)),
        "negative_slack_vertices": int(np.sum(slack < -1e-5)),
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
    if isinstance(value, Fraction):
        return str(value)
    return value


def main():
    edges, weights = parse_edges_weights()
    adj = adjacency(edges)
    triads = triangles(adj)
    rank_cuts = [(name, pocket(name, weights, adj), alpha) for name, alpha in ACCEPTED]
    rows, rhs, matrix = build_rows(edges, triads, rank_cuts, weights)
    objective, result = solve_root_lp(matrix, rhs, weights)
    if abs(objective - KNOWN_OBJECTIVE) > LP_TOLERANCE:
        raise ValueError(f"root objective mismatch {objective}")
    multipliers = np.maximum(-result.ineqlin.marginals, 0.0)
    upper = np.maximum(-result.upper.marginals, 0.0)
    counts, totals = row_objective_by_kind(rows, rhs, multipliers, upper)
    positive_count = sum(counts.values())
    tiny_mass = 0.0
    total_mass = float(np.dot(multipliers, rhs) + np.sum(upper))
    for value, row_rhs in zip(multipliers, rhs):
        contribution = value * row_rhs
        if POSITIVE_TOLERANCE < contribution < 1e-4:
            tiny_mass += contribution
    top_rows = []
    for index, value in enumerate(multipliers):
        if value > POSITIVE_TOLERANCE:
            kind, name, vertices, _coeffs = rows[index]
            top_rows.append({
                "kind": kind,
                "name": name,
                "multiplier": value,
                "rhs": rhs[index],
                "objective_contribution": value * rhs[index],
                "size": len(vertices),
            })
    for vertex, value in enumerate(upper):
        if value > POSITIVE_TOLERANCE:
            top_rows.append({
                "kind": "singleton_bound",
                "name": f"upper_{vertex + 1}",
                "multiplier": value,
                "rhs": 1,
                "objective_contribution": value,
                "size": 1,
            })
    top_rows.sort(key=lambda row: row["objective_contribution"], reverse=True)
    rational = rational_replay(rows, rhs, multipliers, upper, weights, [1024, 10000, 1000000])
    status = "RetireRootLpDualRationalization"
    if positive_count <= MAX_ROWS_TO_FUND and any(attempt["passes"] for attempt in rational):
        status = "FundRootLpDualRationalization"
    report = clean({
        "schema": "forge.hadwiger.w607_root_lp_dual_certificate_diagnostic.v1",
        "root_objective": objective,
        "known_objective": KNOWN_OBJECTIVE,
        "positive_dual_row_count": positive_count,
        "positive_dual_count_by_kind": counts,
        "objective_contribution_by_kind": totals,
        "dual_objective_float": total_mass,
        "tiny_objective_mass_below_1e_minus_4": tiny_mass,
        "tiny_mass_fraction": tiny_mass / total_mass if total_mass else 0,
        "coverage_slack": slack_stats(matrix, multipliers, upper, weights),
        "rationalization_attempts": rational,
        "top_dual_rows": top_rows[:50],
        "status": status,
        "thresholds": {
            "max_rows_to_fund": MAX_ROWS_TO_FUND,
            "max_certificate_objective": MAX_CERT_OBJECTIVE,
            "positive_tolerance": POSITIVE_TOLERANCE,
        },
    })
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    CERT_OUT_PATH.write_text(
        json.dumps(clean(certificate_artifact(rows, rhs, multipliers, 1024)), separators=(",", ":"))
        + "\n"
    )
    summary = {key: report[key] for key in report if key != "top_dual_rows"}
    print(json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
