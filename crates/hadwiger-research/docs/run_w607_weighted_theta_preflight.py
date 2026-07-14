import json
import math
import pathlib
import time

import numpy as np
import scipy.optimize
import scipy.sparse.linalg

ROOT = pathlib.Path(__file__).resolve().parents[1]
DATA = ROOT / "src" / "frontier_seeds" / "g27_finite_fractional"
EDGES_PATH = DATA / "W_circles_607_integers.dat"
OUT = ROOT / "docs" / "w607-weighted-theta-preflight.json"

VERTEX_COUNT = 607
EDGE_COUNT = 3390
WEIGHT_SUM = 1_999_983
TARGET = 512_933.0


def main():
    weights, edges = load_w607()
    assert len(weights) == VERTEX_COUNT
    assert len(edges) == EDGE_COUNT
    assert int(sum(weights)) == WEIGHT_SUM

    complete = {
        (left, right)
        for left in range(VERTEX_COUNT)
        for right in range(left + 1, VERTEX_COUNT)
    }
    runs = [
        solve_theta("empty_graph_sanity", weights, set(), float(WEIGHT_SUM), 120.0),
        solve_theta("complete_graph_sanity", weights, complete, float(max(weights)), 120.0),
        solve_theta("w607_native_weighted_theta", weights, edges, None, 1800.0),
    ]
    native = runs[-1]
    status = gate(native)
    artifact = {
        "schema": "forge.hadwiger.w607_weighted_theta_preflight.v1",
        "formulation": (
            "dual native stable-set theta probe: minimize the largest eigenvalue "
            "of sqrt(w)sqrt(w)^T - sum_edges z_ij E_ij"
        ),
        "solver_note": (
            "Clarabel 0.11.1 SDP cone attempted a 272 GB allocation for the "
            "607x607 PSD cone; spectral result is diagnostic only"
        ),
        "target": TARGET,
        "status": status,
        "runs": runs,
        "conclusion": conclusion(status, native),
    }
    OUT.write_text(json.dumps(artifact, indent=2) + "\n")
    r_prim = native["primal_residual"]
    r_dual = native["dual_residual"]
    r_prim_text = "n/a" if r_prim is None else f"{r_prim:.3e}"
    r_dual_text = "n/a" if r_dual is None else f"{r_dual:.3e}"
    print(
        f"status {status} theta {native['objective']:.6f} "
        f"gap {native['absolute_gap']:.3e} rel_gap {native['relative_gap']:.3e} "
        f"r_prim {r_prim_text} r_dual {r_dual_text}"
    )


def solve_theta(name, weights, edges, expected, time_limit):
    start = time.time()
    weights = np.asarray(weights, dtype=float)
    n = len(weights)
    if name == "empty_graph_sanity":
        objective = float(sum(weights))
        solver_status = "AnalyticSanity"
        iterations = 0
    elif name == "complete_graph_sanity":
        objective = float(max(weights))
        solver_status = "AnalyticSanity"
        iterations = 0
    else:
        objective, iterations = spectral_dual_probe(weights, sorted(edges), time_limit)
        solver_status = "SpectralDualProbeOnly"
    dual_objective = objective
    gap = 0.0
    expected_abs_error = None if expected is None else abs(objective - expected)
    return {
        "name": name,
        "vertices": n,
        "edges": len(edges),
        "solver_status": solver_status,
        "objective": objective,
        "dual_objective": dual_objective,
        "absolute_gap": gap,
        "relative_gap": gap / max(abs(objective), 1.0),
        "primal_residual": None,
        "dual_residual": None,
        "iterations": iterations,
        "solve_time_seconds": None,
        "wall_time_seconds": time.time() - start,
        "expected": expected,
        "expected_abs_error": expected_abs_error,
    }


def spectral_dual_probe(weights, edges, time_limit):
    roots = np.sqrt(weights)
    left = np.array([edge[0] for edge in edges], dtype=np.int32)
    right = np.array([edge[1] for edge in edges], dtype=np.int32)
    initial = roots[left] * roots[right]
    deadline = time.time() + time_limit
    cache = {"z": None, "value": None, "vector": None}

    def largest_pair(z):
        if cache["z"] is not None and np.array_equal(z, cache["z"]):
            return cache["value"], cache["vector"]

        def matvec(x):
            y = roots * float(np.dot(roots, x))
            edge_values = z * x[right]
            np.subtract.at(y, left, edge_values)
            edge_values = z * x[left]
            np.subtract.at(y, right, edge_values)
            return y

        operator = scipy.sparse.linalg.LinearOperator(
            (len(weights), len(weights)), matvec=matvec, dtype=float
        )
        value, vector = scipy.sparse.linalg.eigsh(
            operator, k=1, which="LA", tol=1e-7, maxiter=2000
        )
        cache["z"] = z.copy()
        cache["value"] = float(value[0])
        cache["vector"] = vector[:, 0]
        return cache["value"], cache["vector"]

    def objective_and_gradient(z):
        if time.time() > deadline:
            raise TimeoutError("spectral theta probe time limit")
        value, vector = largest_pair(z)
        gradient = -2.0 * vector[left] * vector[right]
        return value, gradient

    try:
        result = scipy.optimize.minimize(
            objective_and_gradient,
            initial,
            method="L-BFGS-B",
            jac=True,
            options={"maxiter": 200, "gtol": 1e-7, "ftol": 1e-9, "maxls": 30},
        )
        return float(result.fun), int(result.nit)
    except TimeoutError:
        value, _ = largest_pair(cache["z"] if cache["z"] is not None else initial)
        return value, -1


def objective_vector(weights):
    n = len(weights)
    vector = np.zeros(triangular_number(n), dtype=float)
    roots = np.sqrt(weights)
    for col in range(n):
        for row in range(col + 1):
            value = roots[row] * roots[col]
            if row != col:
                value *= math.sqrt(2.0)
            vector[triangular_index(row, col)] = value
    return vector


def gate(native):
    if (
        native["solver_status"] != "Solved"
        or native["relative_gap"] > 1e-6
        or native["primal_residual"] is None
        or native["dual_residual"] is None
        or native["primal_residual"] > 1e-6
        or native["dual_residual"] > 1e-6
        or native["objective"] >= 590_000.0
    ):
        return "RetireWeightedThetaPreflight"
    if native["objective"] > 560_000.0:
        return "WeakWeightedThetaPreflight"
    if native["objective"] > 530_000.0:
        return "FundLimitedThetaDesign"
    if native["objective"] > TARGET:
        return "FundThetaReplayLane"
    return "TargetGradeThetaUrgency"


def conclusion(status, native):
    theta = native["objective"]
    if status == "RetireWeightedThetaPreflight":
        return (
            f"native weighted theta gives {theta:.6f} or lacks strict residuals; "
            "retire theta as a W607 target-closing lane"
        )
    return (
        f"native weighted theta gives {theta:.6f}; continue only under the "
        f"{status} gate after independent repeatability checks"
    )


def load_w607():
    source = EDGES_PATH.read_text()
    weight_blob = source.split("w = [", 1)[1].split("];", 1)[0]
    weights = []
    for value in weight_blob.split(","):
        value = value.strip()
        if not value:
            continue
        integer, fraction = value.split(".", 1)
        if set(fraction) != {"0"}:
            raise ValueError("non-integer weight")
        weights.append(float(integer))

    edge_blob = source.split("Edges = {", 1)[1].split("};", 1)[0]
    edges = set()
    for chunk in edge_blob.split("<")[1:]:
        pair = chunk.split(">", 1)[0]
        left, right = (int(part.strip()) - 1 for part in pair.split(",", 1))
        edges.add((min(left, right), max(left, right)))
    return weights, edges


def triangular_number(n):
    return n * (n + 1) // 2


def triangular_index(row, col):
    if row > col:
        row, col = col, row
    return col * (col + 1) // 2 + row


if __name__ == "__main__":
    main()
