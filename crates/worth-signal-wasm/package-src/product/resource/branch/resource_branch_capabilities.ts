import { requireRuntimeIssuedResourceEffectEnvelope } from "../effects/resource_effect_envelope.js";
import { assertProjectionCannotAuthorizeCanonicalMerge } from "../effects/projection/resource_optimistic_projection.js";
import {
  bindResourceEffectMergePolicy,
} from "./resource_effect_merge_policy_binding.js";
import {
  createEffectMergeExecutionSummary,
  createEffectMergePlanSummary,
  createMergeExecutionSummary,
  createMergePlanSummary,
} from "./resource_branch_merge_summaries.js";

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
    return mapMaybePromise(
      mergePlan,
      (resolvedMergePlan) => {
        if (resolvedMergePlan.kind === "denied") {
          return resolvedMergePlan;
        }
        requireEffectMergeBranchBinding(resolvedMergePlan, mergeRequest.effect, "planning");
        return createEffectMergePlanSummary(
          resolvedMergePlan,
          mergeRequest.effect,
        );
      },
      (error) => Object.freeze({
        kind: "denied",
        reason: "resourceEffectMergeUnavailable",
        detail: normalizeErrorDetail(error),
      }),
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
    return mapMaybePromise(
      mergeResult,
      (resolvedMergeResult) => {
        if (resolvedMergeResult.kind === "denied") {
          return resolvedMergeResult;
        }
        requireEffectMergeBranchBinding(resolvedMergeResult, mergeRequest.effect, "execution");
        return createEffectMergeExecutionSummary(
          resolvedMergeResult,
          mergeRequest.effect,
        );
      },
      (error) => Object.freeze({
        kind: "denied",
        reason: "resourceEffectMergeUnavailable",
        detail: normalizeErrorDetail(error),
      }),
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
    return mapMaybePromise(
      envelope,
      (resolvedEnvelope) => createMergePlanSummary(resolvedEnvelope),
      (error) => Object.freeze({
        kind: "denied",
        reason: "mergePlanUnavailable",
        detail: normalizeErrorDetail(error),
      }),
    );
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
    return mapMaybePromise(
      envelope,
      (resolvedEnvelope) => createMergeExecutionSummary(resolvedEnvelope),
      (error) => Object.freeze({
        kind: "denied",
        reason: "mergeExecutionUnavailable",
        detail: normalizeErrorDetail(error),
      }),
    );
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
  assertProjectionCannotAuthorizeCanonicalMerge(value);
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

function mapMaybePromise(value, mapValue, mapError) {
  if (isPromiseLike(value)) {
    return Promise.resolve(value).then(mapValue, mapError);
  }
  return mapValue(value);
}

function isPromiseLike(value) {
  return value !== null
    && (typeof value === "object" || typeof value === "function")
    && typeof value.then === "function";
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

export { createResourceBranchNamespace };
