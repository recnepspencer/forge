import json
import math
import re
import time

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_odd_cycle_diagnostic as odd_cycle
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
VERTICES_PATH = CRATE / "src" / "frontier_seeds" / "g27_finite_fractional" / "W_circles_607_vertices.sage"
OUT_PATH = CRATE / "docs" / "w607-web-template-diagnostic.json"

EXPECTED_BASE = 594914.351525072
MAX_POOLS = 20
POOL_SIZE = 60
EARLY_POOL_KILL = 8
MAX_CANDIDATES = 500
MAX_ACCEPTED = 20
K_MIN = 3
K_MAX = 8
N_MIN = 9
N_MAX = 60
VIOLATION_GATE = 0.05
SINGLE_DROP_GATE = 1000.0
BATCH_DROP_GATE = 3000.0
KILL_DROP = 250.0


def parse_coordinates():
    text = VERTICES_PATH.read_text()
    body = text.split("=", 1)[1].strip()
    data = eval(body, {"__builtins__": {}}, {})

    def approx(coeffs):
        return (
            coeffs[0]
            + coeffs[1] * math.sqrt(3.0)
            + coeffs[2] * math.sqrt(11.0)
            + coeffs[3] * math.sqrt(33.0)
        )

    return np.array([(approx(x), approx(y)) for x, y in data], dtype=float)


def adjacency_sets(edges):
    adj = [set() for _ in range(parent.N)]
    edge_set = set()
    for a, b in edges:
        adj[a].add(b)
        adj[b].add(a)
        edge_set.add((min(a, b), max(a, b)))
    return adj, edge_set


def heavy_plateau(weights, x):
    return [v for v in range(parent.N) if weights[v] >= 10000 and abs(x[v] - (1.0 / 3.0)) <= 1e-6]


def tight_triangle_pressure(triads, x):
    pressure = np.zeros(parent.N)
    for tri in triads:
        if abs(sum(x[v] for v in tri) - 1.0) <= 1e-8:
            for v in tri:
                pressure[v] += 1.0
    return pressure


def twohop_pool(seed, adj):
    seen = {seed, *adj[seed]}
    for v in list(seen):
        seen.update(adj[v])
    return seen


def angular_order(vertices, coords):
    center = np.mean(coords[list(vertices)], axis=0)
    return sorted(vertices, key=lambda v: (math.atan2(coords[v][1] - center[1], coords[v][0] - center[0]), v))


def build_pools(weights, x, adj, triads, coords):
    plateau = heavy_plateau(weights, x)
    pressure = tight_triangle_pressure(triads, x)
    extras = [
        int(v)
        for v in np.lexsort((np.arange(parent.N), -(weights * x + pressure * 1000.0)))[:20]
        if v not in plateau
    ][:5]
    seeds = list(dict.fromkeys([*plateau, *extras]))[:MAX_POOLS]
    pools = []
    rank = weights * x + pressure * 1000.0
    for seed in seeds:
        raw = twohop_pool(seed, adj)
        chosen = sorted(raw, key=lambda v: (-rank[v], v))[:POOL_SIZE]
        pools.append((f"seed_{seed+1}", angular_order(chosen, coords)))
    return pools


def required_web_edges(order, k):
    n = len(order)
    for i in range(n):
        for d in range(1, k):
            yield order[i], order[(i + d) % n]


def certifies_web(order, k, edge_set):
    for a, b in required_web_edges(order, k):
        if (min(a, b), max(a, b)) not in edge_set:
            return False
    return True


def exact_alpha(vertices, adj):
    local = {v: i for i, v in enumerate(vertices)}
    rows = []
    for i, a in enumerate(vertices):
        for b in vertices[i + 1 :]:
            if b in adj[a]:
                row = np.zeros(len(vertices))
                row[i] = 1.0
                row[local[b]] = 1.0
                rows.append(row)
    constraints = LinearConstraint(np.vstack(rows), -np.inf, np.ones(len(rows))) if rows else None
    result = milp(
        c=-np.ones(len(vertices)),
        integrality=np.ones(len(vertices)),
        bounds=Bounds(np.zeros(len(vertices)), np.ones(len(vertices))),
        constraints=constraints,
        options={"time_limit": 30.0, "mip_rel_gap": 0.0},
    )
    gap = getattr(result, "mip_gap", None)
    ok = bool(result.success and (gap is None or gap <= 1e-9))
    return int(round(-result.fun)) if ok else None


def candidate_windows(order):
    n = len(order)
    for length in range(N_MIN, min(N_MAX, n) + 1):
        step = max(1, length // 8)
        for start in range(0, n, step):
            yield order[start:] + order[:start] if length == n else (order + order)[start : start + length]


def find_web_cuts(pools, x, adj, edge_set):
    cuts = {}
    reports = []
    candidate_count = 0
    accepted_by_pool = 0
    for pool_index, (pool_name, order) in enumerate(pools):
        pool_checked = 0
        pool_structural = 0
        pool_accepted = 0
        for window in candidate_windows(order):
            if candidate_count >= MAX_CANDIDATES:
                break
            key_base = tuple(sorted(window))
            if len(key_base) != len(set(key_base)):
                continue
            for k in range(K_MIN, K_MAX + 1):
                if candidate_count >= MAX_CANDIDATES:
                    break
                candidate_count += 1
                pool_checked += 1
                rhs_template = len(window) // k
                if sum(x[v] for v in window) - rhs_template < VIOLATION_GATE:
                    continue
                if not certifies_web(window, k, edge_set):
                    continue
                pool_structural += 1
                alpha = exact_alpha(tuple(sorted(window)), adj)
                if alpha is None or alpha > rhs_template:
                    continue
                violation = float(sum(x[v] for v in window) - alpha)
                if violation < VIOLATION_GATE:
                    continue
                key = (key_base, alpha)
                cuts.setdefault(
                    key,
                    {
                        "pool": pool_name,
                        "support": list(key_base),
                        "order": [v + 1 for v in window],
                        "k": k,
                        "n": len(window),
                        "rhs_template": rhs_template,
                        "alpha": alpha,
                        "violation": violation,
                    },
                )
                pool_accepted += 1
                if len(cuts) >= MAX_ACCEPTED:
                    break
            if len(cuts) >= MAX_ACCEPTED or candidate_count >= MAX_CANDIDATES:
                break
        accepted_by_pool += pool_accepted
        reports.append(
            {
                "pool": pool_name,
                "checked": pool_checked,
                "structural_webs": pool_structural,
                "accepted": pool_accepted,
            }
        )
        if pool_index + 1 == EARLY_POOL_KILL and accepted_by_pool == 0:
            break
        if len(cuts) >= MAX_ACCEPTED or candidate_count >= MAX_CANDIDATES:
            break
    return list(cuts.values()), reports, candidate_count


def solve_with_cut(edges, triads, weights, rank_cuts, extra_rows, cut):
    row = {"cycle": cut["support"], "rhs": cut["alpha"]}
    return odd_cycle.solve_lp(edges, triads, weights, rank_cuts, extra_rows, [row])


def clean(value):
    if isinstance(value, dict):
        return {str(k): clean(v) for k, v in value.items()}
    if isinstance(value, list):
        return [clean(v) for v in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def main():
    start = time.time()
    edges, weights = parent.parse_edges_weights()
    weights_float = weights.astype(float)
    adj, edge_set = adjacency_sets(edges)
    adj_sets = parent.adjacency(edges)
    triads = parent.triangles(adj_sets)
    coords = parse_coordinates()
    rank_cuts = parent_lift.root_cuts(weights_float, adj_sets)
    extra_rows = [parent_lift.parent_row(weights_float), branch_slack.p_parent_row(weights)]
    base_obj, x = odd_cycle.solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, solution=True)
    pools = build_pools(weights_float, x, adj, triads, coords)
    cuts, pool_reports, candidate_count = find_web_cuts(pools, x, adj, edge_set)
    cuts = sorted(cuts, key=lambda cut: (-cut["violation"], cut["n"], cut["k"]))
    tested = []
    accepted = []
    for cut in cuts[:MAX_ACCEPTED]:
        obj = solve_with_cut(edges, triads, weights_float, rank_cuts, extra_rows, cut)
        row = dict(cut)
        row["single_drop"] = base_obj - obj
        tested.append(row)
        if row["single_drop"] >= 250.0:
            accepted.append(cut)
    final_obj = base_obj
    if accepted:
        odd_rows = [{"cycle": cut["support"], "rhs": cut["alpha"]} for cut in accepted[:MAX_ACCEPTED]]
        final_obj = odd_cycle.solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, odd_rows)
    total_drop = base_obj - final_obj
    best_single = max((row["single_drop"] for row in tested), default=0.0)
    status = "RetireWebTemplateDiagnostic"
    if tested and tested[0]["violation"] >= VIOLATION_GATE and best_single >= SINGLE_DROP_GATE:
        status = "FundWebTemplateFollowup"
    if total_drop >= BATCH_DROP_GATE:
        status = "FundWebTemplateReplay"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_web_template_diagnostic.v1",
            "authority": "diagnostic_only_structural_web_template_plus_exact_alpha",
            "second_agent_verdict": "web_only_tiny_template_falsifier_clique_family_skipped",
            "base_objective": base_obj,
            "baseline_reproduced": abs(base_obj - EXPECTED_BASE) <= 1e-5,
            "pool_count": len(pools),
            "pool_reports": pool_reports,
            "candidate_count": candidate_count,
            "generated_cut_count": len(cuts),
            "tested_cut_count": len(tested),
            "accepted_cut_count": len(accepted),
            "best_single_drop": best_single,
            "final_objective": final_obj,
            "total_drop": total_drop,
            "status": status,
            "gates": {
                "max_pools": MAX_POOLS,
                "early_pool_kill": EARLY_POOL_KILL,
                "pool_size": POOL_SIZE,
                "max_candidates": MAX_CANDIDATES,
                "max_accepted": MAX_ACCEPTED,
                "k_range": [K_MIN, K_MAX],
                "n_range": [N_MIN, N_MAX],
                "violation_gate": VIOLATION_GATE,
                "single_drop_gate": SINGLE_DROP_GATE,
                "batch_drop_gate": BATCH_DROP_GATE,
                "kill_drop": KILL_DROP,
            },
            "top_tested": sorted(tested, key=lambda row: (row["single_drop"], row["violation"]), reverse=True)[:20],
            "seconds": time.time() - start,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in ("top_tested", "pool_reports")}, indent=2))


if __name__ == "__main__":
    main()
