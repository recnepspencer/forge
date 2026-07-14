import heapq
import json

import numpy as np

import run_w607_full_tree_rank_family as full_family
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-post-family-micro-branch.json"

LEAF_COUNT = 6
CANDIDATE_COUNT = 6
NODE_CAP = 24
DEPTH_TWO = 2
DEPTH_THREE = 3
DEPTH_THREE_GATE = 1500.0
KILL_MOVEMENT = 1500.0
KILL_MAX = 591750.0
CONTINUE_MOVEMENT = 3000.0
CONTINUE_MAX = 590000.0
ALT_MAX = 591000.0


def support_hash(vertices):
    import hashlib

    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def fixed_from_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def fixed_summary(fixed):
    return {
        "included": [v + 1 for v, value in sorted(fixed.items()) if value == 1.0],
        "excluded": [v + 1 for v, value in sorted(fixed.items()) if value == 0.0],
    }


def first_family_cuts(report, leaf, edges, triads, weights, adj, root_cuts, parent_rows):
    fixed = fixed_from_leaf(leaf)
    _base, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, parent_rows, fixed, True)
    candidates = {support_hash(row["vertices"]): row for row in full_family.candidate_rows(weights, x, fixed, adj)}
    cuts = []
    rows = []
    for accepted in report["accepted_rows"]:
        row = candidates[accepted["support_digest"]]
        cuts.append((row["vertices"], int(accepted["alpha_w"])))
        rows.append(
            {
                "template_id": accepted["template_id"],
                "center": accepted["center"],
                "support_digest": accepted["support_digest"],
                "size": accepted["size"],
                "alpha_w": int(accepted["alpha_w"]),
            }
        )
    return cuts, rows


def solve_node(edges, triads, weights, cuts, parent_rows, fixed):
    try:
        objective, x = parent_lift.solve_lp(
            edges, triads, weights, cuts, parent_rows, fixed=fixed, solution=True
        )
        return {"feasible": True, "upper": objective, "x": x}
    except ValueError as error:
        return {"feasible": False, "upper": float("-inf"), "error": str(error), "x": None}


def candidate_pool(weights, x, fixed):
    fixed_vertices = set(fixed)
    candidates = [
        v
        for v in range(parent.N)
        if v not in fixed_vertices and 1e-7 < x[v] < 1.0 - 1e-7 and weights[v] >= 10000
    ]
    ordered = sorted(candidates, key=lambda v: (-weights[v] * min(x[v], 1.0 - x[v]), -weights[v], v))
    return [
        {
            "vertex": int(v + 1),
            "weight": float(weights[v]),
            "x": float(x[v]),
            "score": float(weights[v] * min(x[v], 1.0 - x[v])),
        }
        for v in ordered[:CANDIDATE_COUNT]
    ]


def choose_branch(node, weights, allowed):
    candidates = [v for v in allowed if v not in node["fixed"] and node["x"] is not None and 1e-7 < node["x"][v] < 1.0 - 1e-7]
    if not candidates:
        return None
    return max(candidates, key=lambda v: (weights[v] * min(node["x"][v], 1.0 - node["x"][v]), -v))


def run_tree(edges, triads, weights, cuts, parent_rows, base_fixed, allowed, max_depth):
    counter = 0
    root = solve_node(edges, triads, weights, cuts, parent_rows, base_fixed)
    root.update({"fixed": dict(base_fixed), "depth": 0, "branch_vertex": None})
    heap = [(-root["upper"], counter, root)]
    expanded = []
    closed = []
    child_objectives = []
    while heap and len(expanded) < NODE_CAP:
        _priority, _order, node = heapq.heappop(heap)
        if not node["feasible"]:
            closed.append(node)
            continue
        branch = None if node["depth"] >= max_depth else choose_branch(node, weights, allowed)
        if branch is None:
            closed.append(node)
            continue
        node["branch_vertex"] = branch
        expanded.append(node)
        for value in (0.0, 1.0):
            fixed = dict(node["fixed"])
            fixed[branch] = value
            child = solve_node(edges, triads, weights, cuts, parent_rows, fixed)
            child.update({"fixed": fixed, "depth": node["depth"] + 1, "branch_vertex": None})
            counter += 1
            if child["feasible"]:
                heapq.heappush(heap, (-child["upper"], counter, child))
            else:
                closed.append(child)
            child_objectives.append(
                {
                    "parent_depth": node["depth"],
                    "branch_vertex": branch + 1,
                    "value": value,
                    "feasible": child["feasible"],
                    "upper": child["upper"],
                }
            )
    open_nodes = [item[2] for item in heap]
    all_leaves = closed + open_nodes
    finite = [leaf for leaf in all_leaves if leaf["feasible"]]
    worst = max((leaf["upper"] for leaf in finite), default=float("-inf"))
    return {
        "max_depth": max_depth,
        "nodes_solved": len(expanded) + len(child_objectives) + 1,
        "expanded_count": len(expanded),
        "closed_leaves": len(closed),
        "open_leaves": len(open_nodes),
        "hit_node_cap": bool(heap and len(expanded) >= NODE_CAP),
        "worst_leaf_objective": worst,
        "branch_variables_by_depth": [
            {"depth": node["depth"], "vertex": node["branch_vertex"] + 1, "upper": node["upper"]}
            for node in expanded
        ],
        "child_objectives": child_objectives,
    }


def repeated_variables(leaf_reports):
    counts = {}
    for report in leaf_reports:
        seen = {item["vertex"] for item in report["selected_tree"]["branch_variables_by_depth"]}
        for vertex in seen:
            counts[vertex] = counts.get(vertex, 0) + 1
    return {str(vertex): count for vertex, count in sorted(counts.items()) if count >= 2}


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "x"}
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
    source = json.loads(SOURCE.read_text())
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    root_obj, _root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, parent_rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    selected = sorted(source["leaves"], key=lambda row: -row["final_objective"])[:LEAF_COUNT]
    leaf_reports = []
    for report in selected:
        leaf = finite[report["leaf_index"]]
        fixed = fixed_from_leaf(leaf)
        first_cuts, first_rows = first_family_cuts(report, leaf, edges, triads, weights, adj, root_cuts, parent_rows)
        cuts = root_cuts + first_cuts
        baseline, x = parent_lift.solve_lp(edges, triads, weights, cuts, parent_rows, fixed=fixed, solution=True)
        pool = candidate_pool(weights, x, fixed)
        allowed = [item["vertex"] - 1 for item in pool]
        depth_two = run_tree(edges, triads, weights, cuts, parent_rows, fixed, allowed, DEPTH_TWO)
        selected_tree = depth_two
        depth_three = None
        if baseline - depth_two["worst_leaf_objective"] >= DEPTH_THREE_GATE:
            depth_three = run_tree(edges, triads, weights, cuts, parent_rows, fixed, allowed, DEPTH_THREE)
            selected_tree = depth_three
        leaf_reports.append(
            {
                "leaf_index": report["leaf_index"],
                "tier_a_assignment": fixed_summary(fixed),
                "first_family_rows_used": first_rows,
                "baseline_leaf_objective": baseline,
                "branch_candidate_pool": pool,
                "depth_two_tree": depth_two,
                "depth_three_tree": depth_three,
                "selected_tree": selected_tree,
                "leaf_movement": baseline - selected_tree["worst_leaf_objective"],
            }
        )
    baseline_max = max(row["baseline_leaf_objective"] for row in leaf_reports)
    final_max = max(row["selected_tree"]["worst_leaf_objective"] for row in leaf_reports)
    reps = repeated_variables(leaf_reports)
    compact_score = sum(1 for count in reps.values() if count >= 4)
    total_nodes = sum(row["selected_tree"]["nodes_solved"] for row in leaf_reports)
    status = "RetirePostFamilyMicroBranch"
    if baseline_max - final_max >= CONTINUE_MOVEMENT and final_max <= CONTINUE_MAX:
        status = "FundPostFamilyBranchSubstrate"
    elif final_max <= ALT_MAX and total_nodes <= 144 and compact_score >= 3:
        status = "CompactPostFamilyBranchSubstrate"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_post_family_micro_branch.v1",
            "authority": "diagnostic_leaf_local_branch_substrate_no_parent_authority",
            "baseline_root_objective": root_obj,
            "baseline_full_tree_max": source["final_full_tree_max"],
            "post_family_top_six_leaf_ids": [row["leaf_index"] for row in selected],
            "top_six_baseline_max": baseline_max,
            "top_six_post_branch_max": final_max,
            "max_movement": baseline_max - final_max,
            "worst_leaf_id": max(leaf_reports, key=lambda row: row["selected_tree"]["worst_leaf_objective"])["leaf_index"],
            "repeated_branch_variables": reps,
            "compact_substrate_score": compact_score,
            "total_nodes_solved": total_nodes,
            "gates": {
                "candidate_count": CANDIDATE_COUNT,
                "node_cap": NODE_CAP,
                "depth_two": DEPTH_TWO,
                "depth_three": DEPTH_THREE,
                "depth_three_gate": DEPTH_THREE_GATE,
                "kill_movement": KILL_MOVEMENT,
                "kill_max": KILL_MAX,
                "continue_movement": CONTINUE_MOVEMENT,
                "continue_max": CONTINUE_MAX,
                "alternative_max": ALT_MAX,
            },
            "decision_reason": (
                "continue"
                if status != "RetirePostFamilyMicroBranch"
                else "movement_below_gate"
                if baseline_max - final_max < KILL_MOVEMENT
                else "final_max_above_gate"
            ),
            "leaves": leaf_reports,
            "status": status,
        }
    )
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "leaves"}, indent=2))


if __name__ == "__main__":
    main()
