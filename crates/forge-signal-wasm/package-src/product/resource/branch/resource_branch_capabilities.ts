function createResourceBranchNamespace(rawSignals) {
  return Object.freeze({
    planMerge(request) {
      return planResourceMerge(rawSignals, request);
    },
  });
}

function planResourceMerge(rawSignals, request) {
  try {
    const envelope = rawSignals.history()
      .plan_merge_policy_preview_with_proof(
        normalizeMergePreviewRequest(
          request,
          "history.plan_merge_policy_preview_with_proof",
        ),
      );
    return createMergePlanSummary(envelope);
  } catch (error) {
    return Object.freeze({
      kind: "denied",
      reason: "mergePlanUnavailable",
      detail: normalizeErrorDetail(error),
    });
  }
}

function normalizeMergePreviewRequest(request, operation) {
  if (!request || typeof request !== "object" || Array.isArray(request)) {
    throw new TypeError(`${operation} expects a merge preview request object`);
  }
  return {
    ...request,
    source_branch_id: normalizePreviewBranchId(
      request.source_branch_id,
      `${operation}.source_branch_id`,
    ),
    target_branch_id: normalizePreviewBranchId(
      request.target_branch_id,
      `${operation}.target_branch_id`,
    ),
  };
}

function normalizePreviewBranchId(value, operation) {
  if (typeof value === "bigint") {
    if (value < 0n) {
      throw new RangeError(`${operation} expects a non-negative branch id`);
    }
    if (value > BigInt(Number.MAX_SAFE_INTEGER)) {
      throw new RangeError(
        `${operation} exceeds the safe integer range supported by merge preview requests`,
      );
    }
    return Number(value);
  }
  if (!Number.isSafeInteger(value) || value < 0) {
    throw new TypeError(
      `${operation} expects a non-negative safe integer branch id`,
    );
  }
  return value;
}

function normalizeErrorDetail(error) {
  if (error instanceof Error) {
    return error.message;
  }
  if (error && typeof error === "object" && typeof error.message === "string") {
    return error.message;
  }
  return String(error);
}

function createMergePlanSummary(envelope) {
  const plan = envelope.plan;
  const proof = envelope.proof;
  return Object.freeze({
    kind: "planned",
    sourceBranchId: plan.source_branch_id,
    targetBranchId: plan.target_branch_id,
    mergeKind: plan.merge_kind,
    selectedSemantics: Object.freeze({
      strategy: plan.selected_semantics.strategy_name,
      mergeBase: plan.selected_semantics.merge_base_name,
      conflictPolicy: plan.selected_semantics.conflict_policy_name,
      conflictIsolation: plan.selected_semantics.conflict_isolation_name,
      identityMatcher: plan.selected_semantics.identity_matcher_name,
      sourceOnlyPolicy: plan.selected_semantics.source_only_policy_name,
      deletionPolicy: plan.selected_semantics.deletion_policy_name,
    }),
    breadth: Object.freeze({
      nodeMapCount: plan.node_map.length,
      nodePlanCount: plan.node_plan.length,
      adoptionPlanCount: plan.adoption_core.length,
      conflictRecordCount: plan.resolution_plan?.records.length ?? 0,
    }),
    proof: Object.freeze({
      proofSchemaVersion: proof.proofSchemaVersion,
      planDigest: proof.planDigest,
      semanticsDigest: proof.semanticsDigest,
      selectedStrategyDigest: proof.selectedStrategyDigest,
      selectedMergeBaseDigest: proof.selectedMergeBaseDigest,
      selectedConflictPolicyDigest: proof.selectedConflictPolicyDigest,
      selectedConflictIsolationDigest:
        proof.selectedConflictIsolationDigest,
      selectedIdentityMatcherDigest: proof.selectedIdentityMatcherDigest,
      selectedSourceOnlyPolicyDigest: proof.selectedSourceOnlyPolicyDigest,
      selectedDeletionPolicyDigest: proof.selectedDeletionPolicyDigest,
    }),
  });
}

export { createResourceBranchNamespace };
