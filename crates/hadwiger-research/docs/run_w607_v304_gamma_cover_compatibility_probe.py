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
EXCLUDE_CERT = CRATE / "docs" / "w607-v304-exclude-dual-cover-den1024.json"
PARENT_LIFT = CRATE / "docs" / "w607-v304-projected-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-v304-gamma-cover-compatibility-probe.json"

N = 607
BRANCH = 303
DENOMINATOR = 1024
RETIRE_REL_GAP = 0.005


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


def triangles(adj, active):
    active_set = set(active)
    rows = []
    for a in active:
        for b in adj[a] & active_set:
            if b <= a:
                continue
            for c in adj[a] & adj[b] & active_set:
                if c > b:
                    rows.append((a, b, c))
    return rows


def exclude_coverage(original_weights):
    cert = json.loads(EXCLUDE_CERT.read_text())
    coverage = np.zeros(N, dtype=object)
    for row in cert["rows"]:
        numerator = int(row["numerator"])
        if row["kind"] == "parent_triangle":
            for vertex in row["vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator
        elif row["kind"] == "child_weighted_rank":
            for vertex in row["support_vertices"]:
                if vertex != BRANCH + 1:
                    coverage[vertex - 1] += numerator * int(original_weights[vertex - 1])
    return np.array([int(v) for v in coverage], dtype=float)


def solve_cover_lp(active, row_sets, rhs, weights):
    index = {v: i for i, v in enumerate(active)}
    matrix = lil_matrix((len(active), len(row_sets)), dtype=float)
    for col, vertices in enumerate(row_sets):
        for vertex in vertices:
            if vertex in index:
                matrix[index[vertex], col] = 1.0
    result = linprog(
        c=np.array(rhs, dtype=float),
        A_ub=-matrix.tocsr(),
        b_ub=-weights[active],
        bounds=[(0, None)] * len(row_sets),
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    positive = int(np.sum(result.x > 1e-8))
    return float(result.fun), positive


def domain_report(name, active, target, adj, weights):
    triads = triangles(adj, active)
    edge_rows = [(a, b) for a in active for b in adj[a] if b > a and b in set(active)]
    edge_obj, edge_positive = solve_cover_lp(active, edge_rows, [1] * len(edge_rows), weights)
    tri_rows = edge_rows + triads
    tri_rhs = [1] * len(tri_rows)
    tri_obj, tri_positive = solve_cover_lp(active, tri_rows, tri_rhs, weights)
    best_obj = min(edge_obj, tri_obj)
    best_kind = "edge_triangle" if tri_obj <= edge_obj else "edge"
    rel_gap = (best_obj - target) / target
    return {
        "domain": name,
        "active_vertices": len(active),
        "target_gamma": target,
        "edge_rows": len(edge_rows),
        "triangle_rows": len(triads),
        "edge_cover_lp_objective": edge_obj,
        "edge_positive_rows": edge_positive,
        "edge_triangle_cover_lp_objective": tri_obj,
        "edge_triangle_positive_rows": tri_positive,
        "best_flat_cover_objective": best_obj,
        "best_flat_cover_kind": best_kind,
        "relative_gap_to_target": rel_gap,
        "passes_compatibility_gate": rel_gap <= RETIRE_REL_GAP,
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
    return value


def main():
    edges, original_weights = parse_edges_weights()
    adj = adjacency(edges)
    c0 = exclude_coverage(original_weights)
    lift = json.loads(PARENT_LIFT.read_text())
    gamma0 = int(lift["gamma0_upper_numerator"])
    gamma1 = int(lift["gamma1_upper_numerator"])
    exclude_active = [v for v in range(N) if v != BRANCH and c0[v] > 0]
    include_active = [v for v in range(N) if v != BRANCH and v not in adj[BRANCH] and c0[v] > 0]
    reports = [
        domain_report("x304_exclude", exclude_active, gamma0, adj, c0),
        domain_report("x304_include_residual", include_active, gamma1, adj, c0),
    ]
    status = "RetireFlatCoverGammaHardening"
    if all(report["passes_compatibility_gate"] for report in reports):
        status = "FundFlatCoverGammaHardening"
    elif any(report["passes_compatibility_gate"] for report in reports):
        status = "PartialFlatCoverGammaCompatibility"
    out = clean({
        "schema": "forge.hadwiger.w607_v304_gamma_cover_compatibility_probe.v1",
        "branch_vertex": BRANCH + 1,
        "retire_relative_gap": RETIRE_REL_GAP,
        "reports": reports,
        "status": status,
    })
    OUT_PATH.write_text(json.dumps(out, indent=2) + "\n")
    print(json.dumps(out, indent=2))


if __name__ == "__main__":
    main()
