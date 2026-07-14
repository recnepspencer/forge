import hashlib
import itertools
import json

import numpy as np

import run_w607_full_tree_rank_family as family
import run_w607_post_parent_lift_branch_prescreen as parent_lift
import run_w607_v304_projected_parent_lift_diagnostic as parent


CRATE = parent.CRATE
SOURCE = CRATE / "docs" / "w607-full-tree-rank-family.json"
REPLAY = CRATE / "docs" / "w607-fresh-mixed-branch-replay.json"
DUAL = CRATE / "docs" / "w607-terminal-dual-provenance.json"
GLOBAL = CRATE / "docs" / "w607-conditional-rank-global-dominance.json"
OUT = CRATE / "docs" / "w607-first-family-alignment-probe.json"

ACTIVE_TEMPLATE = "dense180_top_wx_center_2"
POOL = [151, 221, 224, 382, 385, 455]
TIER_A = [222, 223, 302, 304, 383, 384]
HIGH_JACCARD = 0.55
MIN_COMMON_CORE = 40
MIN_SIGNATURE_MATCH = 4
MIN_INVARIANT_MATCH = 6


def digest(value):
    payload = json.dumps(jsonable(value), sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(payload).hexdigest()


def jsonable(value):
    if isinstance(value, dict):
        return {str(key): jsonable(inner) for key, inner in value.items()}
    if isinstance(value, (list, tuple, set)):
        return [jsonable(inner) for inner in value]
    if isinstance(value, np.integer):
        return int(value)
    if isinstance(value, np.floating):
        return float(value)
    return value


def source_by_leaf():
    source = json.loads(SOURCE.read_text())
    return source, {row["leaf_index"]: row for row in source["leaves"]}


def active_dense180_rows(source):
    out = []
    for leaf in source["leaves"]:
        for row in leaf.get("accepted_rows", []):
            if row["template_id"] == ACTIVE_TEMPLATE and leaf["leaf_index"] in {1, 2, 4, 6, 9, 12}:
                out.append({"leaf_index": leaf["leaf_index"], **row})
    return sorted(out, key=lambda row: row["leaf_index"])


def support_map(edges, weights):
    adj = parent.adjacency(edges)
    source, by_leaf = source_by_leaf()
    _expanded, leaves = __import__("run_w607_plateau_affine_disjunction").full_tree(
        edges,
        parent.triangles(adj),
        weights,
        parent_lift.root_cuts(weights, adj),
        [parent_lift.parent_row(weights), __import__("run_w607_branch_slack_mod3_triangle_cg").p_parent_row(weights)],
    )
    finite = [leaf for leaf in leaves if leaf["feasible"]]
    supports = {}
    for row in active_dense180_rows(source):
        leaf = finite[row["leaf_index"]]
        fixed = {int(vertex): float(value) for vertex, value in leaf["fixed"].items()}
        _base, x = __import__("run_w607_multileaf_conditional_rank_bundle").leaf_rank.solve_lp(
            edges,
            parent.triangles(adj),
            weights,
            parent_lift.root_cuts(weights, adj),
            [parent_lift.parent_row(weights), __import__("run_w607_branch_slack_mod3_triangle_cg").p_parent_row(weights)],
            fixed,
            True,
        )
        candidates = {family.support_hash(item["vertices"]): item for item in family.candidate_rows(weights, x, fixed, adj)}
        supports[row["leaf_index"]] = candidates[row["support_digest"]]["vertices"]
    return supports, source


def shell_counts(center, vertices, adj):
    counts = {"center": 0, "neighbors": 0, "distance2": 0, "farther": 0}
    n1 = adj[center]
    n2 = set(n1)
    for vertex in list(n1):
        n2.update(adj[vertex])
    for vertex in vertices:
        if vertex == center:
            counts["center"] += 1
        elif vertex in n1:
            counts["neighbors"] += 1
        elif vertex in n2:
            counts["distance2"] += 1
        else:
            counts["farther"] += 1
    return counts


def internal_edge_count(vertices, adj):
    total = 0
    vertex_set = set(vertices)
    for vertex in vertices:
        total += sum(1 for other in adj[vertex] if other in vertex_set and other > vertex)
    return total


def internal_triangle_count(vertices, adj):
    vertex_set = set(vertices)
    total = 0
    for a in vertices:
        for b in adj[a] & vertex_set:
            if b <= a:
                continue
            for c in adj[a] & adj[b] & vertex_set:
                if c > b:
                    total += 1
    return total


def row_signature(row, vertices, weights, adj):
    center = row["center"] - 1
    vertex_set = set(vertices)
    return {
        "leaf_index": row["leaf_index"],
        "template_id": row["template_id"],
        "center": row["center"],
        "size": len(vertices),
        "alpha_w": row["alpha_w"],
        "support_digest": row["support_digest"],
        "weight_sum": float(sum(weights[list(vertices)])),
        "pool_incidence": sorted(vertex + 1 for vertex in POOL if vertex in vertex_set),
        "tier_a_incidence": sorted(vertex + 1 for vertex in TIER_A if vertex in vertex_set),
        "center_shell_counts": shell_counts(center, vertices, adj),
        "internal_edges": internal_edge_count(vertices, adj),
        "internal_triangles": internal_triangle_count(vertices, adj),
        "support_vertices": [vertex + 1 for vertex in vertices],
    }


def pairwise(rows, supports):
    out = []
    for left, right in itertools.combinations(rows, 2):
        a = set(supports[left["leaf_index"]])
        b = set(supports[right["leaf_index"]])
        inter = len(a & b)
        union = len(a | b)
        out.append(
            {
                "left_leaf": left["leaf_index"],
                "right_leaf": right["leaf_index"],
                "overlap": inter,
                "union": union,
                "jaccard": inter / union,
                "center_pair": [left["center"], right["center"]],
            }
        )
    return out


def dual_contributions():
    dual = json.loads(DUAL.read_text())
    out = {}
    for summary in dual["terminal_summaries"]:
        leaf = summary["leaf_index"]
        if leaf in {1, 2, 4, 6, 9, 12}:
            out[leaf] = summary["positive_dual_mass_by_family"].get("first_family_leaf_rows", 0.0)
    return out


def root_slack_from_global(row):
    if not GLOBAL.exists():
        return None
    global_report = json.loads(GLOBAL.read_text())
    for candidate in global_report.get("rows", []):
        if candidate.get("support_digest") == row["support_digest"]:
            return candidate
    return None


def main():
    edges, weights = parent.parse_edges_weights()
    weights = weights.astype(float)
    adj = parent.adjacency(edges)
    supports, source = support_map(edges, weights)
    rows = active_dense180_rows(source)
    signatures = [row_signature(row, supports[row["leaf_index"]], weights, adj) for row in rows]
    pairwise_rows = pairwise(rows, supports)
    common_core = set.intersection(*(set(supports[row["leaf_index"]]) for row in rows))
    union = set.union(*(set(supports[row["leaf_index"]]) for row in rows))
    duals = dual_contributions()
    jaccards = [row["jaccard"] for row in pairwise_rows]
    signature_groups = {}
    invariant_groups = {}
    for sig in signatures:
        key = json.dumps(
            {
                "pool": sig["pool_incidence"],
                "tier": sig["tier_a_incidence"],
                "shell": sig["center_shell_counts"],
                "edges": sig["internal_edges"],
                "triangles": sig["internal_triangles"],
                "weight_sum": sig["weight_sum"],
            },
            sort_keys=True,
        )
        signature_groups.setdefault(key, []).append(sig["leaf_index"])
        invariant_key = json.dumps(
            {
                "pool": sig["pool_incidence"],
                "tier": sig["tier_a_incidence"],
                "edges": sig["internal_edges"],
                "triangles": sig["internal_triangles"],
                "weight_sum": sig["weight_sum"],
                "size": sig["size"],
                "alpha_w": sig["alpha_w"],
            },
            sort_keys=True,
        )
        invariant_groups.setdefault(invariant_key, []).append(sig["leaf_index"])
    global_slacks = {str(row["leaf_index"]): root_slack_from_global(row) for row in rows}
    average_jaccard = sum(jaccards) / len(jaccards)
    common_signature_count = max((len(group) for group in signature_groups.values()), default=0)
    common_invariant_count = max((len(group) for group in invariant_groups.values()), default=0)
    status = "alignment_retired_unrelated"
    failure_reasons = []
    if average_jaccard < HIGH_JACCARD:
        failure_reasons.append("low_pairwise_jaccard")
    if len(common_core) < MIN_COMMON_CORE:
        failure_reasons.append("small_common_core")
    if common_invariant_count < MIN_INVARIANT_MATCH:
        failure_reasons.append("no_shared_invariant_signature")
    if not failure_reasons:
        status = "family_export_funded"
    report = {
        "schema": "forge.hadwiger.w607_first_family_alignment_probe.v1",
        "authority": "diagnostic_family_alignment_not_export_authority",
        "second_opinion": {
            "agent": "Peirce",
            "decision": "approve_bounded_falsification_probe",
            "primary_failure_mode": "false_family_signal_from_template_metadata",
        },
        "source_binding": {
            "first_family_source": str(SOURCE),
            "first_family_digest": digest(source),
            "fresh_replay_path": str(REPLAY),
            "dual_provenance_path": str(DUAL),
        },
        "active_template": ACTIVE_TEMPLATE,
        "active_leaf_ids": [row["leaf_index"] for row in rows],
        "support_count": len(rows),
        "common_core_size": len(common_core),
        "common_core_vertices": sorted(vertex + 1 for vertex in common_core),
        "union_size": len(union),
        "average_pairwise_jaccard": average_jaccard,
        "min_pairwise_jaccard": min(jaccards),
        "max_pairwise_jaccard": max(jaccards),
        "common_signature_count": common_signature_count,
        "common_invariant_count": common_invariant_count,
        "signature_groups": list(signature_groups.values()),
        "invariant_groups": list(invariant_groups.values()),
        "dual_contribution_by_leaf": {str(key): value for key, value in sorted(duals.items())},
        "global_dominance_lookup": global_slacks,
        "support_signatures": signatures,
        "pairwise_support_comparisons": pairwise_rows,
        "gates": {
            "high_jaccard": HIGH_JACCARD,
            "min_common_core": MIN_COMMON_CORE,
            "min_signature_match": MIN_SIGNATURE_MATCH,
            "min_invariant_match": MIN_INVARIANT_MATCH,
        },
        "failure_reasons": failure_reasons,
        "status": status,
    }
    OUT.write_text(json.dumps(jsonable(report), indent=2) + "\n")
    print(json.dumps({key: value for key, value in report.items() if key not in {"support_signatures", "pairwise_support_comparisons"}}, indent=2))


if __name__ == "__main__":
    main()
