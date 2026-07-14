import hashlib
import json
import random

import numpy as np
from scipy.sparse import lil_matrix

import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
BRANCH_SLACK = CRATE / "docs" / "w607-branch-slack-parent-lift-diagnostic.json"
PROJECTED = CRATE / "docs" / "w607-v304-projected-parent-lift-diagnostic.json"
OUT_PATH = CRATE / "docs" / "w607-mixed-tight-row-cg.json"

EXPECTED_BASE = 594914.351525072
TIGHT_TOL = 1e-8
STRICT_TIGHT_TOL = 1e-10
PARENT_TIGHT_REL = 1e-7
MODULI = (3, 5)
MAX_POOLS = 40
MAX_SOURCE_ROWS = 160
MAX_SOURCE_ROWS_PER_CUT = 120
MAX_VERTEX_SUPPORT = 220
MAX_GENERATED_CUTS = 3000
MAX_TESTED_CUTS = 20
VIOLATION_GATE = 25.0
SINGLE_FUND_DROP = 1000.0
BATCH_FUND_DROP = 3000.0
BATCH_KILL_DROP = 500.0
PLATEAU_FUND_DROP = 4
RANDOM_SEED = 607


def projected_parent_row(weights):
    lift = json.loads(PROJECTED.read_text())
    coverage, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(coverage[v]) for v in range(parent.N) if v != parent.BRANCH and coverage[v]}
    coeffs[parent.BRANCH] = int(lift["new_lift_coefficient"])
    return coeffs, int(lift["new_rhs_numerator"])


def branch_slack_parent_row(weights):
    artifact = json.loads(BRANCH_SLACK.read_text())
    c0, _ = parent.exclude_coverage(weights)
    coeffs = {v: int(c0[v]) * parent.DENOMINATOR for v in range(parent.N) if c0[v]}
    for vertex, coeff in artifact["positive_coefficients_num_d1024"].items():
        index = int(vertex) - 1
        coeffs[index] = coeffs.get(index, 0) + int(coeff)
    coeffs[parent.BRANCH] = int(artifact["lift_coefficient_num_d1024"])
    return coeffs, int(artifact["gamma0_modified_num_d1024"])


def solve_lp(edges, triads, weights, rank_cuts, extra_rows, cg_cuts=(), solution=False):
    rows = len(edges) + len(triads) + len(rank_cuts) + len(extra_rows) + len(cg_cuts)
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


def row_lhs(row, x):
    return sum(coeff * x[vertex] for vertex, coeff in row["coeffs"].items())


def make_source_rows(edges, triads, weights, rank_cuts, extra_rows, x, tol):
    rows = []
    for edge_index, (a, b) in enumerate(edges):
        if abs(x[a] + x[b] - 1.0) <= tol:
            rows.append({"kind": "edge", "name": f"e{edge_index}", "coeffs": {a: 1, b: 1}, "rhs": 1})
    for tri_index, (a, b, c) in enumerate(triads):
        if abs(x[a] + x[b] + x[c] - 1.0) <= tol:
            rows.append({"kind": "triangle", "name": f"t{tri_index}", "coeffs": {a: 1, b: 1, c: 1}, "rhs": 1})
    for rank_index, (vertices, alpha) in enumerate(rank_cuts):
        coeffs = {int(vertex): int(weights[vertex]) for vertex in vertices}
        row = {"kind": "rank", "name": f"r{rank_index}", "coeffs": coeffs, "rhs": int(alpha)}
        if abs(row_lhs(row, x) - row["rhs"]) <= tol * max(1.0, abs(row["rhs"])):
            rows.append(row)
    for parent_index, (coeffs, rhs) in enumerate(extra_rows):
        row = {
            "kind": "parent",
            "name": f"p{parent_index}",
            "coeffs": {int(v): int(c) for v, c in coeffs.items()},
            "rhs": int(rhs),
        }
        if abs(row_lhs(row, x) - row["rhs"]) <= PARENT_TIGHT_REL * max(1.0, abs(row["rhs"])):
            rows.append(row)
    return rows


def plateau_vertices(weights, x):
    return [v for v in range(parent.N) if weights[v] >= 10000 and abs(x[v] - (1.0 / 3.0)) <= 1e-6]


def row_score(row, seed, rank, plateau):
    vertices = set(row["coeffs"])
    hit = seed in vertices or bool(vertices & plateau)
    return (0 if hit else 1, -sum(rank[v] for v in vertices), len(vertices), row["kind"], row["name"])


def build_pools(rows, weights, x):
    rank = weights.astype(float) * x
    plateau = set(plateau_vertices(weights, x))
    top = [int(v) for v in np.lexsort((np.arange(parent.N), -rank))[:32]]
    parent_activity = sorted(
        range(parent.N),
        key=lambda v: -sum(abs(row["coeffs"].get(v, 0)) * x[v] for row in rows if row["kind"] == "parent"),
    )[:16]
    seeds = list(dict.fromkeys([*plateau, *top, *parent_activity]))
    special = [row for row in rows if row["kind"] in ("rank", "parent")]
    pools = []
    for seed in seeds[: MAX_POOLS - 2]:
        local = [row for row in rows if seed in row["coeffs"] or set(row["coeffs"]) & plateau]
        pool = list({row["name"]: row for row in [*special, *sorted(local, key=lambda r: row_score(r, seed, rank, plateau))]}.values())
        pools.append((f"seed_{seed+1}", pool[:MAX_SOURCE_ROWS]))
    top_rows = sorted(rows, key=lambda r: (-sum(rank[v] for v in r["coeffs"]), len(r["coeffs"]), r["name"]))
    pools.append(("top_tight", [*special, *top_rows][:MAX_SOURCE_ROWS]))
    mixed_rows = [row for row in rows if row["kind"] in ("triangle", "rank", "parent")]
    pools.append(("mixed_only", [*special, *mixed_rows][:MAX_SOURCE_ROWS]))
    return [(name, rows) for name, rows in pools[:MAX_POOLS] if len(rows) >= 2]


def inv_mod(value, modulus):
    return pow(value % modulus, modulus - 2, modulus)


def nullspace_mod(matrix, modulus):
    matrix = [row[:] for row in matrix]
    row_count = len(matrix)
    col_count = len(matrix[0]) if matrix else 0
    pivots = []
    row = 0
    for col in range(col_count):
        pivot = next((r for r in range(row, row_count) if matrix[r][col] % modulus), None)
        if pivot is None:
            continue
        matrix[row], matrix[pivot] = matrix[pivot], matrix[row]
        inv = inv_mod(matrix[row][col], modulus)
        matrix[row] = [(value * inv) % modulus for value in matrix[row]]
        for r in range(row_count):
            if r != row and matrix[r][col] % modulus:
                factor = matrix[r][col] % modulus
                matrix[r] = [(matrix[r][c] - factor * matrix[row][c]) % modulus for c in range(col_count)]
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
            vector[pivot_col] = (-matrix[pivot_row][free_col]) % modulus
        basis.append(vector)
    return basis


def pool_basis(pool, modulus):
    vertices = sorted({vertex for row in pool for vertex in row["coeffs"]})
    index = {vertex: i for i, vertex in enumerate(vertices)}
    matrix = [[0] * len(pool) for _ in vertices]
    for col, row in enumerate(pool):
        for vertex, coeff in row["coeffs"].items():
            matrix[index[vertex]][col] = coeff % modulus
    return nullspace_mod(matrix, modulus)


def clean_gcd(coefficients, rhs):
    values = [abs(value) for value in coefficients.values() if value]
    values.append(abs(rhs))
    gcd = 0
    for value in values:
        gcd = int(np.gcd(gcd, value))
    if gcd <= 1:
        return coefficients, rhs, 1
    return {v: c // gcd for v, c in coefficients.items()}, rhs // gcd, gcd


def reject(counter, reason):
    counter[reason] = counter.get(reason, 0) + 1
    return None


def cut_from_vector(pool, vector, modulus, x, counter):
    active = [i for i, value in enumerate(vector) if value % modulus]
    if not active or len(active) > MAX_SOURCE_ROWS_PER_CUT:
        return reject(counter, "source_support")
    kinds = {pool[i]["kind"] for i in active}
    if kinds <= {"edge", "triangle"}:
        return reject(counter, "not_mixed")
    coeff_sum = {}
    rhs_sum = 0
    for row_index in active:
        multiplier = vector[row_index] % modulus
        rhs_sum += multiplier * pool[row_index]["rhs"]
        for vertex, coeff in pool[row_index]["coeffs"].items():
            coeff_sum[vertex] = coeff_sum.get(vertex, 0) + multiplier * coeff
    if rhs_sum % modulus == 0:
        return reject(counter, "rhs_integral")
    if any(value % modulus for value in coeff_sum.values()):
        return reject(counter, "nonintegral_coefficients")
    coefficients = {vertex: value // modulus for vertex, value in coeff_sum.items() if value}
    rhs = rhs_sum // modulus
    if len(coefficients) > MAX_VERTEX_SUPPORT:
        return reject(counter, "dense_cut")
    coefficients, rhs, gcd = clean_gcd(coefficients, rhs)
    lhs = sum(coeff * x[vertex] for vertex, coeff in coefficients.items())
    violation = lhs - rhs
    if violation < VIOLATION_GATE:
        return reject(counter, "weak_violation")
    return {
        "modulus": modulus,
        "coefficients": coefficients,
        "rhs": rhs,
        "source_rows": [pool[i]["name"] for i in active],
        "source_kinds": sorted(kinds),
        "source_count": len(active),
        "coefficient_support": len(coefficients),
        "rhs_fraction_mod": rhs_sum % modulus,
        "gcd_reduction": gcd,
        "raw_violation": violation,
        "uses_parent": "parent" in kinds,
        "uses_rank": "rank" in kinds,
    }


def cut_key(cut):
    items = tuple(sorted(cut["coefficients"].items()))
    return hashlib.sha256(repr((items, cut["rhs"])).encode()).hexdigest()


def candidate_vectors(basis, width, rng, modulus):
    vectors = list(basis[:100])
    for i in range(min(len(basis), 45)):
        for j in range(i + 1, min(len(basis), i + 8)):
            vectors.append([(basis[i][c] + basis[j][c]) % modulus for c in range(width)])
            vectors.append([(basis[i][c] + 2 * basis[j][c]) % modulus for c in range(width)])
    for _ in range(120):
        if not basis:
            break
        chosen = rng.sample(basis, min(len(basis), rng.randint(2, 7)))
        vector = [0] * width
        for basis_vector in chosen:
            factor = rng.randrange(1, modulus)
            vector = [(vector[c] + factor * basis_vector[c]) % modulus for c in range(width)]
        vectors.append(vector)
    return vectors


def generate_cuts(pools, x):
    rng = random.Random(RANDOM_SEED)
    cuts = {}
    reports = []
    for pool_name, pool in pools:
        for modulus in MODULI:
            basis = pool_basis(pool, modulus)
            counter = {}
            for vector in candidate_vectors(basis, len(pool), rng, modulus):
                cut = cut_from_vector(pool, vector, modulus, x, counter)
                if cut is None:
                    continue
                cut["pool"] = pool_name
                cuts.setdefault(cut_key(cut), cut)
                if len(cuts) >= MAX_GENERATED_CUTS:
                    break
            reports.append(
                {
                    "pool": pool_name,
                    "modulus": modulus,
                    "source_rows": len(pool),
                    "basis_dimension": len(basis),
                    "rejections": counter,
                }
            )
            if len(cuts) >= MAX_GENERATED_CUTS:
                return list(cuts.values()), reports
    return list(cuts.values()), reports


def plateau_count(weights, x):
    return len(plateau_vertices(weights, x))


def clean(value):
    if isinstance(value, dict):
        return {str(key): clean(inner) for key, inner in value.items()}
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
    extra_rows = [projected_parent_row(weights), branch_slack_parent_row(weights)]
    base_obj, x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, solution=True)
    strict_rows = make_source_rows(edges, triads, weights, rank_cuts, extra_rows, x, STRICT_TIGHT_TOL)
    source_rows = make_source_rows(edges, triads, weights, rank_cuts, extra_rows, x, TIGHT_TOL)
    pools = build_pools(source_rows, weights, x)
    cuts, pool_reports = generate_cuts(pools, x)
    cuts = sorted(cuts, key=lambda cut: (-cut["raw_violation"], cut["source_count"], cut["coefficient_support"]))
    tested = []
    accepted = []
    for cut in cuts[:MAX_TESTED_CUTS]:
        obj, trial_x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, [cut], solution=True)
        row = dict(cut)
        row["single_drop"] = base_obj - obj
        row["heavy_third_plateau_after_single"] = plateau_count(weights, trial_x)
        tested.append(row)
        accepted.append(cut)
    final_obj, final_x = solve_lp(edges, triads, weights_float, rank_cuts, extra_rows, accepted, solution=True)
    best_single = max((row["single_drop"] for row in tested), default=0.0)
    total_drop = base_obj - final_obj
    plateau_drop = plateau_count(weights, x) - plateau_count(weights, final_x)
    status = "RetireMixedTightRowCg"
    if best_single >= SINGLE_FUND_DROP or total_drop >= BATCH_FUND_DROP or plateau_drop >= PLATEAU_FUND_DROP:
        status = "FundMixedTightRowCgReplay"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_mixed_tight_row_cg.v1",
            "second_agent_verdict": "allow_one_exact_small_denominator_falsification_pass_k3_k5",
            "base_objective": base_obj,
            "expected_base_objective": EXPECTED_BASE,
            "baseline_reproduced": abs(base_obj - EXPECTED_BASE) <= 1e-5,
            "source_row_counts": {
                "tight_1e_8": len(source_rows),
                "tight_1e_10": len(strict_rows),
                "strict_survival_fraction": len(strict_rows) / max(1, len(source_rows)),
            },
            "pool_count": len(pools),
            "pool_reports": pool_reports,
            "generated_cut_count": len(cuts),
            "tested_cut_count": len(tested),
            "best_current_solution_violation": cuts[0]["raw_violation"] if cuts else 0.0,
            "best_single_drop": best_single,
            "batch_cut_count": len(accepted),
            "batch_objective": final_obj,
            "batch_drop": total_drop,
            "base_heavy_third_plateau_count": plateau_count(weights, x),
            "batch_heavy_third_plateau_count": plateau_count(weights, final_x),
            "heavy_third_plateau_drop": plateau_drop,
            "status": status,
            "kill_gates": {
                "violation_gate": VIOLATION_GATE,
                "single_fund_drop": SINGLE_FUND_DROP,
                "batch_fund_drop": BATCH_FUND_DROP,
                "batch_kill_drop": BATCH_KILL_DROP,
                "plateau_fund_drop": PLATEAU_FUND_DROP,
                "moduli": MODULI,
                "max_tested_cuts": MAX_TESTED_CUTS,
            },
            "top_tested_cuts": sorted(tested, key=lambda cut: (cut["single_drop"], cut["raw_violation"]), reverse=True),
        }
    )
    OUT_PATH.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({k: v for k, v in report.items() if k not in ("pool_reports", "top_tested_cuts")}, indent=2))


if __name__ == "__main__":
    main()
