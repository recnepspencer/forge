import { requireRuntimeIssuedResourceEffectEnvelope } from "../effects/resource_effect_envelope.js";
import {
  bindResourceEffectMergePolicy,
  createResourceEffectMergePolicyBinding,
} from "./resource_effect_merge_policy_binding.js";
import {
  createNativeMergeConflictSummary,
  createResourceEffectMergeExecutionArtifact,
  createResourceEffectRebaseArtifact,
} from "./resource_effect_merge_rebase_artifact.js";

function createResourceBranchNamespace(rawSignals) {
  return Object.freeze({
    planMerge(request) {
      return planResourceMerge(rawSignals, request);
    },
    planEffectMerge(request) {
      return planResourceEffectMerge(rawSignals, request);
    },
    mergeEffect(request) {
      return mergeResourceEffect(rawSignals, request);
    },
  });
}

function planResourceEffectMerge(rawSignals, request) {
  try {
    const mergeRequest = normalizeEffectMergeRequest(
      request,
      "resource.branch.planEffectMerge(...)",
      "planning",
    );
    const mergePlan = planResourceMerge(rawSignals, mergeRequest.merge);
    if (mergePlan.kind === "denied") {
      return mergePlan;
    }
    requireEffectMergeBranchBinding(mergePlan, mergeRequest.effect, "planning");
    return createEffectMergePlanSummary(
      mergePlan,
      mergeRequest.effect,
    );
  } catch (error) {
    return Object.freeze({
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail: normalizeErrorDetail(error),
    });
  }
}

function mergeResourceEffect(rawSignals, request) {
  try {
    const mergeRequest = normalizeEffectMergeRequest(
      request,
      "resource.branch.mergeEffect(...)",
      "execution",
    );
    const normalizedMerge = normalizeMergePreviewRequest(
      mergeRequest.merge,
      "resource.branch.mergeEffect(...).merge",
    );
    requireEffectMergeSourceRequestBinding(normalizedMerge, mergeRequest.effect);
    const mergeResult = mergeResource(rawSignals, normalizedMerge);
    if (mergeResult.kind === "denied") {
      return mergeResult;
    }
    requireEffectMergeBranchBinding(mergeResult, mergeRequest.effect, "execution");
    return createEffectMergeExecutionSummary(
      mergeResult,
      mergeRequest.effect,
    );
  } catch (error) {
    return Object.freeze({
      kind: "denied",
      reason: "resourceEffectMergeUnavailable",
      detail: normalizeErrorDetail(error),
    });
  }
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

function mergeResource(rawSignals, request) {
  try {
    const envelope = rawSignals.history()
      .merge_branches_policy_preview_with_proof(
        normalizeMergePreviewRequest(
          request,
          "history.merge_branches_policy_preview_with_proof",
        ),
      );
    return createMergeExecutionSummary(envelope);
  } catch (error) {
    return Object.freeze({
      kind: "denied",
      reason: "mergeExecutionUnavailable",
      detail: normalizeErrorDetail(error),
    });
  }
}

function normalizeEffectMergeRequest(request, operation, phase) {
  if (!request || typeof request !== "object" || Array.isArray(request)) {
    throw new TypeError(`${operation} expects a request object`);
  }
  if (!request.merge || typeof request.merge !== "object" || Array.isArray(request.merge)) {
    throw new TypeError(`${operation} requires merge input`);
  }
  const effect = requireMergeResourceEffect(request.effect, operation, phase);
  return Object.freeze({
    merge: bindResourceEffectMergePolicy(request.merge, effect, operation),
    effect,
  });
}

function requireMergeResourceEffect(effect, operation, phase) {
  if (!effect || typeof effect !== "object" || Array.isArray(effect)) {
    throw new TypeError(`${operation} requires a resource effect envelope`);
  }
  if (!requireRuntimeIssuedResourceEffectEnvelope(effect)) {
    throw new TypeError(
      `${operation} requires a runtime-issued resource effect envelope`,
    );
  }
  if (effect.version !== "resource-effect-envelope-v1") {
    throw new TypeError(
      `${operation} requires a supported resource effect envelope version`,
    );
  }
  if (typeof effect.effectId !== "string" || effect.effectId.length === 0) {
    throw new TypeError(`resource branch effect merge ${phase} requires an effect id`);
  }
  if (!effect.line || typeof effect.line !== "object") {
    throw new TypeError(
      `resource branch effect merge ${phase} requires line identity evidence`,
    );
  }
  if (!effect.locus || typeof effect.locus !== "object" || typeof effect.locus.kind !== "string") {
    throw new TypeError(
      `resource branch effect merge ${phase} requires a semantic resource locus`,
    );
  }
  if (effect.profile?.rebase !== "nativeMergePlan") {
    throw new TypeError(
      `resource branch effect merge ${phase} requires an effect profile with nativeMergePlan rebase posture`,
    );
  }
  return effect;
}

function requireEffectMergeBranchBinding(mergePlan, effect, phase) {
  const effectBranchId = effect.optimistic?.branchId;
  if (!Number.isSafeInteger(effectBranchId) || effectBranchId < 0) {
    throw new TypeError(
      `resource branch effect merge ${phase} requires optimistic branch evidence`,
    );
  }
  if (mergePlan.sourceBranchId !== effectBranchId) {
    throw new TypeError(
      `resource branch effect merge ${phase} requires merge source branch "${mergePlan.sourceBranchId}" to match effect optimistic branch "${effectBranchId}"`,
    );
  }
}

function requireEffectMergeSourceRequestBinding(merge, effect) {
  const effectBranchId = effect.optimistic?.branchId;
  if (!Number.isSafeInteger(effectBranchId) || effectBranchId < 0) {
    throw new TypeError(
      "resource branch effect merge execution requires optimistic branch evidence",
    );
  }
  if (merge.source_branch_id !== effectBranchId) {
    throw new TypeError(
      `resource branch effect merge execution requires merge source branch "${merge.source_branch_id}" to match effect optimistic branch "${effectBranchId}" before native merge execution`,
    );
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
    selectedSemantics: createSelectedSemanticsSummary(plan.selected_semantics),
    breadth: Object.freeze({
      nodeMapCount: plan.node_map.length,
      nodePlanCount: plan.node_plan.length,
      adoptionPlanCount: plan.adoption_core.length,
      conflictRecordCount: plan.resolution_plan?.records.length ?? 0,
    }),
    conflicts: createNativeMergeConflictSummary(plan.resolution_plan),
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

function createMergeExecutionSummary(envelope) {
  const result = envelope.result;
  const proof = envelope.proof;
  return Object.freeze({
    kind: "merged",
    sourceBranchId: result.source_branch,
    targetBranchId: result.target_branch,
    mergeKind: result.merge_kind,
    selectedSemantics: createSelectedSemanticsSummary(result.selected_semantics),
    breadth: Object.freeze({
      recordCount: result.records.length,
      sourceOnlyCount: result.counters.source_only_count,
      targetOnlyCount: result.counters.target_only_count,
      conflictRecordCount: result.resolution_plan?.records.length ?? 0,
    }),
    conflicts: createNativeMergeConflictSummary(result.resolution_plan),
    proof: Object.freeze({
      proofSchemaVersion: proof.proofSchemaVersion,
      resultDigest: proof.resultDigest,
      semanticsDigest: proof.semanticsDigest,
      lineageDigest: proof.lineageDigest,
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

function createSelectedSemanticsSummary(selectedSemantics) {
  return Object.freeze({
    strategy: selectedSemantics.strategy_name,
    mergeBase: selectedSemantics.merge_base_name,
    conflictPolicy: selectedSemantics.conflict_policy_name,
    conflictIsolation: selectedSemantics.conflict_isolation_name,
    identityMatcher: selectedSemantics.identity_matcher_name,
    sourceOnlyPolicy: selectedSemantics.source_only_policy_name,
    deletionPolicy: selectedSemantics.deletion_policy_name,
  });
}

function createEffectMergePlanSummary(mergePlan, effect) {
  const policyBinding = createResourceEffectMergePolicyBinding(effect);
  return Object.freeze({
    ...mergePlan,
    resourceEffect: Object.freeze({
      effectId: effect.effectId,
      provenance: effect.provenance,
      family: effect.family,
      line: effect.line,
      locus: effect.locus,
      topology: effect.locusProof?.topology ?? null,
      effectLocus: effect.locusProof?.locus ?? effect.locus.kind,
      rebase: effect.profile.rebase,
      conflictIsolation: mergePlan.selectedSemantics.conflictIsolation,
      policyBinding,
      rebaseArtifact: createResourceEffectRebaseArtifact(mergePlan, effect, policyBinding),
      proof: Object.freeze({
        planDigest: mergePlan.proof.planDigest,
        semanticsDigest: mergePlan.proof.semanticsDigest,
        effectLocusDigest: effect.locusProof?.effectLocusDigest ?? null,
        compiledLensDigest: effect.locusProof?.compiledLensDigest ?? null,
      }),
    }),
  });
}

function createEffectMergeExecutionSummary(mergeResult, effect) {
  const policyBinding = createResourceEffectMergePolicyBinding(effect);
  return Object.freeze({
    ...mergeResult,
    resourceEffect: Object.freeze({
      effectId: effect.effectId,
      provenance: effect.provenance,
      family: effect.family,
      line: effect.line,
      locus: effect.locus,
      topology: effect.locusProof?.topology ?? null,
      effectLocus: effect.locusProof?.locus ?? effect.locus.kind,
      rebase: effect.profile.rebase,
      conflictIsolation: mergeResult.selectedSemantics.conflictIsolation,
      policyBinding,
      mergeArtifact: createResourceEffectMergeExecutionArtifact(
        mergeResult,
        effect,
        policyBinding,
      ),
      proof: Object.freeze({
        resultDigest: mergeResult.proof.resultDigest,
        semanticsDigest: mergeResult.proof.semanticsDigest,
        lineageDigest: mergeResult.proof.lineageDigest,
        effectLocusDigest: effect.locusProof?.effectLocusDigest ?? null,
        compiledLensDigest: effect.locusProof?.compiledLensDigest ?? null,
      }),
    }),
  });
}

export { createResourceBranchNamespace };
