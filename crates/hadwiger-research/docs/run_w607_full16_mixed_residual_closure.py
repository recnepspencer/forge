import hashlib
import json


FULL16 = "crates/hadwiger-research/docs/w607-full16-micro-branch-stress.json"
LEAF0 = "crates/hadwiger-research/docs/w607-leaf0-residual-pair-closure.json"
OUT = "crates/hadwiger-research/docs/w607-full16-mixed-residual-closure.json"

TARGET_GATE = 586500.0
TARGET_WEIGHTED_ALPHA = 512933
FIXED_POOL = [152, 222, 225, 383, 386, 456]


def digest(value):
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


def leaf_row_identity(leaf):
    return {
        "leaf_index": leaf["leaf_index"],
        "tier_a_assignment": leaf["tier_a_assignment"],
        "first_family_rows_used": leaf["first_family_rows_used"],
    }


def sorted_leaf_table(full, leaf0):
    table = []
    for row in full["leaves"]:
        bound = row["depth3_worst"]
        source = "fixed_pool_depth3"
        if row["leaf_index"] == 0:
            bound = leaf0["final_closed_leaf0_max"]
            source = "leaf0_depth4_residual_pair_closure"
        table.append(
            {
                "leaf_index": row["leaf_index"],
                "top_six": row["top_six"],
                "baseline_after_first_family": row["baseline_after_first_family"],
                "depth3_fixed_pool_bound": row["depth3_worst"],
                "mixed_bound": bound,
                "bound_source": source,
                "movement_from_baseline": row["baseline_after_first_family"] - bound,
                "nodes_solved_depth3": row["nodes"],
                "hit_cap_depth3": row["hit_cap"],
            }
        )
    return sorted(table, key=lambda row: (-row["mixed_bound"], row["leaf_index"]))


def leaf0_exception_block(leaf0):
    triggered = [row for row in leaf0["closures"] if row["triggered"]]
    return {
        "reproduced_depth3_bound": leaf0["depth3_bound"],
        "reproduced_depth4_bound": leaf0["depth4_bound"],
        "triggered_residual_nodes": len(triggered),
        "trigger_assignment": triggered[0]["terminal"]["pool_assignment"] if triggered else None,
        "residual_pair": leaf0["residual_pair_one_based"],
        "children": triggered[0]["children"] if triggered else [],
        "closed_leaf0_bound": leaf0["final_closed_leaf0_max"],
        "extra_lp_solves": leaf0["resource_usage"]["extra_lp_solves"],
    }


def main():
    full = json.load(open(FULL16))
    leaf0 = json.load(open(LEAF0))
    table = sorted_leaf_table(full, leaf0)
    mixed_max = table[0]["mixed_bound"]
    argmax = [row["leaf_index"] for row in table if abs(row["mixed_bound"] - mixed_max) < 1e-6]
    any_cap = full["any_cap_hit"] or leaf0["solver_notes"]["node_cap_hit"]
    timeout = leaf0["solver_notes"]["timeout_hit"]
    reproduction = {
        "full16_fixed_pool_matches": full["fixed_pool"] == FIXED_POOL,
        "full16_depth3_post_max_matches": abs(full["post_microbranch_full_16_max"] - 589302.6440) < 0.01,
        "leaf0_depth3_matches_full16": abs(leaf0["depth3_bound"] - full["leaves"][0]["depth3_worst"]) < 0.01,
        "leaf0_depth4_matches": abs(leaf0["depth4_bound"] - 588378.8643) < 0.01,
        "leaf0_closed_bound_matches": abs(leaf0["final_closed_leaf0_max"] - 581481.0) < 0.01,
        "leaf_count_is_16": len(full["leaves"]) == 16,
    }
    setup_invariants = {
        "new_rows": 0,
        "new_supports": 0,
        "new_mwis": 0,
        "new_variables": 0,
        "fixed_pool": FIXED_POOL,
        "leaf0_exception_only": True,
    }
    row_identities = [leaf_row_identity(row) for row in full["leaves"]]
    report = {
        "schema": "forge.hadwiger.w607_full16_mixed_residual_closure.v1",
        "authority": "export_shape_diagnostic_not_parent_or_root_proof",
        "target_gate": TARGET_GATE,
        "target_weighted_alpha": TARGET_WEIGHTED_ALPHA,
        "second_opinion": {
            "agent": "Huygens",
            "decision": "approve_narrow_scope",
            "primary_failure_mode": "authority_contamination",
        },
        "authority_labels": {
            "root_rows": "proof_substrate_authority",
            "parent_lifts": "proof_substrate_authority",
            "first_family_rows": "per_leaf_enriched_row_system_authority",
            "fixed_pool_trees": "diagnostic_branch_authority_only",
            "leaf0_residual_pair_closure": "diagnostic_branch_authority_only",
        },
        "digests": {
            "root_rows_digest": digest(full["baseline_root_objective"]),
            "parent_lifts_digest": digest(["projected_parent_lift", "branch_slack_parent_lift"]),
            "first_family_rows_digest": digest(row_identities),
            "full16_source_digest": digest({key: full[key] for key in full if key != "leaves"}),
            "leaf0_exception_digest": digest(leaf0_exception_block(leaf0)),
        },
        "source_artifacts": {
            "full16_fixed_pool_stress": FULL16,
            "leaf0_residual_pair_closure": LEAF0,
        },
        "setup_invariants": setup_invariants,
        "reproduction": reproduction,
        "baseline_root_objective": full["baseline_root_objective"],
        "pre_microbranch_full_16_max": full["pre_microbranch_full_16_max"],
        "old_depth3_full16_max": full["post_microbranch_full_16_max"],
        "mixed_full16_max": mixed_max,
        "mixed_full16_argmax_leaves": argmax,
        "margin_to_target_gate": TARGET_GATE - mixed_max,
        "leaf0_exception": leaf0_exception_block(leaf0),
        "resource_usage": {
            "full16_depth3_total_nodes": full["total_nodes"],
            "leaf0_exception_extra_lp_solves": leaf0["resource_usage"]["extra_lp_solves"],
            "mixed_total_nodes_plus_extra_solves": full["total_nodes"] + leaf0["resource_usage"]["extra_lp_solves"],
            "any_cap_hit": any_cap,
            "timeout_hit": timeout,
        },
        "leaf_bounds_sorted": table,
        "gates": {
            "mixed_max_at_or_below_target_gate": mixed_max <= TARGET_GATE,
            "no_cap_hit": not any_cap,
            "no_timeout": not timeout,
            "all_reproduction_checks_pass": all(reproduction.values()),
            "authority_boundary_explicit": True,
        },
    }
    report["failure_reasons"] = [
        reason
        for reason, active in [
            ("mixed_max_above_target_gate", mixed_max > TARGET_GATE),
            ("cap_hit", any_cap),
            ("timeout_hit", timeout),
            ("reproduction_failed", not all(reproduction.values())),
        ]
        if active
    ]
    report["status"] = (
        "fund_export_lift_design"
        if not report["failure_reasons"]
        else "retire_mixed_residual_closure"
    )
    with open(OUT, "w") as handle:
        json.dump(report, handle, indent=2)
        handle.write("\n")
    print(json.dumps({key: value for key, value in report.items() if key != "leaf_bounds_sorted"}, indent=2))


if __name__ == "__main__":
    main()
