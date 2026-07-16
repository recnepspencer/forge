import { resolveResourceEffectBranchPosture } from "./resource_effect_branch_posture.js";

const RESOURCE_EFFECT_PLAN_BRAND = Symbol("WorthSignal.resourceEffectPlan");

function createLocalPatchEffectPlan(
  materialization,
  previousDiagnostics,
  inverseDescriptor,
  patch,
  requestMetadata = {},
) {
  const metadata = normalizeLocalEffectRequestMetadata(requestMetadata);
  return createResourceEffectPlan({
    materialization,
    requestDescriptor: materialization.requestState.readDescriptor(),
    previousDiagnostics,
    admissionKind: "localPatch",
    provenance: "localPatch",
    sequence: materialization.issueEffectAdmissionSequence(),
    idempotencyKey: metadata.idempotencyKey,
    serverCorrelationId: metadata.serverCorrelationId,
    previousEffect: null,
    inverseDescriptor,
    dependencies: patch.dependencies ?? [],
    dependencyCloseoutPolicy:
      patch.dependencyCloseoutPolicy ?? "independent",
  });
}

function normalizeLocalEffectRequestMetadata(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) {
    throw new TypeError("resource line patch options must be an object");
  }
  const idempotencyKey = value.idempotencyKey ?? null;
  const serverCorrelationId = value.serverCorrelationId ?? idempotencyKey;
  for (const [name, candidate] of [
    ["idempotencyKey", idempotencyKey],
    ["serverCorrelationId", serverCorrelationId],
  ]) {
    if (candidate !== null && (
      typeof candidate !== "string" || candidate.length === 0
    )) {
      throw new TypeError(`resource line patch ${name} must be a non-empty string`);
    }
  }
  return Object.freeze({ idempotencyKey, serverCorrelationId });
}

function createDeliveryEffectPlan(
  materialization,
  requestDescriptor,
  previousDiagnostics,
  delivery,
) {
  const deliveryKind = delivery.deliveryKind ?? delivery.kind;
  return createResourceEffectPlan({
    materialization,
    requestDescriptor,
    previousDiagnostics,
    admissionKind: "delivery",
    provenance: deliveryProvenance(deliveryKind),
    sequence: previousDiagnostics.deliveryCount + 1,
    idempotencyKey: delivery.packetId,
    serverCorrelationId: delivery.packetId,
    previousEffect: previousDiagnostics.lastEffect,
  });
}

function requireResourceEffectPlan(value) {
  if (!value || value[RESOURCE_EFFECT_PLAN_BRAND] !== "resourceEffectPlan") {
    throw new TypeError(
      "resource effect envelopes require a lowered resource effect plan",
    );
  }
  return value;
}

function createResourceEffectPlan(options) {
  const lineIdentity = options.materialization.lineIdentity;
  const requestDescriptor = options.requestDescriptor;
  const branchPosture = resolveResourceEffectBranchPosture({
    materialization: options.materialization,
    requestDescriptor,
    admissionKind: options.admissionKind,
    inverseDescriptor: options.inverseDescriptor ?? null,
    dependencies: Object.freeze([...(options.dependencies ?? [])]),
    dependencyCloseoutPolicy:
      options.dependencyCloseoutPolicy ?? "independent",
  });
  return Object.freeze({
    [RESOURCE_EFFECT_PLAN_BRAND]: "resourceEffectPlan",
    planId: createEffectPlanId(options.provenance, lineIdentity, options),
    admissionKind: options.admissionKind,
    provenance: options.provenance,
    causalSequence: createCausalSequence(options, lineIdentity),
    retryLineageId: createRetryLineageId(options, lineIdentity),
    idempotencyKey: options.idempotencyKey,
    serverCorrelationId: options.serverCorrelationId,
    previousEffect: options.previousEffect,
    lineIdentity,
    requestDescriptor,
    branchPosture,
    dependencies: Object.freeze([...(options.dependencies ?? [])]),
    dependencyCloseoutPolicy:
      options.dependencyCloseoutPolicy ?? "independent",
    responseLensProof: options.materialization.patch.responseLensProof,
    inverseDescriptor: options.inverseDescriptor ?? null,
    counters: Object.freeze({
      patchCountBefore: options.previousDiagnostics.patchCount,
      deliveryCountBefore: options.previousDiagnostics.deliveryCount,
      basisAdvanceCountBefore: options.previousDiagnostics.basis.advanceCount,
      planningBreadth: 1,
      executionBreadth: 1,
      branchProofBreadth: branchPosture.proofBreadth,
      branchLifecycleBreadth: 1,
      optimisticLifecycleBreadth: 1,
      serverConfirmationBreadth: options.admissionKind === "delivery" ? 1 : 0,
      rollbackReadinessBreadth: 1,
      responseLensBreadth:
        options.materialization.patch.responseLensProof === null ? 0 : 1,
      effectLocusBreadth: 1,
    }),
  });
}

function createEffectPlanId(provenance, lineIdentity, options) {
  return [
    lineIdentity.family.familyId,
    lineIdentity.canonicalParams.canonicalKey,
    provenance,
    String(options.sequence),
  ].join(":");
}

function createCausalSequence(options, lineIdentity) {
  return [
    lineIdentity.runtimeLineId,
    options.provenance,
    String(options.sequence),
  ].join("#");
}

function createRetryLineageId(options, lineIdentity) {
  return options.idempotencyKey === null
    ? null
    : [
        lineIdentity.family.familyId,
        lineIdentity.canonicalParams.canonicalKey,
        options.idempotencyKey,
      ].join(":");
}

function deliveryProvenance(deliveryKind) {
  switch (deliveryKind) {
    case "patch":
      return "deliveredPatch";
    case "replace":
      return "deliveredReplace";
    case "invalidate":
      return "deliveryInvalidate";
    case "basisRefresh":
      return "deliveryBasisRefresh";
    default:
      throw new TypeError(
        `resource effect plan cannot classify delivery kind "${deliveryKind}"`,
      );
  }
}

export {
  createDeliveryEffectPlan,
  createLocalPatchEffectPlan,
  requireResourceEffectPlan,
};
