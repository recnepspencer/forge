import { resourceDelivery } from "../delivery/resource_delivery.js";
import {
  readMutationResponseTargetStaleness,
} from "./resource_mutation_response_target_basis.js";

function createMutationResponseTargetExecution(
  target,
  targetParams,
  mutationParams,
  lineIdentity,
  responseValue,
  planId,
  submittedTarget,
) {
  if (
    target.reconciliation === null
    || lineIdentity.residency !== "resident"
  ) {
    const createdTargetMaterialization = tryCreateDeclaredTargetMaterialization(
      target,
      targetParams,
      lineIdentity,
      responseValue,
    );
    if (createdTargetMaterialization === null) {
      return createFallbackMutationResponseExecutionArtifact(
        target,
        lineIdentity,
        submittedTarget,
        null,
        null,
      );
    }
    return createExactMutationResponseTargetExecution(
      target,
      targetParams,
      mutationParams,
      submittedTarget,
      createdTargetMaterialization,
      readResidentLineIdentity(createdTargetMaterialization),
      responseValue,
      planId,
      true,
    );
  }
  const targetMaterialization = target.lookupResidentTargetMaterialization(targetParams);
  if (targetMaterialization === null) {
    return createFallbackMutationResponseExecutionArtifact(
      target,
      lineIdentity,
      submittedTarget,
      null,
      null,
    );
  }
  return createExactMutationResponseTargetExecution(
    target,
    targetParams,
    mutationParams,
    submittedTarget,
    targetMaterialization,
    lineIdentity,
    responseValue,
    planId,
    false,
  );
}

function createExactMutationResponseTargetExecution(
  target,
  targetParams,
  mutationParams,
  submittedTarget,
  targetMaterialization,
  lineIdentity,
  responseValue,
  planId,
  ignoreSubmittedDeclaredStaleness,
) {
  const staleness = readMutationResponseTargetStaleness(
    lineIdentity,
    targetMaterialization,
    ignoreSubmittedDeclaredStaleness ? null : submittedTarget,
  );
  if (staleness !== null) {
    return createFallbackMutationResponseExecutionArtifact(
      target,
      lineIdentity,
      submittedTarget,
      staleness,
      null,
    );
  }
  const extractedPatchValue = extractMutationResponsePatchValue(
    target.reconciliation,
    responseValue,
  );
  if (extractedPatchValue.kind === "missing") {
    return createFallbackMutationResponseExecutionArtifact(
      target,
      lineIdentity,
      submittedTarget,
      null,
      createMissingResponseFieldPartialArtifact(extractedPatchValue.field),
    );
  }
  const packetId = `${planId}:${target.targetId}`;
  const currentBasisId = targetMaterialization.requestState.currentBasisId();
  const delivery = createMutationResponseTargetDelivery(
    target.reconciliation,
    packetId,
    currentBasisId,
    responseValue,
    mutationParams,
    extractedPatchValue.kind === "present" ? extractedPatchValue.value : null,
  );
  const artifact = createExactReconciliationExecutionArtifact(
    target,
    lineIdentity,
    target.reconciliation,
    packetId,
    responseValue,
    mutationParams,
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

function tryCreateDeclaredTargetMaterialization(
  target,
  targetParams,
  lineIdentity,
  responseValue,
) {
  if (
    lineIdentity.residency !== "declared"
    || target.reconciliation?.kind !== "replace"
    || target.reconciliation.materializeDeclaredTarget !== true
  ) {
    return null;
  }
  return target.materializeTargetMaterialization(targetParams, responseValue);
}

function createFallbackMutationResponseExecutionArtifact(
  target,
  lineIdentity,
  submittedTarget,
  staleness,
  partial,
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
      partial,
      detail:
        staleness?.detail
        ?? (partial?.kind === "missingResponseField"
          ? createMissingResponseFieldDetail(target, partial.field)
          : createMutationResponseFallbackDetail(target, lineIdentity)),
    }),
  });
}

function createExactReconciliationExecutionArtifact(
  target,
  lineIdentity,
  reconciliation,
  packetId,
  responseValue,
  mutationParams,
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
    itemId: readReconciliationItemId(reconciliation, responseValue, mutationParams),
    placement:
      reconciliation.kind === "insert" ? reconciliation.placement : null,
    field: "field" in reconciliation ? reconciliation.field : null,
    region: "region" in reconciliation ? reconciliation.region : null,
    path: "path" in reconciliation ? reconciliation.path : null,
    summary: "summary" in reconciliation ? reconciliation.summary : null,
    summaryScope:
      "summaryScope" in reconciliation ? reconciliation.summaryScope : null,
  });
}

function readReconciliationScope(reconciliation) {
  if (reconciliation.kind === "replace" || reconciliation.kind === "invalidate") {
    return "line";
  }
  if (reconciliation.kind === "insert" || reconciliation.kind === "delete") {
    return "item";
  }
  return reconciliation.kind;
}

function readResidentLineIdentity(targetMaterialization) {
  return Object.freeze({
    canonicalKey: targetMaterialization.lineIdentity.canonicalParams.canonicalKey,
    runtimeLineId: targetMaterialization.lineIdentity.runtimeLineId,
    residency: "resident",
  });
}

function readReconciliationItemId(reconciliation, responseValue, mutationParams) {
  if (
    reconciliation.kind !== "item"
    && reconciliation.kind !== "insert"
    && reconciliation.kind !== "delete"
  ) {
    return null;
  }
  return reconciliation.readItemId(responseValue, mutationParams);
}

function createMutationResponseTargetDelivery(
  reconciliation,
  packetId,
  currentBasisId,
  responseValue,
  mutationParams,
  extractedPatchValue,
) {
  if (reconciliation.kind === "replace") {
    return resourceDelivery.replace({
      packetId,
      basisId: null,
      nextBasisId: currentBasisId,
      nextValue: responseValue,
    });
  }
  if (reconciliation.kind === "invalidate") {
    return resourceDelivery.invalidate({
      packetId,
      basisId: null,
      nextBasisId: currentBasisId,
    });
  }
  return resourceDelivery.patch({
    packetId,
    basisId: null,
    nextBasisId: currentBasisId,
    patch: reconciliation.createPatch(
      responseValue,
      mutationParams,
      extractedPatchValue,
    ),
  });
}

function extractMutationResponsePatchValue(reconciliation, responseValue) {
  if (typeof reconciliation.extractPatchValue !== "function") {
    return Object.freeze({
      kind: "notNeeded",
      value: null,
    });
  }
  return reconciliation.extractPatchValue(responseValue);
}

function createMissingResponseFieldPartialArtifact(field) {
  return Object.freeze({
    kind: "missingResponseField",
    field,
    digest: `mutation-response-partial|missing-field:${field}`,
  });
}

function createMissingResponseFieldDetail(target, field) {
  return [
    `${target.family.kind} ${target.family.familyId}`,
    `target ${target.targetId}`,
    `stays in partialReconciliation posture because mutation response field "${field}" was absent`,
  ].join(" ");
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
    || kind === "exactDetailInvalidation"
    || kind === "exactCollectionItem"
    || kind === "exactCollectionTombstone"
    || kind === "exactCollectionInsert"
    || kind === "exactCollectionDelete"
    || kind === "exactSummary"
  );
}

export {
  createMutationResponseFallbackDetail,
  createMutationResponseTargetExecution,
  isExactMutationResponseExecutionKind,
};
