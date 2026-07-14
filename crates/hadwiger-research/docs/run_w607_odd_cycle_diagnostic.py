import heapq
import json
import time

import numpy as np
from scipy.sparse import lil_matrix

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
OUT_PATH = CRATE / "docs" / "w607-odd-cycle-diagnostic.json"

EXPECTED_BASE = 594914.351525072
ROUND_LIMIT = 6
BATCH_LIMIT = 64
TOTAL_CUT_LIMIT = 256
MIN_LENGTH = 5
MAX_LENGTH = 101
MIN_ACCEPT_VIOLATION = 0.01
ROUND1_KILL_VIOLATION = 0.02
FUND_BEST_VIOLATION = 0.05
KILL_TOTAL_DROP = 250.0
FUND_TOTAL_DROP = 1000.0
SERIOUS_TOTAL_DROP = 3000.0
SERIOUS_SINGLE_ROUND_DROP = 2000.0
PLATEAU_DROP_GATE = 4


def solve_lp(edges, triads, weights, rank_cuts, extra_rows, odd_cuts=(), solution=False):
    rows = len(edges) + len(triads) + len(rank_cuts) + len(extra_rows) + len(odd_cuts)
    matrix = lil_matrix((rows, parent.N), dtype=float)
    upper = np.ones(rows)
    row = 0
    for a, b in edges:
        matrix[row, a] = matrix[row, b] = 1.0
        row += 1
    for a, b, c in triads:
        matrix[row, a] = matrix[row, b] = matrix[row, c] = 1.0
        row += 1
    for vertices, alpha in rank_cuts:
        for vertex in vertices:
            matrix[row, vertex] = float(weights[vertex])
        upper[row] = float(alpha)
        row += 1
    for coeffs, rhs in extra_rows:
        for vertex, coeff in coeffs.items():
            matrix[row, vertex] = float(coeff)
        upper[row] = float(rhs)
        row += 1
    for cut in odd_cuts:
        for vertex in cut["cycle"]:
            matrix[row, vertex] = 1.0
        upper[row] = float(cut["rhs"])
        row += 1
    result = parent.linprog(
        c=-weights.astype(float),
        A_ub=matrix.tocsr(),
        b_ub=upper,
        bounds=[(0, 1)] * parent.N,
        method="highs",
    )
    if not result.success:
        raise ValueError(result.message)
    objective = -float(result.fun)
    return (objective, result.x) if solution else objective


def adjacency_lists(edges):
    out = [[] for _ in range(parent.N)]
    edge_set = set()
    for a, b in edges:
        out[a].append(b)
        out[b].append(a)
        edge_set.add((min(a, b), max(a, b)))
    return out, edge_set


def edge_length(x, a, b):
    return max(0.0, 1.0 - x[a] - x[b])


def same_edge(a, b, edge):
    return (a == edge[0] and b == edge[1]) or (a == edge[1] and b == edge[0])


def shortest_even_path(adj, x, start, target, excluded_edge):
    dist = [[float("inf"), float("inf")] for _ in range(parent.N)]
    prev = [[None, None] for _ in range(parent.N)]
    heap = []
    dist[start][0] = 0.0
    heapq.heappush(heap, (0.0, start, 0))
    while heap:
        cost, vertex, parity = heapq.heappop(heap)
        if cost > dist[vertex][parity] + 1e-12:
            continue
        if vertex == target and parity == 0:
            path = []
            current = (target, 0)
            while True:
                path.append(current[0])
                if current == (start, 0):
                    break
                current = prev[current[0]][current[1]]
                if current is None:
                    return None
            path.reverse()
            return path, cost
        for next_vertex in adj[vertex]:
            if same_edge(vertex, next_vertex, excluded_edge):
                continue
            next_parity = 1 - parity
            next_cost = cost + edge_length(x, vertex, next_vertex)
            if next_cost + 1e-12 < dist[next_vertex][next_parity]:
                dist[next_vertex][next_parity] = next_cost
                prev[next_vertex][next_parity] = (vertex, parity)
                heapq.heappush(heap, (next_cost, next_vertex, next_parity))
    return None


def is_simple_cycle(path, edge):
    cycle = path[:]
    if len(cycle) != len(set(cycle)):
        return False
    if len(cycle) < MIN_LENGTH or len(cycle) > MAX_LENGTH:
        return False
    return (len(cycle) % 2) == 1 and edge[0] in cycle and edge[1] in cycle


def chord_count(cycle, edge_set):
    present = set()
    for i, vertex in enumerate(cycle):
        other = cycle[(i + 1) % len(cycle)]
        present.add((min(vertex, other), max(vertex, other)))
    count = 0
    for i, left in enumerate(cycle):
        for right in cycle[i + 1 :]:
            key = (min(left, right), max(left, right))
            if key in edge_set and key not in present:
                count += 1
    return count


def cycle_cut(cycle, x, edge_set):
    key = tuple(sorted(cycle))
    rhs = len(cycle) // 2
    violation = float(sum(x[v] for v in cycle) - rhs)
    return {
        "cycle": list(key),
        "rhs": rhs,
        "length": len(cycle),
        "violation": violation,
        "chord_count": chord_count(cycle, edge_set),
    }


def separate_odd_cycles(edges, adj, edge_set, x, seen):
    cuts = {}
    for a, b in edges:
        found = shortest_even_path(adj, x, a, b, (a, b))
        if found is None:
            continue
        path, path_cost = found
        if not is_simple_cycle(path, (a, b)):
            continue
        total_length = path_cost + edge_length(x, a, b)
        actual_violation = (1.0 - total_length) / 2.0
        if actual_violation < MIN_ACCEPT_VIOLATION:
            continue
        cut = cycle_cut(path, x, edge_set)
        key = tuple(cut["cycle"])
        if key in seen:
            continue
        cut["separator_violation"] = actual_violation
        cuts[key] = cut
    return sorted(
        cuts.values(),
        key=lambda cut: (-cut["violation"], cut["length"], cut["chord_count"], cut["cycle"]),
    )


def heavy_third_count(weights, x):
    return sum(1 for v in range(parent.N) if weights[v] >= 10000 and abs(x[v] - (1.0 / 3.0)) <= 1e-6)


def near_half_summary(weights, x):
    vertices = [v for v in range(parent.N) if weights[v] >= 5000 and 0.45 <= x[v] <= 0.55]
    top = sorted(vertices, key=lambda v: (-weights[v] * x[v], v))[:20]
    return {"count": len(vertices), "top_vertices": [v + 1 for v in top]}


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
    start = time.time()
    edges, weights = parent.parse_edges_weights()
    weights_float = weights.astype(float)
    adj_sets = parent.adjacency(edges)
    triads = parent.triangles(adj_sets)
    adj, edge_set = adjacency_lists(edges)
    rank_cuts = parent_lift.root_cuts(weights_float, adj_sets)
    extra_rows = [parent_lift.parent_row(weights_float), branch_slack.p_parent_row(weights)]
    objective, x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, solution=True)
    base_objective = objective
    base_plateau = heavy_third_count(weights, x)
    base_near_half = near_half_summary(weights, x)
    odd_cuts = []
    seen = set()
    rounds = []
    best_violation = 0.0
    for round_index in range(ROUND_LIMIT):
        candidates = separate_odd_cycles(edges, adj, edge_set, x, seen)
        if round_index == 0 and (not candidates or candidates[0]["violation"] < ROUND1_KILL_VIOLATION):
            rounds.append({"round": round_index + 1, "candidate_count": len(candidates), "accepted_count": 0})
            break
        remaining = TOTAL_CUT_LIMIT - len(odd_cuts)
        accepted = candidates[: min(BATCH_LIMIT, remaining)]
        if not accepted:
            rounds.append({"round": round_index + 1, "candidate_count": 0, "accepted_count": 0})
            break
        prior = objective
        for cut in accepted:
            seen.add(tuple(cut["cycle"]))
            odd_cuts.append(cut)
            best_violation = max(best_violation, cut["violation"])
        objective, x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, odd_cuts, True)
        rounds.append(
            {
                "round": round_index + 1,
                "candidate_count": len(candidates),
                "accepted_count": len(accepted),
                "best_violation": accepted[0]["violation"],
                "best_separator_violation": accepted[0]["separator_violation"],
                "lengths": sorted({cut["length"] for cut in accepted}),
                "objective": objective,
                "round_drop": prior - objective,
                "total_drop": base_objective - objective,
            }
        )
        if len(odd_cuts) >= TOTAL_CUT_LIMIT:
            break
    final_plateau = heavy_third_count(weights, x)
    total_drop = base_objective - objective
    best_round_drop = max((row.get("round_drop", 0.0) for row in rounds), default=0.0)
    plateau_drop = base_plateau - final_plateau
    status = "RetireOddCycleDiagnostic"
    if best_violation >= FUND_BEST_VIOLATION and total_drop >= FUND_TOTAL_DROP:
        status = "FundOddCycleFollowup"
    if total_drop >= SERIOUS_TOTAL_DROP or best_round_drop >= SERIOUS_SINGLE_ROUND_DROP or plateau_drop >= PLATEAU_DROP_GATE:
        status = "FundOddCycleReplay"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_odd_cycle_diagnostic.v1",
            "second_agent_verdict": "one_bounded_parity_surprise_falsification_pass",
            "base_objective": base_objective,
            "baseline_reproduced": abs(base_objective - EXPECTED_BASE) <= 1e-5,
            "final_objective": objective,
            "total_drop": total_drop,
            "cut_count": len(odd_cuts),
            "round_count": len([row for row in rounds if row.get("accepted_count", 0) > 0]),
            "best_violation": best_violation,
            "best_round_drop": best_round_drop,
            "base_heavy_third_plateau_count": base_plateau,
            "final_heavy_third_plateau_count": final_plateau,
            "heavy_third_plateau_drop": plateau_drop,
            "base_near_half": base_near_half,
            "final_near_half": near_half_summary(weights, x),
            "status": status,
            "gates": {
                "round_limit": ROUND_LIMIT,
                "batch_limit": BATCH_LIMIT,
                "total_cut_limit": TOTAL_CUT_LIMIT,
                "length_range": [MIN_LENGTH, MAX_LENGTH],
                "min_accept_violation": MIN_ACCEPT_VIOLATION,
                "round1_kill_violation": ROUND1_KILL_VIOLATION,
                "kill_total_drop": KILL_TOTAL_DROP,
                "fund_best_violation": FUND_BEST_VIOLATION,
                "fund_total_drop": FUND_TOTAL_DROP,
                "serious_total_drop": SERIOUS_TOTAL_DROP,
                "plateau_drop_gate": PLATEAU_DROP_GATE,
            },
            "rounds": rounds,
            "top_cuts": sorted(odd_cuts, key=lambda cut: (-cut["violation"], cut["length"]))[:30],
            "seconds": time.time() - start,
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in ("top_cuts", "rounds")}, indent=2))


if __name__ == "__main__":
    main()
