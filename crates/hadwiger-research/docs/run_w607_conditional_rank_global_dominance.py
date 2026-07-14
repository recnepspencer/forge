import hashlib
import json

import numpy as np

import run_w607_branch_slack_mod3_triangle_cg as branch_slack
import run_w607_full_tree_rank_family as full_family
import run_w607_multileaf_conditional_rank_bundle as bundle
import run_w607_plateau_affine_disjunction as affine
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
OUT = CRATE / "docs" / "w607-conditional-rank-global-dominance.json"

SINGLE_DROP_KILL = 250.0
BUNDLE_DROP_KILL = 1000.0
FUND_DROP = 3000.0
FUND_OBJECTIVE = 592000.0


def graph_digest(edges, weights):
    payload = {
        "edges": [[int(a) + 1, int(b) + 1] for a, b in edges],
        "weights": [int(weight) for weight in weights],
    }
    return hashlib.sha256(json.dumps(payload, separators=(",", ":")).encode()).hexdigest()


def support_hash(vertices):
    return hashlib.sha256(",".join(str(v + 1) for v in vertices).encode()).hexdigest()


def fixed_from_leaf(leaf):
    return {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}


def accepted_sources():
    source = json.loads(SOURCE.read_text())
    rows = []
    for leaf in source["leaves"]:
        for row in leaf["accepted_rows"]:
            rows.append({**row, "leaf_index": leaf["leaf_index"]})
    return rows


def reconstruct_rows(edges, triads, weights, adj, root_cuts, rows):
    _expanded, leaves = affine.full_tree(edges, triads, weights, root_cuts, rows)
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    by_digest = {}
    for leaf_index, leaf in enumerate(finite):
        fixed = fixed_from_leaf(leaf)
        base_obj, x = bundle.leaf_rank.solve_lp(edges, triads, weights, root_cuts, rows, fixed, True)
        for candidate in full_family.candidate_rows(weights, x, fixed, adj):
            digest = support_hash(candidate["vertices"])
            by_digest[(leaf_index, digest)] = {
                "vertices": candidate["vertices"],
                "leaf_baseline": base_obj,
                "template_id": candidate["template_id"],
                "center": candidate["center"] + 1,
            }
    out = []
    for source in accepted_sources():
        key = (source["leaf_index"], source["support_digest"])
        reconstructed = by_digest.get(key)
        if reconstructed is None:
            raise ValueError(f"could not reconstruct accepted support {key}")
        out.append({**source, **reconstructed})
    return out


def rank_cut(row):
    return (row["vertices"], int(row["alpha_w"]))


def root_lhs(row, weights, x):
    vertices = row["vertices"]
    return float(np.dot(weights[list(vertices)], x[list(vertices)]))


def individual_reports(edges, triads, weights, root_cuts, parent_rows, root_obj, root_x, accepted):
    reports = []
    for row in accepted:
        obj, x = parent_lift.solve_lp(
            edges,
            triads,
            weights,
            root_cuts + [rank_cut(row)],
            parent_rows,
            solution=True,
        )
        reports.append(
            {
                "leaf_index": row["leaf_index"],
                "template_id": row["template_id"],
                "center": row["center"],
                "support_digest": row["support_digest"],
                "size": len(row["vertices"]),
                "alpha_w": int(row["alpha_w"]),
                "alpha_source": row["alpha_source"],
                "q_lhs_at_root": root_lhs(row, weights, root_x),
                "root_violation": root_lhs(row, weights, root_x) - float(row["alpha_w"]),
                "root_objective": obj,
                "drop": root_obj - obj,
                "new_x304": float(x[parent.BRANCH]),
            }
        )
    return reports


def clean(value):
    if isinstance(value, dict):
        return {key: clean(inner) for key, inner in value.items() if key != "vertices"}
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
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    triads = parent.triangles(adj)
    root_cuts = parent_lift.root_cuts(weights, adj)
    parent_rows = [parent_lift.parent_row(weights), bundle.plateau.p_parent_row(weights)]
    root_obj, root_x = parent_lift.solve_lp(edges, triads, weights, root_cuts, parent_rows, solution=True)
    accepted = reconstruct_rows(edges, triads, weights, adj, root_cuts, parent_rows)
    individuals = individual_reports(edges, triads, weights, root_cuts, parent_rows, root_obj, root_x, accepted)
    all_cuts = [rank_cut(row) for row in accepted]
    bundle_obj, bundle_x = parent_lift.solve_lp(
        edges,
        triads,
        weights,
        root_cuts + all_cuts,
        parent_rows,
        solution=True,
    )
    best_single = min(individuals, key=lambda row: row["root_objective"])
    status = "RetireConditionalRankLiftByDominance"
    if root_obj - bundle_obj >= FUND_DROP or bundle_obj <= FUND_OBJECTIVE:
        status = "FundGlobalConditionalRankReplayBeforeLift"
    elif best_single["drop"] >= SINGLE_DROP_KILL or root_obj - bundle_obj >= BUNDLE_DROP_KILL:
        status = "KeepDiagnosticOnlyGlobalConditionalRankRows"
    report = clean(
        {
            "schema": "forge.hadwiger.w607_conditional_rank_global_dominance.v1",
            "authority": "diagnostic_dominance_precheck_no_new_mwis",
            "graph_digest": graph_digest(edges, weights),
            "row_system": "16_root_rank_rows_plus_projected_parent_lift_plus_branch_slack_parent_lift",
            "source_artifact": str(SOURCE),
            "accepted_row_count": len(accepted),
            "root_objective_before": root_obj,
            "root_x304_before": float(root_x[parent.BRANCH]),
            "root_objective_after_all_global_rows": bundle_obj,
            "root_x304_after_all_global_rows": float(bundle_x[parent.BRANCH]),
            "all_global_rows_drop": root_obj - bundle_obj,
            "best_single": best_single,
            "best_single_drop": best_single["drop"],
            "dominance_argument": (
                "Each accepted conditional rank row uses alpha_w from an ordinary support MWIS, "
                "so q*x <= alpha_w is already globally valid. Any Tier-A literal lift with "
                "nonnegative RHS charges is weaker at the root than this global row."
            ),
            "gates": {
                "single_drop_kill": SINGLE_DROP_KILL,
                "bundle_drop_kill": BUNDLE_DROP_KILL,
                "fund_drop": FUND_DROP,
                "fund_objective": FUND_OBJECTIVE,
            },
            "failure_classification": (
                "funded"
                if status == "FundGlobalConditionalRankReplayBeforeLift"
                else "lp_redundant"
                if root_obj - bundle_obj < BUNDLE_DROP_KILL
                else "diagnostic_only"
            ),
            "individual_rows": individuals,
            "status": status,
        }
    )
    OUT.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key != "individual_rows"}, indent=2))


if __name__ == "__main__":
    main()
