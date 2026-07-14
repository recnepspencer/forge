import hashlib
import json
import random

import numpy as np
from scipy.sparse import lil_matrix

import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-branch-slack-mod3-triangle-cg.json"

TIGHT_TOL = 1e-8
MAX_POOL_TRIANGLES = 120
MAX_GENERATED_CUTS = 3000
MAX_TESTED_CUTS = 200
MAX_ACCEPTED_CUTS = 50
MAX_TRIANGLES_PER_CUT = 120
MAX_VERTEX_SUPPORT = 240
MAX_COEFFICIENT = 8
SINGLE_FUND_DROP = 1000.0
SINGLE_KILL_DROP = 100.0
BATCH_INTERESTING_DROP = 3000.0
BATCH_FUND_DROP = 5000.0
BATCH_KILL_DROP = 1000.0
CONTINUATION_OBJECTIVE = 590000.0
RANDOM_SEED = 607


def p_parent_row(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(c0[v]) * parent.DENOMINATOR for v in range(parent.N) if c0[v]}
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        index = int(vertex) - 1
        coeffs[index] = coeffs.get(index, 0) + int(coeff)
    coeffs[parent.BRANCH] = int(artifact["lift_coefficient_num_d1024"])
    return coeffs, int(artifact["gamma0_modified_num_d1024"])


def solve_lp(edges, triads, weights, rank_cuts, extra_rows, cg_cuts=(), solution=False):
    row_count = len(edges) + len(triads) + len(rank_cuts) + len(extra_rows) + len(cg_cuts)
    matrix = lil_matrix((row_count, parent.N), dtype=float)
    upper = np.ones(row_count)
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
    for cut in cg_cuts:
        for vertex, coeff in cut["coefficients"].items():
            matrix[row, vertex] = float(coeff)
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


def tight_triangles(triads, x):
    return [tri for tri in triads if abs(sum(x[v] for v in tri) - 1.0) <= TIGHT_TOL]


def heavy_plateau(weights, x):
    return [
        v
        for v in range(parent.N)
        if weights[v] >= 10000 and abs(x[v] - (1.0 / 3.0)) <= 1e-6
    ]


def triangle_pools(tight, weights, x):
    plateau = heavy_plateau(weights, x)
    rank = weights.astype(float) * x
    top = [int(v) for v in np.lexsort((np.arange(parent.N), -rank))[:24]]
    seeds = []
    for seed in plateau + top:
        if seed not in seeds:
            seeds.append(seed)
    pools = []
    for seed in seeds[:24]:
        touching = [tri for tri in tight if seed in tri or any(v in plateau and v in tri for v in plateau)]
        touching = sorted(touching, key=lambda tri: (-sum(rank[v] for v in tri), tri))[:MAX_POOL_TRIANGLES]
        pools.append((f"seed_{seed+1}", touching))
    plateau_pool = [tri for tri in tight if any(v in plateau for v in tri)]
    pools.append(("plateau_all", sorted(plateau_pool, key=lambda tri: (-sum(rank[v] for v in tri), tri))[:MAX_POOL_TRIANGLES]))
    top_pool = sorted(tight, key=lambda tri: (-sum(rank[v] for v in tri), tri))[:MAX_POOL_TRIANGLES]
    pools.append(("top_tight", top_pool))
    return pools


def nullspace_mod3(matrix):
    matrix = [row[:] for row in matrix]
    row_count = len(matrix)
    col_count = len(matrix[0]) if matrix else 0
    pivots = []
    row = 0
    for col in range(col_count):
        pivot = next((r for r in range(row, row_count) if matrix[r][col] % 3), None)
        if pivot is None:
            continue
        matrix[row], matrix[pivot] = matrix[pivot], matrix[row]
        inv = 1 if matrix[row][col] == 1 else 2
        matrix[row] = [(value * inv) % 3 for value in matrix[row]]
        for r in range(row_count):
            if r != row and matrix[r][col] % 3:
                factor = matrix[r][col] % 3
                matrix[r] = [(matrix[r][c] - factor * matrix[row][c]) % 3 for c in range(col_count)]
        pivots.append(col)
        row += 1
        if row == row_count:
            break
    free = [col for col in range(col_count) if col not in pivots]
    basis = []
    for free_col in free:
        vector = [0] * col_count
        vector[free_col] = 1
        for pivot_row, pivot_col in enumerate(pivots):
            vector[pivot_col] = (-matrix[pivot_row][free_col]) % 3
        basis.append(vector)
    return basis


def pool_basis(pool):
    vertices = sorted({v for tri in pool for v in tri})
    index = {vertex: i for i, vertex in enumerate(vertices)}
    matrix = [[0] * len(pool) for _ in vertices]
    for col, tri in enumerate(pool):
        for vertex in tri:
            matrix[index[vertex]][col] = 1
    return nullspace_mod3(matrix)


def reject(counter, reason):
    counter[reason] = counter.get(reason, 0) + 1
    return None


def cut_from_vector(pool, vector, x, counter):
    support = [i for i, value in enumerate(vector) if value % 3]
    if not support or len(support) > MAX_TRIANGLES_PER_CUT:
        return reject(counter, "triangle_support")
    incidence = {}
    y_sum = 0
    for tri_index in support:
        y = vector[tri_index] % 3
        y_sum += y
        for vertex in pool[tri_index]:
            incidence[vertex] = incidence.get(vertex, 0) + y
    if y_sum % 3 == 0:
        return reject(counter, "rhs_integral")
    if any(value % 3 for value in incidence.values()):
        return reject(counter, "nonintegral_incidence")
    coefficients = {vertex: value // 3 for vertex, value in incidence.items() if value}
    if len(coefficients) > MAX_VERTEX_SUPPORT or max(coefficients.values(), default=0) > MAX_COEFFICIENT:
        return reject(counter, "dense_or_large_coeff")
    rhs = y_sum // 3
    lhs = sum(coeff * x[vertex] for vertex, coeff in coefficients.items())
    violation = lhs - rhs
    if violation <= 1e-7:
        return reject(counter, "not_violated")
    return {
        "coefficients": coefficients,
        "rhs": rhs,
        "triangle_indices": support,
        "multipliers": [vector[i] % 3 for i in support],
        "triangle_count": len(support),
        "coefficient_support": len(coefficients),
        "max_coefficient": max(coefficients.values(), default=0),
        "raw_violation": violation,
        "rhs_fraction_mod3": y_sum % 3,
    }


def cut_key(cut):
    items = tuple(sorted(cut["coefficients"].items()))
    return hashlib.sha256(repr((items, cut["rhs"])).encode()).hexdigest()


def generate_cuts(pools, x):
    rng = random.Random(RANDOM_SEED)
    cuts = {}
    pool_reports = []
    for pool_name, pool in pools:
        if len(pool) < 2:
            continue
        basis = pool_basis(pool)
        counter = {}
        nonzero_sum_basis = sum(1 for vector in basis if sum(vector) % 3)
        vectors = []
        vectors.extend(basis[:200])
        for i in range(min(len(basis), 80)):
            for j in range(i + 1, min(len(basis), i + 12)):
                vectors.append([(basis[i][c] + basis[j][c]) % 3 for c in range(len(pool))])
                vectors.append([(basis[i][c] + 2 * basis[j][c]) % 3 for c in range(len(pool))])
        for _ in range(200):
            if not basis:
                break
            chosen = rng.sample(basis, min(len(basis), rng.randint(2, 8)))
            vector = [0] * len(pool)
            for basis_vector in chosen:
                factor = rng.choice((1, 2))
                vector = [(vector[c] + factor * basis_vector[c]) % 3 for c in range(len(pool))]
            vectors.append(vector)
        for vector in vectors:
            cut = cut_from_vector(pool, vector, x, counter)
            if cut is None:
                continue
            cut["pool"] = pool_name
            cut["triangles"] = [[v + 1 for v in pool[i]] for i in cut["triangle_indices"]]
            del cut["triangle_indices"]
            cuts.setdefault(cut_key(cut), cut)
            if len(cuts) >= MAX_GENERATED_CUTS:
                pool_reports.append(
                    {
                        "pool": pool_name,
                        "triangles": len(pool),
                        "basis_dimension": len(basis),
                        "basis_with_nonzero_sum_mod3": nonzero_sum_basis,
                        "vectors_tested": len(vectors),
                        "rejections": counter,
                    }
                )
                return list(cuts.values()), pool_reports
        pool_reports.append(
            {
                "pool": pool_name,
                "triangles": len(pool),
                "basis_dimension": len(basis),
                "basis_with_nonzero_sum_mod3": nonzero_sum_basis,
                "vectors_tested": len(vectors),
                "rejections": counter,
            }
        )
    return list(cuts.values()), pool_reports


def plateau_count(x, weights):
    return sum(1 for v in range(parent.N) if weights[v] >= 10000 and abs(x[v] - (1.0 / 3.0)) <= 1e-6)


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
    edges, weights = parent.parse_edges_weights()
    weights_float = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    rank_cuts = parent_lift.root_cuts(weights_float, adj)
    extra_rows = [parent_lift.parent_row(weights_float), p_parent_row(weights)]
    base_obj, x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, solution=True)
    tight = tight_triangles(triads, x)
    pools = triangle_pools(tight, weights, x)
    cuts, pool_reports = generate_cuts(pools, x)
    cuts = sorted(cuts, key=lambda cut: (-cut["raw_violation"], cut["triangle_count"], cut["coefficient_support"]))
    tested = []
    accepted = []
    for cut in cuts[:MAX_TESTED_CUTS]:
        objective, trial_x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, [cut], solution=True)
        single_drop = base_obj - objective
        row = dict(cut)
        row["single_drop"] = single_drop
        row["heavy_plateau_after_single"] = plateau_count(trial_x, weights)
        tested.append(row)
        if single_drop >= SINGLE_KILL_DROP and len(accepted) < MAX_ACCEPTED_CUTS:
            accepted.append(cut)
    final_obj, final_x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, accepted, solution=True)
    total_drop = base_obj - final_obj
    best_single = max((row["single_drop"] for row in tested), default=0.0)
    status = "RetireMod3TriangleCg"
    if final_obj <= CONTINUATION_OBJECTIVE or total_drop >= BATCH_FUND_DROP or best_single >= SINGLE_FUND_DROP:
        status = "FundMod3TriangleCgReplay"
    elif total_drop >= BATCH_INTERESTING_DROP:
        status = "InterestingMod3TriangleCg"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_branch_slack_mod3_triangle_cg.v1",
            "base_objective": base_obj,
            "triangle_count": len(triads),
            "tight_triangle_count": len(tight),
            "pool_count": len(pools),
            "generated_cut_count": len(cuts),
            "pool_reports": pool_reports,
            "tested_cut_count": len(tested),
            "accepted_cut_count": len(accepted),
            "best_single_drop": best_single,
            "final_objective": final_obj,
            "total_drop": total_drop,
            "base_heavy_third_plateau_count": plateau_count(x, weights),
            "final_heavy_third_plateau_count": plateau_count(final_x, weights),
            "status": status,
            "gates": {
                "tight_tolerance": TIGHT_TOL,
                "max_triangles_per_cut": MAX_TRIANGLES_PER_CUT,
                "max_vertex_support": MAX_VERTEX_SUPPORT,
                "max_coefficient": MAX_COEFFICIENT,
                "max_accepted_cuts": MAX_ACCEPTED_CUTS,
                "single_kill_drop": SINGLE_KILL_DROP,
                "single_fund_drop": SINGLE_FUND_DROP,
                "batch_kill_drop": BATCH_KILL_DROP,
                "batch_interesting_drop": BATCH_INTERESTING_DROP,
                "batch_fund_drop": BATCH_FUND_DROP,
                "continuation_objective": CONTINUATION_OBJECTIVE,
            },
            "top_tested_cuts": sorted(
                tested,
                key=lambda cut: (cut["single_drop"], cut["raw_violation"]),
                reverse=True,
            )[:20],
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k != "top_tested_cuts"}, indent=2))


if __name__ == "__main__":
    main()
