import { readResourceLineHandle } from "../sources/form_sources.js";
import { readResourceLineProof } from "../sources/resource_line_proof.js";
import { stageResourcePatchLowering } from "./resource_patch_lowering.js";
import { cloneFormValue, stableValueDigest } from "../values/value_paths.js";

export function executeResourceBackedAction(source, fieldDeclarations, plan, recovery) {
  if (!hasExecutableResourceLowering(plan.resourceAction.source)) {
    return null;
  }
  const line = readResourceLineHandle(source);
  if (isLifecycleResourceAction(plan.resourceAction.source)) {
    return executeLifecycleResourceAction(plan, line);
  }
  if (isRecoveryResourceAction(plan.resourceAction.source)) {
    return executeRecoveryResourceAction(plan, recovery, source);
  }
  return executePatchResourceAction(line, fieldDeclarations, plan);
}

function hasExecutableResourceLowering(resourceActionSource) {
  return (
    resourceActionSource === "submitPatchPlan"
    || resourceActionSource === "declaredPatchPlan"
    || resourceActionSource === "declaredRefresh"
    || resourceActionSource === "declaredRevalidate"
    || resourceActionSource === "declaredReplayExact"
    || resourceActionSource === "declaredRestoreExact"
    || resourceActionSource === "declaredRollbackLastEffect"
  );
}

function isLifecycleResourceAction(resourceActionSource) {
  return resourceActionSource === "declaredRefresh" || resourceActionSource === "declaredRevalidate";
}

function isRecoveryResourceAction(resourceActionSource) {
  return (
    resourceActionSource === "declaredReplayExact"
    || resourceActionSource === "declaredRestoreExact"
    || resourceActionSource === "declaredRollbackLastEffect"
  );
}

function executeLifecycleResourceAction(plan, line) {
  if (line === null) {
    return deniedResourceAction(
      plan.id,
      "declared resource-line lifecycle action requires a resource line",
    );
  }
  const operation = plan.resourceAction.source === "declaredRefresh" ? "refresh" : "revalidate";
  const status = operation === "refresh" ? line.refresh() : line.revalidate();
  const freshness = line.freshness();
  const resourceLifecycle = Object.freeze({
    sourceKind: "resourceLine",
    operation,
    status,
    freshness,
    digest: stableValueDigest({
      operation,
      status,
      freshness,
    }),
  });
  const proof = readResourceLineExecutionProof(line);
  return Object.freeze({
    resultKind: "fulfilled",
    reason: `resource-line action "${plan.id}" invoked resource line ${operation}`,
    effectStarted: true,
    canonicalValue: undefined,
    resourceSubmission: null,
    resourceSettlement: proof.settlement,
    resourceLifecycle,
    resourceRecovery: null,
  });
}

function executeRecoveryResourceAction(plan, recovery, source) {
  const actionId = plan.id;
  const resourceRecovery = recoverResourceAction(
    plan.resourceAction.source,
    actionId,
    recovery,
    {
      form: recovery.form,
      source,
      writeDraft: recovery.writeDraft,
    },
  );
  if (resourceRecovery.resultKind === "unavailable") {
    return deniedResourceAction(actionId, resourceRecovery.reason);
  }
  return Object.freeze({
    resultKind: "fulfilled",
    reason: `resource-line action "${actionId}" completed through resource recovery truth`,
    effectStarted: true,
    canonicalValue: undefined,
    resourceSubmission: null,
    resourceSettlement: null,
    resourceLifecycle: null,
    resourceRecovery,
  });
}

function recoverResourceAction(resourceActionSource, actionId, recovery, context) {
  if (resourceActionSource === "declaredReplayExact") {
    return recovery.replayExactResourceSource(context, {
      reason: `resource-line action "${actionId}" requested exact source replay`,
    });
  }
  if (resourceActionSource === "declaredRestoreExact") {
    return recovery.restoreExactResourceSource(context, {
      reason: `resource-line action "${actionId}" requested exact branch restore`,
    });
  }
  return recovery.rollbackLastResourceEffect(context, {
    reason: `resource-line action "${actionId}" requested rollback of the last resource effect`,
  });
}

function executePatchResourceAction(line, fieldDeclarations, plan) {
  if (!isPatchCapableResourceLine(line)) {
    return deniedPatchCapableResourceLine(plan.id);
  }
  if (plan.patch.empty && plan.patch.replacement === null) {
    return deniedEmptyPatchResourceAction(plan.id);
  }
  const staged = stageResourcePatchLowering(line, fieldDeclarations, plan.patch, plan.id);
  if (staged.kind === "denied") {
    return deniedResourceAction(plan.id, staged.reason);
  }
  const lowered = applyLoweredPatchPlans(line, staged.loweredPlans);
  if (isPromiseLike(lowered)) {
    return lowered.then((settled) =>
      createPatchResourceExecution(line, plan, settled));
  }
  return createPatchResourceExecution(line, plan, lowered);
}

function createPatchResourceExecution(line, plan, lowered) {
  const proof = readResourceLineExecutionProof(line);
  const canonicalValue = cloneFormValue(line.value());
  const resourceSubmission = Object.freeze({
    sourceKind: "resourceLine",
    patchCount: lowered.length,
    patches: Object.freeze(lowered),
    effectProfile: proof.effectProfile,
    rollback: proof.rollback,
    visibleSelection: proof.visibleSelection,
    mutationResponse: proof.mutationResponse,
    verification: proof.verification,
    digest: stableValueDigest({
      patches: lowered,
      effectProfile: proof.effectProfile,
      rollback: proof.rollback,
      visibleSelection: proof.visibleSelection,
      mutationResponse: proof.mutationResponse,
      verification: proof.verification,
    }),
  });
  return Object.freeze({
    resultKind: "fulfilled",
    reason: `resource-line action "${plan.id}" applied through resource line patch effects`,
    effectStarted: true,
    canonicalValue,
    resourceSubmission,
    resourceSettlement: proof.settlement,
    resourceLifecycle: null,
    resourceRecovery: null,
  });
}

function isPatchCapableResourceLine(line) {
  return line !== null && typeof line.patch === "function" && typeof line.reconciliation === "function";
}

function deniedPatchCapableResourceLine(actionId) {
  return deniedResourceAction(
    actionId,
    actionId === "submit"
      ? "resource-line submit requires a patch-capable resource line"
      : "declared resource-line action requires a patch-capable resource line",
  );
}

function deniedEmptyPatchResourceAction(actionId) {
  return deniedResourceAction(
    actionId,
    `resource-line action "${actionId}" requires a non-empty lowered resource patch plan`,
  );
}

function applyLoweredPatchPlans(line, loweredPlans) {
  const lowered = [];
  const applyAt = (position) => {
    if (position >= loweredPlans.length) {
      return Object.freeze(lowered);
    }
    const loweredPlan = loweredPlans[position];
    const patchResult = line.patch(loweredPlan.patch);
    if (isPromiseLike(patchResult)) {
      return patchResult.then((settled) => {
        recordLoweredPatch(line, lowered, loweredPlan, settled);
        return applyAt(position + 1);
      });
    }
    recordLoweredPatch(line, lowered, loweredPlan, patchResult);
    return applyAt(position + 1);
  };
  return applyAt(0);
}

function recordLoweredPatch(line, lowered, loweredPlan, patchResult) {
    const latest = line.diagnosticsSummary().latest;
    lowered.push(Object.freeze({
      field: loweredPlan.field,
      path: loweredPlan.path,
      locusKind: loweredPlan.locusKind,
      locus: loweredPlan.locus,
      operationKind: loweredPlan.operationKind,
      patchKind: loweredPlan.patchKind,
      patchResultKind: patchResult.kind,
      patchScope: patchResult.scope,
      effectDigest: latest.effect === null ? null : stableValueDigest(latest.effect),
      basisId: latest.basisCurrentId ?? null,
    }));
}

function isPromiseLike(value) {
  return value !== null
    && (typeof value === "object" || typeof value === "function")
    && typeof value.then === "function";
}

function readResourceLineExecutionProof(line) {
  return readResourceLineProof(
    line,
    line.request(),
    line.summary(),
    line.status(),
    line.freshness(),
    line.mutationResponse(),
  );
}

function deniedResourceAction(actionId, reason) {
  return Object.freeze({
    resultKind: "denied",
    reason,
    effectStarted: false,
    resourceSubmission: null,
    resourceSettlement: null,
    resourceRecovery: null,
  });
}
