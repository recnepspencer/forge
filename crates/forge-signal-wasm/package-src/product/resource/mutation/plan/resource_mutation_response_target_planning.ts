import {
  createMutationResponseFallbackDetail,
  isExactMutationResponseExecutionKind,
} from "../resource_mutation_response_target_execution.js";
import {
  createPlannedMutationResponseTargetIdentityDigest,
} from "./resource_mutation_response_plan_digests.js";

function createPublicMutationResponseTarget(plannedTarget, createPublicReconciliationDigest) {
  return Object.freeze({
    targetId: plannedTarget.target.targetId,
    family: plannedTarget.target.family,
    line: Object.freeze({
      familyKind: plannedTarget.target.family.kind,
      familyId: plannedTarget.target.family.familyId,
      canonicalKey: plannedTarget.lineIdentity.canonicalKey,
      runtimeLineId: plannedTarget.lineIdentity.runtimeLineId,
      residency: plannedTarget.lineIdentity.residency,
    }),
    fallback: Object.freeze({
      kind: plannedTarget.target.fallback,
      detail: createMutationResponseFallbackDetail(
        plannedTarget.target,
        plannedTarget.lineIdentity,
      ),
    }),
    submittedTarget: plannedTarget.submittedTarget,
    reconciliation:
      plannedTarget.target.reconciliation === null
        ? null
        : createPublicReconciliationDigest(plannedTarget.target.reconciliation),
    execution: plannedTarget.execution.artifact,
    targetDigest: createPlannedMutationResponseTargetIdentityDigest(
      plannedTarget.target,
      plannedTarget.lineIdentity,
      plannedTarget.execution.artifact,
    ),
  });
}

function readMutationResponsePartialAdmission(plannedTargets, reconciliationAtomicity) {
  const hasPartialFallback = plannedTargets.some((target) =>
    target.execution.artifact.kind === "fallback"
    && target.execution.artifact.partial !== null);
  if (!hasPartialFallback) {
    return "notNeeded";
  }
  return reconciliationAtomicity === "partialAllowed" ? "admitted" : "denied";
}

function applyMutationResponsePartialAdmission(
  plannedTargets,
  reconciliationAtomicity,
  partialAdmission,
) {
  if (
    partialAdmission !== "denied"
    || reconciliationAtomicity !== "allOrNone"
  ) {
    return plannedTargets;
  }
  const blockingTarget = plannedTargets.find((target) =>
    target.execution.artifact.kind === "fallback"
    && target.execution.artifact.partial !== null);
  return Object.freeze(
    plannedTargets.map((target) =>
      target.execution.artifact.kind === "fallback"
      || isExactMutationResponseExecutionKind(target.execution.artifact.kind) === false
        ? target
        : Object.freeze({
          ...target,
          execution: Object.freeze({
            preparedExecution: Object.freeze({
              kind: "fallback",
              targetId: target.target.targetId,
              fallback: "partialReconciliation",
            }),
            artifact: Object.freeze({
              artifactId: `${target.target.targetId}:fallback`,
              targetId: target.target.targetId,
              kind: "fallback",
              fallback: "partialReconciliation",
              familyKind: target.target.family.kind,
              familyId: target.target.family.familyId,
              canonicalKey: target.lineIdentity.canonicalKey,
              runtimeLineId: target.lineIdentity.runtimeLineId,
              residency: target.lineIdentity.residency,
              submittedTarget: target.submittedTarget,
              staleness: null,
              partial: null,
              detail: [
                `${target.target.family.kind} ${target.target.family.familyId}`,
                `target ${target.target.targetId}`,
                `stays in partialReconciliation posture because mutation.atomicity=allOrNone does not admit sibling partial target ${blockingTarget?.target.targetId ?? "unknown"}`,
              ].join(" "),
            }),
          }),
        })),
  );
}

export {
  applyMutationResponsePartialAdmission,
  createPublicMutationResponseTarget,
  readMutationResponsePartialAdmission,
};
