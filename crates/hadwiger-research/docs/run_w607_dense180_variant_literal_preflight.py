import json


REPLAY = "crates/hadwiger-research/docs/w607-fresh-mixed-branch-replay.json"
ALIGN = "crates/hadwiger-research/docs/w607-first-family-alignment-probe.json"
PACKAGING = "crates/hadwiger-research/docs/w607-dense180-core-packaging-probe.json"
OUT = "crates/hadwiger-research/docs/w607-dense180-variant-literal-preflight.json"

TIER_A = [223, 224, 303, 305, 384, 385]
TARGET_GATE = 586500.0
ACTIVE_LEAVES = {1, 2, 4, 6, 9, 12}


def included_key(leaf):
    included = leaf["tier_a_assignment"]["included"]
    return included[0] if len(included) == 1 else None


def support_by_leaf(align):
    return {
        row["leaf_index"]: {
            "center": row["center"],
            "support_digest": row["support_digest"],
            "support": set(row["support_vertices"]),
            "alpha_w": row["alpha_w"],
            "invariant": {
                "weight_sum": row["weight_sum"],
                "pool_incidence": row["pool_incidence"],
                "tier_a_incidence": row["tier_a_incidence"],
                "internal_edges": row["internal_edges"],
                "internal_triangles": row["internal_triangles"],
            },
        }
        for row in align["support_signatures"]
    }


def main():
    replay = json.load(open(REPLAY))
    align = json.load(open(ALIGN))
    packaging = json.load(open(PACKAGING))
    support = support_by_leaf(align)
    variant_rows = {row["leaf_index"]: row for row in packaging["variants"]}
    active = []
    for leaf in replay["leaves"]:
        leaf_id = leaf["leaf_index"]
        if leaf_id not in ACTIVE_LEAVES:
            continue
        active.append(
            {
                "leaf_index": leaf_id,
                "included_tier_a": included_key(leaf),
                "excluded_tier_a": leaf["tier_a_assignment"]["excluded"],
                "mixed_bound": leaf["final_mixed_bound"],
                "center": support[leaf_id]["center"],
                "support_digest": support[leaf_id]["support_digest"],
                "variant_size": len(variant_rows[leaf_id]["vertices"]),
                "variant_alpha_w": variant_rows[leaf_id]["mwis"]["alpha_w"],
            }
        )
    included_cover = sorted(row["included_tier_a"] for row in active)
    non_active = [
        {
            "leaf_index": leaf["leaf_index"],
            "included": leaf["tier_a_assignment"]["included"],
            "excluded": leaf["tier_a_assignment"]["excluded"],
            "mixed_bound": leaf["final_mixed_bound"],
        }
        for leaf in replay["leaves"]
        if leaf["leaf_index"] not in ACTIVE_LEAVES
    ]
    worst_non_active = max(non_active, key=lambda row: row["mixed_bound"])
    singleton_non_active = [row for row in non_active if len(row["included"]) == 1]
    active_bounds = [row["mixed_bound"] for row in active]
    variant_alphas = [row["variant_alpha_w"] for row in active]
    invariant_signatures = {
        json.dumps(support[row["leaf_index"]]["invariant"], sort_keys=True)
        for row in active
    }
    status = "retire_variant_literal_preflight"
    failure_reasons = []
    if included_cover != TIER_A:
        failure_reasons.append("active_leaves_do_not_cover_singleton_tier_a")
    if singleton_non_active:
        failure_reasons.append("singleton_tier_a_leaf_outside_active_set")
    if max(active_bounds) > TARGET_GATE:
        failure_reasons.append("active_singleton_bound_above_gate")
    if worst_non_active["mixed_bound"] > TARGET_GATE:
        failure_reasons.append("non_active_leaf_above_gate")
    if len(set(variant_alphas)) != 1:
        failure_reasons.append("variant_alpha_not_constant")
    if len(invariant_signatures) != 1:
        failure_reasons.append("dense180_invariant_signature_not_constant")
    if not failure_reasons:
        status = "fund_singleton_literal_disjunction"
    report = {
        "schema": "forge.hadwiger.w607_dense180_variant_literal_preflight.v1",
        "authority": "diagnostic_literal_face_alignment_not_export_authority",
        "second_opinion": {
            "agent": "Singer",
            "decision": "approve_bounded_preflight",
            "primary_failure_mode": "six_face_separator_not_root_valid_without_all16_check",
        },
        "source_artifacts": {
            "fresh_replay": REPLAY,
            "alignment": ALIGN,
            "packaging": PACKAGING,
        },
        "tier_a_vertices": TIER_A,
        "target_gate": TARGET_GATE,
        "active_singleton_leaves": sorted(active, key=lambda row: row["included_tier_a"]),
        "included_tier_a_cover": included_cover,
        "singleton_non_active_leaves": singleton_non_active,
        "worst_non_active_leaf": worst_non_active,
        "variant_alpha_values": variant_alphas,
        "dense180_invariant_signature_count": len(invariant_signatures),
        "validity_scope": {
            "six_singleton_faces_checked": True,
            "all16_bounds_checked_against_gate": True,
            "root_valid_cut_constructed": False,
            "literal_lift_still_required": True,
        },
        "failure_reasons": failure_reasons,
        "status": status,
    }
    with open(OUT, "w") as handle:
        json.dump(report, handle, indent=2)
        handle.write("\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
