function countEntries(value, path) {
  let current = value;
  for (const segment of path) {
    current = current?.[segment];
  }
  return Array.isArray(current) ? current.length : 0;
}

export function summarizeMergePlan(plan) {
  return {
    sourceBranchId: plan.source_branch_id ?? null,
    targetBranchId: plan.target_branch_id ?? null,
    mergeKind: plan.merge_kind ?? null,
    divergence: plan.divergence ?? null,
    mergeStrategy: plan.merge_strategy ?? null,
    sourceSnapshotId: plan.source_snapshot_id ?? null,
    targetSnapshotIdBefore: plan.target_snapshot_id_before ?? null,
    candidateCount: countEntries(plan, ["planned_candidates", "nodes"]),
    sharedNodeCount: countEntries(plan, ["proof_minimal_overlap", "shared_nodes"]),
    expandedNodeCount: countEntries(plan, ["conservative_overlap", "expanded_nodes"]),
    supportNodeCount: countEntries(plan, ["conservative_overlap", "support_nodes"]),
    nodePlanCount: countEntries(plan, ["node_plan"]),
    adoptionCount: countEntries(plan, ["adoption_core"]),
    hasResolutionPlan: !!plan.resolution_plan
  };
}

export function summarizeMergeResult(result) {
  const counters = result.counters ?? {};
  return {
    sourceBranchId: result.source_branch ?? null,
    targetBranchId: result.target_branch ?? null,
    mergeKind: result.merge_kind ?? null,
    divergence: result.divergence ?? null,
    mergeStrategy: result.merge_strategy ?? null,
    mergedSnapshotId: result.merged_snapshot_id ?? null,
    targetSnapshotIdBefore: result.target_snapshot_id_before ?? null,
    targetSnapshotIdAfter: result.target_snapshot_id_after ?? null,
    sourceSnapshotId: result.source_snapshot_id ?? null,
    recordCount: Array.isArray(result.records) ? result.records.length : 0,
    adoptedCount: counters.adopted_count ?? 0,
    introducedCount: counters.introduced_node_count ?? 0,
    replacedCount: counters.replaced_count ?? 0,
    preservedTargetCount: counters.preserved_target_count ?? 0,
    equivalentUnchangedCount: counters.equivalent_unchanged_count ?? 0,
    skippedNonAdoptableCount: counters.skipped_non_adoptable_count ?? 0,
    conflictCount: Array.isArray(result.records)
      ? result.records.reduce(
          (total, record) => total + (record.resolved_conflict_kinds?.length ?? 0),
          0
        )
      : 0,
    hasResolutionPlan: !!result.resolution_plan
  };
}
