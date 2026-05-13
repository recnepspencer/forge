import { resourceDelivery } from "../delivery/resource_delivery.js";
import {
  readMutationResponseTargetStaleness,
} from "./resource_mutation_response_target_basis.js";

function createMutationResponseTargetExecution(
  target,
  targetParams,
  lineIdentity,
  responseValue,
  planId,
  submittedTarget,
) {
  if (
    target.reconciliation === null
    || lineIdentity.residency !== "resident"
  ) {
    return createFallbackMutationResponseExecutionArtifact(
      target,
      lineIdentity,
      submittedTarget,
      null,
    );
  }
  const targetMaterialization = target.lookupResidentTargetMaterialization(targetParams);
  if (targetMaterialization === null) {
    return createFallbackMutationResponseExecutionArtifact(
      target,
      lineIdentity,
      submittedTarget,
      null,
    );
  }
  const staleness = readMutationResponseTargetStaleness(
    lineIdentity,
    targetMaterialization,
    submittedTarget,
  );
  if (staleness !== null) {
    return createFallbackMutationResponseExecutionArtifact(
      target,
      lineIdentity,
      submittedTarget,
      staleness,
    );
  }
  const packetId = `${planId}:${target.targetId}`;
  const currentBasisId = targetMaterialization.requestState.currentBasisId();
  const delivery =
    target.reconciliation.kind === "replace"
      ? resourceDelivery.replace({
          packetId,
          basisId: null,
          nextBasisId: currentBasisId,
          nextValue: responseValue,
        })
      : resourceDelivery.patch({
          packetId,
          basisId: null,
          nextBasisId: currentBasisId,
          patch: target.reconciliation.createPatch(responseValue),
        });
  const artifact = createExactReconciliationExecutionArtifact(
    target,
    lineIdentity,
    target.reconciliation,
    packetId,
    responseValue,
    submittedTarget,
  );
  return Object.freeze({
    preparedExecution: Object.freeze({
      kind: artifact.kind,
      targetId: target.targetId,
      targetMaterialization,
      delivery,
    }),
    artifact,
  });
}

function createFallbackMutationResponseExecutionArtifact(
  target,
  lineIdentity,
  submittedTarget,
  staleness,
) {
  return Object.freeze({
    preparedExecution: Object.freeze({
      kind: "fallback",
      targetId: target.targetId,
      fallback: target.fallback,
    }),
    artifact: Object.freeze({
      artifactId: `${target.targetId}:fallback`,
      targetId: target.targetId,
      kind: "fallback",
      fallback: target.fallback,
      familyKind: target.family.kind,
      familyId: target.family.familyId,
      canonicalKey: lineIdentity.canonicalKey,
      runtimeLineId: lineIdentity.runtimeLineId,
      residency: lineIdentity.residency,
      submittedTarget,
      staleness,
      detail: staleness?.detail
        ?? createMutationResponseFallbackDetail(target, lineIdentity),
    }),
  });
}

function createExactReconciliationExecutionArtifact(
  target,
  lineIdentity,
  reconciliation,
  packetId,
  responseValue,
  submittedTarget,
) {
  return Object.freeze({
    artifactId: `${target.targetId}:${reconciliation.executionKind}`,
    targetId: target.targetId,
    kind: reconciliation.executionKind,
    scope: readReconciliationScope(reconciliation),
    familyKind: target.family.kind,
    familyId: target.family.familyId,
    canonicalKey: lineIdentity.canonicalKey,
    runtimeLineId: lineIdentity.runtimeLineId,
    residency: lineIdentity.residency,
    packetId,
    submittedTarget,
    staleness: null,
    itemId: readReconciliationItemId(reconciliation, responseValue),
    field: "field" in reconciliation ? reconciliation.field : null,
    region: "region" in reconciliation ? reconciliation.region : null,
    path: "path" in reconciliation ? reconciliation.path : null,
    summary: "summary" in reconciliation ? reconciliation.summary : null,
    summaryScope:
      "summaryScope" in reconciliation ? reconciliation.summaryScope : null,
  });
}

function readReconciliationScope(reconciliation) {
  if (reconciliation.kind === "replace") {
    return "line";
  }
  return reconciliation.kind;
}

function readReconciliationItemId(reconciliation, responseValue) {
  if (reconciliation.kind !== "item") {
    return null;
  }
  return reconciliation.readItemId(responseValue);
}

function createMutationResponseFallbackDetail(target, lineIdentity) {
  return [
    `${target.family.kind} ${target.family.familyId}`,
    `line ${lineIdentity.canonicalKey}`,
    `stays in ${target.fallback} posture until a later mutation-response phase admits exact reconciliation`,
  ].join(" ");
}

function isExactMutationResponseExecutionKind(kind) {
  return (
    kind === "exactDetail"
    || kind === "exactCollectionItem"
    || kind === "exactSummary"
  );
}

export {
  createMutationResponseFallbackDetail,
  createMutationResponseTargetExecution,
  isExactMutationResponseExecutionKind,
};
